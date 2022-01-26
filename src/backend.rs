pub mod local;
use crate::Result;
use std::process::ExitStatus;

pub trait Backend {
    fn run_command<I, S>(&self, cmd: &str, args: I) -> Result<BackendOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>;
}

pub struct BackendOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}
