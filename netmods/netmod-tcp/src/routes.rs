//! Route management module
//!
//! Because ratman performes route lookups via public key IDs, we need
//! to keep a mapping from target ID to IP addresses.  Furthermore,
//! beacuse TCP is meant to be client-server, we also need to make
//! sure that we can establish a DUPLEX link with each peer.
//!
//! A lot of this logic is handled directly in `peer.rs`, while this
//! module handles the overall state changes to the local IP routing
//! table.  When discovering a new peer, it needs to be written to
//! this table, and introduced to.  Once a peer worker has been
//! spawned, it will make sure the duplex link is never dropped.

use crate::{DstAddr, Peer, SourceAddr};
use async_std::sync::{Arc, RwLock};
use std::collections::BTreeMap;
use tracing::{error, trace};

/// Routing table for local IP scope
#[derive(Clone, Default)]
pub(crate) struct Routes {
    /// Store which port this instance is listening to
    port: u16,
    /// A map of all the peers known to this system
    peers: Arc<RwLock<BTreeMap<usize, Arc<Peer>>>>,
    /// Map source addresses to peer ID
    src_map: Arc<RwLock<BTreeMap<SourceAddr, usize>>>,
    /// Map destination address to peer ID
    dst_map: Arc<RwLock<BTreeMap<DstAddr, usize>>>,
}

impl Routes {
    /// Create a new empty routes table
    pub(crate) fn new(port: u16) -> Arc<Self> {
        Arc::new(Self {
            port,
            ..Self::default()
        })
    }

    /// Get the underlying peer for an ID
    pub(crate) async fn get_peer(self: &Arc<Self>, id: usize) -> Option<Arc<Peer>> {
        self.peers.read().await.get(&id).map(|p| Arc::clone(&p))
    }

    /// Add a new peer to the system with a destination address
    pub(crate) async fn add_via_dst(self: &Arc<Self>, dst: DstAddr) -> usize {
        let p = Peer::open(dst.clone(), self.port);
        let id = p.id;

        self.peers.write().await.insert(id, p);
        self.dst_map.write().await.insert(dst, id);
        id
    }

    /// Add a peer via it's source address
    ///
    /// These peers are not valid and either need to be merged with a
    /// dst-peer, or upgraded to have a destination handler.  This can
    /// be done later by caling `upgrade_merge(id, port)`.  The
    /// required port can be read from a valid HELLO packet.
    pub(crate) async fn add_via_src(self: &Arc<Self>, src: &SourceAddr) -> usize {
        let p = Peer::from_src(src.clone());
        let id = p.id;

        self.peers.write().await.insert(id, p);
        self.src_map.write().await.insert(src.clone(), id);
        id
    }

    /// Perform a peer lookup via source address
    pub(crate) async fn find_via_src(self: &Arc<Self>, src: &SourceAddr) -> Option<usize> {
        self.src_map.read().await.get(src).map(|id| *id)
    }

    /// Upgrade an existing peer with a destination address
    ///
    /// The existing src peer will be dropped.  If a dst peer is
    /// found, it will be upgraded, if none was found, a new one will
    /// be created with both dst and src addresses.
    ///
    /// ## Confused?
    ///
    /// That's okay, me too.  This function assumes that a peer with
    /// only a SRC address set.  This happens when accepting a new
    /// connection in `server.rs`, before reading from the socket.
    /// Three cases can occur, and need to be handled:
    ///
    /// 1. SRC peer found, but no outgoing DST peer
    ///
    ///    This is either a race-condition where the set of local
    ///    peers is large and no connection has been opened to the
    ///    peer yet (rare).
    ///
    ///    Alternatively, when the node is running in DYNAMIC mode,
    ///    this might be an entirely new peer all together.  In this
    ///    case we spawn a new DST peer, and attach the SRC address to
    ///    it, making it a full DUPLEX peer.
    ///    
    /// 2. SRC peer found, and DST peer found
    ///
    ///    This will be the most common case, even in STATIC mode: we
    ///    have started a connection to the peer, and were waiting for
    ///    a reverse connection.  We remove the SRC peer and upgrade
    ///    the DST peer with the SRC address.  Easy :)
    ///
    /// 3. Neither SRC nor DST peer found
    ///
    ///    This indicates some bad state and we panic.  This _should_
    ///    never happen, but might when calling this function in the
    ///    wrong position in the accept loop.
    pub(crate) async fn upgrade(self: &Arc<Self>, id: usize, port: u16) {
        let mut peers = self.peers.write().await;
        let mut src_map = self.src_map.write().await;
        let mut dst_map = self.dst_map.write().await;

        // Remove the existing SRC peer no matter what
        let peer = peers
            .remove(&id)
            .expect(&format!("Peer with id {} wasn't found!", id));
        let src = peer
            .get_src()
            .expect("Invalid variant: peer must have SRC at this point");
        src_map.remove(&src);

        if let Some(ref dst) = peer.get_dst() {
            trace!("Trying to upgrade a DUPLEX connection. Was there a connection drop?");
            dst_map.remove(dst);
        }

        // Create the implied DST address
        let dst = DstAddr::new(src.ip(), port);

        match dst_map.get(&dst) {
            // If a peer with the implied DST address exists, we drop the
            // SRC peer, and upgrade this to a duplex connection.
            Some(id) => {
                trace!("Upgrading peer {} with SRC address", id);
                let peer = peers.get(&id).unwrap();
                peer.set_src(src);
            }
            // If no such peer exists, we create one with SRC and DST addresses
            None => {
                let p = Peer::open(dst, port);
                p.set_src(src);

                // Insert peer into lookup tables
                src_map.insert(src, p.id);
                dst_map.insert(dst, p.id);
                peers.insert(p.id, p);
            }
        }
    }
}
