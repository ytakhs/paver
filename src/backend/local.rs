use super::{Backend, BackendOutput};
use crate::Result;
use std::process::Command;

pub struct LocalBackend;

impl Backend for LocalBackend {
    fn run_command<I, S>(&self, cmd: &str, args: I) -> Result<BackendOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let mut command = Command::new(cmd);
        command.args(args);
        let output = command.output()?;

        let stdout = std::str::from_utf8(&output.stdout).map(|s| s.to_string())?;

        let stderr = std::str::from_utf8(&output.stderr).map(|s| s.to_string())?;

        let status = output.status;

        Ok(BackendOutput {
            stdout,
            stderr,
            status,
        })
    }
}
