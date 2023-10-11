use reqwest::Client;
use sqlx::{Pool, Postgres};

use crate::{
    errors::{ExtensionRegistryError, Result},
    extensions::get_extension_id,
    repository::{self, Registry},
};

#[derive(Debug, PartialEq)]
struct GitHubProject<'a> {
    owner: &'a str,
    name: &'a str,
    subdir: Option<&'a str>,
}

impl<'a> GitHubProject<'a> {
    pub fn parse_url(url: &'a str) -> Option<Self> {
        let remaining = url.strip_prefix("https://github.com/")?;

        let mut parts = remaining.split('/');
        let owner = parts.next()?;
        let name = parts.next()?;
        let subdir = if let Some("tree") = parts.next() {
            parts.last()
        } else {
            None
        };

        Some(Self {
            owner,
            name,
            subdir,
        })
    }

    fn build_readme_url(&self) -> String {
        let Self {
            owner,
            name,
            subdir,
        } = *self;

        match subdir {
            Some(subdir) if owner != "postgres" => {
                format!("https://api.github.com/repos/{owner}/{name}/readme/{subdir}")
            }
            _ => format!("https://api.github.com/repos/{owner}/{name}/readme"),
        }
    }
}

async fn fetch_readme(http_client: &Client, project_url: &str) -> Result<String> {
    // TODO: deal with error
    let project = GitHubProject::parse_url(project_url).unwrap();
    let token = "todo";

    let readme_url = project.build_readme_url();

    http_client
        .get(readme_url)
        .header("Accept", "application/vnd.github.html")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "request")
        .bearer_auth(&token)
        .send()
        .await?
        .text()
        .await
        .map_err(Into::into)
}

pub async fn fetch_and_save_readme(
    http_client: &Client,
    registry: &Registry,
    extension_name: &str,
) -> Result {
    let (extension_id, extension_url) = registry
        .get_repository_url(extension_name)
        .await?;

    let url = extension_url.ok_or(ExtensionRegistryError::ResourceNotFound)?;

    let readme = fetch_readme(http_client, &url).await?;
    registry.save_readme(extension_id, &readme).await?;

    Ok(())
}
