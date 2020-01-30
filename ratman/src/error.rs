//! Ratman specific network error types

/// A Ratman specific result wrapper
pub type Result<T> = std::result::Result<T, Error>;

/// A Ratman error type
pub enum Error {
    /// An error occured during router initialisation
    InitFailed,
    /// The router is already initialised and can't be modified
    AlreadyInit,
    /// While sending an encoding operation failed
    EncodeFailed,
    /// While sending, a dispatch operation failed
    ///
    /// What this usually means is that there was some underlying
    /// network module failure that prevented Ratman from sending at
    /// least part of the message.
    DispatchFaled,
    /// The provided payload was too large and was rejected
    PayloadTooLarge,
}
