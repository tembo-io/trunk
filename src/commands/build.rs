use super::SubCommand;
use clap::Args;

#[derive(Args)]
pub struct BuildCommand {
    #[arg(long = "path", default_value = ".")]
    path: String,
}

impl SubCommand for BuildCommand {
    fn execute(&self) {
        println!("{}", self.path);
        println!("trunk build: not implemented")
    }
}
