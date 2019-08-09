//! `netmod-mem` is an in-memory `netmod` endpoint
//!
//! This aims to make testing any structure that binds against
//! `netmod` easier and reproducable.

use std::cell::Cell;

/// Represent a single netmod endpoint that can connect to exactly one other
///
/// Both `lacency` and `bn` are public so that they can be
/// dynamically adjusted in a simulation.
pub struct MemMod<'p> {
    /// In this scenario an endpoint talks to one other endpoint
    pair: Cell<Option<&'p MemMod<'p>>>,
    /// Apply artificial lacency
    pub lacency: u8,
    /// The troughput limit in bytes per second
    pub bn: u32,
}

impl<'p> MemMod<'p> {
    pub fn new(lacency: u8) -> Self {
        Self {
            pair: Cell::new(None),
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
