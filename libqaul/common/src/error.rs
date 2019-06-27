//! Common `libqaul` error type

/// `libqaul` service Errors
pub enum Error {
    Unknown
}

/// Convenience type around `libqaul` Errors
pub type Result<T> = std::result::Result<T, Error>;
