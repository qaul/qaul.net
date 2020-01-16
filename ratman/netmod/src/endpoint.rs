//! Endpoint abstraction module

use crate::{Frame, Result, Target};
use async_trait::async_trait;

/// A `RATMAN` `netmod` endpoint describes a networking interface
///
/// For more information about the rationale of this interface, check
/// the `netmod` crate documentation!
#[async_trait]
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
    /// The target ID is a way to instruct a netmod where to send a
    /// frame in a one-to-many mapping.  When implementing a
    /// one-to-one endpoint, this ID can be ignored (set to 0).
    ///
    /// **IMPORTANT** setting the target to `-1` should result in a
    /// broadcast on that channel.
    ///
    /// Send errors _can_ be encoded in the return value, and so if
    /// physically a `Frame` is too large for the transport layer,
    /// using the error value `Error::FrameTooLarge` is permitted.
    async fn send(&mut self, frame: Frame, target: Target) -> Result<()>;

    /// Get next available Frame from the network, or an error explaining why that's
    /// not a possibility.
    async fn next(&mut self) -> Result<(Frame, Target)>;
}
