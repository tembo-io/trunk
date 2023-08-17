use super::SubCommand;
use crate::commands::generic_build::build_generic;
use crate::commands::pgrx::build_pgrx;
use crate::config;
use crate::trunk_toml::{cli_or_trunk, cli_or_trunk_opt};
use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use std::fs;
use std::fs::File;
use std::path::Path;
use tokio_task_manager::Task;
use toml::Table;

#[derive(Args)]
pub struct BuildCommand {
    /// The file path of the extension to build
    #[arg(short = 'p', long = "path", default_value = ".")]
    path: String,
    #[arg(short = 'o', long = "output-path")]
    output_path: Option<String>,
    #[arg(short = 'v', long = "version")]
    version: Option<String>,
    #[arg(short = 'n', long = "name")]
    name: Option<String>,
    #[arg(short = 'e', long = "extension_name")]
    extension_name: Option<String>,
    #[arg(short = 's', long = "shared-preload-libraries")]
    shared_preload_libraries: Option<Vec<String>>,
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
    extension_name: Option<String>,
    shared_preload_libraries: Option<Vec<String>>,
    platform: Option<String>,
    dockerfile_path: Option<String>,
    install_command: Option<String>,
}

impl BuildCommand {
    fn settings(&self) -> Result<BuildSettings, anyhow::Error> {
        // path cannot be set from Trunk.toml, since --path can also
        // be used to specify the path to the directory that includes a
        // Trunk.toml file.
        let build_path = self.path.clone();
        let trunkfile_path = Path::new(&build_path).join("Trunk.toml");
        let trunk_toml = match File::open(trunkfile_path) {
            Ok(file) => Some(config::parse_trunk_toml(file)?),
            Err(_e) => {
                println!("Trunk.toml not found");

                None
            }
        };

        // If output_path is not specified, default to .trunk directory in
        // the directory specified by --path
        let output_path = self.output_path.clone();
        let output_path = match output_path {
            Some(output_path) => output_path,
            None => {
                let output_path = Path::new(&build_path).join(".trunk");
                output_path
                    .to_str()
                    .expect("Failed trying to specify a subdirectory .trunk of the --path argument")
                    .to_string()
            }
        };

        let name = cli_or_trunk(&self.name, |toml| &toml.extension.name, &trunk_toml);

        let extension_name = cli_or_trunk_opt(
            &self.extension_name,
            |toml| &toml.extension.extension_name,
            &trunk_toml,
        );
      
        let shared_preload_libraries = cli_or_trunk_opt(&self.shared_preload_libraries, |toml| &toml.extension.shared_preload_libraries, &trunk_toml);

        let platform = cli_or_trunk(&self.platform, |toml| &toml.build.platform, &trunk_toml);

        let install_command = cli_or_trunk(
            &self.install_command,
            |toml| &toml.build.install_command,
            &trunk_toml,
        );

        // Dockerfile is handled slightly differently in Trunk.toml as the CLI.
        // On CLI, the argument is --dockerfile_path, and it means the path relative
        // to the current working directory where the command line argument is executed.
        // In Trunk.toml, the field is called "dockerfile", and it means the file relative
        // to the Trunk.toml file.
        let dockerfile_path = self.dockerfile_path.clone().or_else(|| {
            let dockerfile = trunk_toml?.build.dockerfile?;

            Some(
                Path::new(&build_path)
                    .join(dockerfile)
                    .to_string_lossy()
                    .into(),
            )
        });

        Ok(BuildSettings {
            path: build_path,
            output_path,
            version,
            name,
            extension_name,
            shared_preload_libraries,
            platform,
            dockerfile_path,
            install_command,
        })
    }
}

fn get_dockerfile(path: Option<String>) -> Result<String, std::io::Error> {
    if let Some(dockerfile_path) = path {
        println!("Using Dockerfile at {}", &dockerfile_path);
        return Ok(fs::read_to_string(dockerfile_path.as_str())?);
    } else {
        return Ok(include_str!("./builders/Dockerfile.generic").to_string());
    }
}

#[async_trait]
impl SubCommand for BuildCommand {
    async fn execute(&self, task: Task) -> Result<(), anyhow::Error> {
        let build_settings = self.settings()?;
        println!("Building from path {}", build_settings.path);
        let path = Path::new(&build_settings.path);
        if path.join("Cargo.toml").exists() {
            let cargo_toml: Table =
                toml::from_str(&std::fs::read_to_string(path.join("Cargo.toml")).unwrap()).unwrap();
            let dependencies = cargo_toml.get("dependencies").unwrap().as_table().unwrap();
            if dependencies.contains_key("pgrx") {
                println!("Detected that we are building a pgrx extension");
                // if user provides name, check that it matches Cargo.toml name
                if build_settings.name.is_some() {
                    let package = cargo_toml.get("package");
                    let cargo_name = package.unwrap().get("name");
                    if build_settings.name
                        != Some(cargo_name.unwrap().as_str().unwrap().to_string())
                    {
                        return Err(anyhow!(
                            "User-provided name must match name in Cargo.toml\n \
                             User-provided name: {}\n \
                             Cargo.toml name: {}\n\
                            ",
                            build_settings.name.unwrap(),
                            cargo_name.unwrap().as_str().unwrap().to_string()
                        ));
                    }
                }
                // if user provides version, check that it matches Cargo.toml version
                if build_settings.version.is_some() {
                    let package = cargo_toml.get("package");
                    let cargo_version = package.unwrap().get("version");
                    if build_settings.version
                        != Some(cargo_version.unwrap().as_str().unwrap().to_string())
                    {
                        return Err(anyhow!(
                            "User-provided version must match version in Cargo.toml\n \
                             User-provided version: {}\n \
                             Cargo.toml version: {}\n\
                            ",
                            build_settings.version.unwrap(),
                            cargo_version.unwrap().as_str().unwrap().to_string()
                        ));
                    }
                }

                build_pgrx(
                    build_settings.dockerfile_path.clone(),
                    build_settings.platform.clone(),
                    path,
                    &build_settings.output_path,
                    build_settings.extension_name,
                    build_settings.shared_preload_libraries,
                    cargo_toml,
                    task,
                )
                .await?;
                return Ok(());
            }
        }

        // Check if version or name are missing
        if build_settings.version.is_none() || build_settings.name.is_none() {
            return Err(anyhow!(
                "--version and --name are required unless building a PGRX extension"
            ));
        }

        let dockerfile: String = get_dockerfile(build_settings.dockerfile_path.clone()).unwrap();

        let mut install_command_split: Vec<&str> = vec![];
        if let Some(install_command) = build_settings.install_command.as_ref() {
            install_command_split.push("/bin/sh");
            install_command_split.push("-c");
            install_command_split.push(install_command);
        } else {
            println!(
                "WARN: Install command is not specified, guessing the command is 'make install'"
            );
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
            build_settings.extension_name,
            build_settings.shared_preload_libraries,
            build_settings.version.clone().unwrap().as_str(),
            task,
        )
        .await?;
        return Ok(());
    }
}
