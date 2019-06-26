//! `libqaul` service API error handling
//!
//! This module exposes API specific errors,
//! that can wrap a various number of other `libqaul`
//! internal errors.

/// Convenience type for API functions
pub type QResult<T> = Result<T, Error>;

/// Service API error wrapper
pub enum Error {
    Unknown,
}
