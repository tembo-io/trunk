use crate::package::Package;
use crate::repository::WriteableRepository;
use async_trait::async_trait;
use std::fs::File;
use std::path::PathBuf;

pub struct LocalRepository {
    directory: PathBuf,
}

impl LocalRepository {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }
}

#[async_trait]
impl WriteableRepository for LocalRepository {
    type Output = ();
    type Error = anyhow::Error;

    async fn publish(&mut self, package: &mut Package) -> Result<Self::Output, Self::Error> {
        tokio::fs::create_dir_all(&self.directory).await?;
        let manifest = package.manifest()?;
        let file = package.original_file()?;
        use read_write_pipe::*;
        let mut dest = File::create(self.directory.join(format!(
                "{}-{}.tar.{}",
                manifest.extension_name,
                manifest.extension_version,
                package
                    .original_file_path()
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap()
            )))?;
        dest.write_reader(file)?;
        Ok(())
    }
}
