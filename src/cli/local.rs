use crate::action::Storage;
use crate::backend::local::LocalBackend;
use crate::builtins;
use crate::error::{Error, Result};

use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;
use tera::{Context, Tera};

#[derive(Deserialize, Debug)]
struct Job {
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
        tera.add_template_file(self.filepath.as_str(), None)
            .or(Err(Error::CommandError))?;

        let content = tera
            .render(self.filepath.as_str(), &Context::new())
            .map_err(|e| dbg!(e))
            .or(Err(Error::CommandError))?;

        let mut value: HashMap<String, Value> =
            serde_yaml::from_str(content.as_str()).or(Err(Error::CommandError))?;

        if let Some(val) = value.remove("jobs") {
            let jobs: HashMap<String, Job> =
                serde_yaml::from_value(val).or(Err(Error::CommandError))?;

            for (name, t) in jobs {
                dbg!(name);

                if let Some(action_builder) = storage.fetch(&t.action_name) {
                    let backend = LocalBackend {};
                    let action = action_builder.build(backend, t.options)?;

                    action.run()?;
                }
            }
        }

        Ok(())
    }
}
