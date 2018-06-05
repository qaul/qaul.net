//! Primary routing code components

extern crate serde;

use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
use std::io::Error as IoError;

/// A common endpoint trait that is used to emulate
/// receivers on the network
trait Endpoint {
    fn name(&self) -> Cow<&str>;
    fn address(&self) -> Cow<&str>;
    fn version(&self) -> Cow<&str>;
}

/// The primary router trait that drives the routing core
/// and builds an abstraction layer on top of different
/// networking solutions.
///
/// A router needs to be able to handle a few common requests,
/// the rest of which is implementation detail left to the
/// end-developer of a routing component
trait Router {
    /// Initialise the router
    fn initialise() -> Self;

    /// Broadcast some data
    fn broadcast(&mut self, data: impl Serialize) -> Result<(), IoError>;

    /// Send a message to a specific endpoint (client)
    fn send(&mut self, target: impl Endpoint, data: impl Serialize) -> Result<(), IoError>;

    /// Listen for messages from a specific sender
    fn listen<T: DeserializeOwned>(&mut self, sender: impl Endpoint) -> Result<T, IoError>;

    /// Setup a listener that will call a function on a structure that was received from the network
    fn listen_all<T: DeserializeOwned, F: 'static, E: Endpoint>(&mut self, handler: F)
    where
        F: FnMut(E, T) -> Result<(), IoError>;

    /// Safely shut-down this router
    fn shutdown() -> Result<(), IoError>;
}
