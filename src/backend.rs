pub mod local;
use crate::Result;
use std::process::ExitStatus;

pub trait Backend {
    fn run_command<I, S>(&self, cmd: &str, args: I) -> Result<BackendOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>;

    fn check_commands_available(&self, cmds: &[&str]) -> Result<()> {
        for cmd in cmds {
            self.run_command("which", [cmd])?;
        }

        Ok(())
    }
}

pub struct BackendOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}
