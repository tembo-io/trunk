mod commands;
mod config;
mod control_file;
mod manifest;
mod sync_utils;
mod trunk_toml;
mod tui;

use crate::commands::SubCommand;
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use log::error;
use log::Level;

use colorful::{Color, Colorful, RGB};
use std::io::Write;
use std::{process::ExitCode, time::Duration};
use tokio_task_manager::{Task, TaskManager};
use tui::{indent, TRUNK_SAND_COLOR};

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
    /// Verify a local install of a Postgres extension by running its regression tests
    Verify(commands::verify::VerifyCommand),
}

#[async_trait]
impl SubCommand for SubCommands {
    async fn execute(&self, task: Task) -> Result<(), anyhow::Error> {
        match self {
            SubCommands::Build(cmd) => cmd.execute(task).await,
            SubCommands::Publish(cmd) => cmd.execute(task).await,
            SubCommands::Install(cmd) => cmd.execute(task).await,
            SubCommands::Verify(cmd) => cmd.execute(task).await,
        }
    }
}

fn main() -> ExitCode {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            let level_str = match record.level() {
                Level::Info => String::from("info").color(RGB::new(
                    TRUNK_SAND_COLOR.r,
                    TRUNK_SAND_COLOR.g,
                    TRUNK_SAND_COLOR.b,
                )),
                Level::Error => String::from("error").color(Color::Red),
                Level::Warn => String::from("warn").color(Color::Yellow),
                Level::Debug => String::from("debug").color(RGB::new(234, 67, 118)),
                Level::Trace => String::from("trace").color(Color::Green),
            };
            writeln!(buf, "{}: {}", level_str, record.args())
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
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            // Any errors returned will get propogated up and gracefuly logged to the user here
            print!("{}", indent(1));
            error!("{}", e);
            ExitCode::from(1)
        }
    }
}

pub fn pg_version_to_str(pg_version: u8) -> &'static str {
    match pg_version {
        14 => "14",
        15 => "15",
        16 => "16",
        _ => panic!("Unsupported Postgres version!"),
    }
}

pub fn pg_release_for_version(pg_version: u8) -> &'static str {
    match pg_version {
        14 => "REL_14_10",
        15 => "REL15_3",
        16 => "REL16_1",
        _ => panic!("Unsupported Postgres version!"),
    }
}
