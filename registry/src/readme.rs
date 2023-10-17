use std::path::Path;

use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    errors::{ExtensionRegistryError, Result},
    repository::Registry,
};

fn is_markdown(file_name: &str) -> bool {
    let maybe_extension = Path::new(file_name).extension();

    maybe_extension.map(|ext| ext == "md").unwrap_or(
        // If the extension is missing, the README might not be Markdown
        // but is likely simple enough that rendering it as Markdown would look better
        // than rendering it as HTML, e.g. Postgres' readme
        true,
    )
}

mod b64 {
    use crate::errors::Result;
    use std::ops::Not;

    /// GitHub base64 contains whitespace, which the base64 crate does not support
    ///
    /// Due to this behavior, this function will first strip out any ASCII whitespace
    /// and then decode the given base64
    pub fn ws_decode<V: Into<Vec<u8>>>(encoded: V) -> Result<Vec<u8>> {
        use base64::{engine::general_purpose::STANDARD, Engine};

        let mut encoded = encoded.into();
        encoded.retain(|ch| ch.is_ascii_whitespace().not());

        STANDARD.decode(encoded).map_err(Into::into)
    }
}

pub struct GithubApiClient {
    token: String,
    client: Client,
}

impl GithubApiClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    async fn get_text(&self, url: &str) -> Result<String> {
        self.client
            .get(url)
            .header("Accept", "application/vnd.github.html")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "request")
            .bearer_auth(&self.token)
            .send()
            .await?
            .text()
            .await
            .map_err(Into::into)
    }

    async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.client
            .get(url)
            .header("Accept", "application/vnd.github.json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "request")
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn fetch_readme(&self, repository_url: &str) -> Result<String> {
        let project = GitHubProject::parse_url(repository_url)?;

        let readme_url = project.build_readme_url();

        #[derive(Deserialize)]
        struct Response {
            name: String,
            content: String,
        }

        let Response {
            name: file_name,
            content,
        } = self.get_json(&readme_url).await?;

        if is_markdown(&file_name) {
            // We already got the README in Markdown, although base64-encoded
            String::from_utf8(b64::ws_decode(content)?).map_err(Into::into)
        } else {
            self.get_text(&readme_url).await
        }
    }
}

#[derive(Debug, PartialEq)]
struct GitHubProject<'a> {
    owner: &'a str,
    name: &'a str,
    subdir: Option<&'a str>,
}

impl<'a> GitHubProject<'a> {
    pub fn parse_url(url: &'a str) -> Result<Self> {
        let parse = |url: &'a str| {
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
        };

        parse(url).ok_or_else(|| ExtensionRegistryError::InvalidGithubRepo(url.into()))
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

pub async fn fetch_and_save_readme(
    client: &GithubApiClient,
    registry: &Registry,
    extension_name: &str,
) -> Result {
    let (extension_id, extension_url) = registry.get_repository_url(extension_name).await?;

    let url = extension_url.ok_or(ExtensionRegistryError::ResourceNotFound)?;

    let readme = client.fetch_readme(&url).await?;
    registry.upsert_readme(extension_id, &readme).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::readme::GitHubProject;

    #[test]
    fn parses_github_urls() {
        let pgmq = "https://github.com/tembo-io/pgmq";
        let auth_delay = "https://github.com/postgres/postgres/tree/master/contrib/auth_delay";

        assert_eq!(
            GitHubProject::parse_url(pgmq).unwrap(),
            GitHubProject {
                owner: "tembo-io",
                name: "pgmq",
                subdir: None
            }
        );
        assert_eq!(
            GitHubProject::parse_url(auth_delay).unwrap(),
            GitHubProject {
                owner: "postgres",
                name: "postgres",
                subdir: Some("auth_delay")
            }
        );
    }

    #[test]
    fn builds_readme_urls() {
        let pgmq = "https://github.com/tembo-io/pgmq";
        let auth_delay = "https://github.com/postgres/postgres/tree/master/contrib/auth_delay";

        assert_eq!(
            GitHubProject::parse_url(pgmq).unwrap().build_readme_url(),
            "https://api.github.com/repos/tembo-io/pgmq/readme"
        );
        assert_eq!(
            GitHubProject::parse_url(auth_delay)
                .unwrap()
                .build_readme_url(),
            "https://api.github.com/repos/postgres/postgres/readme"
        );
    }
}
