//! Routing journal module
#![allow(unused)]

use crate::{core::Envelope, slicer::Slicer, Core, Message};
use identity::Identity;
use netmod::Recipient;
use std::{
    collections::HashSet,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

/// Local `RATMAN` frame journal with attached workers
///
/// The journal is initialised with a receiver from the internal
/// routing worker, to handle incoming frames, as well as as callback
/// to the routing core, to re-send frames not meant for this node.
///
/// Frames addressed to a local user (that is passed into
/// initialisation) are buffered until all frames in the message
/// sequence are present.
pub(crate) struct Journal {
    local: Arc<Mutex<HashSet<Identity>>>,
    worker: JoinHandle<()>,
}

impl Journal {
    /// Start the journal management thread
    ///
    /// The discovery sender is provided by the `Router`, which
    /// returns this to any application layer that wants to hook into
    /// `RATMAN` message delivery to local devices. Any other frames,
    /// that are not addressed to a local address will be forwarded
    /// back to the routing core `send` logic. Local-addressed
    /// messages will be de-sliced and passed up the stack.
    pub(crate) fn start(
        recv: Receiver<Envelope>,
        core: Arc<Mutex<Core>>,
    ) -> (Self, Receiver<Message>) {
        let local = Arc::new(Mutex::new(HashSet::new()));
        let (discovery, d_recv) = channel();
        (
            Self {
                local: Arc::clone(&local),
                worker: thread::spawn(move || loop {
                    let Envelope(id, frame) = recv.recv().unwrap();
                    let local = local.lock().unwrap();

                    match frame.recipient.clone() {
                        Recipient::User(ref u) if local.contains(u) => {
                            // TODO: Implement de-sequencing
                            let msg = Slicer::unslice(vec![frame]);
                            discovery.send(msg).unwrap();
                        }
                        Recipient::User(ref u) => {
                            let env = core.lock().unwrap().lookup(u, vec![frame]);
                            core.lock().unwrap().send(env);
                        }
                        Recipient::Flood => core.lock().unwrap().send(vec![Envelope(id, frame)]),
                    }
                }),
            },
            d_recv,
        )
    }
}
