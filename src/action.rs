use crate::error::Result;

pub trait Action {
    fn run(&self) -> Result<()>;
}

struct ActionStorage {}
