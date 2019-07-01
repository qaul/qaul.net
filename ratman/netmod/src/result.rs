//! Error handling types

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A collection of errors around sending data frames
#[derive(Debug)]
pub enum Error {
    /// Some operations are not implemented by all backends
    OperationNotSupported,
    /// The required recipient wasn't found
    RecipientUnknown,
    /// The constructed frame was too large to send
    FrameTooLarge,
    /// The connection was lost mid-transfer
    ConnectionLost,
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
