use std::include_str;
use std::path::Path;
use tar::{Header};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PgxBuildError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn build_pgx(path: &Path, _output_path: &str) -> Result<(), PgxBuildError> {
    // your code for building a pgx extension goes here
    println!("Building pgx extension at path {}", &path.display());
    let dockerfile = include_str!("./pgx_builder/Dockerfile");
    println!("{dockerfile}");

    let mut tar = tar::Builder::new(Vec::new());
    tar.append_dir_all(".", path)?;

    let mut header = Header::new_gnu();
    header.set_size(dockerfile.len() as u64);
    header.set_cksum();
    tar.append_data(&mut header, "Dockerfile", dockerfile.as_bytes())?;
    println!("{:?}", tar.into_inner().unwrap());
    Ok(())
}
