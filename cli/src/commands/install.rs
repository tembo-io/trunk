use super::SubCommand;
use crate::control_file::ControlFile;
use crate::manifest::{Manifest, PackagedFile};
use anyhow::anyhow;
use async_recursion::async_recursion;
use async_trait::async_trait;
use clap::Args;
use flate2::read::GzDecoder;
use log::{error, info, warn};
use reqwest;
use reqwest::Url;
use slicedisplay::SliceDisplay;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::ops::Not;
use std::path::{Path, PathBuf};
use tar::{Archive, EntryType};
use tokio_task_manager::Task;

#[derive(Args)]
pub struct InstallCommand {
    name: String,
    #[arg(long = "pg-config", short = 'p')]
    pg_config: Option<PathBuf>,
    #[arg(long = "file", short = 'f')]
    file: Option<PathBuf>,
    #[arg(long = "version", short = 'v', default_value = "latest")]
    version: String,
    #[arg(
        long = "registry",
        short = 'r',
        default_value = "https://registry.pgtrunk.io"
    )]
    registry: String,
}

#[derive(thiserror::Error, Debug)]
pub enum InstallError {
    #[error("unknown file type")]
    UnknownFileType,

    #[error("pg_config not found")]
    PgConfigNotFound,

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Package manifest not found")]
    ManifestNotFound,
}

#[async_trait]
impl SubCommand for InstallCommand {
    async fn execute(&self, _task: Task) -> Result<(), anyhow::Error> {
        let installed_pg_config = which::which("pg_config").ok();
        let pg_config = self
            .pg_config
            .as_ref()
            .or_else(|| installed_pg_config.as_ref())
            .ok_or(InstallError::PgConfigNotFound)?;
        println!("Using pg_config: {}", pg_config.display());

        let package_lib_dir = std::process::Command::new(pg_config)
            .arg("--pkglibdir")
            .output()?
            .stdout;

        let package_lib_dir = String::from_utf8_lossy(&package_lib_dir)
            .trim_end()
            .to_string();

        let package_lib_dir = std::fs::canonicalize(package_lib_dir)?;

        let sharedir = std::process::Command::new(pg_config)
            .arg("--sharedir")
            .output()?
            .stdout;

        let sharedir = PathBuf::from(String::from_utf8_lossy(&sharedir).trim_end().to_string());

        if !package_lib_dir.exists() && !package_lib_dir.is_dir() {
            println!(
                "The package lib dir {} does not exist",
                package_lib_dir.display()
            );
            return Ok(());
        }

        println!("Using pkglibdir: {package_lib_dir:?}");
        println!("Using sharedir: {sharedir:?}");

        install(
            &self.name,
            &self.version,
            &self.file,
            &self.registry,
            package_lib_dir,
            sharedir,
        )
        .await?;

        Ok(())
    }
}

#[async_recursion]
async fn install(
    name: &str,
    version: &str,
    file: &Option<PathBuf>,
    registry: &str,
    package_lib_dir: PathBuf,
    sharedir: PathBuf,
) -> Result<(), anyhow::Error> {
    // If file is specified
    if let Some(ref file) = file {
        install_file(name, file, package_lib_dir, sharedir, registry).await?;
    } else {
        // If a file is not specified, then we will query the registry
        // and download the latest version of the package
        // Using the reqwest crate, we will run the equivalent of this curl command:
        // curl --request GET --url 'http://localhost:8080/extensions/{self.name}/{self.version}/download'
        let response = reqwest::get(&format!(
            "{}/extensions/{}/{}/download",
            registry, name, version
        ))
        .await?;

        let response_body = response.text().await?;
        info!("Downloading from: {response_body}");

        let url = Url::parse(&response_body)?;

        // Get the path segments as an iterator
        let segments = url
            .path_segments()
            .ok_or(anyhow!("Cannot extract path segments"))?;

        // Get the last segment, which should be the file name
        let file_name = segments
            .last()
            .ok_or(anyhow!("Cannot extract file name from URL"))?;

        // Write the bytes of the archive to a temporary directory
        let temp_dir = tempfile::tempdir()?;
        let dest_path = temp_dir.path().join(file_name);

        let response = reqwest::get(url).await?;

        let mut dest_file = File::create(&dest_path)?;
        // write the response body to the file
        let bytes = response.bytes().await?;
        dest_file.write_all(&bytes)?;

        install_file(
            name.clone(),
            &dest_path,
            package_lib_dir,
            sharedir,
            registry,
        )
        .await?;
    }
    Ok(())
}

async fn install_file(
    _name: &str,
    file: &PathBuf,
    package_lib_dir: PathBuf,
    sharedir: PathBuf,
    registry: &str,
) -> Result<(), anyhow::Error> {
    let f = File::open(file)?;

    let mut input = match file
        .extension()
        .into_iter()
        .filter_map(|s| s.to_str())
        .next()
    {
        Some("gz") => {
            // unzip the archive into a temporary file
            let decoder = GzDecoder::new(f);
            let mut tempfile = tempfile::tempfile()?;
            use read_write_pipe::*;
            tempfile.write_reader(decoder)?;
            tempfile.rewind()?;
            tempfile
        }
        Some("tar") => f,
        _ => return Err(InstallError::UnknownFileType)?,
    };

    // Handle symlinks
    let sharedir = std::fs::canonicalize(&sharedir)?;
    let package_lib_dir = std::fs::canonicalize(&package_lib_dir)?;

    // First pass: get to the manifest
    // Because we're going over entries with `Seek` enabled, we're not reading everything.
    let mut archive = Archive::new(&input);

    // Extensions the extension being installed depends on
    let mut control_file = None;
    let mut dependent_extensions_to_install: Vec<String> = Vec::new();
    let mut extensions_to_install: Vec<String> = Vec::new();
    let mut manifest: Option<Manifest> = None;
    {
        let entries = archive.entries_with_seek()?;
        for this_entry in entries {
            let mut entry = this_entry?;
            let fname = entry.path()?;
            if entry.header().entry_type() == EntryType::file()
                && fname.clone() == Path::new("manifest.json")
            {
                let manifest_json = serde_json::from_reader(entry)?;
                // if the manifest_version key does not exist, then create it with a value of 1
                let manifest_json = match manifest_json {
                    serde_json::Value::Object(mut map) => {
                        if !map.contains_key("manifest_version") {
                            map.insert(
                                "manifest_version".to_string(),
                                serde_json::Value::Number(1.into()),
                            );
                        }
                        if !map.contains_key("architecture")
                            && map["manifest_version"].as_i64() < Some(2)
                        {
                            // If we are installing a legacy package without architecture specified,
                            // then just assume x86 architecture. All the packages published before that
                            // were published as x86, so this is a correct assumption.
                            map.insert(
                                "architecture".to_string(),
                                serde_json::Value::String("x86".to_string()),
                            );
                        }
                        serde_json::Value::Object(map)
                    }
                    _ => manifest_json,
                };
                let manifest_result = serde_json::from_value(manifest_json);
                manifest.replace(manifest_result?);
            } else if entry.header().entry_type() == EntryType::file()
                && fname.extension().and_then(OsStr::to_str) == Some("control")
            {
                // add extension name to extensions_to_install
                let ext = fname.file_stem().unwrap().to_string_lossy().to_string();

                let mut control_file_contents = String::new();
                entry.read_to_string(&mut control_file_contents)?;
                let parsed_control_file = ControlFile::parse(&control_file_contents);

                extensions_to_install.push(ext);

                let deps = parsed_control_file.dependencies();

                // For each dependency, check if it's not in depenedent_extensions_to_install and not in extensions_to_install.
                // If not, add to depenedent_extensions_to_install.
                // We don't want to install dependencies that are already present in the tar.gz
                for dep in deps {
                    if !dependent_extensions_to_install.contains(dep)
                        && !extensions_to_install.contains(dep)
                    {
                        dependent_extensions_to_install.push(dep.to_string());
                    }
                }

                control_file = Some(parsed_control_file);
            }
        }
    }

    let maybe_manifest_deps = manifest
        .as_ref()
        .and_then(|manifest| manifest.extension_dependencies.as_ref());
    if let Some(manifest_deps) = maybe_manifest_deps {
        for dep in manifest_deps {
            // If the extension is not in dependent_extensions_to_install,
            // it wasn't specified in the control file
            if dependent_extensions_to_install.contains(dep).not() {
                dependent_extensions_to_install.push(dep.to_string());
            }
        }
    }

    info!("Dependent extensions to be installed: {dependent_extensions_to_install:?}");
    for dependency in dependent_extensions_to_install {
        // check a control file is present in sharedir for each dependency
        let control_file_path = sharedir
            .join("extension")
            .join(format!("{dependency}.control"));
        if !control_file_path.exists() {
            info!("Dependency {dependency} not found in sharedir {sharedir:?}. Installing...");
            install(
                &dependency,
                "latest",
                &None,
                registry,
                package_lib_dir.clone(),
                sharedir.clone(),
            )
            .await?;
        }
    }

    // Set up path used in manifest file version 1
    let extension_dir = get_extension_location(&sharedir, control_file.as_ref());

    // Second pass: extraction
    input.rewind()?;

    let mut archive = Archive::new(&input);

    if let Some(mut manifest) = manifest {
        let manifest_files = manifest.files.take().unwrap_or_default();
        info!(
            "Installing {} {}",
            manifest.name, manifest.extension_version
        );
        let host_arch = if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else if cfg!(target_arch = "arm") {
            "aarch32"
        } else if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target = "x86") {
            "x86"
        } else {
            "unsupported"
        };

        if manifest.manifest_version > 1 && host_arch != manifest.architecture {
            warn!(
                "This package is not compatible with your architecture: {}, it is compatible with {}",
                host_arch,
                manifest.architecture
            );
            return Ok(());
        }

        let entries = archive.entries_with_seek()?;
        for entry in entries {
            let mut entry = entry?;
            let name = entry.path()?;
            if let Some(file) = manifest_files.get(name.as_ref()) {
                match file {
                    PackagedFile::ControlFile { .. } => {
                        if manifest.manifest_version > 1 {
                            println!("[+] {} => {}", name.display(), sharedir.display());
                            entry.unpack_in(&sharedir)?;
                        } else {
                            // In manifest v1, the control file is in the root of the archive
                            // and in following versions, it will be prefixed by its path under
                            // pg_config --sharedir
                            println!("[+] {} => {}", name.display(), extension_dir.display());
                            entry.unpack_in(&extension_dir)?;
                        }
                    }
                    PackagedFile::SqlFile { .. } => {
                        if manifest.manifest_version > 1 {
                            println!("[+] {} => {}", name.display(), sharedir.display());
                            entry.unpack_in(&sharedir)?;
                        } else {
                            // In manifest v1, sql files are in the root of the archive
                            // and in following versions, they will be prefixed by path under
                            // pg_config --sharedir
                            println!("[+] {} => {}", name.display(), extension_dir.display());
                            entry.unpack_in(&extension_dir)?;
                        }
                    }
                    PackagedFile::SharedObject { .. } => {
                        println!("[+] {} => {}", name.display(), package_lib_dir.display());
                        entry.unpack_in(&package_lib_dir)?;
                    }
                    PackagedFile::Bitcode { .. } => {
                        println!("[+] {} => {}", name.display(), package_lib_dir.display());
                        entry.unpack_in(&package_lib_dir)?;
                    }
                    PackagedFile::Extra { .. } => {
                        println!("[+] {} => {}", name.display(), sharedir.display());
                        entry.unpack_in(&sharedir)?;
                    }
                    PackagedFile::LicenseFile { .. } => {
                        info!("Skipping license file {}", name.display());
                    }
                }
            }
        }

        print_post_installation_guide(&manifest);
    } else {
        return Err(InstallError::ManifestNotFound)?;
    }
    Ok(())
}

fn get_extension_location(sharedir: &Path, control_file: Option<&ControlFile>) -> PathBuf {
    // If the `directory` field in the extension's `control` file is set, the location of the extension's files will be
    // `$(pg_config --sharedir)/$(dir)`, where `dir` is the value set in the `directory` field.
    //
    // If this is not set, the default value is `$(pg_config --sharedir)/extension`.
    //
    // Docs: https://www.postgresql.org/docs/current/extend-extensions.html
    let maybe_directory = control_file.map(|file| &file.directory);
    let directory = if let Some(Some(directory)) = maybe_directory {
        directory
    } else {
        "extension"
    };

    sharedir.join(directory)
}

fn print_post_installation_guide(manifest: &Manifest) {
    let extension_name = manifest.extension_name.as_ref().unwrap_or(&manifest.name);

    println!("\n***************************");
    println!("* POST INSTALLATION STEPS *");
    println!("***************************");

    if let Some(dependency_declaration) = &manifest.dependencies {
        println!("\nInstall the following system-level dependencies:");
        for (package_manager, dependencies) in dependency_declaration {
            println!("\tOn systems using {package_manager}:");
            for dependency in dependencies {
                println!("\t\t{dependency}");
            }
        }
    }
    // If the manifest has extension_dependencies, then we need to install and enable the
    // appropriate extension
    if let Some(extension_dependencies) = &manifest.extension_dependencies {
        let extension_dependencies = extension_dependencies.join(",");
        println!("\nInstall and enable the following extensions:");
        println!("\textension_dependencies = '{}'", extension_dependencies)
    }
    // If the manifest has preload_libraries, then we need to add the extension to preload_libraries
    // Output will look like preload_libraries = 'spl1,spl2,spl3'
    if let Some(preload_libraries) = &manifest.loadable_libraries {
        let libraries: Vec<_> = preload_libraries
            .iter()
            .map(|lib| &lib.library_name)
            .collect();
        println!("\nAdd the following to your postgresql.conf:");
        println!(
            "\tshared_preload_libraries = {}",
            libraries.display().terminator('\'', '\'')
        );
    }

    println!("\nEnable the extension with:");
    println!("\tCREATE EXTENSION IF NOT EXISTS {extension_name} CASCADE;");
}
