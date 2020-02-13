//! Error handling types

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Error return types emitted by `Endpoint` implementations
///
/// Underlying hardware errors are entirely shadowed, because it
/// wouldn't reasonably be possible to encode them all and error
/// messages are prone to confusion. Instead, a simple set of common
/// hardware and buffer related errors was selected to be retunrable.
///
/// Routing layers (such as `R.A.T.M.A.N.` are expected to respond
/// gracefully to all of these errors, so none of them should be
/// considered fatal.
#[derive(Debug)]
pub enum Error {
    /// The requested operation is not supported by an adapter
    ///
    /// Valid reasons to return this error might be a routing layer
    /// trying to setup a `listen` handle on platforms that only
    /// support basic polling.
    ///
    ///This error **must not** be used for dealing with a `Frame` that
    /// exceeds available buffer capacity!
    NotSupported,
    /// The provided `Frame` was too large to send on this adapter
    ///
    /// Sometimes a routing layer (such as `R.A.T.M.A.N.`) will
    /// partially ignore the provided `size_hint` for efficiency
    /// reasons and provide a `Frame` to an adapter that is larger. If
    /// a backend has an upper size limit, encoded in the `size_hint`
    /// (or larger), and a `Frame` exceeds this limit, returning this
    /// error is permited.
    ///
    /// It will result in the routing later resubmitting a smaller
    /// `Frame` sequence.
    FrameTooLarge,
    /// During the most recent transmission a connection drop occured
    ///
    /// This error can be thrown both during `send` and `poll`, but
    /// should not be returned by `listen`, as an invalid `Frame` can
    /// simply be dropped.
    ConnectionLost,
    /// During desequencing an error occured
    DesequenceFault,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl StdError for Error {}

/// A `netmod` specific `Result` wrapper
pub type Result<T> = std::result::Result<T, Error>;
