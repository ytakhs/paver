use crate::action::Action;
use crate::builtins;
use crate::error::Result;

pub struct Apply {
    filepath: String,
}

impl Apply {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    pub fn run(&self) -> Result<()> {
        builtins::git::Git::new().run()?;

        Ok(())
    }
}
