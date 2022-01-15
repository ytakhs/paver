use crate::error::{Error, Result};

use std::path::Path;
use std::process;

pub struct Git {
    dest: String,
    repository: String,
    recursive: bool,
    depth: Option<usize>,
    branch: Option<String>,
}

impl Git {
    pub fn new<T>(dest: T) -> Self
    where
        T: Into<String>,
    {
        let dest = dest.into();
        let repository = "git@github.com:ytakhs/pvner.git".to_string();
        let recursive = false;
        let depth = None;
        let branch = Some("HEAD".to_string());

        Self {
            dest,
            repository,
            recursive,
            branch,
            depth,
        }
    }

    pub fn apply(&self) -> Result<()> {
        let dest_path = std::path::Path::new(self.dest.as_str());
        if !dest_path.exists() {
            self.git_clone()?;
        }

        Ok(())
    }

    fn git_clone(&self) -> Result<()> {
        let mut command = process::Command::new("git");
        command.arg("clone");

        if self.recursive {
            command.arg("--recursive");
        }
        if let Some(d) = &self.depth {
            command.args(["--depth", d.to_string().as_str()]);
        }
        if let Some(b) = &self.branch {
            command.args(["--branch", b.to_string().as_str()]);
        }

        command.arg(self.repository.as_str());
        command.arg(self.dest.as_str());

        command.output().or(Err(Error::CommandError))?;

        Ok(())
    }

    fn run_git_command_in_repo<I, S>(&self, args: I) -> Result<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let mut command = process::Command::new("git");
        command.current_dir(&self.dest);
        command.args(args);
        let output = command.output().or(Err(Error::CommandError))?;
        let result = std::str::from_utf8(&output.stdout)
            .or(Err(Error::CommandError))
            .map(|s| s.to_string())?;

        Ok(result)
    }
}

fn first_output(output_str: &str) -> Result<String> {
    output_str
        .split("\n")
        .next()
        .ok_or(Error::CommandError)
        .map(|s| s.to_string())
}
