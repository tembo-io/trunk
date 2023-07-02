use super::SubCommand;
use crate::commands::generic_build::build_generic;
use crate::commands::pgrx::build_pgrx;
use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use std::fs;
use std::path::Path;
use tokio_task_manager::Task;
use toml::Table;
use crate::config::get_from_trunk_toml_if_not_set_on_cli;

#[derive(Args)]
pub struct BuildCommand {
    #[arg(short = 'p', long = "path", default_value = ".")]
    path: String,
    #[arg(short = 'o', long = "output-path", default_value = "./.trunk")]
    output_path: String,
    #[arg(short = 'v', long = "version")]
    version: Option<String>,
    #[arg(short = 'n', long = "name")]
    name: Option<String>,
    #[arg(short = 'P', long = "platform")]
    platform: Option<String>,
    #[arg(short = 'd', long = "dockerfile")]
    dockerfile_path: Option<String>,
    #[arg(short = 'i', long = "install-command")]
    install_command: Option<String>,
}

pub struct BuildSettings {
    path: String,
    output_path: String,
    version: Option<String>,
    name: Option<String>,
    platform: Option<String>,
    dockerfile_path: Option<String>,
    install_command: Option<String>,
}

impl BuildCommand {
    fn settings(&self, trunk_toml: Option<Table>) -> Result<BuildSettings, anyhow::Error> {
        let path = self.path.clone();
        let output_path = self.output_path.clone();
        // These settings can default to Trunk.toml
        let version = get_from_trunk_toml_if_not_set_on_cli(
            self.version.clone(),
            trunk_toml.clone(),
            "extension",
            "version",
        );
        let name = get_from_trunk_toml_if_not_set_on_cli(
            self.name.clone(),
            trunk_toml.clone(),
            "extension",
            "name",
        );
        let platform = get_from_trunk_toml_if_not_set_on_cli(
            self.platform.clone(),
            trunk_toml.clone(),
            "build",
            "platform",
        );
        let dockerfile_path = get_from_trunk_toml_if_not_set_on_cli(
            self.dockerfile_path.clone(),
            trunk_toml.clone(),
            "build",
            "dockerfile_path",
        );
        let install_command = get_from_trunk_toml_if_not_set_on_cli(
            self.install_command.clone(),
            trunk_toml.clone(),
            "build",
            "install_command",
        );

        Ok(BuildSettings {
            path,
            output_path,
            version,
            name,
            platform,
            dockerfile_path,
            install_command,
        })
    }
}

#[async_trait]
impl SubCommand for BuildCommand {
    async fn execute(&self, task: Task, trunk_toml: Option<Table>) -> Result<(), anyhow::Error> {
        let build_settings = self.settings(trunk_toml)?;
        println!("Building from path {}", build_settings.path);
        let path = Path::new(&build_settings.path);
        if path.join("Cargo.toml").exists() {
            let cargo_toml: Table =
                toml::from_str(&std::fs::read_to_string(path.join("Cargo.toml")).unwrap()).unwrap();
            let dependencies = cargo_toml.get("dependencies").unwrap().as_table().unwrap();
            if dependencies.contains_key("pgrx") {
                println!("Detected that we are building a pgrx extension");
                if build_settings.version.is_some() || build_settings.name.is_some() {
                    return Err(anyhow!("--version and --name are collected from Cargo.toml when building pgrx extensions, please do not configure"));
                }

                build_pgrx(
                    build_settings.dockerfile_path.clone(),
                    build_settings.platform.clone(),
                    path,
                    &build_settings.output_path,
                    cargo_toml,
                    task,
                )
                .await?;
                return Ok(());
            }
        }

        // Check for Makefile
        if path.join("Makefile").exists() {
            println!("Detected a Makefile, guessing that we are building an extension with 'make', 'make install...'");
            // Check if version or name are missing
            if build_settings.version.is_none() || build_settings.name.is_none() {
                println!("Error: --version and --name are required when building a makefile based extension");
                return Err(anyhow!(
                    "--version and --name are required when building a makefile based extension"
                ));
            }
            let mut dockerfile = String::new();
            if build_settings.dockerfile_path.clone().is_some() {
                let dockerfile_path_unwrapped = build_settings.dockerfile_path.clone().unwrap();
                println!("Using Dockerfile at {}", &dockerfile_path_unwrapped);
                dockerfile = fs::read_to_string(dockerfile_path_unwrapped.as_str())?;
            } else {
                dockerfile = include_str!("./builders/Dockerfile.generic").to_string();
            }

            let mut install_command_split: Vec<&str> = vec![];
            if let Some(install_command) = build_settings.install_command.as_ref() {
                install_command_split.push("/bin/sh");
                install_command_split.push("-c");
                install_command_split.push(install_command);
            } else {
                install_command_split = vec!["make", "install"];
            }
            println!(
                "Using install command {}",
                install_command_split.clone().join(" ")
            );

            let dockerfile = dockerfile.as_str();
            build_generic(
                dockerfile,
                build_settings.platform.clone(),
                install_command_split,
                path,
                &build_settings.output_path,
                build_settings.name.clone().unwrap().as_str(),
                build_settings.version.clone().unwrap().as_str(),
                task,
            )
            .await?;
            return Ok(());
        }
        println!("Did not understand what to build");
        Ok(())
    }
}
