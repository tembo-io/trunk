use semver::{Version, VersionReq};
use std::fs;
use std::path::{Path, StripPrefixError};
use std::string::FromUtf8Error;

use thiserror::Error;

use bollard::Docker;

use crate::commands::containers::{
    build_image, exec_in_container, package_installed_extension_files, run_temporary_container,
};
use tokio::sync::mpsc;

use super::build::BuildSettings;
use crate::commands::license::{copy_licenses, find_licenses};
use tokio::task::JoinError;
use tokio_task_manager::Task;
use toml::Value;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum PgrxBuildError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Docker Error: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("Error converting binary to utf8: {0}")]
    FromUft8Error(#[from] FromUtf8Error),

    #[error("Internal sending error: {0}")]
    InternalSendingError(#[from] mpsc::error::SendError<Vec<u8>>),

    #[error("Cargo manifest error: {0}")]
    ManifestError(String),

    #[error("Async join error: {0}")]
    JoinError(#[from] JoinError),

    #[error("Parsing ELF file error: {0}")]
    ElfError(#[from] elf::ParseError),

    #[error("Tar layout error: trunk-output not found")]
    TarLayoutError(#[from] StripPrefixError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

fn semver_from_range(pgrx_range: &str) -> Result<String, PgrxBuildError> {
    let versions = [
        "0.14.1", "0.14.0", "0.13.1", "0.13.0", "0.12.9", "0.12.8", "0.12.7", "0.12.6", "0.12.5",
        "0.12.4", "0.12.3", "0.12.2", "0.12.1", "0.12.0", "0.11.4", "0.11.3", "0.11.2", "0.11.1",
        "0.11.0", "0.10.2", "0.10.1", "0.10.0", "0.9.8", "0.9.7", "0.9.1", "0.9.0", "0.8.4",
        "0.8.3", "0.8.0", "0.7.4",
    ];

    if versions.contains(&pgrx_range) {
        // If the input is already a specific version, return it as-is
        return Ok(pgrx_range.to_string());
    }

    // If the version is a semver range, convert to a specific version
    let pgrx_semver = if let Ok(range) = VersionReq::parse(pgrx_range) {
        // The pgrx version is a range, so we need to find the highest
        // version that satisfies the range
        versions
            .iter()
            .filter_map(|&s| Version::parse(s).ok())
            .filter(|v| range.matches(v))
            .max()
            .ok_or(PgrxBuildError::ManifestError(format!(
                "No supported version of pgrx satisfies the range {pgrx_range}. \nSupported versions: {versions:?}"
            )))?
    } else {
        // The pgrx version is already a specific version
        Version::parse(pgrx_range).map_err(|_| {
            PgrxBuildError::ManifestError(format!("Invalid pgrx version string: {pgrx_range}"))
        })?
    };

    let pgrx_version = pgrx_semver.to_string();
    Ok(pgrx_version)
}

pub async fn build_pgrx(
    build_settings: &BuildSettings,
    path: &Path,
    cargo_toml: toml::Table,
    task: Task,
) -> Result<(), PgrxBuildError> {
    let cargo_package_info = cargo_toml
        .get("package")
        .into_iter()
        .filter_map(Value::as_table)
        .next()
        .ok_or(PgrxBuildError::ManifestError(
            "Could not find package info in Cargo.toml".to_string(),
        ))?;
    let name = cargo_package_info
        .get("name")
        .into_iter()
        .filter_map(Value::as_str)
        .next()
        .ok_or(PgrxBuildError::ManifestError(
            "Could not find package name in Cargo.toml".to_string(),
        ))?;
    let extension_version = cargo_package_info
        .get("version")
        .into_iter()
        .filter_map(Value::as_str)
        .next()
        .ok_or(PgrxBuildError::ManifestError(
            "Could not find package version in Cargo.toml".to_string(),
        ))?;
    let pgrx_range = cargo_toml
        .get("dependencies")
        .into_iter()
        .filter_map(Value::as_table)
        .next()
        .ok_or(PgrxBuildError::ManifestError(
            "Could not find dependencies info in Cargo.toml".to_string(),
        ))?
        .get("pgrx")
        .into_iter()
        .filter_map(Value::as_str)
        .next()
        .ok_or(PgrxBuildError::ManifestError(
            "Could not find pgrx dependency info in Cargo.toml".to_string(),
        ))?;

    println!("Detected pgrx version range {}", &pgrx_range);

    let pgrx_version = semver_from_range(pgrx_range)?;
    println!("Using pgrx version {pgrx_version}");

    println!("Building pgrx extension at path {}", &path.display());

    let build_args = build_settings.get_docker_build_args(name, extension_version)?;
    let image_name_prefix = "pgrx_builder_".to_string();

    let docker = Docker::connect_with_local_defaults()?;
    let dockerfile = build_settings.get_dockerfile("pgrx").unwrap();

    let image_name = build_image(
        &build_settings.platform,
        docker.clone(),
        &image_name_prefix,
        &dockerfile,
        path,
        build_args,
    )
    .await?;

    let temp_container = run_temporary_container(
        docker.clone(),
        build_settings.platform.clone(),
        image_name.as_str(),
        task,
    )
    .await?;

    println!("Determining installation files...");
    let _exec_output = exec_in_container(
        &docker,
        &temp_container.id,
        vec![
            "cp",
            "--verbose",
            "-R",
            format!("target/release/{name}-pg{}/usr", build_settings.pg_version).as_str(),
            "/",
        ],
        None,
        None,
    )
    .await?;

    // Search for license files to include
    println!("Determining license files to include...");
    let license_vec = find_licenses(docker.clone(), &temp_container.id).await?;

    // Create directory /usr/licenses/
    let _exec_output = exec_in_container(
        &docker,
        &temp_container.id,
        vec!["mkdir", "/usr/licenses/"],
        None,
        None,
    )
    .await?;

    // Iterate through license files and copy to /usr/licenses/. If filename exists in /usr/licenses,
    // append numbered suffix. Example:
    // ❯ tar -tvf .trunk/pg_stat_statements-1.10.0.tar.gz | grep -i copyright
    //     -rw-r--r-- 0/0            4362 2023-05-15 19:28 COPYRIGHT
    //     -rw-r--r-- 0/0            1192 2023-05-15 19:28 COPYRIGHT.~1~
    copy_licenses(
        license_vec,
        &temp_container.id,
        docker.clone(),
        Some(vec!["VERSION_CONTROL=numbered"]),
    )
    .await?;

    // output_path is the locally output path
    fs::create_dir_all(&build_settings.output_path)?;
    package_installed_extension_files(build_settings, docker.clone(), &temp_container.id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_from_range_specific_version() {
        // Test that a specific version string is returned as-is
        let result = semver_from_range("0.8.3");
        assert_eq!(result.unwrap(), "0.8.3");
        let result = semver_from_range("0.8.4");
        assert_eq!(result.unwrap(), "0.8.4");
    }

    #[test]
    fn test_semver_from_range_specific_version_with_equals() {
        // Test that a specific version string is returned as-is
        let result = semver_from_range("=0.8.3");
        assert_eq!(result.unwrap(), "0.8.3");
        let result = semver_from_range("=0.8.4");
        assert_eq!(result.unwrap(), "0.8.4");
    }

    #[test]
    fn test_semver_from_range_semver_range() {
        // Test that a semver range is converted to the highest matching version
        let result = semver_from_range(">=0.8.0, <0.9.0");
        assert_eq!(result.unwrap(), "0.8.4");
        let result = semver_from_range(">=0.9.0, <0.10.0");
        assert_eq!(result.unwrap(), "0.9.8");
        let result = semver_from_range(">=0.10.0, <0.11.0");
        assert_eq!(result.unwrap(), "0.10.2");
    }
}
