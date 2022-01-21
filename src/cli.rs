mod local;

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
    Local { filepath: String },
}

pub fn run_command() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        SubCommand::Local { filepath } => local::Local::new(filepath.clone()).run(),
    }
}
