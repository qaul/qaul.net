//! Common `libqaul` error type

/// `libqaul` service Errors
#[derive(Debug)]
pub enum Error {
    Unknown
}

// Required by error trait
use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "libqual Error: ")?;
        match self {
            Error::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::error::Error for Error { }

// To allow us to use ? in handlers
use iron::error::IronError;
impl From<Error> for IronError {
    fn from(e: Error) -> IronError {
        use iron::status::Status;
        IronError::new(e, Status::InternalServerError)
    }
}

/// Convenience type around `libqaul` Errors
pub type Result<T> = std::result::Result<T, Error>;
