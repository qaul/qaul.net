//! Endpoint abstraction module

use crate::{Frame, Result};

/// A `RATMAN` `netmod` endpoint describes a networking interface
///
/// For more information about the rationale of this interface, check
/// the `netmod` crate documentation!
pub trait Endpoint {
    /// Return a desired frame size in bytes
    ///
    /// `R.A.T.M.A.N.` uses this metric to slice `Message` payloads
    /// into `Frame` sequencies, that are easier for a network to
    /// handle than large packets. While diagnostics can vary heavily
    /// between `netmod` implementations, this interface allows the
    /// slicing logic to live in the routing abstraction layer,
    /// without having to be aware of hardware specifics.
    ///
    /// **Note**: It is therefore desired for this hint, despite only
    /// being a *hint* to be accurate and honest. `R.A.T.M.A.N.`
    /// reserves the right however to ignore a *hint* and so a
    /// `netmod` implementation **must not** panic on a `Frame` that
    /// exceeds the size "limit".
    fn size_hint(&self) -> usize;

    // Returns a number between 0 and 255
    // fn link_strength(&self) -> u8;

    /// Dispatch a `Frame` into the network
    ///
    /// Sending characteristics are entirely up to the `netmod`
    /// implementation. As mentioned in the `size_hint` documentation,
    /// this function **must not** panic on a `Frame` for size reasons.
    ///
    /// Send errors _can_ be encoded in the return value, and so if
    /// physically a `Frame` is too large for the transport layer,
    /// using the error value `Error::FrameTooLarge` is permitted.
    ///
    /// **NOTE: ASYNC THIS**
    fn send(&mut self, frame: Frame) -> Result<()>;

    /// Get next available Frame, without blocking
    ///
    /// Because the poll might not have data to return, a valid, but
    /// not fatal return is `Ok(None)`, which means that the
    /// connection is healthy, but in the last poll cycle no new data
    /// was transmitted.
    ///
    /// **NOTE: ASYNC THIS**
    fn poll(&mut self) -> Result<Option<Frame>>;

    /// Setup a listener via a handler function
    ///
    /// This function assumes that relevant state can be captured via
    /// the handler's closure, meaning that no data needs to be
    /// returned from the function for it to process incoming frames.
    ///
    /// For a more "classical" poll function, see `poll` instead
    fn listen(&mut self, handler: Box<dyn FnMut(Frame) -> Result<()>>) -> Result<()>;
}
