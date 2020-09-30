//! TCP ovelay specific error handling

use failure::Fail;

pub type Result<T> = std::result::Result<T, Error>;

/// A generic initialisation error
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "the selected mode does not allow for this operation")]
    InvalidMode,
    #[fail(display = "failed to initialise socket: invalid address")]
    InvalidAddr,
    #[fail(display = "failed to send packet!")]
    FailedToSend
}

impl From<async_std::io::Error> for Error {
    fn from(e: async_std::io::Error) -> Self {
        use async_std::io::ErrorKind::*;
        match e.kind() {
            PermissionDenied | AddrInUse | AddrNotAvailable => Self::InvalidAddr,
            e => panic!("Unhandled io error: `{:?}`", e),
        }
    }
}
