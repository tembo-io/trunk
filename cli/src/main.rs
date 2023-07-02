mod commands;
mod manifest;
mod sync_utils;

use crate::commands::SubCommand;
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio_task_manager::{Task, TaskManager};
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
        let trunk_toml = parse_trunk_toml()?;

        match self {
            SubCommands::Build(cmd) => cmd.execute(task, trunk_toml).await,
            SubCommands::Publish(cmd) => cmd.execute(task, trunk_toml).await,
            SubCommands::Install(cmd) => cmd.execute(task, trunk_toml).await,
        }
    }
}

fn parse_trunk_toml() -> Result<Option<Table>, anyhow::Error> {
    let trunk_toml: Result<Option<Table>, toml::de::Error> =
        match std::fs::read_to_string("Trunk.toml") {
            Ok(body) => match toml::from_str::<Table>(&body) {
                Ok(table) => Ok(Some(table)),
                Err(e) => {
                    println!("Trunk.toml is not valid toml");
                    Err(e)
                }
            },
            Err(_) => {
                println!("Trunk.toml not found");
                Ok(None)
            }
        };
    if trunk_toml.is_err() {
        return Err(trunk_toml.unwrap_err().into());
    }
    return Ok(trunk_toml.unwrap());
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
