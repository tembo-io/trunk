mod commands;
mod config;
mod control_file;
mod manifest;
mod sync_utils;
mod trunk_toml;

use crate::commands::SubCommand;
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use log::Level;
use log::error;

use std::time::Duration;
use tokio_task_manager::{Task, TaskManager};
use env_logger;
use colorful::{Color, Colorful, RGB};
use std::io::Write;

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

fn main() {
    env_logger::builder()
		.filter_level(log::LevelFilter::Info)
		.format(|buf, record| {
            let level_str = match record.level() {
                Level::Info => String::from("info").color(RGB::new(255, 247, 240)),
                Level::Error => String::from("error").color(Color::Red),
                Level::Warn => String::from("warn").color(Color::Yellow),
                Level::Debug => String::from("debug").color(RGB::new(234, 67, 118)),
                Level::Trace => String::from("trace").color(Color::Green),
            };
            return writeln!(buf, "{}: {}", level_str, record.args());
        })
		.try_init()
		.ok();

    let tm = TaskManager::new(Duration::from_secs(60));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    match rt.block_on(async {
        let cli = Cli::parse();
        let result = cli.command.execute(tm.task()).await;
        tm.wait().await;
        result
    }) {
        Ok(_) => {} // Do nothing if we succeed (let the command finish)
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
