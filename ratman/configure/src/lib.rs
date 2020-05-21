//! Ratman configuration toolkit
//!
//! Creating networks via Ratman is pretty easy but can involve a fair
//! amount of boilerplate.  To make the network initialisation easier
//! and less repetitive, this library is meant to handle network
//! module state and initialisation, at runtime, either via a
//! configuration language parser, or via the pure code API.

mod parser;
pub use parser::parse_json;

pub mod config;

use config::{Endpoint, Id, Network, Params, Patch};
use std::{collections::BTreeMap, net::SocketAddr};

/// A rust API builder equivalent of the json parser
///
/// You can easily construct ratman router configurations with this
/// type, either to connect to other routers across the world, or
/// locally in memory to test changes made to the router code itself.
pub struct NetBuilder {
    id_ctr: Id,
    endpoints: BTreeMap<Id, Endpoint>,
}

impl NetBuilder {
    pub fn new() -> Self {
        Self {
            id_ctr: 0,
            endpoints: BTreeMap::new(),
        }
    }

    pub fn endpoint(mut self, epb: EpBuilder) -> Self {
        let (id, ep) = epb.build(&mut self.id_ctr);
        self.endpoints.insert(id, ep);
        self
    }

    pub fn build(self) -> Network {
        Network {
            endpoints: self.endpoints,
            patches: Default::default(),
        }
    }
}

pub struct EpBuilder {
    p: Params,
}

impl EpBuilder {
    pub fn virt() -> Self {
        Self { p: Params::Virtual }
    }

    pub fn tcp<I: Into<SocketAddr>>(addr: String, port: u16, peers: Vec<I>, dynamic: bool) -> Self {
        Self {
            p: Params::Tcp {
                addr,
                port,
                peers: peers.into_iter().map(Into::into).collect(),
                dynamic,
            },
        }
    }

    pub fn local_udp(addr: String) -> Self {
        Self {
            p: Params::LocalUpd { addr },
        }
    }

    #[cfg(features = "android")]
    pub fn wifi_direct() -> Self {
        Self {
            p: Params::WifiDirect,
        }
    }

    fn build(self, id: &mut Id) -> (Id, Endpoint) {
        let this = *id;
        *id += 1;
        (
            this,
            Endpoint {
                id: this,
                params: self.p,
            },
        )
    }
}
