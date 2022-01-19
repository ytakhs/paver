use crate::action::ActionStorage;
use crate::builtins;
use crate::error::{Error, Result};

pub struct Apply {
    filepath: String,
}

impl Apply {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    pub fn run(&self) -> Result<()> {
        let mut action_storage = ActionStorage::new();
        let key = "git".to_string();

        let content =
            std::fs::read_to_string(self.filepath.as_str()).or(Err(Error::CommandError))?;

        let value: serde_yaml::Value =
            serde_yaml::from_str(content.as_str()).or(Err(Error::CommandError))?;

        dbg!("{}", value);

        action_storage.store(key.clone(), Box::new(builtins::git::Git::new()));

        if let Some(g) = action_storage.fetch(&key) {
            g.run()?;
        }

        Ok(())
    }
}
