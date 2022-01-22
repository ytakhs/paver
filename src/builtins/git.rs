use crate::action::{Action, ActionBuilder};
use crate::backend::Backend;
use crate::error::{Error, Result};

use serde::Deserialize;
use std::path::Path;
use std::process;

pub struct Git {
    dest: String,
    repository: String,
    recursive: bool,
    depth: Option<usize>,
    branch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitOptions {
    dest: String,
    repository: String,
    recursive: Option<bool>,
    depth: Option<usize>,
    branch: Option<String>,
}

pub struct GitBuilder {}

impl<B> ActionBuilder<B> for GitBuilder
where
    B: Backend,
{
    fn build(&self, raw_options: serde_yaml::Value) -> Result<Box<dyn Action<B>>> {
        let options: GitOptions =
            serde_yaml::from_value(raw_options).or(Err(Error::CommandError))?;

        let git = Git::new(options);

        Ok(Box::new(git))
    }
}

impl<B> Action<B> for Git
where
    B: Backend,
{
    fn run(&self, backend: B) -> Result<()> {
        let mut command = process::Command::new("ls");
        command.args(["-A", self.dest.as_str()]);
        let output = first_output(command.output().or(Err(Error::CommandError))?)?;

        let mut command = process::Command::new("test");
        command.args(["-z", output.as_str()]);
        let result = command.output().or(Err(Error::CommandError))?;

        if result.status.success() {
            self.git_clone()?;
        }

        Ok(())
    }
}

impl Git {
    pub fn new(options: GitOptions) -> Self {
        let dest = options.dest;
        let repository = options.repository;
        let recursive = options.recursive.unwrap_or(false);
        let depth = options.depth;
        let branch = options.branch;

        Self {
            dest,
            repository,
            recursive,
            branch,
            depth,
        }
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

        let output = command.output().or(Err(Error::CommandError))?;
        if !output.status.success() {
            let err = std::str::from_utf8(&output.stderr).or(Err(Error::CommandError))?;
            eprintln!("{}", err);

            return Err(Error::CommandError);
        }

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

fn first_output(output: process::Output) -> Result<String> {
    let s = std::str::from_utf8(&output.stdout)
        .or(Err(Error::CommandError))
        .map(|s| s.to_string())?;

    s.split("\n")
        .next()
        .ok_or(Error::CommandError)
        .map(|s| s.to_string())
}
