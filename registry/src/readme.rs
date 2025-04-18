use std::path::Path;

use aho_corasick::AhoCorasick;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    errors::{ExtensionRegistryError, Result},
    repository::Registry,
};

use self::markdown_parsing::{parse_markdown_links, MarkdownLink};

fn is_markdown(file_name: &str) -> bool {
    let maybe_extension = Path::new(file_name).extension();
    const MD_EXT: &[&str] = &["md", "mmd", "mdwn", "mdown", "txt", "text"];

    maybe_extension
        .map(|ext| MD_EXT.iter().any(|md| ext == *md))
        .unwrap_or(
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

    /// A Markdown image declaration of the form `![alt-text](path-or-link)`, if the
    /// link is an image, or `[Text](path-or-link)` if it's a regular link
    #[derive(Debug)]
    #[cfg_attr(test, derive(PartialEq))]
    pub struct MarkdownLink<'a> {
        #[allow(unused)]
        pub alt_text: &'a str,
        pub path_or_link: &'a str,
    }

    fn parse_delimited(input: &str, start: char, end: char) -> IResult<&str, &str> {
        delimited(char(start), take_while(|ch| ch != end), char(end))(input)
    }

    fn parse_markdown_link(input: &str) -> IResult<&str, MarkdownLink<'_>> {
        let (remaining, alt_text) = parse_delimited(input, '[', ']')?;
        let (remaining, path_or_link) = parse_delimited(remaining, '(', ')')?;

        Ok((
            remaining,
            MarkdownLink {
                alt_text,
                path_or_link,
            },
        ))
    }

    pub fn parse_markdown_links(input: &str) -> Vec<MarkdownLink<'_>> {
        let mut links = Vec::new();
        for (idx, _) in input.match_indices('[') {
            let remaining = &input[idx..];

            if let Ok((_, link)) = parse_markdown_link(remaining) {
                // We're only interested in relative paths, so if we parsed a URL, skip it.
                // Sometimes a README links to another subsection, so if the link or path starts with #, we'll skip over that as well.
                if link.path_or_link.starts_with("http") || link.path_or_link.starts_with('#') {
                    continue;
                }
                links.push(link);
            }
        }

        links
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
        let relative_links = parse_markdown_links(&readme);

        if relative_links.is_empty() {
            return readme;
        }

        match self
            .replace_relative_links(&readme, relative_links, project)
            .await
        {
            Ok(updated_readme) => updated_readme,
            Err(err) => {
                tracing::warn!("Failed to update README: {err}, continuing.");
                readme
            }
        }
    }

    async fn replace_relative_links<'a>(
        &self,
        readme: &'a str,
        links: Vec<MarkdownLink<'a>>,
        project: GitHubProject<'a>,
    ) -> anyhow::Result<String> {
        let mut patterns = Vec::new();
        let mut replace_with = Vec::new();

        #[derive(Deserialize)]
        struct Response {
            download_url: String,
        }

        for link in links {
            let path = link.path_or_link;
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
            let remaining = url
                .strip_prefix("https://github.com/")
                .or_else(|| url.strip_prefix("http://github.com/"))?;

            let mut parts = remaining.split('/');
            let owner = parts.next()?;
            let name = parts.next()?;
            let subdir = if let Some("tree") = parts.next() {
                parts.next_back()
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
    use super::markdown_parsing::{parse_markdown_links, MarkdownLink};
    use crate::readme::GitHubProject;

    #[test]
    fn parses_markdown_links() {
        let test_case = r#"
        | File                                                              | Description                                                                   |
        |-------------------------------------------------------------------|-------------------------------------------------------------------------------|
        | [pg_partman.md](doc/pg_partman.md)                                | Main reference documentation for pg_partman.                                  |
        | [pg_partman_howto.md](doc/pg_partman_howto.md)                    | A How-To guide for general usage of pg_partman. Provides examples for setting up new partition sets and migrating existing tables to partitioned tables.                                                    |
        | [migrate_to_partman.md](doc/migrate_to_partman.md)                | How to migrate existing partition sets to being managed by pg_partman.        |
        | [migrate_to_declarative.md](doc/migrate_to_declarative.md)        | How to migrate from trigger-based partitioning to declarative partitioning.   |
        | [pg_partman_5.0.1_upgrade.md](doc/pg_partman_5.0.1_upgrade.md)    | If pg_partman is being upgraded to version 5.x from any prior version, special considerations may need to be made. Please carefully review this document before performing any upgrades to 5.x or higher.   |
        | [fix_missing_procedures.md](doc/fix_missing_procedures.md)        | If pg_partman had been installed prior to PostgreSQL 11 and upgraded since then, it may be missing procedures. This document outlines how to restore those procedures and preserve the current configuration.
        "#;

        let links = [
            MarkdownLink {
                alt_text: "pg_partman.md",
                path_or_link: "doc/pg_partman.md",
            },
            MarkdownLink {
                alt_text: "pg_partman_howto.md",
                path_or_link: "doc/pg_partman_howto.md",
            },
            MarkdownLink {
                alt_text: "migrate_to_partman.md",
                path_or_link: "doc/migrate_to_partman.md",
            },
            MarkdownLink {
                alt_text: "migrate_to_declarative.md",
                path_or_link: "doc/migrate_to_declarative.md",
            },
            MarkdownLink {
                alt_text: "pg_partman_5.0.1_upgrade.md",
                path_or_link: "doc/pg_partman_5.0.1_upgrade.md",
            },
            MarkdownLink {
                alt_text: "fix_missing_procedures.md",
                path_or_link: "doc/fix_missing_procedures.md",
            },
        ];

        assert_eq!(parse_markdown_links(test_case), links);
    }

    #[test]
    fn parses_markdown_images() {
        use super::markdown_parsing::{parse_markdown_links, MarkdownLink};

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
            parse_markdown_links(test_case),
            &[
                MarkdownLink {
                    alt_text: "Citus Banner",
                    path_or_link: "images/citus-readme-banner.png"
                },
                MarkdownLink {
                    alt_text: "Tembo Banner",
                    path_or_link: "images/tembo-readme-banner.png"
                }
            ]
        );
    }

    #[test]
    fn parses_github_urls() {
        for (name, url, exp) in [
            (
                "pgmq",
                "https://github.com/tembo-io/pgmq",
                GitHubProject {
                    owner: "tembo-io",
                    name: "pgmq",
                    subdir: None,
                },
            ),
            (
                "auth_delay",
                "https://github.com/postgres/postgres/tree/master/contrib/auth_delay",
                GitHubProject {
                    owner: "postgres",
                    name: "postgres",
                    subdir: Some("auth_delay"),
                },
            ),
            (
                "http_only",
                "http://github.com/theory/pg-pair",
                GitHubProject {
                    owner: "theory",
                    name: "pg-pair",
                    subdir: None,
                },
            ),
        ] {
            assert_eq!(exp, GitHubProject::parse_url(url).unwrap(), "{name}");
        }
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

    #[test]
    fn is_markdown() {
        for bn in ["README", "readme", "hi", "go on", "😀😆"] {
            for ext in ["md", "mmd", "mdown", "mdwn", "txt", "text"] {
                assert!(crate::readme::is_markdown(&format!("{bn}.{ext}")));
            }
            for ext in ["ascii", "doc", "xml", "json", "rs", "go"] {
                assert!(!crate::readme::is_markdown(&format!("{bn}.{ext}")));
            }
        }
    }
}
