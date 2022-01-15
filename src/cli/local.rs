use crate::builtins;
use crate::error::Result;

pub struct Local {
    filepath: String,
}

impl Local {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }

    pub fn run(&self) -> Result<()> {
        builtins::git::Git::new(self.filepath.clone()).apply()?;

        Ok(())
    }
}
