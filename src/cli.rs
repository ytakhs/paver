mod apply;

use crate::error::Result;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "pvner")]
pub struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Apply { filepath: String },
}

pub fn run_command() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        SubCommand::Apply { filepath } => apply::Apply::new(filepath.clone()).run(),
    }
}
