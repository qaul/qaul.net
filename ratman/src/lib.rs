//! `R.A.T.M.A.N.` decentralised routing protocol
//!
//! <small> Nananananananana Nananananananana RAT MAN!</small>
//! A modern approach to fully delay-tolerant mesh routing,
//! implemented network agnostically and entirely in userspace.

mod core;

// use netmod::{Endpoint, Recipient};
// use std::sync::{mpsc::Receiver, Arc, Mutex};

// /// A temporary structure used to initialise a `R.A.T.M.A.N.` `Router`
// ///
// /// Use this structure only for destructuring, it has no useful
// /// attributes beyond named fields.
// pub struct RouterInit {
//     /// Initialised `Router`
//     pub router: Router,
//     /// Upwards communication channel for Discovery
//     // pub channel: Receiver<Message>,
// }

// impl RouterInit {
//     pub fn modify(&self) -> &Router {
//         &self.router
//     }
// }

/// A `R.A.T.M.A.N.` routing context
///
/// Handles `Message` splicing, `Frame` sequencing, routing tables,
/// journal keeping, re-transmissions of non-local frames, as well as
/// the discovery protocol.
pub struct Router {
    // core: Arc<Core>,
// #[allow(unused)]
// journal: Arc<Journal>,
}

// impl Router {
//     /// Create a new `Router` with internal mutability
//     pub fn new() -> Self {
//         // let (core, j_rcv) = Some(Core::new())
//         //     .map(|(c, r)| (Arc::new(c), r))
//         //     .unwrap();
//         // let (journal, d_send) = Some(Journal::start(j_rcv, Arc::clone(&core)))
//         //     .map(|(j, s)| (Arc::new(j), s))
//         //     .unwrap();

//         // RouterInit {
//         //     router: Self { core, journal },
//         //     channel: d_send,
//         // }
//         unimplemented!()
//     }

//     /// Add an `netmod` endpoint to this router
//     pub fn add_ep(&self, ep: impl Endpoint + 'static + Send) {
//         // self.core.lock().unwrap().add_if(ep);
//         unimplemented!()
//     }

//     /// ONLY USE FOR DEBUGGING!
//     ///
//     /// This function does not properly advertise one-to-many mappings!
//     #[deprecated]
//     pub fn discover(&self, id: Identity, ifid: u8) {
//         // self.core.lock().unwrap().id_reachable(id, Target(ifid, 0));
//         unimplemented!()
//     }

//     /// Teach the `Router` about local users
//     pub fn local(&self, id: Identity) {
//         //self.journal.add_local(id);
//         unimplemented!()
//     }

//     pub fn local_del(&self, id: Identity) {
//         //self.journal.rm_local(id);
//         unimplemented!()
//     }

//     /// Queue a `R.A.T.M.A.N.` message for sending
//     pub async fn send(&self, msg: Message) -> Result<()> {
//         unimplemented!()
//         // // FIXME: This is so pointless...
//         // let recp = msg.recipient.clone();

//         // let frames = Slicer::slice(0, msg);
//         // let core = self.core.lock().unwrap();

//         // core.send(match recp {
//         //     Recipient::User(ref id) => core.lookup(id, frames),
//         //     Recipient::Flood => core
//         //         .get_ifs()
//         //         .into_iter()
//         //         .fold(vec![], |mut vec, (ifid, _)| {
//         //             // Setting the target to -1 carries the semantic
//         //             // meanting for netmod to treat this send as a
//         //             // broadcast if it is a one-to-many mapped driver!
//         //             let mut set = frames.iter().cloned().map(|f| Envelope(Target(ifid, -1), f)).collect();
//         //             vec.append(&mut set);
//         //             vec
//         //         }),
//         // })
//     }
// }
