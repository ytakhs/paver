use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "pvner")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Apply { file: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Apply { file } => {}
    }
}
