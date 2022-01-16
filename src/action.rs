use crate::error::Result;

pub trait Action {
    fn apply(&self) -> Result<()>;
}

struct ActionStorage {}
