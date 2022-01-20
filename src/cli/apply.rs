use crate::action::Storage;
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

pub struct Apply {
    filepath: String,
}

impl Apply {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    pub fn run(&self) -> Result<()> {
        let mut storage = Storage::new();
        let key = "git".to_string();

        let content =
            std::fs::read_to_string(self.filepath.as_str()).or(Err(Error::CommandError))?;

        storage.store(key.clone(), Box::new(builtins::git::GitBuilder {}));

        let mut value: std::collections::HashMap<String, Value> =
            serde_yaml::from_str(content.as_str()).or(Err(Error::CommandError))?;

        if let Some(val) = value.remove("tasks") {
            let tasks: Vec<Task> = serde_yaml::from_value(val).or(Err(Error::CommandError))?;

            for t in tasks {
                if let Some(action_builder) = storage.fetch(&t.action_name) {
                    let action = action_builder.build(t.options)?;

                    action.run()?;
                }
            }
        }

        Ok(())
    }
}
