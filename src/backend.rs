pub mod local;
use crate::error::Result;
use std::process::ExitStatus;

pub trait Backend {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<BackendOutput>;
}

pub struct BackendOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}
