use super::SubCommand;
use crate::commands::categories::VALID_CATEGORY_SLUGS;
use crate::commands::publish::PublishError::InvalidExtensionName;
use crate::config;
use crate::manifest::Manifest;
use crate::trunk_toml::{cli_or_trunk, cli_or_trunk_opt, SystemDependencies};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use clap::Args;
use flate2::read::GzDecoder;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::Seek;
use std::path::{Path, PathBuf};
use std::{env, fs};
use tar::{Archive, EntryType};
use tokio_task_manager::Task;

#[derive(Args)]
pub struct PublishCommand {
    name: Option<String>,
    #[arg(short = 'e', long = "extension_name")]
    extension_name: Option<String>,
    #[arg(short = 's', long = "shared_preload_libraries")]
    shared_preload_libraries: Option<Vec<String>>,
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
    extension_name: Option<String>,
    shared_preload_libraries: Option<Vec<String>>,
    version: String,
    file: Option<PathBuf>,
    description: Option<String>,
    documentation: Option<String>,
    homepage: Option<String>,
    license: Option<String>,
    registry: String,
    repository: Option<String>,
    system_dependencies: Option<SystemDependencies>,
    categories: Option<Vec<String>>,
}

impl PublishCommand {
    fn settings(&self) -> Result<PublishSettings, anyhow::Error> {
        // The file path of the extension to publish
        let publish_path = ".";
        let trunkfile_path = Path::new(&publish_path).join("Trunk.toml");

        let trunk_toml = match File::open(trunkfile_path) {
            Ok(file) => Some(config::parse_trunk_toml(file)?),
            Err(_e) => {
                println!("Trunk.toml not found");
                None
            }
        };

        let maybe_name = cli_or_trunk(&self.name, |toml| &toml.extension.name, &trunk_toml);
        let Some(name) = maybe_name else {
            panic!(
                "Extension name must be provided when publishing. Please specify the extension name \
                 as the first argument, or under extension.name in Trunk.toml"
            )
        };

        let extension_name = cli_or_trunk_opt(
            &self.extension_name,
            |toml| &toml.extension.extension_name,
            &trunk_toml,
        );

        let shared_preload_libraries = cli_or_trunk_opt(
            &self.shared_preload_libraries,
            |toml| &toml.extension.shared_preload_libraries,
            &trunk_toml,
        );

        let maybe_version =
            cli_or_trunk(&self.version, |toml| &toml.extension.version, &trunk_toml);
        let Some(version) = maybe_version else {
            panic!(
                "Extension version must be provided when publishing. Please specify the extension version \
                 with --version, or under extension.version in Trunk.toml"
            )
        };

        let file = self
            .file
            .as_ref()
            .or_else(|| {
                trunk_toml
                    .as_ref()
                    .map(|toml| {
                        let file = toml.extension.file.as_ref()?;
                        println!(
                            "Trunk.toml: using setting `extension.file`: {}",
                            file.display()
                        );
                        Some(file.into())
                    })
                    .flatten()
            })
            .cloned();

        let description = cli_or_trunk_opt(
            &self.description,
            |toml| &toml.extension.description,
            &trunk_toml,
        );
        let documentation = cli_or_trunk_opt(
            &self.documentation,
            |toml| &toml.extension.documentation,
            &trunk_toml,
        );
        let homepage =
            cli_or_trunk_opt(&self.homepage, |toml| &toml.extension.homepage, &trunk_toml);

        let license = cli_or_trunk(&self.license, |toml| &toml.extension.license, &trunk_toml);

        let registry =
            cli_or_trunk_opt(&self.registry, |toml| &toml.extension.registry, &trunk_toml)
                .unwrap_or_else(|| "https://registry.pgtrunk.io".to_string());

        let repository = cli_or_trunk_opt(
            &self.repository,
            |toml| &toml.extension.repository,
            &trunk_toml,
        );

        let categories = cli_or_trunk(
            &self.category,
            |toml| &toml.extension.categories,
            &trunk_toml,
        );

        let system_dependencies = trunk_toml
            .as_ref()
            .map(|toml| toml.dependencies.as_ref())
            .flatten()
            .cloned();

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
            extension_name,
            system_dependencies,
            categories,
            shared_preload_libraries,
        })
    }
}

#[async_trait]
impl SubCommand for PublishCommand {
    async fn execute(&self, _task: Task) -> Result<(), anyhow::Error> {
        let mut publish_settings = self.settings()?;
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

        // TODO(ianstanton) DRY this up
        // If extension_name is not provided by the user, check for value in manifest.json
        if publish_settings.extension_name.is_none() {
            println!("Fetching extension_name from manifest.json...");
            // Get file path
            let file = match &publish_settings.file {
                Some(..) => {
                    // If file is specified, use it
                    publish_settings.file.clone().unwrap()
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
                    path
                }
            };

            let f = File::open(&file)?;
            let input = match &file
                .extension()
                .into_iter()
                .filter_map(|s| s.to_str())
                .next()
            {
                Some("gz") => {
                    // unzip the archive into a temporary file
                    let decoder = GzDecoder::new(f);
                    let mut tempfile = tempfile::tempfile()?;
                    use read_write_pipe::*;
                    tempfile.write_reader(decoder)?;
                    tempfile.rewind()?;
                    tempfile
                }
                Some("tar") => f,
                _ => return Err(InvalidExtensionName)?,
            };

            let mut archive = Archive::new(&input);
            let mut manifest: Option<Manifest> = None;
            {
                let entries = archive.entries_with_seek()?;
                for this_entry in entries {
                    let entry = this_entry?;
                    let fname = entry.path()?;
                    if entry.header().entry_type() == EntryType::file()
                        && fname.clone() == Path::new("manifest.json")
                    {
                        let manifest_json = serde_json::from_reader(entry)?;
                        let manifest_result = serde_json::from_value(manifest_json);
                        manifest.replace(manifest_result?);
                    }
                }
            }

            if let Some(manifest) = manifest {
                publish_settings.extension_name = Option::from(
                    manifest
                        .extension_name
                        .unwrap_or(publish_settings.name.clone()),
                );
                println!(
                    "Found extension_name: {}",
                    publish_settings.extension_name.clone().unwrap()
                );
            } else {
                println!(
                    "Did not find extension_name in manifest.json. Falling back to '{}'",
                    publish_settings.name.clone()
                );
            }
        }

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
            "extension_name": publish_settings.extension_name,
            "shared_preload_libraries": publish_settings.shared_preload_libraries,
            "vers": publish_settings.version,
            "description": publish_settings.description,
            "documentation": publish_settings.documentation,
            "homepage": publish_settings.homepage,
            "license": publish_settings.license,
            "repository": publish_settings.repository,
            "system_dependencies": publish_settings.system_dependencies,
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
