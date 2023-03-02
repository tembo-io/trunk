pub mod build;
pub mod install;
pub mod publish;
mod pgx;

use clap::ValueEnum;

pub trait SubCommand {
    fn execute(&self);
}
