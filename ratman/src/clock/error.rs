use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

/// An Error type for various clock related failures
///
/// These are returned when creating and validating clock inputs.
/// Because internal tasks are running detached with no way to return
/// Errors to the library user, the core will simply crash if the
/// results are ignored.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    /// Either the Interval or offset was invalid
    InvalidTime,
    /// Set the clock type to `Stepped` but didn't provide a fence
    NoFence,
    /// No interval was provided
    NoInterval
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidTime => "Provided time was invalid (probably 0)",
                Error::NoFence => "Stepped is impossible without providing a fence",
                Error::NoInterval => "No interval known for a clock value",
            }
        )
    }
}

impl StdError for Error {}
