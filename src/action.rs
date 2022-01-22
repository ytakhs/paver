use crate::backend::Backend;
use crate::error::Result;

use std::collections::HashMap;

pub trait Action<B>
where
    B: Backend,
{
    fn run(&self, backend: B) -> Result<()>;
}

pub trait ActionBuilder<B>
where
    B: Backend,
{
    fn build(&self, raw_options: serde_yaml::Value) -> Result<Box<dyn Action<B>>>;
}

pub struct Storage<B>
where
    B: Backend,
{
    storage: HashMap<String, Box<dyn ActionBuilder<B>>>,
}

impl<B> Storage<B>
where
    B: Backend,
{
    pub fn new() -> Self {
        let storage = HashMap::new();

        Self { storage }
    }

    pub fn store(&mut self, key: String, action: Box<dyn ActionBuilder<B>>) {
        self.storage.insert(key, action);
    }

    pub fn fetch(&self, key: &String) -> Option<&Box<dyn ActionBuilder<B>>> {
        self.storage.get(key)
    }
}
