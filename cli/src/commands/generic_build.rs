use std::path::Path;

use std::fs;

use thiserror::Error;

use bollard::Docker;

use tokio::sync::mpsc;

use tokio_task_manager::Task;

use crate::commands::containers::{
    build_image, exec_in_container, exec_in_container_with_exit_code, locate_makefile,
    makefile_contains_target, package_installed_extension_files, run_temporary_container,
    start_postgres,
};
use crate::commands::license::{copy_licenses, find_licenses};

use super::build::BuildSettings;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
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
    build_settings: &BuildSettings,
    install_command: &[String],
    path: &Path,
    _task: Task,
) -> Result<(), GenericBuildError> {
    let name = build_settings.name.clone().unwrap();
    let extension_version = build_settings.version.clone().unwrap();
    println!("Building with name {}", &name);
    println!("Building with version {}", &extension_version);
    println!("Building for PostgreSQL {}", build_settings.pg_version);

    let build_args = build_settings.get_docker_build_args()?;
    let image_name_prefix = "make_builder_".to_string();

    let docker = Docker::connect_with_local_defaults()?;
    let dockerfile = build_settings.get_dockerfile("pgxs").unwrap();

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
        _task,
    )
    .await?;

    if build_settings.should_test {
        let extension_name = &build_settings.extension_name.as_deref().unwrap_or(&name);
        // Check if there are extensions to run
        run_tests(&docker, &temp_container.id, extension_name).await?;
    }

    println!("Determining installation files...");
    let _exec_output = exec_in_container(
        &docker,
        &temp_container.id,
        install_command.iter().map(|x| x.as_str()).collect(),
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

    package_installed_extension_files(
        docker.clone(),
        &temp_container.id,
        &build_settings.output_path,
        build_settings.system_dependencies.clone(),
        &name,
        build_settings.extension_name.clone(),
        &extension_version,
        build_settings.extension_dependencies.clone(),
        &build_settings.glob_patterns_to_include,
        build_settings.configurations.clone(),
        build_settings.loadable_libraries.clone(),
        build_settings.pg_version,
    )
    .await?;

    Ok(())
}

async fn run_tests(
    docker: &Docker,
    container_id: &str,
    extension_name: &str,
) -> anyhow::Result<()> {
    let Some(project_dir) = locate_makefile(docker, container_id, extension_name).await? else {
        println!("Makefile not found!");
        return Ok(());
    };

    let project_dir_utf8 = project_dir.to_str().expect("Expected UTF8");

    let has = |target| async move {
        makefile_contains_target(docker, container_id, project_dir_utf8, target).await
    };

    if has("check").await? {
        println!("make check was found in the Makefile");
        start_postgres(docker, container_id).await?;

        let (_, exit_code) = exec_in_container_with_exit_code(
            docker,
            container_id,
            vec!["make", "check"],
            Some(project_dir_utf8),
            Some(vec!["PGUSER=postgres"]),
        )
        .await?;

        anyhow::ensure!(matches!(exit_code, Some(0)), "Tests failed!");

        println!("Tests passed successfully!");
        return Ok(());
    }

    if has("installcheck").await? {
        start_postgres(docker, container_id).await?;

        println!("make installcheck was found in the Makefile");
        exec_in_container(
            docker,
            container_id,
            vec!["make", "install"],
            Some(project_dir_utf8),
            None,
        )
        .await?;
        let (_output, exit_code) = exec_in_container_with_exit_code(
            docker,
            container_id,
            vec!["make", "installcheck"],
            Some(project_dir_utf8),
            Some(vec!["PGUSER=postgres"]),
        )
        .await?;

        anyhow::ensure!(matches!(exit_code, Some(0)), "Tests failed!");

        println!("Tests passed successfully!");
        return Ok(());
    }

    println!("Test target not found in Makefile.");
    Ok(())
}
