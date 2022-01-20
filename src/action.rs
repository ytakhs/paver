use crate::error::Result;

use std::collections::HashMap;

pub trait Action {
    fn run(&self) -> Result<()>;
}

pub trait ActionBuilder {
    fn build(&self, raw_options: serde_yaml::Value) -> Result<Box<dyn Action>>;
}

pub struct Storage {
    storage: HashMap<String, Box<dyn ActionBuilder>>,
}

impl Storage {
    pub fn new() -> Self {
        let storage = HashMap::new();

        Self { storage }
    }

    pub fn store(&mut self, key: String, action: Box<dyn ActionBuilder>) {
        self.storage.insert(key, action);
    }

    pub fn fetch(&self, key: &String) -> Option<&Box<dyn ActionBuilder>> {
        self.storage.get(key)
    }
}
