use super::SubCommand;
use crate::manifest::{Manifest, PackagedFile};
use anyhow::anyhow;
use async_recursion::async_recursion;
use async_trait::async_trait;
use clap::Args;
use flate2::read::GzDecoder;
use reqwest;
use reqwest::Url;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek, Write};
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

        let sharedir = std::process::Command::new(&pg_config)
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
            self.name.clone(),
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
    name: String,
    version: &str,
    file: &Option<PathBuf>,
    registry: &str,
    package_lib_dir: PathBuf,
    sharedir: PathBuf,
) -> Result<(), anyhow::Error> {
    // If file is specified
    if let Some(ref file) = file {
        install_file(name.clone(), file, package_lib_dir, sharedir, registry).await?;
    } else {
        // If a file is not specified, then we will query the registry
        // and download the latest version of the package
        // Using the reqwest crate, we will run the equivalent of this curl command:
        // curl --request GET --url 'http://localhost:8080/extensions/{self.name}/{self.version}/download'
        let response = reqwest::get(&format!(
            "{}/extensions/{}/{}/download",
            registry,
            name.clone(),
            version
        ))
        .await?;

        let response_body = response.text().await?;
        println!("Downloading from: {response_body}");

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
    name: String,
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

    // Set up path used in manifest file version 1
    let extension_dir_path = sharedir.join("extension");
    let extension_dir = std::fs::canonicalize(extension_dir_path)?;

    // First pass: get to the manifest
    // Because we're going over entries with `Seek` enabled, we're not reading everything.
    let mut archive = Archive::new(&input);

    // Extensions the extension being installed depends on
    let mut dependent_extensions_to_install: Vec<String> = Vec::new();
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
                && fname.clone().file_name() == Some(OsStr::new(format!("{name}.control").as_str()))
            {
                let mut control_file = String::new();
                entry.read_to_string(&mut control_file)?;
                dependent_extensions_to_install = read_dependent_extensions(&control_file);
            }
        }
    }
    println!("Dependent extensions to be installed: {dependent_extensions_to_install:?}");
    for dependency in dependent_extensions_to_install {
        // check a control file is present in sharedir for each dependency
        let control_file_path = sharedir
            .join("extension")
            .join(format!("{dependency}.control"));
        if !control_file_path.exists() {
            println!("Dependency {dependency} not found in sharedir {sharedir:?}. Installing...");
            install(
                dependency,
                "latest",
                &None,
                registry,
                package_lib_dir.clone(),
                sharedir.clone(),
            )
            .await?;
        }
    }

    // Second pass: extraction
    input.rewind()?;

    let mut archive = Archive::new(&input);

    if let Some(mut manifest) = manifest {
        let manifest_files = manifest.files.take().unwrap_or_default();
        println!(
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
            println!(
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
                        println!("Skipping license file {}", name.display());
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

fn print_post_installation_guide(manifest: &Manifest) {
    let extension_name = manifest.extension_name.as_ref().unwrap_or(&manifest.name);

    println!("\n***************************");
    println!("* POST INSTALLATION STEPS *");
    println!("***************************");

    if let Some(dependency_declaration) = &manifest.dependencies {
        println!("\n\tNeeded system-level dependencies:");
        for (package_manager, dependencies) in dependency_declaration {
            println!("\n\t* On systems using {package_manager}:");
            for dependency in dependencies {
                println!("\t\t{dependency}\n");
            }
        }
    }

    println!("\nEnable the extension with:");
    println!("CREATE EXTENSION IF NOT EXISTS {extension_name} CASCADE;");
}

fn read_dependent_extensions(contents: &str) -> Vec<String> {
    let mut dependencies: Vec<String> = Vec::new();

    for line in contents.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("requires") {
            let dep_line = trimmed_line.strip_prefix("requires =").unwrap().trim();
            let dep_list = dep_line
                .trim_matches('\'')
                .split(',')
                .filter(|dep| !dep.trim().is_empty()) // Filter out empty entries
                .map(|dep| dep.trim().to_string())
                .collect::<Vec<String>>();
            dependencies.extend(dep_list);
            break;
        }
    }

    dependencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_dependencies_with_requires_line() {
        let sample_data = r#"comment = 'Distributed message queues'
default_version = '0.4.2'
module_pathname = '$libdir/pgmq'
relocatable = false
superuser = false
requires = 'pg_partman, dep2, dep3'
"#;

        let dependencies = read_dependent_extensions(sample_data);
        assert_eq!(dependencies, vec!["pg_partman", "dep2", "dep3"]);
    }

    #[test]
    fn test_read_dependencies_without_requires_line() {
        let sample_data = r#"comment = 'Distributed message queues'
default_version = '0.4.2'
module_pathname = '$libdir/pgmq'
relocatable = false
superuser = false
"#;

        let dependencies = read_dependent_extensions(sample_data);
        assert_eq!(dependencies, Vec::<String>::new());
    }

    #[test]
    fn test_read_dependencies_with_empty_requires_line() {
        let sample_data = r#"comment = 'Distributed message queues'
default_version = '0.4.2'
module_pathname = '$libdir/pgmq'
relocatable = false
superuser = false
requires = ''
"#;

        let dependencies = read_dependent_extensions(sample_data);
        assert_eq!(dependencies, Vec::<String>::new());
    }
}
