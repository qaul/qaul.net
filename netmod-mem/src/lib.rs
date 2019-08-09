//! `netmod-mem` is an in-memory `netmod` endpoint
//!
//! This aims to make testing any structure that binds against
//! `netmod` easier and reproducable.

use ratman_netmod::{Endpoint, Frame, NetError, NetResult};
use std::{
    cell::Cell,
    sync::mpsc::{channel, Receiver, Sender},
};

/// A simple I/O wrapper around channels
struct Io {
    out: Sender<Frame>,
    inc: Receiver<Frame>,
}

/// Represent a single netmod endpoint that can connect to exactly one other
///
/// Both `lacency` and `bn` are public so that they can be
/// dynamically adjusted in a simulation.
pub struct MemMod<'p> {
    /// In this scenario an endpoint talks to one other endpoint
    pair: Cell<Option<&'p MemMod<'p>>>,
    /// Internal memory access to send/receive
    io: Option<Io>,
    /// Apply artificial lacency
    pub lacency: u8,
    /// The troughput limit in bytes per second
    pub bn: u32,
}

impl<'p> MemMod<'p> {
    pub fn new(lacency: u8) -> Self {
        Self {
            pair: Cell::new(None),
            io: None,
            lacency,
            bn: 1024 * 64, /* 64kb */
        }
    }

    /// Establish a 1-1 link between two MemMods
    pub fn link(&self, pair: &'p MemMod<'p>) {
        self.pair.swap(&Cell::new(Some(pair)));
    }

    /// Remove the connection between MemMods.
    ///
    /// **Warning**: will panic if no connection exists
    pub fn split(&self) -> &'p MemMod<'p> {
        self.pair.replace(None).expect("No connection found!")
    }
}

impl<'p> Endpoint for MemMod<'p> {
    /// Provides maximum frame-size information to `RATMAN`
    fn size_hint(&self) -> usize {
        self.bn as usize
    }

    /// Send a message to a specific endpoint (client)
    fn send(&mut self, frame: Frame) -> NetResult<()> {
        unimplemented!()
    }

    /// Listen for messages from a specific sender
    fn listen(&mut self, sender: impl Endpoint) -> NetResult<Frame> {
        unimplemented!()
    }

    /// Setup a listener that will call a function on a structure that was received from the network
    fn listen_all<F: 'static, E: Endpoint>(&mut self, handler: F)
    where
        F: FnMut(E, Frame) -> NetResult<()>,
    {
        unimplemented!()
    }
}

