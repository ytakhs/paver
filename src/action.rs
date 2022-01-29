use crate::backend::Backend;
use crate::Result;

pub trait Action {
    type Options;

    fn new(options: Self::Options) -> Self;
    fn run(&self, backend: impl Backend) -> Result<()>;
}
