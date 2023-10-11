use reqwest::Client;

use crate::{
    errors::{ExtensionRegistryError, Result},
    repository::Registry,
};

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

    pub async fn fetch_readme(&self, project_url: &str) -> Result<String> {
        // TODO: deal with error
        let project = GitHubProject::parse_url(project_url).unwrap();

        let readme_url = project.build_readme_url();

        self.client
            .get(readme_url)
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
}

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
