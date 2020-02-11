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
    /// While sending, a dispatch operation failed
    DispatchFaled,
    /// The provided payload was too large and was rejected
    PayloadTooLarge,
    /// An action failed because of a user collision
    DuplicateUser,
    /// An action failed because of a missing user
    NoUser,
}
