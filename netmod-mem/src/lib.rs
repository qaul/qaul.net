//! `netmod-mem` is an in-memory `netmod` endpoint
//!
//! This aims to make testing any structure that binds against
//! `netmod` easier and reproducable.

use async_std::{
    future::{self, Future},
    pin::Pin,
    sync::{Arc, RwLock},
    task::{self, Poll},
};
use async_trait::async_trait;
use crossbeam_channel::TryRecvError;
use ratman_netmod::{Endpoint, Error as NetError, Frame, Result as NetResult, Target};
use std::time::Duration;

/// An input/output pair of `mpsc::channel`s.
///
/// This is the actual mechanism by which data is moved around between `MemMod`s in
/// different places.
pub(crate) mod io;
// Simulated transmission media.
// pub mod media;

/// Represents a single netmod endpoint that can connect to exactly one other, either
/// as a 1-to-1 link between libqaul instances or as a link into a transmission
/// medium of some kind.
pub struct MemMod {
    /// Internal memory access to send/receive
    io: Arc<RwLock<Option<io::Io>>>,
}

impl MemMod {
    /// Create a new, unpaired `MemMod`.
    pub fn new() -> Self {
        Self {
            io: Default::default(),
        }
    }

    /// Create two already-paired `MemMod`s, ready for use.
    pub fn make_pair() -> (Self, Self) {
        let (a, b) = (MemMod::new(), MemMod::new());
        a.link(&b);
        (a, b)
    }

    /// Return `true` if the MemMod is linked to another one or
    /// `false` otherwise.
    pub fn linked(&self) -> bool {
        task::block_on(async { self.io.read().await.is_some() })
    }

    /// Establish a 1-to-1 link between two `MemMod`s.
    ///
    /// # Panics
    ///
    /// Panics if this MemMod, or the other one, is already linked.
    pub fn link(&self, pair: &MemMod) {
        if self.linked() || pair.linked() {
            panic!("Attempted to link an already linked MemMod.");
        }
        let (my_io, their_io) = io::Io::make_pair();

        self.set_io_async(my_io);
        pair.set_io_async(their_io);
    }

    /// Establish a link to an `Io` module
    ///
    /// # Panics
    /// Panics if this MemMod is already linked.
    pub(crate) fn link_raw(&mut self, io: io::Io) {
        if self.linked() {
            panic!("Attempted to link an already linked MemMod.");
        }
        self.set_io_async(io);
    }

    /// Remove the connection between MemMods.
    pub fn split(&self) {
        // The previous value in here will now be dropped,
        // so future messages will fail.
        self.set_io_async(None);
    }

    fn set_io_async<I: Into<Option<io::Io>>>(&self, val: I) {
        task::block_on(async { *self.io.write().await = val.into() });
    }
}

#[async_trait]
impl Endpoint for MemMod {
    /// Provides maximum frame-size information to `RATMAN`
    fn size_hint(&self) -> usize {
        ::std::u32::MAX as usize
    }

    /// Send a message to a specific endpoint (client)
    ///
    /// # Errors
    ///
    /// Returns `OperationNotSupported` if attempting to send through
    /// a connection that is not yet connected.
    async fn send(&self, frame: Frame, _: Target) -> NetResult<()> {
        let io = self.io.read().await;
        match *io {
            None => Err(NetError::NotSupported),
            Some(ref io) => Ok(io.out.send(frame).await),
        }
    }

    async fn next(&self) -> NetResult<(Frame, Target)> {
        let io = self.io.read().await;
        match *io {
            None => Err(NetError::NotSupported),
            Some(ref io) => match io.inc.recv().await {
                Some(f) => Ok((f, Target::default())),
                None => Err(NetError::ConnectionLost),
            },
        }
    }
}
