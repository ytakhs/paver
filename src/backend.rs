pub mod local;
use crate::error::Result;

pub trait Backend {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<()>;
}
