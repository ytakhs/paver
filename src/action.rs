use crate::backend::Backend;
use crate::Result;

pub trait Action {
    fn run(&self, backend: impl Backend) -> Result<()>;
}
