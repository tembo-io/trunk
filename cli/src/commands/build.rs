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
    #[arg(short = 'pl', long = "platform")]
    platform: Option<String>,
    #[arg(short = 'd', long = "dockerfile")]
    dockerfile_path: Option<String>,
    #[arg(short = 'i', long = "install-command")]
    install_command: Option<String>,
}

#[async_trait]
impl SubCommand for BuildCommand {
    async fn execute(&self, task: Task) -> Result<(), anyhow::Error> {
        println!("Building from path {}", self.path);
        let path = Path::new(&self.path);
        if path.join("Cargo.toml").exists() {
            let cargo_toml: Table =
                toml::from_str(&std::fs::read_to_string(path.join("Cargo.toml")).unwrap()).unwrap();
            let dependencies = cargo_toml.get("dependencies").unwrap().as_table().unwrap();
            if dependencies.contains_key("pgrx") {
                println!("Detected that we are building a pgrx extension");
                if self.version.is_some() || self.name.is_some() {
                    return Err(anyhow!("--version and --name are collected from Cargo.toml when building pgrx extensions, please do not configure"));
                }

                build_pgrx(
                    self.dockerfile_path.clone(),
                    self.platform.clone(),
                    path,
                    &self.output_path,
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
            if self.version.is_none() || self.name.is_none() {
                println!("Error: --version and --name are required when building a makefile based extension");
                return Err(anyhow!(
                    "--version and --name are required when building a makefile based extension"
                ));
            }
            let mut dockerfile = String::new();
            if self.dockerfile_path.clone().is_some() {
                let dockerfile_path_unwrapped = self.dockerfile_path.clone().unwrap();
                println!("Using Dockerfile at {}", &dockerfile_path_unwrapped);
                dockerfile = fs::read_to_string(dockerfile_path_unwrapped.as_str())?;
            } else {
                dockerfile = include_str!("./builders/Dockerfile.generic").to_string();
            }

            let mut install_command_split: Vec<&str> = vec![];
            if let Some(install_command) = self.install_command.as_ref() {
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
                self.platform.clone(),
                install_command_split,
                path,
                &self.output_path,
                self.name.clone().unwrap().as_str(),
                self.version.clone().unwrap().as_str(),
                task,
            )
            .await?;
            return Ok(());
        }
        println!("Did not understand what to build");
        Ok(())
    }
}
