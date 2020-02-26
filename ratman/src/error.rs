//! Ratman specific network error types

/// A Ratman specific result wrapper
pub type Result<T> = std::result::Result<T, Error>;

/// A Ratman error type
#[derive(Debug)]
pub enum Error {
    /// An error occured during router initialisation
    InitFailed,
    /// While sending an encoding operation failed
    EncodeFailed,
    /// Decoding a payload failed
    DecodeFailed,
    /// While sending, a dispatch operation failed
    DispatchFailed,
    /// The provided payload was too large and was rejected
    PayloadTooLarge,
    /// An action failed because of a user collision
    DuplicateUser,
    /// An action failed because of a missing user
    NoUser,
    /// Indicates that something isn't supported on the platform
    NotSupportedOnPlatform,
}

use netmod::Error as NmError;

impl From<NmError> for Error {
    fn from(e: NmError) -> Self {
        match e {
            NmError::ConnectionLost => Self::DispatchFailed,
            NmError::DesequenceFault => Self::DecodeFailed,
            NmError::FrameTooLarge => Self::PayloadTooLarge,
            NmError::NotSupported => Self::NotSupportedOnPlatform,
        }
    }
}
