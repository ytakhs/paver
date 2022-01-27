use crate::action::{Action, ActionBuilder};
use crate::backend::Backend;
use crate::error::Error;
use crate::Result;

use serde::Deserialize;

pub struct Git<B>
where
    B: Backend,
{
    backend: B,
    options: GitOptions,
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
    B: Backend + 'static,
{
    fn build(&self, backend: B, raw_options: serde_yaml::Value) -> Result<Box<dyn Action>> {
        let options: GitOptions = serde_yaml::from_value(raw_options)?;

        let git = Git::new(backend, options);

        Ok(Box::new(git))
    }
}

impl<B> Action for Git<B>
where
    B: Backend,
{
    fn run(&self) -> Result<()> {
        self.backend.check_commands_available(&["git"])?;

        let output = self
            .backend
            .run_command("ls", ["-A", self.options.dest.as_str()])?;

        let result = self
            .backend
            .run_command("test", ["-z", output.stdout.as_str()])?;

        if result.status.success() {
            self.git_clone()?;
        }

        Ok(())
    }
}

impl<B> Git<B>
where
    B: Backend,
{
    pub fn new(backend: B, options: GitOptions) -> Self {
        Self { backend, options }
    }

    fn git_clone(&self) -> Result<()> {
        let mut args = vec!["clone"];

        if self.options.recursive.unwrap_or(false) {
            args.push("--recursive");
        }
        if let Some(d) = &self.options.depth {
            args.append(vec!["--depth", d.to_string().as_str()]);
        }
        if let Some(b) = &self.options.branch {
            args.append(vec!["--branch", b.to_string().as_str()]);
        }

        args.push(self.options.repository.as_str());
        args.push(self.options.dest.as_str());

        let output = self.backend.run_command("git", args)?;
        if !output.status.success() {
            return Err(Error::ActionError(output.stderr));
        }

        Ok(())
    }
}
