use std::path::Path;

use aho_corasick::AhoCorasick;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    errors::{ExtensionRegistryError, Result},
    repository::Registry,
};

use self::markdown_parsing::{parse_markdown_images, MarkdownImage};

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

mod markdown_parsing {
    use nom::{
        bytes::complete::take_while, character::complete::char, sequence::delimited, IResult,
    };

    /// A Markdown image declaration of the form `![alt-text](path-or-link)`.
    #[derive(Debug)]
    #[cfg_attr(test, derive(PartialEq))]
    pub struct MarkdownImage<'a> {
        pub alt_text: &'a str,
        pub path_or_link: &'a str,
    }

    fn parse_delimited(input: &str, start: char, end: char) -> IResult<&str, &str> {
        delimited(char(start), take_while(|ch| ch != end), char(end))(input)
    }

    fn parse_markdown_image(input: &str) -> IResult<&str, MarkdownImage<'_>> {
        let (remaining, _) = char('!')(input)?;
        let (remaining, alt_text) = parse_delimited(remaining, '[', ']')?;
        let (remaining, path_or_link) = parse_delimited(remaining, '(', ')')?;

        Ok((
            remaining,
            MarkdownImage {
                alt_text,
                path_or_link,
            },
        ))
    }

    pub fn parse_markdown_images(input: &str) -> Vec<MarkdownImage<'_>> {
        let mut images = Vec::new();
        for (idx, _) in input.match_indices('!') {
            let remaining = &input[idx..];

            if let Ok((_, image)) = parse_markdown_image(remaining) {
                // We're only interested in relative paths, so if we parsed a URL, skip it
                if image.path_or_link.starts_with("http") {
                    continue;
                }
                images.push(image);
            }
        }

        images
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
            let readme = String::from_utf8(b64::ws_decode(content)?)?;

            // One problem now is that READMEs often link images to within the repository instead of some HTTP URL.
            // We will look for relative paths in the README and attempt to replace them with links that could be rendered in pgt.dev
            Ok(self.prepare_readme(readme, project).await)
        } else {
            self.get_text(&readme_url).await
        }
    }

    async fn prepare_readme(&self, readme: String, project: GitHubProject<'_>) -> String {
        let relative_images = parse_markdown_images(&readme);

        if relative_images.is_empty() {
            return readme;
        }

        match self
            .replace_relative_images(&readme, relative_images, project)
            .await
        {
            Ok(updated_readme) => updated_readme,
            Err(err) => {
                tracing::warn!("Failed to update README: {err}, continuing.");
                readme
            }
        }
    }

    async fn replace_relative_images<'a, 'b>(
        &self,
        readme: &'a str,
        images: Vec<MarkdownImage<'a>>,
        project: GitHubProject<'b>,
    ) -> anyhow::Result<String> {
        let mut patterns = Vec::new();
        let mut replace_with = Vec::new();

        #[derive(Deserialize)]
        struct Response {
            download_url: String,
        }

        for image in images {
            let path = image.path_or_link;
            let url = project.build_content_url(path);
            let Ok(response) = self.get_json::<Response>(&url).await else {
                continue;
            };

            patterns.push(path);
            replace_with.push(response.download_url);
        }

        let ac = AhoCorasick::new(patterns)?;

        ac.try_replace_all(readme, &replace_with)
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

    /// Builds the URL for these endpoints:
    /// * https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-repository-content
    /// * https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-a-repository-readme-for-a-directory
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

    /// Builds the URL for the following endpoint:
    /// * https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-repository-content
    fn build_content_url(&self, path: &str) -> String {
        let Self {
            owner,
            name,
            subdir: _,
        } = *self;

        format!("https://api.github.com/repos/{owner}/{name}/contents/{path}")
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
    fn parses_markdown_images() {
        use super::markdown_parsing::{parse_markdown_images, MarkdownImage};

        let test_case = r#"
        | **<br/>The Citus database is 100% open source.<br/><img width=1000/><br/>Learn what's new in the [Citus 12.1 release blog](https://www.citusdata.com/blog/2023/09/22/adding-postgres-16-support-to-citus-12-1/) and the [Citus Updates page](https://www.citusdata.com/updates/).<br/><br/>**|
        |---|
        <br/>



        ![Citus Banner](images/citus-readme-banner.png)

        [![Latest Docs](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.citusdata.com/)
        [![Stack Overflow](https://img.shields.io/badge/Stack%20Overflow-%20-545353?logo=Stack%20Overflow)](https://stackoverflow.com/questions/tagged/citus)
        [![Slack](https://cituscdn.azureedge.net/images/social/slack-badge.svg)](https://slack.citusdata.com/)
        ![Tembo Banner](images/tembo-readme-banner.png)
        [![Code Coverage](https://codecov.io/gh/citusdata/citus/branch/master/graph/badge.svg)](https://app.codecov.io/gh/citusdata/citus)
        [![Twitter](https://img.shields.io/twitter/follow/citusdata.svg?label=Follow%20@citusdata)](https://twitter.com/intent/follow?screen_name=citusdata)"#;

        assert_eq!(
            parse_markdown_images(test_case),
            &[
                MarkdownImage {
                    alt_text: "Citus Banner",
                    path_or_link: "images/citus-readme-banner.png"
                },
                MarkdownImage {
                    alt_text: "Tembo Banner",
                    path_or_link: "images/tembo-readme-banner.png"
                }
            ]
        );
    }

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
