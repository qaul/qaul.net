//! The routing core module of `RATMAN`
#![allow(unused)]

use identity::Identity;
use netmod::{Endpoint, Frame, Result, Error};
use std::{
    collections::BTreeMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type EndpointMap = Arc<Mutex<BTreeMap<u8, Arc<Mutex<dyn Endpoint + Send>>>>>;

/// A target is a specific endpoint, sending to a specific recipient
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Target(pub(crate) u8, pub(crate) i16);

/// A message envelope which encodes the corresponding interface
pub(crate) struct Envelope(pub(crate) Target, pub(crate) Frame);

/// A wrapper around a Message routing worker
struct Worker {
    /// Underlying worker thread (or pool?)
    _thread: JoinHandle<()>,
    /// Messages scheduled to be sent
    to_send: Arc<Mutex<Sender<Envelope>>>,
}

impl Worker {
    /// Start a worker that sends frames and receives them
    fn start(ifs: EndpointMap) -> (Self, Receiver<Envelope>) {
        // Setup sending channel pair
        let (sending, rx) = channel();
        let to_send = Arc::new(Mutex::new(sending));

        // Setup receiving channel pair
        let (tx, received) = channel();

        let _thread = thread::spawn(move || loop {
            // Send queued Messages
            if let Ok(Envelope(id, msg)) = rx.try_recv() {
                dbg!("Sending queued mesage!");
                let ifs = ifs.lock().unwrap();
                let epm = ifs.get(&id.0).unwrap();
                let mut ep = epm.lock().unwrap();
                ep.send(msg, id.1);
            }

            // Poll all available interfaces
            ifs.lock().unwrap().iter().for_each(|(id, epm)| {
                let mut ep = epm.lock().unwrap();
                if let Ok(Some((f, target))) = ep.() {
                    tx.send(Envelope(Target(*id, target), f)).unwrap();
                }
            });
        });

        (
            Self {
                _thread,
                to_send,
            },
            received,
        )
    }
}

/// The routing core powering `RATMAN`
///
/// Keeps track of available interfaces, the routing table (mapping
/// `Identity` -> IF), as well as handling routing workers
pub(crate) struct Core {
    /// A continuously increasing ID for interfaces
    cnt: u8,
    /// A routing worker that handles routing
    worker: Worker,
    /// Mapping network IDs to interface targets
    pub(crate) routes: Arc<Mutex<BTreeMap<Identity, Target>>>,
    /// A list of available interfaces, assigned sequentials IDs
    pub(crate) ifs: EndpointMap,
}

impl Core {
    /// Create a new routing core
    pub(crate) fn new() -> (Self, Receiver<Envelope>) {
        let ifs = Arc::new(Mutex::new(BTreeMap::new()));
        let routes = Arc::new(Mutex::new(BTreeMap::new()));
        let (worker, journal) = Worker::start(Arc::clone(&ifs));

        (
            Core {
                cnt: 0,
                worker,
                routes,
                ifs,
            },
            journal,
        )
    }

    /// Add an interface, assigning it a unique ID
    pub(crate) fn add_if(&mut self, ep: impl Endpoint + 'static + Send) {
        let id = self.cnt;
        self.cnt += 1;
        self.ifs
            .lock()
            .expect("Poisoned Endpoint Map Mutex")
            .insert(id, Arc::new(Mutex::new(ep)));
    }

    /// Remove an interface with unique ID
    pub(crate) fn del_if(&mut self, id: u8) {
        self.ifs
            .lock()
            .expect("Poisoned Endpoint Map Mutex")
            .remove(&id);
    }

    /// Remove a list of interface names and their unique IDs
    pub(crate) fn get_ifs(&self) -> Vec<(u8, String)> {
        self.ifs
            .lock()
            .unwrap()
            .keys()
            .map(|k| (*k, "".into()))
            .collect()
    }

    pub(crate) fn id_reachable(&self, id: Identity, target: Target) {
        let mut routes = self.routes.lock().unwrap();
        routes.insert(id, target);
    }

    /// Map a set of Frames into a set of Envelopes
    ///
    /// An envelope contains interface information for routing, which
    /// is used by the route worker to send a frame on a specific
    /// device.
    pub(crate) fn lookup(&self, id: &Identity, frames: Vec<Frame>) -> Vec<Envelope> {
        let routes = dbg!(self.routes.lock().unwrap());
        let target = routes.get(id).unwrap();
        frames.into_iter().map(|f| Envelope(*target, f)).collect()
    }

    /// Send a properly enveloped message out into the network
    pub(crate) fn send(&self, envs: Vec<Envelope>) -> Result<()> {
        dbg!("Dispatching frames");
        let sender = self.worker.to_send.lock().unwrap();
        envs.into_iter().for_each(|env| sender.send(env).unwrap());
        Ok(())
    }
}
