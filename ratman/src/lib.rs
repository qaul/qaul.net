//! RATMAN decentralised routing protocol
//!
//! A modern approach to fully delay-tolerant mesh routing,
//! implemented network agnostically and entirely in userspace.

mod core;
mod data;
mod journal;
mod protocol;
mod slicer;

pub use crate::{
    data::{Message, Payload, Signature},
    protocol::Protocol,
};
pub use netmod;

use crate::{core::Core, journal::Journal};
use netmod::Endpoint;

use std::sync::{Arc, Mutex};

/// A `RATMAN` router context
pub struct Router {
    core: Arc<Mutex<Core>>,
    journal: Arc<Journal>,
}

impl Router {
    pub fn new() -> Self {
        let (core, j_rcv) = Some(Core::new())
            .map(|(c, r)| (Arc::new(Mutex::new(c)), r))
            .unwrap();
        let (journal, d_send) = Some(Journal::start(j_rcv, Arc::clone(&core)))
            .map(|(j, s)| (Arc::new(j), s))
            .unwrap();

        Self { core, journal }
    }

    /// Add an `netmod` endpoint to this router
    pub fn add_ep(&self, ep: impl Endpoint + 'static + Send) {
        self.core.lock().unwrap().add_if(ep);
    }
}
