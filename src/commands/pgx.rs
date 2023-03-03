use bollard::container::{
    Config, CreateContainerOptions, DownloadFromContainerOptions, StartContainerOptions,
};
use bollard::models::HostConfig;
use std::default::Default;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;
use std::string::FromUtf8Error;
use std::{cmp, io};
use std::{fs, include_str};

use futures_util::stream::StreamExt;

use rand::Rng;
use tar::{Archive, Header};
use thiserror::Error;

use bollard::image::BuildImageOptions;
use bollard::Docker;

use bollard::models::BuildInfo;
use hyper::Body;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;
use tokio::task;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Error, Debug)]
pub enum PgxBuildError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Docker Error: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("Error converting binary to utf8: {0}")]
    FromUft8Error(#[from] FromUtf8Error),

    #[error("Internal sending error: {0}")]
    InternalSendingError(#[from] mpsc::error::SendError<Vec<u8>>),
}

/// Sends a byte stream in chunks to [tokio::mpsc] channel
///
/// It implements [std::io::Write] so it can be used in a sync task
pub(crate) struct ByteStream {
    sender: mpsc::Sender<Result<Vec<u8>, io::Error>>,
    buffer: Vec<u8>,
}

impl ByteStream {
    /// Creates a new ByteStream
    pub(crate) fn new() -> (
        mpsc::Receiver<Result<Vec<u8>, io::Error>>,
        mpsc::Sender<Result<Vec<u8>, io::Error>>,
        Self,
    ) {
        let (sender, receiver) = mpsc::channel(1);
        let stream = Self {
            sender: sender.clone(),
            buffer: Vec::new(),
        };
        (receiver, sender, stream)
    }
}

impl Drop for ByteStream {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

const BUFFER_SIZE: usize = 8192;

impl Write for ByteStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        if self.buffer.len() > BUFFER_SIZE {
            self.flush()?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut message = std::mem::take(&mut self.buffer);
        loop {
            match self.sender.try_send(Ok(message)) {
                // Success
                Ok(()) => return Ok(()),
                // Retry
                Err(TrySendError::Full(Ok(msg))) => message = msg,
                // We never send errors, so this is unreachable
                Err(TrySendError::Full(Err(_))) => unreachable!(),
                // No longer need to send anything
                Err(TrySendError::Closed(_)) => {
                    return Err(std::io::Error::from(ErrorKind::BrokenPipe))
                }
            }
        }
    }
}

struct ByteStreamReceiver {
    receiver: mpsc::Receiver<Vec<u8>>,
    buffer: Vec<u8>,
}

impl Read for ByteStreamReceiver {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Serve from the buffer first
        let mut received_bytes = if !self.buffer.is_empty() {
            std::mem::replace(&mut self.buffer, Vec::new())
        } else {
            // Otherwise, read from the receiver
            match self.receiver.blocking_recv() {
                None => return Ok(0),
                Some(bytes) => {
                    dbg!(bytes.len());
                    bytes
                }
            }
        };
        // Combine existing buffer with the received bytes
        // TODO: optimize for the first case (of serving the buffer)
        let mut bytes = std::mem::replace(&mut self.buffer, Vec::new());
        bytes.append(&mut received_bytes);

        // Finding how much we can it into `buf`
        let amt = cmp::min(buf.len(), bytes.len());
        let (a, b) = bytes.split_at(amt);

        // The remainder of the entire buffer goes back into the buffer
        if b.len() > 0 {
            self.buffer.extend_from_slice(b);
        }

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        Ok(amt)
    }
}

pub async fn build_pgx(
    path: &Path,
    output_path: &str,
    extension_name: &str,
    extension_version: &str,
) -> Result<(), PgxBuildError> {
    println!("Building pgx extension at path {}", &path.display());
    let dockerfile = include_str!("./pgx_builder/Dockerfile");

    let (receiver, sender, stream) = ByteStream::new();
    // Making path owned so we can send it to the tarring task below without having to worry
    // about the lifetime of the reference.
    let path = path.to_owned();
    task::spawn_blocking(move || {
        let f = || {
            let mut tar = tar::Builder::new(stream);
            tar.append_dir_all(".", path)?;

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

    let mut image_name = "pgx_builder_".to_string();

    let random_suffix = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..1000000).to_string()
    };

    image_name.push_str(&random_suffix);
    let image_name = image_name.as_str().to_owned();

    // TODO: build args in the Dockerfile such as postgres version should be configurable
    let options = BuildImageOptions {
        dockerfile: "Dockerfile",
        t: &image_name.clone(),
        rm: true,
        ..Default::default()
    };

    let docker = Docker::connect_with_local_defaults()?;
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
                return Err(err)?;
            }
        }
    }

    let options = Some(CreateContainerOptions {
        name: image_name.to_string(),
        platform: None,
    });

    let host_config = HostConfig {
        auto_remove: Some(true),
        ..Default::default()
    };

    let config = Config {
        image: Some(image_name.to_string()),
        entrypoint: Some(vec!["sleep".to_string()]),
        cmd: Some(vec!["300".to_string()]),
        host_config: Some(host_config),
        ..Default::default()
    };

    let container = docker.create_container(options, config).await?;
    docker
        .start_container(&container.id, None::<StartContainerOptions<String>>)
        .await?;

    let pg_config_file_path = "/app/.pg_config";

    let options = Some(DownloadFromContainerOptions {
        path: pg_config_file_path,
    });

    let mut file_stream = docker.download_from_container(&container.id, options);

    let (sender, receiver) = mpsc::channel::<Vec<u8>>(128);
    let pg_config_handle = task::spawn_blocking(move || {
        let mut pg_config_buffer = Vec::new();
        let mut pg_config_archive = Archive::new(ByteStreamReceiver {
            receiver,
            buffer: Vec::new(),
        });
        if let Ok(entries) = pg_config_archive.entries() {
            for entry in entries {
                if let Ok(mut entry) = entry {
                    dbg!(entry.path()?);
                    if entry.path()?.to_str() == Some(".pg_config") {
                        entry.read_to_end(&mut pg_config_buffer)?;
                        return Ok::<_, anyhow::Error>(Some(pg_config_buffer));
                    }
                }
            }
        }
        Ok(None)
    });
    while let Some(next) = file_stream.next().await {
        match next {
            Ok(bytes) => {
                println!("sending {}", bytes.len());
                sender.send(bytes.into()).await?;
            }
            Err(err) => {
                return Err(err)?;
            }
        }
    }
    dbg!();
    drop(sender);
    let pg_config = String::from_utf8(
        pg_config_handle
            .await
            .unwrap()
            .unwrap_or_default()
            .unwrap_or_default(),
    )?;

    println!("{}", pg_config);

    // TODO: name what these what they are called in pg_config output
    let lib_dir = format!("/app/target/release/{extension_name}-pg15/usr/lib/postgresql/15/lib/");
    let extensions_dir =
        format!("/app/target/release/{extension_name}-pg15/usr/share/postgresql/15/extension/");

    let binary_file = format!("{extension_name}.so");
    let control_file = format!("{extension_name}.control");
    let sql_file = format!("{extension_name}--{extension_version}.sql");

    let files_to_copy = vec![
        (lib_dir, binary_file),
        (extensions_dir.clone(), control_file),
        (extensions_dir.clone(), sql_file),
    ];

    fs::create_dir_all(output_path)?;

    for file in files_to_copy {
        let file_dir = file.0;
        let file_name = file.1;
        let file_path = format!("{file_dir}{file_name}");

        let options = Some(DownloadFromContainerOptions { path: file_path });

        let mut file_stream = docker.download_from_container(&container.id, options);

        let mut file = File::create(format!("{output_path}/{file_name}"))?;
        while let Some(next) = file_stream.next().await {
            match next {
                Ok(bytes) => {
                    file.write_all(&bytes).unwrap();
                }
                Err(err) => {
                    return Err(err)?;
                }
            }
        }
    }

    // stop the container
    // docker.stop_container(&container.id, None).await?;

    Ok(())
}
