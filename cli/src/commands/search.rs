use clap::{Args, Subcommand};
use tokio_task_manager::Task;

use super::SubCommand;

#[derive(Args)]
pub struct SearchCommand {
    #[command(subcommand)]
    inner: SearchSubcommand,
}

#[derive(Subcommand)]
pub enum SearchSubcommand {
    Extension(FindExtension),
    Contains(DescriptionContains),
}

/// Look for extensions whose name contains the given term
#[derive(Args)]
pub struct FindExtension {
    term: String,
}

/// Look for extensions whose descriptions contains the given terms
#[derive(Args)]
pub struct DescriptionContains {
    terms: Vec<String>,
}

#[async_trait::async_trait]
impl SubCommand for SearchCommand {
    async fn execute(&self, _: Task) -> Result<(), anyhow::Error> {
        // Stub
        Ok(())
    }
}
