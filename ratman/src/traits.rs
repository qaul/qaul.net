//! A collection of traits to power the routing core

use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

use ::errors::RouteError;

/// A common endpoint trait that is used to emulate
/// receivers on the network
pub trait Endpoint {
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
pub trait Router {
    /// Initialise the router
    ///
    /// It takes a list of Endpoint which will represent itself in the network. Different
    /// endpoint types can co-exist, meaning that a router can connect between different
    /// backend backplanes, to allow transparent routing.
    /// 
    /// If a `None` type is provided it is possible to implement an anonymous
    /// (transparent) router as well. Although that feature might not be supported
    /// by all implementations (thus, see `Result<Self, RouteError>`)
    fn initialise<S: Router>(with: Option<Vec<impl Endpoint>>) -> Result<S, RouteError>;

    /// Broadcast some data into the network
    fn broadcast(&mut self, data: impl Serialize) -> Result<(), RouteError>;

    /// Send a message to a specific endpoint (client)
    fn send(&mut self, target: impl Endpoint, data: impl Serialize) -> Result<(), RouteError>;

    /// Listen for messages from a specific sender
    fn listen<T: DeserializeOwned>(&mut self, sender: impl Endpoint) -> Result<T, RouteError>;

    /// Setup a listener that will call a function on a structure that was received from the network
    fn listen_all<T: DeserializeOwned, F: 'static, E: Endpoint>(&mut self, handler: F)
    where
        F: FnMut(E, T) -> Result<(), RouteError>;

    /// Safely shut-down this router
    fn shutdown(&mut self) -> Result<(), RouteError>;
}
