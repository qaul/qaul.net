//! RATMAN decentralised routing protocol
//!
//! A modern approach to fully delay-tolerant mesh routing,
//! implemented network agnostically and entirely in userspace.

mod core;
mod data;
mod protocol;
mod slicer;
mod journal;

pub use crate::{
    data::{Message, Payload, Signature},
    protocol::Protocol,
};
pub use netmod;

use crate::core::Core;
use netmod::Endpoint;

/// A `RATMAN` router context
pub struct Router {
    core: Core,
}

impl Router {
    pub fn new() -> Self {
        Self { core: Core::new() }
    }

    /// Add an `netmod` endpoint to this router
    pub fn add_ep(&mut self, ep: impl Endpoint + 'static + Send) {
        self.core.add_if(ep);
    }
}
