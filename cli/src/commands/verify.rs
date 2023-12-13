use std::{
    fs::File,
    io::{BufWriter, Cursor, Read, Write},
    ops::Not,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use clap::Args;
use duct::cmd;
use flate2::bufread::GzDecoder;
use serde::Deserialize;
use similar::{ChangeTag, TextDiff};
use tar::{Entry, EntryType};
use tempfile::TempDir;
use tokio_task_manager::Task;

use super::SubCommand;
type Result<T = ()> = std::result::Result<T, anyhow::Error>;

#[derive(Args)]
pub struct VerifyCommand {
    /// The name of the extension to test
    extension_name: String,
    /// The psql connection string on which commands will be executed in
    connstring: String,
}

#[derive(Deserialize, Debug)]
struct TrunkProjectInfo {
    pub repository_link: String,
}

#[derive(Debug, Deserialize)]
pub struct Extension {
    pub extension_name: String,
}

#[derive(Debug)]
struct ExtractedTestCases {
    sql_files: Vec<PathBuf>,
    expected_files: Vec<PathBuf>,
}

#[derive(Default)]
struct TestSummary {
    lines_equal: u32,
    lines_differed: u32,
}

async fn extract_sql_and_expected_files(
    tempdir: &TempDir,
    github_project: GitHubProject<'_>,
) -> Result<ExtractedTestCases> {
    fn check_parent(expected_parent: &str, path: &Path) -> bool {
        let Some(parent_obtained) = path
            .parent()
            .map(|parent| parent.components().last())
            .flatten()
        else {
            return false;
        };

        parent_obtained.as_os_str() == expected_parent
    }

    fn create_ensuring_parent(path: &Path) -> Result<File> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        File::create(path).map_err(Into::into)
    }

    fn read_entry<R: Read>(tempdir: &TempDir, entry: &mut Entry<'_, R>) -> Result<PathBuf> {
        let path = tempdir.path().join(entry.path()?);
        let file = create_ensuring_parent(&path)?;

        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;

        BufWriter::new(file).write_all(&buf)?;

        Ok(path)
    }

    let mut sql_files = Vec::new();
    let mut expected_files = Vec::new();

    let tar_gz = reqwest::Client::new()
        .get(github_project.build_tarball_url())
        .header("User-Agent", "request")
        .send()
        .await?
        .bytes()
        .await?;

    let mut buf = Vec::with_capacity(tar_gz.len() * 8);
    GzDecoder::new(tar_gz.as_ref()).read_to_end(&mut buf)?;
    let mut archive = tar::Archive::new(Cursor::new(buf));

    for maybe_entry in archive.entries()? {
        let mut entry = maybe_entry?;
        let header = entry.header();

        if header.entry_type() != EntryType::Regular {
            continue;
        }

        let path = entry.path()?;

        if check_parent("sql", &path) {
            let path_written = read_entry(tempdir, &mut entry)?;
            sql_files.push(path_written);
        } else if check_parent("expected", &path) {
            let path_written = read_entry(tempdir, &mut entry)?;
            expected_files.push(path_written);
        } else {
            continue;
        }
    }

    Ok(ExtractedTestCases {
        sql_files,
        expected_files,
    })
}

#[async_trait::async_trait]
impl SubCommand for VerifyCommand {
    async fn execute(&self, _: Task) -> Result<()> {
        let tempdir = TempDir::new()?;
        // Given an extension name, let's fetch its Trunk project
        let trunk_project = {
            let url = format!(
                "https://registry.pgtrunk.io/api/v1/trunk-projects?extension-name={}",
                self.extension_name
            );
            let mut resp: Vec<TrunkProjectInfo> = reqwest::get(url).await?.json().await?;
            resp.pop()
                .with_context(|| "Found no Trunk project for this extension!")?
        };

        // We'll download the extension's repository from the GitHub API
        // and then extract its `sql/` and `expected/` folders.
        let github_project = GitHubProject::parse_url(&trunk_project.repository_link)?;
        let ExtractedTestCases {
            sql_files,
            expected_files,
        } = extract_sql_and_expected_files(&tempdir, github_project).await?;

        let mut summary = TestSummary::default();

        // Now, run every test file with psql and compare its output
        for expected_file in expected_files {
            let test_stem = expected_file
                .file_stem()
                .with_context(|| "Expected filename")?
                .to_str()
                .with_context(|| "Expected UTF-8")?;

            let Some(sql_path) = sql_files.iter().find(|path| {
                path.file_stem()
                    .map(|path| path == test_stem)
                    .unwrap_or(false)
            }) else {
                bail!("Found no matching SQL file for {:?}", expected_file);
            };

            let obtained = run_psql(&sql_path, &self.connstring)?;
            let obtained = obtained.lines().map(remove_psql_message);
            let expected = std::fs::read_to_string(expected_file)?;

            for (obtained_line, expected_line) in obtained.zip(expected.lines()) {
                let diff = TextDiff::from_lines(obtained_line, expected_line);

                for change in diff.iter_all_changes() {
                    let sign = match change.tag() {
                        ChangeTag::Delete => {
                            summary.lines_differed += 1;
                            "-"
                        }
                        ChangeTag::Insert => {
                            summary.lines_differed += 1;
                            "+"
                        }
                        ChangeTag::Equal => {
                            summary.lines_equal += 1;
                            " "
                        }
                    };
                    print!("{}{}", sign, change);
                }
            }
        }

        println!(
            "Summary: {} lines equal, {} lines differed, {} lines total.",
            summary.lines_equal,
            summary.lines_differed,
            summary.lines_equal + summary.lines_differed
        );

        Ok(())
    }
}

fn run_psql(script_path: &Path, connstring: &str) -> Result<String> {
    let output = cmd!(
        "psql",
        "--echo-errors",
        "--echo-all",
        "--quiet",
        "-f",
        script_path,
        connstring
    )
    .unchecked()
    .stderr_to_stdout()
    .read()?;

    let mut file = File::create("/home/vrmiguel/trunk-test.out")?;
    file.write_all(output.as_bytes())?;

    Ok(output)
}

#[derive(Debug, PartialEq)]
struct GitHubProject<'a> {
    owner: &'a str,
    name: &'a str,
    subdir: Option<&'a str>,
}

// FIXME: this is copied from trunk-registry, DRY
impl<'a> GitHubProject<'a> {
    pub fn parse_url(url: &'a str) -> Result<Self> {
        let parse = |url: &'a str| {
            let remaining = url.strip_prefix("https://github.com/")?;

            let mut parts = remaining.split('/');
            let owner = parts.next()?;
            let name = parts.next()?;
            let subdir = if let Some("tree") = parts.next() {
                // TODO: join instead of last?
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

        parse(url).with_context(|| "Failed to parse repository URL as a GitHub project")
    }

    /// Builds the URL for the following endpoint:
    /// * https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-repository-content
    fn build_tarball_url(&self) -> String {
        let Self {
            owner,
            name,
            subdir: _,
        } = *self;

        format!("https://api.github.com/repos/{owner}/{name}/tarball")
    }
}

pub fn remove_psql_message(input: &str) -> &str {
    if input.starts_with("psql").not() {
        return input;
    }

    let Some((pos, _)) = input.match_indices("ERROR").next() else {
        return input;
    };

    &input[pos..]
}
