//! New peer abstraction module
//!
//! The lifecycle of a peer is encoded is the following flow-chart:
//!
//! ```text
//!                               +-> Failed to -+
//!                               |    connect   |
//!                               |              v
//! Start --- for all peers ---> Start a connection
//!                                |            |
//!                                |            |
//!                                v            v
//!                       REVERSE stream       Send a HELLO to init
//!                      (already) exists       reverse connection
//!                               |                      |
//!                               |                      |
//!                               |                      v
//!                               v               Wait for REVERSE connection
//!                      Valid DUPLEX                   |
//!                       connection  <-----------------+
//! ```
//!
//! If at any point sending a message fails, this re-connection needs
//! to be repeated and the packet held until then.
//!
//! All operations on a peer are async, and will be queued via a
//! channel, which means they will return immediately, even if the
//! connection is currently down.

use crate::{AtomPtr, IoPair, Packet};
use async_std::{
    io::prelude::WriteExt,
    net::TcpStream,
    sync::{Arc, RwLock},
    task,
};
use bincode::serialize;
use byteorder::{BigEndian, ByteOrder};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{net::SocketAddr, time::Duration};
use tracing::{error, trace};

/// Utility module to generate monotonic peer IDs
mod id {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static ID_CTR: AtomicUsize = AtomicUsize::new(0);

    /// Get the next monotonically increasing ID
    pub fn next() -> usize {
        ID_CTR.fetch_add(1, Ordering::Relaxed)
    }
}

/// Address from which packets are sent
pub(crate) type SourceAddr = SocketAddr;

/// Address to which packets are sent
pub(crate) type DstAddr = SocketAddr;

/// A thread-safe locked sending stream
type LockedStream = Arc<RwLock<Option<TcpStream>>>;

/// Encode the different states a `Peer` can be in
pub(crate) enum PeerState {
    /// Only a receiving channel exists
    ///
    /// This is either the case for unknown dynamic peers, or a
    /// race-condition on static peers.
    RxOnly,
    /// Only a transmission channel exists
    ///
    /// This is the inverse of RxOnly, and usually a race-condition,
    /// unless the local node is running in DYNAMIC mode
    TxOnly,
    /// A valid two-way connection
    Duplex,
    /// Something has gone really wrong
    Invalid,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Peer {
    /// Unique numeric Id for each peer
    pub(crate) id: usize,
    /// Peer source address
    src: AtomPtr<Option<SourceAddr>>,
    /// Peer destination address
    dst: Option<DstAddr>,
    /// Sending stream for this peer (if it existst)
    sender: LockedStream,
    /// Secret run condition
    #[doc(hidden)]
    _run: Arc<AtomicBool>,
    /// Store packets until they can be delivered
    io: Arc<IoPair<Packet>>,
}

impl Peer {
    /// Initialise a new peer only with it's source address
    ///
    /// This function is meant to initialise a peer who has introduced
    /// itself to this node, but where an outgoing connection couldn't
    /// be established yet.  This indicates either a race condition
    /// which will be resolved soon (because the local peer
    /// initialisation loop hasn't spawned the sending channel yet),
    /// or an unknown peer when running in `dynamic` mode
    pub(crate) fn from_src(src: SourceAddr) -> Arc<Self> {
        Arc::new(Self {
            id: id::next(),
            src: AtomPtr::new(Some(src)),
            ..Default::default()
        })
    }

    /// Open a connection to this peer
    ///
    /// While this function returns immediately, it spawns an async
    /// worker that will try to establish a connection to the peer,
    /// exiting until `stop()` is called on this peer
    pub(crate) fn open(dst: DstAddr, port: u16) -> Arc<Self> {
        let p = Arc::new(Self {
            id: id::next(),
            dst: Some(dst),
            ..Default::default()
        });

        // Start introduction loop
        Arc::clone(&p).introduce(port);
        Arc::clone(&p).run_io_sender(port);
        return p;
    }

    /// Set this peer's source address
    pub(crate) fn set_src(&self, src: SourceAddr) {
        self.src.swap(Some(src));
    }

    /// Stop all tasks associated with this peer
    pub(crate) fn stop(&self) {
        self._run.fetch_and(false, Ordering::Relaxed);
    }

    /// Get the current state for this peer
    pub(crate) fn state(&self) -> PeerState {
        match (self.get_src(), self.dst) {
            (Some(_), Some(_)) => PeerState::Duplex,
            (Some(_), None) => PeerState::RxOnly,
            (None, Some(_)) => PeerState::TxOnly,
            (None, None) => PeerState::Invalid,
        }
    }

    /// Internal utility to verify that this peer is still alive
    pub(crate) fn alive(&self) -> bool {
        self._run.load(Ordering::Relaxed)
    }

    /// Start an async worker to send packets to this peer
    ///
    /// The worker can be stopped after spawning by calling `stop()`.
    /// If at any time sending was'n successful, this loop will
    /// automatically re-init the connection.
    ///
    /// There's currently no way to get diagnostics from failed sends
    /// back to ratman.  **FIXME**: implement this!
    pub(crate) fn run_io_sender(self: Arc<Self>, port: u16) {
        task::spawn(async move {
            while self.alive() {
                if let Some(packet) = self.io.rx.recv().await {
                    // Check if the stream is still there
                    let mut s;
                    while self.alive() {
                        s = self.sender.write().await;
                        if s.is_none() {
                            // We need to drop `s` here because the
                            // introduction loop will want to have
                            // access to it to initialise the stream.
                            // Otherwise we will create a deadlock!
                            drop(s);

                            // Run introduction again
                            Arc::clone(&self).introduce_blocking(port).await;
                        } else {
                            // At this point we should have a valid stream
                            let addr = s.as_ref().unwrap().peer_addr().unwrap().to_string();
                            match packet {
                                Packet::Hello { .. } => trace!("Sending HELLO to {}", addr),
                                Packet::KeepAlive => trace!("Sending KEEP-ALIVE to {}", addr),
                                _ => {}
                            }

                            // Serialise the payload and pre-pend the length
                            let mut vec = serialize(&packet).unwrap();
                            let mut buf = vec![0; 8];
                            BigEndian::write_u64(&mut buf, vec.len() as u64);
                            buf.append(&mut vec);

                            // And woosh!
                            if let Err(e) = s.as_mut().unwrap().write_all(&buf).await {
                                error!("Failed to send message: {}!", e.to_string());
                                *s = None; // We mark ourselves as missing uplink
                                continue; // try again?
                            }

                            // If we reach this point we're good to go
                            break;
                        }
                    }
                }
            }

            trace!("Shutting down packet sender for peer {}", self.id);
        });
    }

    /// Run this Peer's initialisation sequence
    ///
    /// When creating a new connection, or an existing connection has
    /// been lost, call this function to re-establish the DUPLEX link
    /// with this peer.
    ///
    /// Takes it's own listening port as a parameter because otherwise
    /// it's impossible for the other side to associate an incoming
    /// stream to a destination stream.
    pub(crate) fn introduce(self: Arc<Self>, port: u16) {
        let _self = Arc::clone(&self);
        task::spawn(async move { _self.introduce_blocking(port).await });
    }

    /// The same as `introduce()` but without spawning a new task
    async fn introduce_blocking(self: Arc<Self>, port: u16) {
        let id = self.id.clone();
        let dst = self.dst.clone().unwrap();

        let run = Arc::clone(&self._run);
        let sender = Arc::clone(&self.sender);
        let mut ctr = 0;

        while run.load(Ordering::Relaxed) {
            ctr += 1; // increment the attempt counter
            let pre = match ctr {
                0 => "".into(),
                n => format!("[retry #{}]", n),
            };

            // Exit if we are already connected
            if sender.read().await.is_some() {
                trace!(
                    "Peer `{}` (ID: {}) is already connected!",
                    dst.to_string(),
                    id
                );
                break;
            }

            trace!(
                "{}: Attempting to connect to peer `{}`",
                pre,
                dst.to_string()
            );
            let s = match TcpStream::connect(dst).await {
                Ok(s) => s,
                Err(_) => {
                    error!(
                        "Failed to connect to peer `{}`.  Starting timeout...",
                        dst.to_string()
                    );

                    // FIXME: Make this configurable
                    task::sleep(Duration::from_secs(20)).await;
                    continue;
                }
            };

            trace!("Successfully connected to peer `{}`", &dst);
            *sender.write().await = Some(s);

            // Queue a HELLO sending and exit this loop
            self.send(Packet::Hello { port }).await;
            break;
        }
    }

    /// Send some arbitrary packet to this peer
    ///
    /// If the connection has become invalid in the meantime, this
    /// function will automatically call introduce and wait for a
    /// connection to occur before retrying.  In this case it will
    /// spawn an async worker and return, even if the data was not
    /// successfully delivered.
    pub(crate) async fn send(&self, packet: Packet) {
        self.io.tx.send(packet).await;
    }

    pub(crate) fn get_src(&self) -> Option<SourceAddr> {
        *self.src.get_ref().clone()
    }

    pub(crate) fn get_dst(&self) -> Option<DstAddr> {
        self.dst.clone()
    }
}
