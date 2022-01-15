use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("")]
    CommandError,
}

pub type Result<T> = std::result::Result<T, Error>;
