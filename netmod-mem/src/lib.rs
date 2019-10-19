//! `netmod-mem` is an in-memory `netmod` endpoint
//!
//! This aims to make testing any structure that binds against
//! `netmod` easier and reproducable.

use netmod::{Endpoint, Frame, NetError, NetResult};
use std::{
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    sync::RwLock,
};

/// A simple I/O wrapper around channels
struct Io {
    out: Sender<Frame>,
    inc: Receiver<Frame>,
}

impl Io {
    fn make_pair() -> (Io, Io) {
        let (a_to_b, b_from_a) = mpsc::channel();
        let (b_to_a, a_from_b) = mpsc::channel();
        let a = Io {
            out: a_to_b,
            inc: a_from_b,
        };
        let b = Io {
            out: b_to_a,
            inc: b_from_a,
        };
        return (a, b);
    }
}

/// Represent a single netmod endpoint that can connect to exactly one
/// other
///
/// Both `latency` and `bn` are public so that they can be dynamically
/// adjusted in a simulation.
pub struct MemMod {
    /// Internal memory access to send/receive
    io: RwLock<Option<Io>>,
    /// Apply artificial latency (not implemented, needs async)
    pub latency: u8,
    /// The troughput limit in bytes per second (not implemented, needs async)
    pub bn: u32,
}

impl MemMod {
    /// Create a new, unpaired `MemMod`.
    pub fn new() -> Self {
        Self {
            io: RwLock::new(None),
            latency: 1,
            bn: std::u32::MAX,
        }
    }

    /// Return `true` if the MemMod is linked to another one or
    /// `false` otherwise.
    pub fn linked(&self) -> bool {
        self.io.read().expect("RWLock poisoned").is_some()
    }

    /// Establish a 1-1 link between two MemMods
    ///
    /// # Panics
    ///
    /// Panics if this MemMod, or the other one, is already linked.
    pub fn link(&mut self, pair: &mut MemMod) {
        if self.linked() || pair.linked() {
            panic!("Attempted to link an already linked MemMod.");
        }
        let (my_io, their_io) = Io::make_pair();
        *self.io.get_mut().expect("RWLock poisoned") = Some(my_io);
        *pair.io.get_mut().expect("RWLock poisoned") = Some(their_io);
    }

    /// Remove the connection between MemMods.
    pub fn split(&self) {
        // The previous value in here will now be dropped,
        // so future messages will fail.
        *self.io.write().expect("RwLock poisoned") = None;
    }
}

impl Endpoint for MemMod {
    /// Provides maximum frame-size information to `RATMAN`
    fn size_hint(&self) -> usize {
        self.bn as usize
    }

    /// Send a message to a specific endpoint (client)
    ///
    /// # Errors
    ///
    /// Returns `OperationNotSupported` if attempting to send through
    /// a connection that is not yet connected.
    fn send(&mut self, frame: Frame) -> NetResult<()> {
        match &*self.io.read().expect("RWLock poisoned") {
            None => Err(NetError::OperationNotSupported),
            Some(ref io) => {
                match io.out.send(frame) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(NetError::ConnectionLost),
                }
            }
        }
    }

    fn poll(&mut self) -> NetResult<Option<Frame>> {
        match *self.io.get_mut().expect("RWLock poisoned") {
            None => Err(NetError::OperationNotSupported),
            Some(ref mut io) => match io.inc.try_recv() {
                Ok(v) => Ok(Some(v)),
                Err(TryRecvError::Empty) => Ok(None),
                Err(_) => Err(NetError::ConnectionLost),
            },
        }
    }

    fn listen(&mut self, mut handler: Box<dyn FnMut(Frame) -> NetResult<()>>) -> NetResult<()> {
        match &mut *self.io.get_mut().expect("RWLock poisoned") {
            None => Err(NetError::OperationNotSupported),
            Some(ref mut io) => match io.inc.recv() {
                Ok(v) => handler(v),
                Err(_) => return Err(NetError::ConnectionLost),
            },
        }
    }
}
