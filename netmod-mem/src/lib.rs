//! `netmod-mem` is an in-memory `netmod` endpoint
//!
//! This aims to make testing any structure that binds against
//! `netmod` easier and reproducable.

use async_std::future;
use async_std::task::Poll;
use async_trait::async_trait;
use ratman_netmod::{Endpoint, Error as NetError, Frame, Result as NetResult, Target};
use std::sync::mpsc::TryRecvError;

/// An input/output pair of `mpsc::channel`s.
///
/// This is the actual mechanism by which data is moved around between `MemMod`s in
/// different places.
pub(crate) mod io;
/// Simulated transmission media.
pub mod media;

/// Represents a single netmod endpoint that can connect to exactly one other, either
/// as a 1-to-1 link between libqaul instances or as a link into a transmission
/// medium of some kind.
pub struct MemMod {
    /// Internal memory access to send/receive
    io: Option<io::Io>,
}

impl MemMod {
    /// Create a new, unpaired `MemMod`.
    pub fn new() -> Self {
        Self { io: None }
    }

    /// Create two already-paired `MemMod`s, ready for use.
    pub fn make_pair() -> (Self, Self) {
        let (mut a, mut b) = (MemMod::new(), MemMod::new());
        a.link(&mut b);
        (a, b)
    }

    /// Return `true` if the MemMod is linked to another one or
    /// `false` otherwise.
    pub fn linked(&self) -> bool {
        self.io.is_some()
    }

    /// Establish a 1-to-1 link between two `MemMod`s.
    ///
    /// # Panics
    ///
    /// Panics if this MemMod, or the other one, is already linked.
    pub fn link(&mut self, pair: &mut MemMod) {
        if self.linked() || pair.linked() {
            panic!("Attempted to link an already linked MemMod.");
        }
        let (my_io, their_io) = io::Io::make_pair();
        self.io = Some(my_io);
        pair.io = Some(their_io);
    }

    /// Establish a link to an `Io` module
    ///
    /// # Panics
    /// Panics if this MemMod is already linked.
    pub(crate) fn link_raw(&mut self, io: io::Io) {
        if self.linked() {
            panic!("Attempted to link an already linked MemMod.");
        }
        self.io = Some(io);
    }

    /// Remove the connection between MemMods.
    pub fn split(&mut self) {
        // The previous value in here will now be dropped,
        // so future messages will fail.
        self.io = None;
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
    async fn send(&mut self, frame: Frame, _: Target) -> NetResult<()> {
        match self.io {
            None => Err(NetError::NotSupported),
            Some(ref io) => match io.out.send(frame) {
                Ok(_) => Ok(()),
                Err(_) => Err(NetError::ConnectionLost),
            },
        }
    }

    async fn next(&mut self) -> NetResult<(Frame, Target)> {
        future::poll_fn(|_| match self.io {
            None => Poll::Ready(Err(NetError::NotSupported)),
            Some(ref mut io) => match io.inc.try_recv() {
                Ok(v) => Poll::Ready(Ok((v, Target::default()))),
                Err(TryRecvError::Empty) => Poll::Pending,
                Err(_) => Poll::Ready(Err(NetError::ConnectionLost)),
            },
        })
        .await
    }
}
