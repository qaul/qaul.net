//!

use ratman::Identity;
use std::collections::{BTreeMap, BTreeSet};
use std::{net::SocketAddr, ops::Deref};

pub type Id = usize;

/// A wrapper type for parameters that are required for an endpoint
#[derive(Clone, Debug)]
pub enum Params {
    /// Virtual testing endpoint purely in memory
    ///
    /// Because it is only used to connect with one other endpoint in
    /// memory no parameters are required to make a network of virtual
    /// endpoints work.
    Virtual,
    /// Internet tcp overlay endpoint
    ///
    /// To initialise this module requires a bind address, and a port.
    /// Optionally a `dynamic` flag can be passed along which will set
    /// up the endpoint to accept new peers introduced dynamically.
    Tcp {
        addr: String,
        port: u16,
        peers: Vec<SocketAddr>,
        dynamic: bool,
    },
    /// Purely local udp broadcast endpoint
    ///
    /// Because of how multicast works on Linux, all udp modules in a
    /// network need to be running on the same port.  This means that
    /// two udp endpoints can't be running on the same computer at the
    /// same time for testing purposes, without network namespaces.
    LocalUpd { addr: String },
    /// Android wifi direct support
    #[cfg(features = "android")]
    WifiDirect,
}

/// Configuration for a single endpoint
#[derive(Clone, Debug)]
pub struct Endpoint {
    /// A unique ID for this endpoint
    pub id: usize,
    /// Type and required parameter set for initialisation
    pub params: Params,
}

/// A network endpoint patch type
///
/// When creating internal test networks (running inside the same
/// daemon, but communicating via different endpoint backends, you can
/// chose the "internal" option to patch them together.  This means
/// manually introducing Tcp endpoints to each other, or simply
/// hooking up the internal memory channel of `netmod-mem`.
///
/// Use the `External` type to use the endpoint to configure against
/// an external target (meaning actual network traffic).
#[derive(Clone, Debug)]
pub enum Patch {
    Internal(Id),
    External,
}

/// A set of endpoints to connect to various networks
///
/// The list of endpoints defines
#[derive(Clone, Debug, Default)]
pub struct Network {
    /// Set of endpoints for this network backend
    pub endpoints: BTreeMap<Id, Endpoint>,
    /// Manual internal patch configuration
    pub patches: BTreeMap<Id, Patch>,
}

impl Network {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for Network {
    type Target = BTreeMap<usize, Endpoint>;
    fn deref(&self) -> &Self::Target {
        &self.endpoints
    }
}
