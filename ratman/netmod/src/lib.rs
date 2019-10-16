//! `netmod` is a network module abstraction for `RATMAN`
//!
//! It provides a small interface to interact with endpoints
//! (send/ receive frames) and basic data frame definitions.
//!
//! The interface itself makes no assumption about underlying
//! address spacing or resend behaviour.
//! Using `netmod` as a library allows you to write
//! RATMAN-compatible network adapters.
//!
//! An easy example of this is `netmod-fake` which simulates a network

mod frame;
mod payload;
mod result;


pub use frame::Frame;
pub use payload::Payload;
pub use result::{Error as NetError, Result as NetResult};

// A `RATMAN` `netmod` endpoint describes a networking interface
pub trait Endpoint {
    /// Provides maximum frame-size information to `RATMAN`
    fn size_hint(&self) -> usize;

    /// Send a message to a specific endpoint (client)
    fn send(&mut self, frame: Frame) -> NetResult<()>;

    /// Get next available Frame, without blocking
    ///
    /// Because the poll might not have data to return, a valid, but
    /// not fatal return is `Ok(None)`, which means that the
    /// connection is healthy, but in the last poll cycle no new data
    /// was transmitted.
    ///
    /// **ASYNC THIS** this is a prime candidate to use async/await
    fn poll(&mut self) -> NetResult<Option<Frame>>;

    /// Setup a listener via a handler function
    ///
    /// This function assumes that relevant state can be captured via
    /// the handler's closure, meaning that no data needs to be
    /// returned from the function for it to process incoming frames.
    ///
    /// For a more "classical" poll function, see `poll` instead
    fn listen(&mut self, handler: Box<FnMut(Frame) -> NetResult<()>>) -> NetResult<()>;
}
