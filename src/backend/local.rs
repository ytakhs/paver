use super::{Backend, BackendOutput};
use crate::error::{Error, Result};
use std::process::Command;

pub struct LocalBackend;

impl Backend for LocalBackend {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<BackendOutput> {
        let mut command = Command::new(cmd);
        command.args(args);
        let output = command.output().or(Err(Error::CommandError))?;

        let stdout = std::str::from_utf8(&output.stdout)
            .or(Err(Error::CommandError))
            .map(|s| s.to_string())?;

        let stderr = std::str::from_utf8(&output.stderr)
            .or(Err(Error::CommandError))
            .map(|s| s.to_string())?;

        let status = output.status;

        Ok(BackendOutput {
            stdout,
            stderr,
            status,
        })
    }
}
