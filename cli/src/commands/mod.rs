use async_trait::async_trait;
use tokio_task_manager::Task;

pub mod build;
pub mod categories;
mod containers;
mod generic_build;
pub mod install;
pub mod license;
mod pgrx;
pub mod publish;
pub mod search;
pub mod test;

#[async_trait]
pub trait SubCommand {
    async fn execute(&self, task: Task) -> Result<(), anyhow::Error>;
}
