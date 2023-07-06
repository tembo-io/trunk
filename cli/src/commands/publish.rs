use super::SubCommand;
use crate::commands::categories::VALID_CATEGORY_SLUGS;
use crate::commands::publish::PublishError::InvalidExtensionName;
use crate::config;
use crate::config::{
    get_from_trunk_toml_if_not_set_on_cli, get_string_vec_from_trunk_toml_if_not_set_on_cli,
};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use clap::Args;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs};
use tokio_task_manager::Task;

#[derive(Args)]
pub struct PublishCommand {
    name: Option<String>,
    #[arg(long = "version", short = 'v')]
    version: Option<String>,
    #[arg(long = "file", short = 'f')]
    file: Option<PathBuf>,
    #[arg(long = "description", short = 'd')]
    description: Option<String>,
    #[arg(long = "documentation", short = 'D')]
    documentation: Option<String>,
    #[arg(long = "homepage", short = 'H')]
    homepage: Option<String>,
    #[arg(long = "license", short = 'l')]
    license: Option<String>,
    #[arg(long = "registry", short = 'r')]
    registry: Option<String>,
    #[arg(long = "repository", short = 'R')]
    repository: Option<String>,
    #[arg(long = "category", short = 'c')]
    category: Option<Vec<String>>,
}

#[derive(thiserror::Error, Debug)]
pub enum PublishError {
    #[error("extension name can include alphanumeric characters and underscores")]
    InvalidExtensionName,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub name: String,
    pub description: String,
    pub slug: String,
}

pub struct PublishSettings {
    name: String,
    version: String,
    file: Option<PathBuf>,
    description: Option<String>,
    documentation: Option<String>,
    homepage: Option<String>,
    license: Option<String>,
    registry: String,
    repository: Option<String>,
    categories: Option<Vec<String>>,
}

impl PublishCommand {
    fn settings(&self) -> Result<PublishSettings, anyhow::Error> {
        let path = ".".to_string();
        let trunkfile_path = Path::new(&path.clone()).join("Trunk.toml");
        let trunk_toml = match File::open(trunkfile_path) {
            Ok(file) => config::parse_trunk_toml(file),
            Err(_e) => {
                println!("Trunk.toml not found");
                Ok(None)
            }
        }?;

        let name = match self.name.clone() {
            Some(name) => name,
            None => match get_from_trunk_toml_if_not_set_on_cli(
                None,
                trunk_toml.clone(),
                "extension",
                "name",
            ) {
                Some(trunk_toml_name) => trunk_toml_name,
                None => panic!(
                    "Extension name must be provided when publishing. Please specify the extension name \
                     as the first argument, or under extension.name in Trunk.toml"
                ),
            },
        };

        let version = match self.version.clone() {
            Some(version) => version,
            None => match get_from_trunk_toml_if_not_set_on_cli(
                None,
                trunk_toml.clone(),
                "extension",
                "version",
            ) {
                Some(trunk_toml_version) => trunk_toml_version,
                None => panic!(
                    "Extension version must be provided when publishing. Please specify the extension version \
                     with --version, or under extension.version in Trunk.toml"
                ),
            },
        };

        // file
        let file = match self.file.clone() {
            Some(file) => Some(file),
            None => match trunk_toml.clone() {
                Some(table) => match table.get("extension") {
                    Some(extension) => match extension.get("file") {
                        Some(value) => {
                            let result = value.as_str().unwrap_or_else(|| {
                                panic!("Trunk.toml: extension.file should be a string")
                            });
                            let mut path = PathBuf::new();
                            let _ = &path.push(result);
                            println!(
                                "Trunk.toml: using setting extension.file: {}",
                                path.to_str().unwrap()
                            );
                            Some(path)
                        }
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
        };

        // description
        let description = get_from_trunk_toml_if_not_set_on_cli(
            self.description.clone(),
            trunk_toml.clone(),
            "extension",
            "description",
        );

        // documentation
        let documentation = get_from_trunk_toml_if_not_set_on_cli(
            self.documentation.clone(),
            trunk_toml.clone(),
            "extension",
            "documentation",
        );

        // homepage
        let homepage = get_from_trunk_toml_if_not_set_on_cli(
            self.homepage.clone(),
            trunk_toml.clone(),
            "extension",
            "homepage",
        );

        // license
        let license = get_from_trunk_toml_if_not_set_on_cli(
            self.license.clone(),
            trunk_toml.clone(),
            "extension",
            "license",
        );

        // registry
        let registry = match self.registry.clone() {
            Some(registry) => registry,
            None => match get_from_trunk_toml_if_not_set_on_cli(
                None,
                trunk_toml.clone(),
                "extension",
                "registry",
            ) {
                Some(trunk_toml_registry) => trunk_toml_registry,
                None => "https://registry.pgtrunk.io".to_string(),
            },
        };

        // repository
        let repository = get_from_trunk_toml_if_not_set_on_cli(
            self.repository.clone(),
            trunk_toml.clone(),
            "extension",
            "repository",
        );

        // categories
        let categories = get_string_vec_from_trunk_toml_if_not_set_on_cli(
            self.category.clone(),
            trunk_toml.clone(),
            "extension",
            "categories",
        );

        Ok(PublishSettings {
            version,
            file,
            description,
            documentation,
            homepage,
            license,
            registry,
            repository,
            name,
            categories,
        })
    }
}

#[async_trait]
impl SubCommand for PublishCommand {
    async fn execute(&self, _task: Task) -> Result<(), anyhow::Error> {
        let publish_settings = self.settings()?;
        // Validate extension name input
        let mut slugs = Vec::new();

        check_input(&publish_settings.name)?;
        // Validate categories input if provided
        if publish_settings.categories.is_some() {
            let response =
                reqwest::get(&format!("{}/categories/all", publish_settings.registry)).await?;
            match response.status() {
                StatusCode::OK => {
                    let response_body = response.text().await?;
                    let resp: Vec<Category> = serde_json::from_str(&response_body)?;
                    // Collect list of valid category slugs
                    for r in resp {
                        slugs.push(r.slug);
                    }
                }
                _ => {
                    // Fall back to local file if we fail to fetch valid slugs from registry
                    println!("Error fetching valid category slugs from {}/categories/all. Falling back to local definitions in categories.rs", publish_settings.registry);
                    slugs = VALID_CATEGORY_SLUGS
                        .to_vec()
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect();
                }
            }

            let categories = publish_settings.categories.clone().unwrap();
            for category in categories {
                if !slugs.contains(&category) {
                    return Err(anyhow!("Invalid category slug: {}. \nValid category slugs: {:?} \nMore details can be found at {}/categories/all", category, slugs, publish_settings.registry));
                }
            }
        }

        let (file, name) = match &publish_settings.file {
            Some(..) => {
                // If file is specified, use it
                let path = publish_settings.file.clone().unwrap();
                let name = path.file_name().unwrap().to_str().unwrap().to_owned();
                let f = fs::read(publish_settings.file.clone().unwrap())
                    .context(format!("Could not find file '{}'", path.to_str().unwrap()))?;
                (f, name)
            }
            None => {
                // If no file is specified, read file from working dir with format
                // .trunk/<extension_name>-<version>.tar.gz.
                // Error if file is not found
                let mut path = PathBuf::new();
                let _ = &path.push(format!(
                    ".trunk/{}-{}.tar.gz",
                    publish_settings.name, publish_settings.version
                ));
                let name = path.file_name().unwrap().to_str().unwrap().to_owned();
                let f = fs::read(path.clone())
                    .context(format!("Could not find file '{}'", path.to_str().unwrap()))?;
                (f, name)
            }
        };
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
        // Add token header from env var
        let auth = env::var("TRUNK_API_TOKEN").expect("TRUNK_API_TOKEN not set");
        headers.insert(AUTHORIZATION, auth.parse()?);
        let file_part = reqwest::multipart::Part::bytes(file)
            .file_name(name)
            .headers(headers.clone());
        let m = json!({
            "name": publish_settings.name,
            "vers": publish_settings.version,
            "description": publish_settings.description,
            "documentation": publish_settings.documentation,
            "homepage": publish_settings.homepage,
            "license": publish_settings.license,
            "repository": publish_settings.repository,
            "categories": publish_settings.categories
        });
        let metadata = reqwest::multipart::Part::text(m.to_string()).headers(headers);
        let form = reqwest::multipart::Form::new()
            .part("metadata", metadata)
            .part("file", file_part);
        let client = reqwest::Client::new();
        let url = format!("{}/extensions/new", publish_settings.registry);
        let res = client
            .post(url)
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;
        // Print response from registry
        println!("{res}");
        Ok(())
    }
}

pub fn check_input(input: &String) -> Result<(), PublishError> {
    let valid = input
        .as_bytes()
        .iter()
        .all(|&c| c.is_ascii_alphanumeric() || c == b'_');
    match valid {
        true => Ok(()),
        false => Err(InvalidExtensionName),
    }
}
