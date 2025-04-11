use super::SubCommand;
use crate::commands::generic_build::build_generic;
use crate::commands::pgrx::build_pgrx;
use crate::config::{self, ExtensionConfiguration, LoadableLibrary};
use crate::trunk_toml::{cli_or_trunk, cli_or_trunk_opt, SystemDependencies};
use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use log::{info, warn};
use slicedisplay::SliceDisplay;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io;
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
    #[arg(short = 'x', long = "extension_dependencies")]
    extension_dependencies: Option<Vec<String>>,
    #[arg(short = 's', long = "preload-libraries")]
    preload_libraries: Option<Vec<String>>,
    #[arg(short = 'P', long = "platform")]
    platform: Option<String>,
    #[arg(short = 'd', long = "dockerfile")]
    dockerfile_path: Option<String>,
    #[arg(short = 'i', long = "install-command")]
    install_command: Option<String>,
    /// Run this extension's integration tests after building
    #[clap(long, short, action)]
    test: bool,
    /// The PostgreSQL version to build this extension against
    #[clap(default_value = "15", long, action)]
    pg_version: u8,
    /// Set a Dockerfile build argument
    #[arg(short = 'a', long = "build-arg")]
    build_args: Vec<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct BuildSettings {
    pub path: String,
    pub output_path: String,
    pub version: Option<String>,
    pub name: Option<String>,
    pub extension_name: Option<String>,
    pub extension_dependencies: Option<Vec<String>>,
    pub configurations: Option<Vec<ExtensionConfiguration>>,
    pub system_dependencies: Option<SystemDependencies>,
    pub glob_patterns_to_include: Vec<glob::Pattern>,
    pub platform: Option<String>,
    pub dockerfile_path: Option<String>,
    pub install_command: Option<String>,
    pub should_test: bool,
    pub loadable_libraries: Option<Vec<LoadableLibrary>>,
    pub pg_version: u8,
    pub build_args: Vec<String>,
}

impl BuildSettings {
    // Load and return the contents of the  Dockerfile specified by
    // `self.dockerfile_path`. If its value is `None`, load and return the
    // contents of the `default` dockerfile.
    pub(crate) fn get_dockerfile(&self, default: &str) -> Result<String, io::Error> {
        if let Some(dockerfile_path) = &self.dockerfile_path {
            info!("Using Dockerfile at {}", &dockerfile_path);
            fs::read_to_string(dockerfile_path.as_str())
        } else {
            match default {
                "pgxs" => Ok(include_str!("./builders/Dockerfile.generic").to_string()),
                "pgrx" => Ok(include_str!("./builders/Dockerfile.pgrx").to_string()),
                _ => Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    anyhow!("Unknown dockerfile type: {default}"),
                )),
            }
        }
    }

    // get_docker_build_args returns the values to use for `--build-arg`
    // options to `docker build.` Pass name and version to override values
    // read from `Trunk.toml` and the command-line. Pass empty strings to
    // require values from rom `Trunk.toml` and the command-line.
    pub(crate) fn get_docker_build_args<'a>(
        &'a self,
        name: &'a str,
        version: &'a str,
    ) -> Result<HashMap<&'a str, &'a str>, anyhow::Error> {
        let mut build_args = HashMap::new();
        build_args.insert(
            "EXTENSION_NAME",
            if name.is_empty() {
                self.name.as_ref().unwrap().as_str()
            } else {
                name
            },
        );
        build_args.insert(
            "EXTENSION_VERSION",
            if version.is_empty() {
                self.version.as_ref().unwrap().as_str()
            } else {
                version
            },
        );
        build_args.insert("PG_VERSION", self.pg_version_string());
        build_args.insert("PG_RELEASE", self.pg_release_tag());
        for arg in &self.build_args {
            match arg.split_once("=") {
                Some((k, v)) => build_args.insert(k, v),
                None => return Err(anyhow!("Invalid build arg: {arg}")),
            };
        }
        Ok(build_args)
    }

    pub(crate) fn get_install_command(&self, default: &[&'static str]) -> Vec<String> {
        match &self.install_command {
            None => {
                warn!(
                    "Install command is not specified, guessing the command is '{}'",
                    default.join(" ")
                );
                default.iter().map(|x| x.to_string()).collect()
            }
            Some(cmd) => {
                // Replace all instances of strings like `postgresql/15` and `pg15`.
                let re = regex::Regex::new(r"(postgresql/|pg)\d+").unwrap();
                let cmd = re.replace_all(cmd, |caps: &regex::Captures| -> String {
                    format!("{}{}", &caps[1], self.pg_version)
                });

                vec![
                    "/usr/bin/bash".to_string(),
                    "-c".to_string(),
                    cmd.to_string(),
                ]
            }
        }
    }

    // Returns the string representation self.pg_version.
    pub(crate) fn pg_version_string(&self) -> &'static str {
        match self.pg_version {
            14 => "14",
            15 => "15",
            16 => "16",
            17 => "17",
            _ => panic!("Unsupported Postgres version!"),
        }
    }

    // Returns the Git release tag for self.pg_version.
    pub(crate) fn pg_release_tag(&self) -> &'static str {
        match self.pg_version {
            14 => "REL_14_17",
            15 => "REL_15_12",
            16 => "REL_16_8",
            17 => "REL_17_4",
            _ => panic!("Unsupported Postgres version!"),
        }
    }
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
                warn!("Trunk.toml not found");

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

        let loadable_libraries = trunk_toml
            .as_ref()
            .and_then(|toml| toml.extension.loadable_libraries.as_ref())
            .cloned();

        let extension_name = cli_or_trunk_opt(
            &self.extension_name,
            |toml| &toml.extension.extension_name,
            &trunk_toml,
        );

        let extension_dependencies = cli_or_trunk_opt(
            &self.extension_dependencies,
            |toml| &toml.extension.extension_dependencies,
            &trunk_toml,
        );

        let version = cli_or_trunk(&self.version, |toml| &toml.extension.version, &trunk_toml);

        let platform = cli_or_trunk(&self.platform, |toml| &toml.build.platform, &trunk_toml);

        let install_command = cli_or_trunk_opt(
            &self.install_command,
            |toml| &toml.build.install_command,
            &trunk_toml,
        );

        let glob_patterns_to_include = trunk_toml
            .as_ref()
            .map(|toml| toml.build.build_glob_patterns())
            .transpose()?
            .unwrap_or(Vec::new());

        warn!(
            "will include files matching {}",
            glob_patterns_to_include.display()
        );

        let configurations = trunk_toml
            .as_ref()
            .and_then(|toml| toml.extension.configurations.as_ref())
            .cloned();

        let system_dependencies = trunk_toml
            .as_ref()
            .and_then(|toml| toml.dependencies.as_ref())
            .cloned();

        // Dockerfile is handled slightly differently in Trunk.toml as the CLI.
        // On CLI, the argument is --dockerfile_path, and it means the path relative
        // to the current working directory where the command line argument is executed.
        // In Trunk.toml, the field is called "dockerfile", and it means the file relative
        // to the Trunk.toml file.
        let dockerfile_path = self.dockerfile_path.clone().or_else(|| {
            let dockerfile = &trunk_toml?.build.dockerfile?;

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
            extension_dependencies,
            system_dependencies,
            glob_patterns_to_include,
            platform,
            dockerfile_path,
            install_command,
            should_test: self.test,
            configurations,
            loadable_libraries,
            pg_version: self.pg_version,
            build_args: self.build_args.clone(),
        })
    }
}

#[async_trait]
impl SubCommand for BuildCommand {
    async fn execute(&self, task: Task) -> Result<(), anyhow::Error> {
        let build_settings = self.settings()?;
        info!("Building from path {}", build_settings.path);
        let path = Path::new(&build_settings.path);

        if path.join("Cargo.toml").exists() {
            let cargo_toml: Table =
                toml::from_str(&std::fs::read_to_string(path.join("Cargo.toml")).unwrap()).unwrap();
            let dependencies = cargo_toml.get("dependencies").unwrap().as_table().unwrap();
            if dependencies.contains_key("pgrx") {
                info!("Detected that we are building a pgrx extension");
                // if user provides name, check that it matches Cargo.toml name
                if let Some(name) = &build_settings.name {
                    let package = cargo_toml.get("package");
                    let cargo_name = package.unwrap().get("name");
                    if name != cargo_name.unwrap().as_str().unwrap() {
                        return Err(anyhow!(
                            "User-provided name must match name in Cargo.toml\n \
                             User-provided name: {name}\n \
                             Cargo.toml name: {}\n\
                            ",
                            cargo_name.unwrap().as_str().unwrap().to_string()
                        ));
                    }
                }
                // if user provides version, check that it matches Cargo.toml version
                if let Some(version) = &build_settings.version {
                    let package = cargo_toml.get("package");
                    let cargo_version = package.unwrap().get("version");
                    if version != cargo_version.unwrap().as_str().unwrap() {
                        return Err(anyhow!(
                            "User-provided version must match version in Cargo.toml\n \
                             User-provided version: {version}\n \
                             Cargo.toml version: {}\n\
                            ",
                            cargo_version.unwrap().as_str().unwrap().to_string()
                        ));
                    }
                }

                build_pgrx(&build_settings, path, cargo_toml, task).await?;
                return Ok(());
            }
        }

        // Check if version or name are missing
        if build_settings.version.is_none() || build_settings.name.is_none() {
            return Err(anyhow!(
                "--version and --name are required unless building a PGXS extension"
            ));
        }

        let install_command =
            build_settings.get_install_command(&["make", "install", "USE_PGXS=1"]);
        info!("Using install command {}", install_command.join(" "));

        build_generic(&build_settings, &install_command, path, task).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn from_root(path: &str) -> PathBuf {
        [env!("CARGO_MANIFEST_DIR"), path].iter().collect()
    }

    #[test]
    fn settings_get_dockerfile() {
        // Test successes.
        for (name, val, path) in [
            ("pgxs", "pgxs", None),
            ("pgrx", "pgrx", None),
            (
                "dir_field",
                "pgxs",
                Some("tests/test_dir_field/Dockerfile".to_string()),
            ),
            (
                "http",
                "pgrx",
                Some("tests/test_builders/Dockerfile.http".to_string()),
            ),
        ] {
            let bs = BuildSettings {
                dockerfile_path: path.clone(),
                ..Default::default()
            };

            let dockerfile = bs.get_dockerfile(val).unwrap();
            let p = path.unwrap_or_else(|| match val {
                "pgxs" => "src/commands/builders/Dockerfile.generic".to_string(),
                "pgrx" => "src/commands/builders/Dockerfile.pgrx".to_string(),
                _ => panic!("Unknown val {val}"),
            });

            let file = from_root(&p);
            let contents = fs::read_to_string(file).unwrap();
            assert_eq!(contents, dockerfile, "{name}");
        }

        // Test failures.
        for (name, val, path, err) in [
            (
                "unknown default",
                "nonesuch",
                None,
                "Unknown dockerfile type: nonesuch".to_string(),
            ),
            (
                "file not found",
                "nonesuch",
                Some("Dockerfile.none".to_string()),
                "No such file or directory (os error 2)".to_string(),
            ),
        ] {
            let bs = BuildSettings {
                dockerfile_path: path.clone(),
                ..Default::default()
            };

            match bs.get_dockerfile(val) {
                Ok(_) => panic!("{name} unexpectedly succeeded"),
                Err(e) => assert_eq!(err, e.to_string(), "{name}"),
            }
        }
    }
}
