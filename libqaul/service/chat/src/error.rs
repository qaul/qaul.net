use libqaul::error::Error as QError;

/// A result alias for qaul-chat
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoSuchRoom,
    Libqaul(QError),
    #[doc(hidden)]
    Timeout,
}

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
