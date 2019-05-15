//! `netmod` is a network module abstraction for RATMAN
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
mod result;

pub use frame::Frame;
pub use result::Result;


// A `netmod` endpoint describes a networking interface
// pub trait Endpoint {

//     /// Broadcast some data into the network
//     fn broadcast(&mut self, frame: Frame) -> Result<(), RouteError>;

//     /// Send a message to a specific endpoint (client)
//     fn send(&mut self, frame: Frame) -> Result<(), RouteError>;

//     /// Listen for messages from a specific sender
//     fn listen<T: DeserializeOwned>(&mut self, sender: impl Endpoint) -> Result<T, RouteError>;

//     /// Setup a listener that will call a function on a structure that was received from the network
//     fn listen_all<T: DeserializeOwned, F: 'static, E: Endpoint>(&mut self, handler: F)
//     where
//         F: FnMut(E, T) -> Result<(), RouteError>;

// }