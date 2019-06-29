use iron::{
    error::{
        HttpError,
        IronError,
    },
    status::Status,
};
use std::{
    error::Error as StdError,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
};

pub type QaulResult<T> = Result<T, Error>;

/// Abstraction over several internal qual errors
///
/// Should probably not be used on anything external facing as it's rather
/// unhelpful if you don't have access to the internals
#[derive(Debug)]
pub enum Error {
    HttpError(HttpError),
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Qaul Error: ")?;
        match self {
            Error::HttpError(e) => write!(f, "Http ({})", e),
            Error::Unknown => write!(f, "Unknown"),
        }
    }
}

impl StdError for Error { }

// allows using ? from handlers
impl From<Error> for IronError {
    fn from(e: Error) -> IronError {
        IronError::new(e,  Status::InternalServerError)
    }
}

impl From<HttpError> for Error {
    fn from(e: HttpError) -> Error {
        Error::HttpError(e)
    }
}
