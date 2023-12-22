use anyhow::{bail, Context};
use bollard::container::{
    CreateContainerOptions, DownloadFromContainerOptions, StartContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::service::ExecInspectResponse;
use bollard::Docker;
use std::borrow::Cow;
use std::collections::HashMap;

use bollard::container::Config;
use bollard::image::BuildImageOptions;
use bollard::models::{BuildInfo, HostConfig};
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use crate::commands::generic_build::GenericBuildError;
use crate::config::{ExtensionConfiguration, LoadableLibrary};
use crate::control_file::ControlFile;
use crate::manifest::Manifest;
use crate::sync_utils::{ByteStreamSyncReceiver, ByteStreamSyncSender};
use crate::trunk_toml::SystemDependencies;
use futures_util::stream::StreamExt;
use hyper::Body;
use rand::Rng;
use tar::{Archive, Builder, EntryType, Header};
use tee_readwrite::TeeReader;
use tokio::task;
use tokio_stream::wrappers::ReceiverStream;
use tokio_task_manager::Task;

/// Used to stop container when dropped, relies on using [tokio_task_manager::TaskManager::wait]
/// to ensure `Drop` will run to completion
pub struct ReclaimableContainer {
    pub id: String,
    docker: Docker,
    task: Task,
}

impl ReclaimableContainer {
    #[must_use]
    pub fn new(name: String, docker: Docker, task: Task) -> Self {
        Self {
            id: name,
            docker,
            task,
        }
    }
}

impl Drop for ReclaimableContainer {
    fn drop(&mut self) {
        let docker = self.docker.clone();
        let id = self.id.clone();
        let handle = tokio::runtime::Handle::current();
        let mut task = self.task.clone();
        handle.spawn(async move {
            docker
                .stop_container(id.clone().as_str(), None)
                .await
                .expect("error stopping container");
            task.wait().await;
        });
    }
}
pub async fn exec_in_container(
    docker: &Docker,
    container_id: &str,
    command: Vec<&str>,
    env: Option<Vec<&str>>,
) -> Result<String, anyhow::Error> {
    exec_in_container_with_exit_code(docker, container_id, command, env)
        .await
        .map(|(output, _code)| output)
}

pub async fn exec_in_container_with_exit_code(
    docker: &Docker,
    container_id: &str,
    command: Vec<&str>,
    env: Option<Vec<&str>>,
) -> Result<(String, Option<i64>), anyhow::Error> {
    println!("Executing in container: {:?}", command.join(" "));

    let config = CreateExecOptions {
        cmd: Some(command),
        env,
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        ..Default::default()
    };
    let exec = docker.create_exec(container_id, config).await?;
    let start_exec_options = Some(StartExecOptions {
        detach: false,
        ..StartExecOptions::default()
    });
    let log_output = docker.start_exec(&exec.id, start_exec_options);
    let start_exec_result = log_output.await?;
    let mut total_output = String::new();
    match start_exec_result {
        StartExecResults::Attached { output, .. } => {
            let mut output = output
                .map(|result| match result {
                    Ok(log_output) => {
                        println!("{log_output}");
                        total_output.push_str(log_output.to_string().as_str());
                    }
                    Err(error) => eprintln!("Error while reading log output: {error}"),
                })
                .fuse();
            // Run the output stream to completion.
            while output.next().await.is_some() {}
        }
        StartExecResults::Detached => {
            println!("Exec started in detached mode");
        }
    }

    let ExecInspectResponse { exit_code, .. } = docker.inspect_exec(&exec.id).await?;

    Ok((total_output, exit_code))
}

pub async fn run_temporary_container(
    docker: Docker,
    platform: Option<String>,
    image: &str,
    _task: Task,
) -> Result<ReclaimableContainer, anyhow::Error> {
    let options = Some(CreateContainerOptions {
        name: image.to_string(),
        platform,
    });

    let host_config = HostConfig {
        auto_remove: Some(true),
        ..Default::default()
    };

    let config = Config {
        image: Some(image.to_string()),
        entrypoint: Some(vec!["sleep".to_string()]),
        cmd: Some(vec!["300".to_string()]),
        user: Some("root".to_string()),
        host_config: Some(host_config),
        ..Default::default()
    };

    let container = docker.create_container(options, config).await?;
    docker
        .start_container(&container.id, None::<StartContainerOptions<String>>)
        .await?;

    // This will stop the container, whether we return an error or not
    Ok(ReclaimableContainer::new(
        container.id.clone(),
        docker,
        _task,
    ))
}

pub struct ExtensionFiles {
    /// Files stored in `pg_config --sharedir`
    sharedir: Vec<String>,
    /// Files stored in `pg_config --pkglibdir`
    pkglibdir: Vec<String>,
    /// The parsed contents of the extension's control file, if it exists
    control_file: Option<ControlFile>,
}

/// Read the contents of a file in the given container
async fn read_from_container(
    docker: &Docker,
    container_id: &str,
    path: &str,
) -> anyhow::Result<String> {
    let contents = exec_in_container(docker, container_id, vec!["cat", path], None).await?;

    Ok(contents)
}

pub async fn find_installed_extension_files(
    docker: &Docker,
    container_id: &str,
    inclusion_patterns: &[glob::Pattern],
) -> Result<ExtensionFiles, anyhow::Error> {
    let mut control_file = None;
    let sharedir =
        exec_in_container(docker, container_id, vec!["pg_config", "--sharedir"], None).await?;
    let sharedir = sharedir.trim();

    let pkglibdir =
        exec_in_container(docker, container_id, vec!["pg_config", "--pkglibdir"], None).await?;
    let pkglibdir = pkglibdir.trim();

    // collect changes from container filesystem
    let changes = docker
        .container_changes(container_id)
        .await?
        .expect("Expected to find changed files");

    let mut pkglibdir_list = vec![];
    let mut sharedir_list = vec![];

    for change in changes {
        let file_added = change.path;

        // If this file is not a `.so`, `.bc`, `.sql`, `.control` but
        // was specified in `build.include` within the Trunk.toml
        let is_extra = inclusion_patterns
            .iter()
            .any(|pattern| pattern.matches(&file_added));

        if file_added.ends_with(".so")
            || file_added.ends_with(".bc")
            || file_added.ends_with(".sql")
            || file_added.ends_with(".control")
            || is_extra
        {
            if file_added.starts_with(pkglibdir) {
                let file_in_pkglibdir = &file_added;
                let file_in_pkglibdir = file_in_pkglibdir.strip_prefix(pkglibdir);
                let file_in_pkglibdir = file_in_pkglibdir.unwrap();
                let file_in_pkglibdir = file_in_pkglibdir.trim_start_matches('/');
                pkglibdir_list.push(file_in_pkglibdir.to_owned());
            } else if file_added.starts_with(sharedir) {
                let file_in_sharedir = &file_added;
                let file_in_sharedir = file_in_sharedir.strip_prefix(sharedir);
                let file_in_sharedir = file_in_sharedir.unwrap();
                let file_in_sharedir = file_in_sharedir.trim_start_matches('/');
                sharedir_list.push(file_in_sharedir.to_owned());
            } else {
                println!(
                    "WARNING: file {} is not in pkglibdir or sharedir",
                    file_added
                );
            }
        }

        // If there's a control file, let's read in its contents for later use
        if file_added.ends_with(".control") {
            let contents = read_from_container(docker, container_id, &file_added).await?;
            let parsed = ControlFile::parse(&contents);

            control_file = Some(parsed);
        }
    }

    println!("Sharedir files:");
    for sharedir_file in &sharedir_list {
        println!("\t{sharedir_file}");
    }
    println!("Pkglibdir files:");
    for pkglibdir_file in &pkglibdir_list {
        println!("\t{pkglibdir_file}");
    }

    Ok(ExtensionFiles {
        sharedir: sharedir_list,
        pkglibdir: pkglibdir_list,
        control_file,
    })
}

pub async fn find_license_files(
    docker: &Docker,
    container_id: &str,
) -> Result<Vec<String>, anyhow::Error> {
    let licensedir = "/usr/licenses/";

    // collect changes from container filesystem
    let changes = docker
        .container_changes(container_id)
        .await?
        .expect("Expected to find changed files");

    let mut licensedir_list = vec![];

    for change in changes {
        if change.path.starts_with(licensedir) {
            let file_in_licensedir = change.path;
            let file_in_licensedir = file_in_licensedir.strip_prefix(licensedir);
            let file_in_licensedir = file_in_licensedir.unwrap();
            let file_in_licensedir = file_in_licensedir.trim_start_matches('/');
            licensedir_list.push(file_in_licensedir.to_owned());
        }
    }

    println!("License files:");
    for license_file in licensedir_list.clone() {
        println!("\t{license_file}");
    }
    println!();

    Ok(licensedir_list)
}

// Build an image
// The Dockerfile and build directory can be in different directories.
// The caller provides an image name prefix, and this function returns
// the complete image name.
pub async fn build_image(
    platform: Option<String>,
    docker: Docker,
    image_name_prefix: &str,
    dockerfile_path: &str,
    build_directory: &Path,
    build_args: HashMap<&str, &str>,
) -> Result<String, anyhow::Error> {
    let dockerfile = dockerfile_path.to_owned();

    let random_suffix = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..1000000).to_string()
    };

    let image_name = format!("{}{}", image_name_prefix.to_owned(), &random_suffix);

    let (receiver, sender, stream) = ByteStreamSyncSender::new();

    // Making build_directory owned so we can send it to the tarring task below without having to worry
    // about the lifetime of the reference.
    let build_directory = build_directory.to_owned();

    // The docker API receives the build environment as a tar ball.
    task::spawn_blocking(move || {
        let f = || {
            let mut tar = tar::Builder::new(stream);
            tar.append_dir_all(".", build_directory)?;

            let mut header = Header::new_gnu();
            header.set_size(dockerfile.len() as u64);
            header.set_cksum();
            tar.append_data(&mut header, "Dockerfile", dockerfile.as_bytes())?;
            Ok(())
        };
        match f() {
            Ok(()) => (),
            Err(err) => sender.try_send(Err(err)).map(|_| ()).unwrap_or_default(),
        }
    });

    let build_args = build_args.clone();
    let image_name = image_name.to_owned();

    let mut options = BuildImageOptions {
        dockerfile: "Dockerfile",
        t: &image_name.clone(),
        rm: true,
        pull: true,
        buildargs: build_args,
        ..Default::default()
    };

    if platform.is_some() {
        let platform_value = platform.as_ref().unwrap();
        options.platform = platform_value.as_str();
    }

    let mut image_build_stream = docker.build_image(
        options,
        None,
        Some(Body::wrap_stream(ReceiverStream::new(receiver))),
    );

    while let Some(next) = image_build_stream.next().await {
        match next {
            Ok(BuildInfo {
                stream: Some(s), ..
            }) => {
                print!("{s}");
            }
            Ok(BuildInfo {
                error: Some(err),
                error_detail,
                ..
            }) => {
                eprintln!(
                    "ERROR: {} (detail: {})",
                    err,
                    error_detail.unwrap_or_default().message.unwrap_or_default()
                );
            }
            Ok(_) => {}
            Err(err) => {
                dbg!(&err);
                return Err(err)?;
            }
        }
    }

    Ok(image_name)
}

// Scan sharedir and package lib dir from a Trunk builder container for files from a provided list.
// Package these files into a Trunk package.
#[allow(clippy::too_many_arguments)]
pub async fn package_installed_extension_files(
    docker: Docker,
    container_id: &str,
    package_path: &str,
    system_dependencies: Option<SystemDependencies>,
    name: &str,
    mut extension_name: Option<String>,
    extension_version: &str,
    extension_dependencies: Option<Vec<String>>,
    inclusion_patterns: Vec<glob::Pattern>,
    configurations: Option<Vec<ExtensionConfiguration>>,
    loadable_libraries: Option<Vec<LoadableLibrary>>,
    pg_version: u8,
) -> Result<(), anyhow::Error> {
    let name = name.to_owned();
    let extension_version = extension_version.to_owned();

    let target_arch = exec_in_container(&docker, container_id, vec!["uname", "-m"], None).await?;
    let target_arch = target_arch.trim().to_string();

    let sharedir =
        exec_in_container(&docker, container_id, vec!["pg_config", "--sharedir"], None).await?;
    let sharedir = sharedir.trim();

    let pkglibdir = exec_in_container(
        &docker,
        container_id,
        vec!["pg_config", "--pkglibdir"],
        None,
    )
    .await?;
    let pkglibdir = pkglibdir.trim();

    let extension_files =
        find_installed_extension_files(&docker, container_id, &inclusion_patterns).await?;
    let license_files = find_license_files(&docker, container_id).await?;

    let sharedir_list = extension_files.sharedir;
    let pkglibdir_list = extension_files.pkglibdir;
    let licensedir_list = license_files;

    let sharedir = sharedir.to_owned();
    let pkglibdir = pkglibdir.to_owned();
    let licensedir = "/usr/licenses".to_owned();

    // In this function, we open and work with .tar only, then we finalize the package with a .gz in a separate call
    let package_path = format!("{package_path}/{name}-{extension_version}-pg{pg_version}.tar.gz");
    println!("Creating package at: {package_path}");
    let file = File::create(&package_path)?;

    // Stream used to pass information from docker to tar
    let receiver = ByteStreamSyncReceiver::new();
    let receiver_sender = receiver.sender();

    // Open stream to docker for copying files
    // Is there some way to copy from both sharedir and pkglibdir,
    // then combine the streams instead of scanning the whole /usr directory?
    // Looping over everything in that directory makes this way slower.
    let options_usrdir = Some(DownloadFromContainerOptions { path: "/usr" });
    let file_stream = docker.download_from_container(container_id, options_usrdir);

    // TODO: If extension_dependencies is none, check for control file and fetch 'requires' field (similar to below)
    //  example: https://github.com/paradedb/paradedb/blob/9a0b1601a9c7026e5c89eef51a422b9d284b3058/pg_search/pg_search.control#L6C1-L6C9

    if let Some(control) = sharedir_list.iter().find(|path| path.contains(".control")) {
        // If extension_name parameter is none, check for control file and fetch extension_name
        if extension_name.is_none() {
            println!("Fetching extension_name from control file: {control}");
            let path = Path::new(control);
            let file_stem = path
                .file_stem()
                .with_context(|| format!("Path {control} did not have a file stem"))?
                .to_string_lossy()
                .to_string();

            println!("Using extension_name: {}", file_stem);
            extension_name = Some(file_stem);
        }
    }

    // If extension_name is still none, we can assume no control file was found
    if extension_name.is_none() {
        println!(
            "No control file found. Falling back to extension name '{}'",
            &name
        );
        extension_name = Some(name.clone())
    }

    // Create a sync task within the tokio runtime to copy the file from docker to tar
    let tar_handle = task::spawn_blocking(move || {
        // Send ownership of the control file to the closure
        let control_file = extension_files.control_file;
        let mut archive = Archive::new(receiver);
        let mut new_archive = Builder::new(flate2::write::GzEncoder::new(
            file,
            flate2::Compression::default(),
        ));
        let mut manifest = Manifest {
            name,
            extension_name,
            extension_version,
            extension_dependencies,
            manifest_version: 2,
            architecture: target_arch,
            sys: "linux".to_string(),
            files: None,
            dependencies: system_dependencies,
            configurations,
            loadable_libraries,
            pg_version,
        };
        // If the docker copy command starts to stream data
        println!("Create Trunk bundle:");
        let entries = archive
            .entries()
            .expect("Expected to find some files in the /usr directory");
        for entry in entries.flatten() {
            // If we can get the file from the stream
            // Then we will handle packaging the file
            let path = entry.path()?.to_path_buf();
            // Check if we found a file to package in pkglibdir, sharedir or licensedir
            let full_path = format!("/{}", path.to_str().unwrap_or(""));
            let trimmed = full_path
                .trim_start_matches(&format!("{}/", pkglibdir.clone()))
                .trim_start_matches(&format!("{}/", sharedir.clone()))
                .trim_start_matches(&format!("{}/", licensedir.clone()))
                .to_string();
            let pkglibdir_match = pkglibdir_list.contains(&trimmed);
            let sharedir_match = sharedir_list.contains(&trimmed);
            let licensedir_match = licensedir_list.contains(&trimmed);
            // Check if we found a file to package
            if !(sharedir_match || pkglibdir_match || licensedir_match) {
                continue;
            }
            if path.to_str() == Some("manifest.json") {
                println!("Found manifest.json, merging additions with existing manifest");
                manifest.merge(serde_json::from_reader(entry)?);
            } else {
                let root_path = Path::new("/");
                let path = root_path.join(path);
                // The path of this file once ready to be inserted to the archive
                let prepared_path;

                // trim pkglibdir, sharedir or licensedir from start of path
                if path.to_string_lossy().contains(&pkglibdir) {
                    prepared_path = path.strip_prefix(format!("{}/", &pkglibdir))?.into();
                } else if path.to_string_lossy().contains(&sharedir) {
                    prepared_path = prepare_sharedir_file(&sharedir, control_file.as_ref(), &path)?;
                } else if path.to_string_lossy().contains(&licensedir) {
                    prepared_path = path.strip_prefix("/usr/")?.into();
                } else {
                    println!(
                        "WARNING: Skipping file because it's not in sharedir, pkglibdir or licensedir {:?}",
                        &path
                    );
                    continue;
                }

                if !prepared_path.to_string_lossy().is_empty() {
                    let mut header = Header::new_gnu();
                    header.set_mode(entry.header().mode()?);
                    header.set_mtime(entry.header().mtime()?);
                    header.set_size(entry.size());
                    header.set_cksum();
                    let entry_type = entry.header().entry_type();

                    let mut buf = Vec::new();
                    let mut tee = TeeReader::new(entry, &mut buf, true);

                    new_archive.append_data(&mut header, &prepared_path, &mut tee)?;

                    let (_entry, _buf) = tee.into_inner();

                    if entry_type == EntryType::file() {
                        let _ = manifest.add_file(&prepared_path);
                        println!("\t{}", prepared_path.to_string_lossy());
                    }
                }
            }
        }

        let manifest = serde_json::to_string_pretty(&manifest).unwrap_or_default();
        let mut header = Header::new_gnu();
        header.set_size(manifest.as_bytes().len() as u64);
        header.set_cksum();
        header.set_mode(0o644);
        new_archive.append_data(&mut header, "manifest.json", Cursor::new(manifest))?;
        println!("\tmanifest.json");
        Ok::<_, GenericBuildError>(())
    });

    // Wait until completion of streaming, but ignore its error as it would only error out
    // if tar_handle errors out.
    let _ = receiver_sender.stream_to_end(file_stream).await;
    // Handle the error
    tar_handle.await??;

    println!("Packaged to {package_path}");

    Ok(())
}

/// Assumes `file_to_package.starts_with(sharedir)`.
fn prepare_sharedir_file<'p>(
    sharedir: &str,
    control_file: Option<&ControlFile>,
    file_to_package: &'p Path,
) -> anyhow::Result<Cow<'p, Path>> {
    debug_assert!(file_to_package.starts_with(sharedir));

    // If the control file was not supplied, or it was and didn't have the `directory` field filled in,
    // assume the file should go to `$(sharedir)/extension`.
    let maybe_directory = control_file.and_then(|file| file.directory.as_ref());

    let file_to_package = file_to_package.strip_prefix(sharedir)?;

    match maybe_directory {
        Some(diretory) => {
            // If the file starts with `extension/`, remove it so that we can add the correct directory path supplied
            // in the `directory` field.
            let file_to_package = file_to_package
                .strip_prefix("extension")
                .unwrap_or(file_to_package);

            Ok(Path::new(diretory).join(file_to_package).into())
        }
        None => Ok(file_to_package.into()),
    }
}

/// Attempt to locate the path of the Makefile within this container
pub async fn locate_makefile(
    docker: &Docker,
    container_id: &str,
    extension_name: &str,
) -> anyhow::Result<Option<PathBuf>> {
    let stdout = exec_in_container(
        docker,
        container_id,
        vec!["find", ".", "-type", "f", "-iname", "Makefile"],
        None,
    )
    .await?;

    // A project may contain several Makefiles.
    //
    // The idea here is that the "root" Makefile of a project would
    // therefore be the Makefile with the smallest path
    let maybe_makefile = stdout
        .lines()
        .filter(|line| line.contains(extension_name))
        .min();

    let maybe_makefile = maybe_makefile.or_else(|| stdout.lines().min());

    let Some(makefile) = maybe_makefile else {
        return Ok(None);
    };

    let path = Path::new(makefile);

    Ok(Some(
        path.parent()
            .with_context(|| "Makefile should have a parent folder")?
            .to_owned(),
    ))
}

/// Returns true if the file in `path` exists
pub async fn file_exists(docker: &Docker, container_id: &str, path: &str) -> bool {
    exec_in_container_with_exit_code(docker, container_id, vec!["test", "-e", path], None)
        .await
        .map(|(_, exit_code)| exit_code == Some(0))
        .unwrap_or(false)
}

/// Returns true if the Makefile in this container contains the given target
pub async fn makefile_contains_target(
    docker: &Docker,
    container_id: &str,
    dir: &str,
    target: &str,
) -> anyhow::Result<bool> {
    let command = vec!["make", "-C", dir, "-q", target];

    let (output, exit_code) =
        exec_in_container_with_exit_code(docker, container_id, command, None).await?;

    if output.contains("is not supported") {
        return Ok(false);
    }

    let successful = matches!(exit_code, Some(0) | Some(1));

    Ok(successful)
}

pub async fn start_postgres(docker: &Docker, container_id: &str) -> anyhow::Result<()> {
    // Make /app writable by the Postgres user. This is important if pg_regress tries to save files to the filesystem.
    exec_in_container(
        docker,
        container_id,
        vec!["chown", "-R", "postgres:postgres", "/app"],
        None,
    )
    .await?;

    let (_output, status_code) = exec_in_container_with_exit_code(
        docker,
        container_id,
        vec![
            "su",
            "postgres",
            "-c",
            "bash -c \"mkdir db/ && /usr/lib/postgresql/15/bin/initdb -D /app/db && /usr/lib/postgresql/15/bin/pg_ctl -D /app/db -l logfile start\"",
        ],
        None,
    ).await?;

    if status_code == Some(0) {
        println!("Postgres is up!");
    } else {
        bail!("Failed to start Postgres!");
    }

    Ok(())
}
