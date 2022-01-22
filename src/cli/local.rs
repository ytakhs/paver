use crate::action::Storage;
use crate::backend::local::LocalBackend;
use crate::builtins;
use crate::error::{Error, Result};

use serde::Deserialize;
use serde_yaml::Value;

#[derive(Deserialize, Debug)]
struct Task {
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

        let content =
            std::fs::read_to_string(self.filepath.as_str()).or(Err(Error::CommandError))?;
        let mut value: std::collections::HashMap<String, Value> =
            serde_yaml::from_str(content.as_str()).or(Err(Error::CommandError))?;

        if let Some(val) = value.remove("tasks") {
            let tasks: Vec<Task> = serde_yaml::from_value(val).or(Err(Error::CommandError))?;

            for t in tasks {
                if let Some(action_builder) = storage.fetch(&t.action_name) {
                    let action = action_builder.build(t.options)?;
                    let backend = LocalBackend {};

                    action.run(backend)?;
                }
            }
        }

        Ok(())
    }
}