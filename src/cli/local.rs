use crate::action::Storage;
use crate::backend::local::LocalBackend;
use crate::builtins;
use crate::Result;

use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;
use tera::{Context, Tera};

#[derive(Deserialize, Debug)]
struct Job {
    steps: Vec<Step>,
}

#[derive(Deserialize, Debug)]
struct Step {
    #[serde(rename(deserialize = "action"))]
    action_name: String,
    options: Value,
}

pub struct Local {
    filepath: String,
}

impl Local {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    pub fn run(&self) -> Result<()> {
        let mut storage = Storage::new();
        storage.store("git".to_string(), Box::new(builtins::git::GitBuilder {}));

        let mut tera = Tera::default();
        tera.add_template_file(self.filepath.as_str(), None)?;

        let content = tera.render(self.filepath.as_str(), &Context::new())?;
        let mut value: HashMap<String, Value> = serde_yaml::from_str(content.as_str())?;

        if let Some(val) = value.remove("jobs") {
            let jobs: HashMap<String, Job> = serde_yaml::from_value(val)?;

            for (name, job) in jobs {
                dbg!(name);

                for s in job.steps {
                    if let Some(action_builder) = storage.fetch(&s.action_name) {
                        let backend = LocalBackend {};
                        let action = action_builder.build(backend, s.options)?;

                        action.run()?;
                    }
                }
            }
        }

        Ok(())
    }
}
