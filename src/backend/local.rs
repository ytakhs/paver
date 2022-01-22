use super::Backend;
use crate::error::{Error, Result};
use std::process::Command;

pub struct LocalBackend;

impl Backend for LocalBackend {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<()> {
        let mut command = Command::new(cmd);

        command.args(args);

        command
            .output()
            .or(Err(Error::CommandError))
            .map(|_| Ok(()))?
    }
}
