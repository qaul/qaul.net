//! Error handling types

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Error {
    NotAValidEndpoint,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

/// Implement common error functions
impl StdError for Error {}

/// Export `Result` as a `netmod` specific type
pub type Result<T> = std::result::Result<T, Error>;