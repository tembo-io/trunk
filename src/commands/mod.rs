pub mod build;
pub mod install;
mod pgx;
pub mod publish;

use clap::ValueEnum;

pub trait SubCommand {
    fn execute(&self);
}
