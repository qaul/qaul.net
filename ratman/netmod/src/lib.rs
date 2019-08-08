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
pub use result::{Error as NetError, Result as NetResult};

// A `RATMAN` `netmod` endpoint describes a networking interface
pub trait Endpoint {

    /// Provides maximum frame-size information to `RATMAN`
    fn size_hint(&self) -> usize;
    
    /// Send a message to a specific endpoint (client)
    fn send(&mut self, frame: Frame) -> NetResult<()>;

    /// Listen for messages from a specific sender
    fn listen(&mut self, sender: impl Endpoint) -> NetResult<Frame>;

    /// Setup a listener that will call a function on a structure that was received from the network
    fn listen_all<F: 'static, E: Endpoint>(&mut self, handler: F)
    where
        F: FnMut(E, Frame) -> NetResult<()>;
}
