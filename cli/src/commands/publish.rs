use super::SubCommand;
use crate::commands::publish::PublishError::InvalidExtensionName;
use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::{env, fs};
use tokio_task_manager::Task;

#[derive(Args)]
pub struct PublishCommand {
    name: String,
    #[arg(long = "version", short = 'v')]
    version: String,
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
    #[arg(
        long = "registry",
        short = 'r',
        default_value = "https://registry.pgtrunk.io"
    )]
    registry: String,
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

#[async_trait]
impl SubCommand for PublishCommand {
    async fn execute(&self, _task: Task) -> Result<(), anyhow::Error> {
        // Validate extension name input
        check_input(&self.name)?;
        // Validate categories input if provided
        if self.category.is_some() {
            let response = reqwest::get(&format!("{}/categories/all", self.registry)).await?;

            // Fall back to local file if network issue

            let response_body = response.text().await?;
            let resp: Vec<Category> = serde_json::from_str(&response_body)?;

            // Collect list of valid category slugs
            let mut slugs = Vec::new();
            for r in resp {
                slugs.push(r.slug);
            }
            let categories = self.category.clone().unwrap();
            for category in categories {
                if !slugs.contains(&category) {
                    return Err(anyhow!("invalid category slug: {}. valid category slugs can be found at {}/categories/all", category, self.registry));
                }
            }
        }

        let (file, name) = match &self.file {
            Some(..) => {
                // If file is specified, use it
                let path = self.file.clone().unwrap();
                let name = path.file_name().unwrap().to_str().unwrap().to_owned();
                let f = fs::read(self.file.clone().unwrap())?;
                (f, name)
            }
            None => {
                // If no file is specified, read file from working dir with format
                // <extension_name>-<version>.tar.gz.
                // Error if file is not found
                let mut path = PathBuf::new();
                let _ = &path.push(format!("./{}-{}.tar.gz", self.name, self.version));
                let name = path.file_name().unwrap().to_str().unwrap().to_owned();
                let f = fs::read(path.clone())?;
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
            "name": self.name,
            "vers": self.version,
            "description": self.description,
            "documentation": self.documentation,
            "homepage": self.homepage,
            "license": self.license,
            "repository": self.repository,
            "categories": self.category
        });
        let metadata = reqwest::multipart::Part::text(m.to_string()).headers(headers);
        let form = reqwest::multipart::Form::new()
            .part("metadata", metadata)
            .part("file", file_part);
        let client = reqwest::Client::new();
        let url = format!("{}/extensions/new", self.registry);
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

pub fn check_input(input: &str) -> Result<(), PublishError> {
    let valid = input
        .as_bytes()
        .iter()
        .all(|&c| c.is_ascii_alphanumeric() || c == b'_');
    match valid {
        true => Ok(()),
        false => Err(InvalidExtensionName),
    }
}
