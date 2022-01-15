mod builtins;
mod cli;
mod error;

use cli::run_command;

fn main() {
    match run_command() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);

            std::process::exit(1);
        }
    }
}
