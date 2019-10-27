//! `libqaul` Error and Result handling
//!
//!

use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    result::Result as StdResult,
};

/// Convenience type for API functions
pub type Result<T> = StdResult<T, Error>;

/// `libqaul` service API error states
///
/// All errors that can occur in interaction with the API are encoded
/// as variants on this enum. In most cases, no additional metadata is
/// provided and needs to be inferred from whatever context or
/// function call emitted the error. Check the variant doc comments
/// for a broad overview, as well as detailed usage instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Not authorised to perform this action
    NotAuthorised,
    /// The desired user was not known
    UnknownUser,
    /// Invalid search query
    InvalidQuery,
    /// Invalid payload (probably too big)
    InvalidPayload,
    /// A function callback timed out
    CallbackTimeout,
    /// Signature with an unknown public key
    UnknownSign,
    /// Fraudulent signature for a known public key
    BadSign,
    /// A generic networking error occured
    NetworkError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                _ => "An unknown Error occured",
            }
        )
    }
}

impl StdError for Error {}
