use super::SubCommand;
use crate::control_file::ControlFile;
use crate::manifest::{Manifest, PackagedFile};
use crate::retry::get_retry;
use crate::semver::compare_by_semver;
use crate::v1::TrunkProjectView;
use anyhow::Result;
use anyhow::{anyhow, bail, ensure, Context};
use async_recursion::async_recursion;
use async_trait::async_trait;
use clap::Args;

use flate2::read::GzDecoder;
use log::{error, info, warn};
use reqwest;
use reqwest::Url;
use sha2::{Digest, Sha256};
use slicedisplay::SliceDisplay;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, Cursor, Read, Write};
use std::ops::Not;
use std::path::{Path, PathBuf};
use system_dependencies::OperatingSystem;
use tar::EntryType;
use tokio_task_manager::Task;

mod system_dependencies;

#[derive(Clone, Copy)]
/// A Trunk project name versus an extension name.
/// Although related, a project's name may differ from its extension name.
/// For example, the `pgvector` project is installed with the extension name `vector`,
/// meaning its Trunk project name is `pgvector`, while its extension name is `vector`.
pub enum Name<'a> {
    TrunkProject(&'a str),
    Extension(&'a str),
}

impl Display for Name<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Name::TrunkProject(name) => name,
            Name::Extension(name) => name,
        };

        f.write_str(name)
    }
}

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
    /// The PostgreSQL version for which this extension should be installed.
    #[clap(long, action)]
    pg_version: Option<u8>,
    /// Skip dependency resolution.
    #[clap(long, short, action)]
    skip_dependencies: bool,
    /// Installs required system dependencies for the extension
    #[clap(long = "deps", action)]
    install_system_dependencies: bool,
    /// Installation location for architecture-independent support files.
    #[clap(long = "sharedir", action)]
    sharedir: Option<PathBuf>,
    /// Installation location for dynamically loadable modules.
    #[clap(long = "pkglibdir", action)]
    pkglibdir: Option<PathBuf>,
    /// Strip `$libdir/` from module_pathname before installing control file.
    #[clap(long = "strip-libdir", action)]
    strip_libdir: bool,
}

impl InstallCommand {
    pub fn pgconfig(&self) -> Result<PgConfig> {
        let pg_config_path = self
            .pg_config
            .clone()
            .map(Ok)
            .unwrap_or_else(|| which::which("pg_config"))?;

        Ok(PgConfig { pg_config_path })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InstallError {
    #[error("unknown file type")]
    UnknownFileType,

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Package manifest not found")]
    ManifestNotFound,
}

pub struct PgConfig {
    pub pg_config_path: PathBuf,
}

impl PgConfig {
    fn exec(&self, arg: &str) -> Result<String> {
        let mut bytes = std::process::Command::new(&self.pg_config_path)
            .arg(arg)
            .output()
            .with_context(|| format!("Failed to run pgconfig {arg}"))?
            .stdout;

        if bytes.last() == Some(&b'\n') {
            bytes.pop();
        }

        String::from_utf8(bytes).with_context(|| "pgconfig returned invalid UTF-8")
    }

    pub fn pkglibdir(&self) -> Result<PathBuf> {
        fs::canonicalize(self.exec("--pkglibdir")?)
            .with_context(|| "Failed to canonicalize pkglibdir")
    }

    pub fn sharedir(&self) -> Result<PathBuf> {
        self.exec("--sharedir").map(PathBuf::from)
    }

    /// The major version of the currently set PostgreSQL server
    pub fn postgres_version(&self) -> Result<u8> {
        let version = self.exec("--version")?;

        let version = if version.starts_with("PostgreSQL 14") {
            14
        } else if version.starts_with("PostgreSQL 15") {
            15
        } else if version.starts_with("PostgreSQL 16") {
            16
        } else if version.starts_with("PostgreSQL 17") {
            17
        } else {
            bail!("Currently unsupported Postgres version: {version}")
        };

        Ok(version)
    }
}

#[async_trait]
impl SubCommand for InstallCommand {
    async fn execute(&self, _task: Task) -> Result<()> {
        let pg_config = self.pgconfig()?;

        let package_lib_dir = match &self.pkglibdir {
            Some(p) => p.clone(),
            None => pg_config.pkglibdir()?,
        };
        let sharedir = match &self.sharedir {
            Some(p) => p.clone(),
            None => pg_config.sharedir()?,
        };

        if !package_lib_dir.exists() && !package_lib_dir.is_dir() {
            println!(
                "The package lib dir {} does not exist",
                package_lib_dir.display()
            );
            return Ok(());
        }

        let postgres_version = if let Some(pg_version) = self.pg_version {
            pg_version
        } else {
            pg_config.postgres_version()?
        };

        println!("Using pkglibdir: {package_lib_dir:?}");
        println!("Using sharedir: {sharedir:?}");
        println!("Using Postgres version: {postgres_version}");

        install(
            Name::TrunkProject(&self.name),
            &self.version,
            self.file.as_deref(),
            &self.registry,
            InstallConfig {
                package_lib_dir,
                sharedir,
                postgres_version,
                skip_dependency_resolution: self.skip_dependencies,
                install_system_dependencies: self.install_system_dependencies,
                strip_libdir: self.strip_libdir,
            },
        )
        .await?;

        Ok(())
    }
}

fn find_trunk_project(
    projects: Vec<TrunkProjectView>,
    name: &str,
    version: &str,
) -> Result<TrunkProjectView> {
    let project = if version == "latest" {
        let mut projects: Vec<_> = projects
            .into_iter()
            .filter(|proj| proj.name == name)
            .collect();
        projects.sort_by(|a, b| compare_by_semver(&a.version, &b.version));
        // Take the last element since, now we've sorted, it'll be the element with the latest version
        projects
            .pop()
            .with_context(|| format!("Found no Trunk project with name {name}"))?
    } else {
        projects
            .into_iter()
            .find(|project| project.name == name && project.version == version)
            .with_context(|| {
                format!("Found no Trunk project with name {name} and version {version}")
            })?
    };

    Ok(project)
}

fn ensure_extension_uniqueness(projects: &[TrunkProjectView], extension_name: &str) -> Result<()> {
    let mut matching_projects = projects.iter().filter(|proj| {
        proj.extensions
            .iter()
            .any(|ext| ext.extension_name == extension_name)
    });

    let Some(first_project) = matching_projects.next() else {
        return Ok(());
    };

    for project in matching_projects {
        // Err if a different Trunk project provides the same extension
        ensure!(
            project.name == first_project.name,
            "Extension with name {} is provided by both {} and {}",
            extension_name,
            project.name,
            first_project.name
        );
    }

    Ok(())
}

fn find_extension(
    projects: Vec<TrunkProjectView>,
    name: &str,
    version: &str,
) -> Result<TrunkProjectView> {
    ensure_extension_uniqueness(&projects, name)?;

    let project = if version == "latest" {
        let mut projects: Vec<_> = projects
            .into_iter()
            .filter(|proj| proj.extensions.iter().any(|ext| ext.extension_name == name))
            .collect();
        projects.sort_by(|a, b| compare_by_semver(&a.version, &b.version));
        // Take the last element since, now we've sorted, it'll be the element with the latest version
        projects
            .pop()
            .with_context(|| format!("Found no Trunk project with name {name}"))?
    } else {
        projects
            .into_iter()
            .find(|proj| {
                proj.extensions.iter().any(|ext| ext.extension_name == name)
                    && proj.version == version
            })
            .with_context(|| {
                format!("Found no Trunk project with name {name} and version {version}")
            })?
    };

    Ok(project)
}

async fn fetch_archive_from_v1(
    registry: &str,
    name: Name<'_>,
    version: &str,
    postgres_version: u8,
) -> Result<(Url, String, Option<String>)> {
    let endpoint = match name {
        Name::TrunkProject(name) => format!("{registry}/api/v1/trunk-projects/{name}"),
        Name::Extension(name) => format!("{registry}/api/v1/trunk-projects?extension-name={name}"),
    };

    let response = reqwest::get(endpoint).await?;
    let status = response.status();

    if status.is_success() {
        let body: Vec<TrunkProjectView> = response.json().await?;

        let mut project = match name {
            Name::TrunkProject(name) => find_trunk_project(body, name, version)?,
            Name::Extension(name) => find_extension(body, name, version)?,
        };

        let download = project.downloads
            .with_context(|| "Trunk project had no `downloads` object")?
            .into_iter()
            .find(|download| download.pg_version == postgres_version)
            .with_context(|| format!("Failed to find an archive for {name} v{version} built for PostgreSQL {postgres_version}"))?;

        let extension_name = match name {
            Name::TrunkProject(_) => {
                if project.extensions.len() == 1 {
                    project
                        .extensions
                        .pop()
                        .map(|extension| extension.extension_name)
                } else {
                    None
                }
            }
            Name::Extension(ext_name) => Some(ext_name.to_owned()),
        };

        let url = Url::parse(&download.link).with_context(|| "Failed to parse URL")?;
        Ok((url, download.sha256, extension_name))
    } else {
        let body = response.text().await?;

        Err(anyhow!("Request to registry failed: {body}"))
    }
}

async fn fetch_archive_legacy(registry: &str, name: &str, version: &str) -> Result<Url> {
    let endpoint = format!("{}/extensions/{}/{}/download", registry, name, version);

    let response = get_retry(&endpoint).await?;
    let status = response.status();

    if status.is_success() {
        let body = response.text().await?;

        Url::parse(&body).with_context(|| "Failed to parse URL")
    } else {
        let body = response.text().await?;

        Err(anyhow!("Request to registry failed: {body}"))
    }
}

async fn fetch_archive_from_registry(
    name: Name<'_>,
    version: &str,
    registry: &str,
    postgres_version: u8,
) -> Result<(Vec<u8>, Option<String>, Option<String>)> {
    // Try to fetch archive details from the v1 API.
    let (url, maybe_hash, maybe_api_extension_name) = match fetch_archive_from_v1(
        registry,
        name,
        version,
        postgres_version,
    )
    .await
    {
        Ok((url, hash, ext_name)) => (url, Some(hash), ext_name),
        Err(err) if postgres_version == 15 => {
            // For Postgres 15, fallback to the legacy endpoint.
            let name_str = match name {
                Name::TrunkProject(n) => n,
                Name::Extension(n) => n,
            };
            eprintln!("Failed to fetch archive from v1 API: {err}");
            (
                fetch_archive_legacy(registry, name_str, version).await?,
                None,
                None,
            )
        }
        Err(err) => {
            if let Some(msg) = err.downcast_ref::<String>() {
                if msg.starts_with("Extension with name") {
                    warn!("Manual intervention required: {msg}");
                    // In this scenario the original code would return early.
                    // You might decide to either stop installation or signal this specially.
                    bail!("Manual intervention required: {msg}");
                }
            }
            eprintln!("Failed to fetch archive from v1 API: {err}");
            bail!("Cannot install extension for Postgres version {postgres_version} through the legacy endpoint");
        }
    };

    info!("Downloading from: {url}");
    // Extract the file name from the URL.
    let mut segments = url
        .path_segments()
        .ok_or(anyhow!("Cannot extract path segments from URL"))?;
    let file_name = segments.next_back().map(|s| s.to_string());

    let response = get_retry(url).await?;
    let data = response.bytes().await?.to_vec();
    assert_sha256_matches(&data, maybe_hash)?;

    Ok((data, file_name, maybe_api_extension_name))
}

#[derive(Clone)]
struct InstallConfig {
    package_lib_dir: PathBuf,
    sharedir: PathBuf,
    postgres_version: u8,
    skip_dependency_resolution: bool,
    install_system_dependencies: bool,
    strip_libdir: bool,
}

#[async_recursion]
async fn install<'name: 'async_recursion>(
    name: Name<'name>,
    version: &str,
    file: Option<&Path>,
    registry: &str,
    config: InstallConfig,
) -> Result<()> {
    let extension_name = match name {
        Name::TrunkProject(_) => None,
        Name::Extension(name) => Some(name.to_owned()),
    };

    // Load the raw archive bytes and (optionally) a file name.
    // For the network (non --file) code path, delegate to the helper.
    let (raw_data, file_name, maybe_extension_name) = if let Some(ref file) = file {
        let data = fs::read(file)?;
        let file_name = file.file_name().and_then(|s| s.to_str()).map(String::from);
        (data, file_name, None)
    } else {
        fetch_archive_from_registry(name, version, registry, config.postgres_version).await?
    };
    let extension_name = extension_name.or(maybe_extension_name);

    // Decompress the archive if needed.
    let archive_data = match file_name
        .as_deref()
        .and_then(|name| Path::new(name).extension().and_then(OsStr::to_str))
    {
        Some("gz") => {
            // Decompress gzipped data into a tar archive.
            let mut decoder = GzDecoder::new(Cursor::new(raw_data));
            let mut decompressed = Vec::new();
            std::io::copy(&mut decoder, &mut decompressed)?;
            decompressed
        }
        Some("tar") => raw_data,
        _ => return Err(InstallError::UnknownFileType.into()),
    };

    install_trunk_archive(
        &archive_data,
        registry,
        // Send the v1-supplied extension name, in case the manifest.json
        // doesn't have extension_name set
        extension_name,
        config,
    )
    .await?;

    Ok(())
}

async fn install_trunk_archive(
    // Bytes for the project's .tar archive
    archive_data: &[u8],
    registry: &str,
    extension_name: Option<String>,
    config: InstallConfig,
) -> Result<()> {
    // Handle symlinks
    let sharedir = std::fs::canonicalize(&config.sharedir)?;
    let package_lib_dir = std::fs::canonicalize(&config.package_lib_dir)?;

    // First pass: get to the manifest
    // Because we're going over entries with `Seek` enabled, we're not reading everything.
    let mut archive = tar::Archive::new(Cursor::new(archive_data));

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

                // For each dependency, check if it's not in dependent_extensions_to_install and not in extensions_to_install.
                // If not, add to dependent_extensions_to_install.
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

    if config.skip_dependency_resolution {
        warn!(
            "Skipping dependency resolution! {} dependencies are unmet.",
            dependent_extensions_to_install.len()
        );
    } else {
        info!("Dependent extensions to be installed: {dependent_extensions_to_install:?}");
        for dependency in dependent_extensions_to_install {
            // check a control file is present in sharedir for each dependency
            let control_file_path = sharedir
                .join("extension")
                .join(format!("{dependency}.control"));
            if !control_file_path.exists() {
                info!("Dependency {dependency} not found in sharedir {sharedir:?}. Installing...");
                install(
                    Name::Extension(&dependency),
                    "latest",
                    None,
                    registry,
                    config.clone(),
                )
                .await?;
            }
        }
    }

    // Set up path used in manifest file version 1
    let extension_dir = get_extension_location(&sharedir, control_file.as_ref());

    // Second pass: extraction
    let mut archive = tar::Archive::new(Cursor::new(archive_data));
    let mut manifest = manifest.ok_or(InstallError::ManifestNotFound)?;

    let manifest_files = manifest.files.take().unwrap_or_default();
    info!(
        "Installing {} {}",
        manifest.name, manifest.extension_version
    );
    let host_arch = if cfg!(test) {
        "x86_64"
    } else {
        std::env::consts::ARCH
    };

    if manifest.manifest_version > 1 && host_arch != manifest.architecture {
        bail!(
            "This package is not compatible with your architecture: {}, it is compatible with {}",
            host_arch,
            manifest.architecture
        );
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
                        if config.strip_libdir {
                            // Copy line by line and remove the $libdir prefix
                            // from module_pathname.
                            let mut dst = fs::File::create(sharedir.join(name))?;
                            let buf = std::io::BufReader::new(entry);
                            for line in buf.lines().map_while(Result::ok) {
                                if line.starts_with("module_pathname") {
                                    writeln!(&mut dst, "{}", line.replace("$libdir/", ""))?;
                                } else {
                                    writeln!(&mut dst, "{line}")?;
                                }
                            }
                            continue;
                        }
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
    if manifest.extension_name.is_none() {
        manifest.extension_name = extension_name;
    }

    if config.install_system_dependencies {
        if let Some(system_deps) = &manifest.dependencies {
            let operating_system = OperatingSystem::detect()?;
            for package_manager in operating_system.package_managers() {
                if let Some(packages_to_install) = system_deps.get(package_manager.as_str()) {
                    for package in packages_to_install {
                        let installation_command = package_manager.install(package);

                        let status = mockcmd::Command::new("sh")
                            .arg("-c")
                            .arg(&installation_command)
                            .status()
                            .with_context(|| {
                                format!("Failed to execute command: {}", installation_command)
                            })?;

                        if !status.success() {
                            anyhow::bail!(
                                "Installation of package '{}' via {} failed",
                                package,
                                package_manager.as_str()
                            );
                        }
                    }
                }
            }
        }
    }

    post_installation(&manifest);

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

fn post_installation(manifest: &Manifest) {
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

fn assert_sha256_matches(contents: &[u8], maybe_hash: Option<String>) -> Result<()> {
    if let Some(expected_hash) = maybe_hash {
        let mut hasher = Sha256::new();
        hasher.update(contents);
        let digest = hasher.finalize();
        let hash_gotten = hex::encode(digest);

        anyhow::ensure!(
            hash_gotten == expected_hash,
            "Expected SHA-256 {}, got {}!",
            hash_gotten,
            expected_hash
        );
    }

    Ok(())
}

#[tokio::test]
async fn install_with_system_dependencies() -> Result<()> {
    let file = None;

    let pg_config = PgConfig {
        pg_config_path: which::which("pg_config")?,
    };
    let package_lib_dir = pg_config.pkglibdir()?;
    let sharedir = pg_config.sharedir()?;

    install(
        Name::Extension("citus"),
        "13.0.1",
        file,
        "https://registry.pgtrunk.io",
        InstallConfig {
            package_lib_dir,
            sharedir: sharedir.clone(),
            postgres_version: 17,
            skip_dependency_resolution: false,
            install_system_dependencies: true,
            strip_libdir: true,
        },
    )
    .await?;

    let system_deps = [
        "libpq5", "openssl", "libc6", "liblz4-1", "libzstd1", "libssl3", "libcurl4",
    ];
    for dep in system_deps {
        assert!(mockcmd::was_command_executed(&[
            "sh",
            "-c",
            &format!("sudo apt-get install -y {}", dep)
        ]));
    }

    // Validate that strip_libdir properly stripped the $libdir prefix.
    let ctrl = fs::File::open(sharedir.join("extension").join("citus.control"))?;
    let buf = std::io::BufReader::new(ctrl);
    for line in buf.lines().map_while(Result::ok) {
        if line.starts_with("module_pathname") {
            assert!(!line.contains("$libdir/"));
        }
    }

    Ok(())
}
