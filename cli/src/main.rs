mod commands;
mod manifest;
mod sync_utils;

use crate::commands::SubCommand;
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use tokio_task_manager::{Task, TaskManager};
use toml;
use toml::Table;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = false)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Build(commands::build::BuildCommand),
    Publish(commands::publish::PublishCommand),
    Install(commands::install::InstallCommand),
}

#[async_trait]
impl SubCommand for SubCommands {
    async fn execute(&self, task: Task, _trunk_toml: Option<Table>) -> Result<(), anyhow::Error> {
        let trunk_toml = match File::open("Trunk.toml") {
            Ok(file) => parse_trunk_toml(file),
            Err(e) => {
                println!("Trunk.toml not found");
                Ok(None)
            }
        }?;

        match self {
            SubCommands::Build(cmd) => cmd.execute(task, trunk_toml).await,
            SubCommands::Publish(cmd) => cmd.execute(task, trunk_toml).await,
            SubCommands::Install(cmd) => cmd.execute(task, trunk_toml).await,
        }
    }
}

fn parse_trunk_toml<R: Read>(mut reader: R) -> Result<Option<Table>, anyhow::Error> {
    let mut body = String::new();
    reader.read_to_string(&mut body)?;
    match toml::from_str::<Table>(&body) {
        Ok(table) => Ok(Some(table)),
        Err(e) => {
            println!("Trunk.toml is not valid toml");
            Err(e.into())
        }
    }
}

fn main() {
    let tm = TaskManager::new(Duration::from_secs(60));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let cli = Cli::parse();
        let result = cli.command.execute(tm.task(), None).await;
        tm.wait().await;
        result
    })
    .expect("error occurred");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_trunk_toml_valid() {
        let toml = r#"
        [extension]
        name = "pg_cron"
        version = "1.5.2"

        [build]
        dockerfile = "Dockerfile"
        install_command = "cd pg_cron && make install"
        "#;
        let result = parse_trunk_toml(toml.as_bytes()).unwrap();
        let table = result.expect("Expected a table");
        assert_eq!(table["extension"]["name"].as_str().unwrap(), "pg_cron");
        assert_eq!(table["extension"]["version"].as_str().unwrap(), "1.5.2");
        assert_eq!(table["build"]["dockerfile"].as_str().unwrap(), "Dockerfile");
        assert_eq!(
            table["build"]["install_command"].as_str().unwrap(),
            "cd pg_cron && make install"
        );
    }

    #[test]
    fn test_parse_trunk_toml_invalid() {
        let toml = "this is not valid toml";
        let result = parse_trunk_toml(toml.as_bytes());
        assert!(result.is_err());
    }
}
