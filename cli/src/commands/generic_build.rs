use std::collections::HashMap;

use std::path::Path;

use std::fs;

use thiserror::Error;

use bollard::Docker;

use tokio::sync::mpsc;

use tokio_task_manager::Task;

use crate::commands::containers::{
    build_image, exec_in_container, package_installed_extension_files, run_temporary_container,
};
use crate::commands::license::{copy_licenses, find_licenses};
use crate::trunk_toml::SystemDependencies;

#[derive(Error, Debug)]
pub enum GenericBuildError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Docker Error: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("Internal sending error: {0}")]
    InternalSendingError(#[from] mpsc::error::SendError<Vec<u8>>),

    #[error("Parsing ELF file error: {0}")]
    ElfError(#[from] elf::ParseError),

    #[error("Tar layout error: trunk-output not found")]
    TarLayoutError(#[from] std::path::StripPrefixError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

// Generic Trunk builder procedure:
//
// Run image build, providing the user-provided build command
//
// docker build -t test .
//
// Start container from the builder image, with a lifetime
//
// docker run -it --rm --entrypoint=sleep -d test 600
//
// Connect into running container, and run the user-provided install command
//
// docker exec -it 05a11b4b1bd5 make install
//
// Find the files that have changed from the install command
//
// docker diff 05a11b4b1bd5
//
// Any file that has changed, copy out of the container and into the trunk package
pub async fn build_generic(
    dockerfile: &str,
    platform: Option<String>,
    install_command: Vec<&str>,
    path: &Path,
    output_path: &str,
    name: &str,
    extension_name: Option<String>,
    preload_libraries: Option<Vec<String>>,
    system_dependencies: Option<SystemDependencies>,
    extension_version: &str,
    inclusion_patterns: Vec<glob::Pattern>,
    _task: Task,
) -> Result<(), GenericBuildError> {
    println!("Building with name {}", &name);
    println!("Building with version {}", &extension_version);

    let mut build_args = HashMap::new();
    build_args.insert("EXTENSION_NAME", name);
    build_args.insert("EXTENSION_VERSION", extension_version);

    let image_name_prefix = "make_builder_".to_string();

    let docker = Docker::connect_with_local_defaults()?;

    let image_name = build_image(
        platform.clone(),
        docker.clone(),
        &image_name_prefix,
        dockerfile,
        path,
        build_args,
    )
    .await?;

    let temp_container =
        run_temporary_container(docker.clone(), platform.clone(), image_name.as_str(), _task)
            .await?;

    println!("Determining installation files...");
    let _exec_output =
        exec_in_container(&docker, &temp_container.id, install_command, None).await?;

    // Search for license files to include
    println!("Determining license files to include...");
    let license_vec = find_licenses(docker.clone(), &temp_container.id).await?;

    // Create directory /usr/licenses/
    let _exec_output = exec_in_container(
        &docker,
        &temp_container.id,
        vec!["mkdir", "/usr/licenses/"],
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
    fs::create_dir_all(output_path)?;

    package_installed_extension_files(
        docker.clone(),
        &temp_container.id,
        output_path,
        preload_libraries,
        system_dependencies,
        name,
        extension_name,
        extension_version,
        inclusion_patterns,
    )
    .await?;

    Ok(())
}
