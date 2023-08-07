use clap::Args;
use tokio_task_manager::Task;

use super::SubCommand;

#[derive(Args)]
pub struct TestCommand {
    name: Option<String>,
}

#[async_trait::async_trait]
impl SubCommand for TestCommand {
    async fn execute(&self, _: Task) -> Result<(), anyhow::Error> {
        // Stub
        Ok(())
    }
}
