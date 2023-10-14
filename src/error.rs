use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug, Default)]
pub enum Error {
    #[default]
    #[error("unknown error source")]
    Unknown
}