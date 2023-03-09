use super::SubCommand;
use crate::package::Package;
use crate::repository::local::LocalRepository;
use crate::repository::WriteableRepository;
use async_trait::async_trait;
use clap::Args;
use std::path::PathBuf;
use tokio_task_manager::Task;

#[derive(Args)]
pub struct PublishCommand {
    #[arg(long = "file", short = 'f')]
    file: Option<PathBuf>,

    #[arg(long = "repository", short = 'r', default_value = ".trunk-repo")]
    repo: PathBuf,
}

#[async_trait]
impl SubCommand for PublishCommand {
    async fn execute(&self, _task: Task) -> Result<(), anyhow::Error> {
        let mut repository = LocalRepository::new(self.repo.clone());
        let mut package = Package::new(
            self.file
                .clone()
                .ok_or(anyhow::Error::msg("file is required"))?,
        )?;
        repository.publish(&mut package).await?;
        Ok(())
    }
}
