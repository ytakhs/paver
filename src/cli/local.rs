use crate::action::Action;
use crate::backend::local::LocalBackend;
use crate::builtins;
use crate::error::Error;
use crate::template;
use crate::Result;

pub struct Local {
    filepath: String,
}

impl Local {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    pub fn run(&self) -> Result<()> {
        let template = template::parse_template(self.filepath)?;

        for (name, job) in template.jobs {
            dbg!(name);

            for step in job.steps {
                let s = step
                    .into_iter()
                    .next()
                    .ok_or(Error::ActionError("step error".to_string()))?;

                let action = match s.0.as_str() {
                    "git" => builtins::git::Git::try_from(s.1)?,
                    _ => Err(Error::ActionError("".to_string()))?,
                };

                let backend = LocalBackend {};
                action.run(backend)?;
            }
        }

        Ok(())
    }
}
