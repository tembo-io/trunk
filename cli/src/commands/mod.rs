use async_trait::async_trait;
use tokio_task_manager::Task;

pub mod build;
mod containers;
mod generic_build;
pub mod install;
mod pgrx;
pub mod publish;
pub mod license;

#[async_trait]
pub trait SubCommand {
    async fn execute(&self, task: Task) -> Result<(), anyhow::Error>;
}
