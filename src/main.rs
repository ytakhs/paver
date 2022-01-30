mod action;
mod backend;
mod builtins;
mod cli;
mod error;
mod template;

use cli::run_command;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    match run_command() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);

            std::process::exit(1);
        }
    }
}
