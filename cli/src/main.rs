mod commands;
mod config;
mod manifest;
mod sync_utils;
mod trunk_toml;

use crate::commands::SubCommand;
use async_trait::async_trait;
use clap::{Parser, Subcommand};

use std::{process::ExitCode, time::Duration};
use tokio_task_manager::{Task, TaskManager};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = false)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Build a PGRX or C based Postgres extension
    Build(commands::build::BuildCommand),
    /// Publish a Postgres extension to the Trunk registry
    Publish(commands::publish::PublishCommand),
    /// Install a Postgres extension from the Trunk registry
    Install(commands::install::InstallCommand),
    /// Test a Postgres extension (coming soon)
    Test(commands::test::TestCommand),
}

#[async_trait]
impl SubCommand for SubCommands {
    async fn execute(&self, task: Task) -> Result<(), anyhow::Error> {
        match self {
            SubCommands::Build(cmd) => cmd.execute(task).await,
            SubCommands::Publish(cmd) => cmd.execute(task).await,
            SubCommands::Install(cmd) => cmd.execute(task).await,
            SubCommands::Test(cmd) => cmd.execute(task).await,
        }
    }
}

fn main() -> ExitCode {
    let tm = TaskManager::new(Duration::from_secs(60));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let res = rt.block_on(async {
        let cli = Cli::parse();
        let result = cli.command.execute(tm.task()).await;
        tm.wait().await;
        result
    });

    if let Err(err) = res {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}
