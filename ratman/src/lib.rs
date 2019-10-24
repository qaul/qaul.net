//! `R.A.T.M.A.N.` decentralised routing protocol
//!
//! <small> Nananananananana Nananananananana RAT MAN!</small>
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

use crate::{
    core::{Core, Envelope},
    journal::Journal,
    slicer::Slicer,
};
use netmod::{Endpoint, Recipient};
use std::sync::{mpsc::Receiver, Arc, Mutex};

/// A temporary structure used to initialise a `R.A.T.M.A.N.` `Router`
///
/// Use this structure only for destructuring, it has no useful
/// attributes beyond named fields.
pub struct RouterInit {
    router: Router,
    channel: Receiver<Message>,
}

/// A `R.A.T.M.A.N.` routing context
///
/// Handles `Message` splicing, `Frame` sequencing, routing tables,
/// journal keeping, re-transmissions of non-local frames, as well as
/// the discovery protocol.
pub struct Router {
    core: Arc<Mutex<Core>>,
    journal: Arc<Journal>,
}

impl Router {
    /// Create a new `Router` with internal mutability
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

    /// Queue a `R.A.T.M.A.N.` message for sending
    pub fn send(&self, msg: Message) {
        // FIXME: This is so pointless...
        let recp = msg.recipient.clone();

        let frames = Slicer::slice(0, msg);
        let core = self.core.lock().unwrap();

        core.send(match recp {
            Recipient::User(ref id) => core.lookup(id, frames),
            Recipient::Flood => core
                .get_ifs()
                .into_iter()
                .fold(vec![], |mut vec, (ifid, _)| {
                    let mut set = frames.iter().cloned().map(|f| Envelope(ifid, f)).collect();
                    vec.append(&mut set);
                    vec
                }),
        });
    }
}
