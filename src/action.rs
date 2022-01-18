use crate::error::Result;

use std::collections::HashMap;

pub trait Action {
    fn run(&self) -> Result<()>;
}

pub struct ActionStorage {
    storage: HashMap<String, Box<dyn Action>>,
}

impl ActionStorage {
    pub fn new() -> Self {
        let storage = HashMap::new();

        Self { storage }
    }

    pub fn store(&mut self, key: String, action: Box<dyn Action>) {
        self.storage.insert(key, action);
    }

    pub fn fetch(&self, key: &String) -> Option<&Box<dyn Action>> {
        self.storage.get(key)
    }
}
