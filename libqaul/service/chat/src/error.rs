use libqaul::error::Error as QError;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

/// A result alias for qaul-chat
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoSuchRoom,
    Libqaul(QError),
    #[doc(hidden)]
    Timeout,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSuchRoom => write!(f, "No such room!"),
            Self::Libqaul(e) => write!(f, "{}", e),
            Self::Timeout => write!(f, "A timeout has occured waiting for this operation"),
        }
    }
}

impl StdError for Error {}

impl From<QError> for Error {
    fn from(q: QError) -> Self {
        Self::Libqaul(q)
    }
}

impl From<async_std::future::TimeoutError> for Error {
    fn from(_: async_std::future::TimeoutError) -> Self {
        Self::Timeout
    }
}
