use crate::action::Action;
use crate::backend::Backend;
use crate::error::Error;
use crate::Result;

use serde::Deserialize;

pub struct Git {
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

impl TryFrom<serde_yaml::Value> for Git {
    type Error = crate::error::Error;

    fn try_from(value: serde_yaml::Value) -> Result<Git> {
        let options = serde_yaml::from_value(value)?;

        Ok(Git { options })
    }
}

impl Action for Git {
    fn run(&self, backend: impl Backend) -> Result<()> {
        backend.check_commands_available(&["git"])?;

        let output = backend.run_command("ls", ["-A", self.options.dest.as_str()])?;
        let result = backend.run_command("test", ["-z", output.stdout.as_str()])?;

        if result.status.success() {
            self.git_clone(backend)?;
        }

        Ok(())
    }
}

impl Git {
    fn git_clone(&self, backend: impl Backend) -> Result<()> {
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

        let output = backend.run_command("git", args)?;
        if !output.status.success() {
            return Err(Error::ActionError(output.stderr));
        }

        Ok(())
    }
}
