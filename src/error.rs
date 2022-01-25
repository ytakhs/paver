use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error")]
    CommandError,
}

pub type Result<T> = std::result::Result<T, Error>;
