//! Peer tracking

use crate::error::PeerErrs;
use async_std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::{error, trace, warn};

type SourceAddr = SocketAddr;
type DstAddr = SocketAddr;

/// A set of errors that can occur when dealing with peer states
#[derive(Debug, Clone, Copy)]
pub(crate) enum PeerInsertError {
    /// A peer with matching source address already exists
    SameSrcAddr,
    /// A peer with matching dst address already exists
    SameDstAddr,
    /// The requested peer (via Id) was not found
    NoSuchPeer,
}

#[derive(Clone, Debug)]
pub(crate) struct Peer {
    pub id: usize,
    pub src: Option<SocketAddr>,
    pub dst: Option<SocketAddr>,
    pub known: bool,
}

impl Peer {
    /// Get the current peer's LinkState
    pub(crate) fn link_state(&self) -> LinkState {
        use LinkState::*;
        match (self.src, self.dst) {
            (None, None) => NoLink,
            (Some(_), None) => DownOnly,
            (None, Some(_)) => UpOnly,
            (Some(_), Some(_)) => Duplex,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LinkState {
    /// No established link with this peer
    NoLink,
    /// This peer can send us data, but we have no return channel
    DownOnly,
    /// We can send this peer data, but they have no return channel
    UpOnly,
    /// A bi-directional link
    Duplex,
}

#[derive(Debug, Default)]
pub(crate) struct Peers {
    /// Lookup table by source address
    src_to_id: RwLock<HashMap<SourceAddr, usize>>,
    /// Lookup table by destination address
    dst_to_id: RwLock<HashMap<DstAddr, usize>>,
    /// Mapping from Ids to peer data
    peers: RwLock<HashMap<usize, Peer>>,
    /// Used to monotonically create Ids
    curr: RwLock<usize>,
}

impl Peers {
    /// Create a new empty peer list
    pub(crate) fn new() -> Arc<Self> {
        Default::default()
    }

    /// Get all peers currently known to this server
    pub(crate) async fn all_known(self: &Arc<Self>) -> Vec<Peer> {
        self.peers
            .read()
            .await
            .iter()
            .map(|(_, p)| p.clone())
            .collect()
    }

    /// Get a peer by it's unique numerical Id
    pub(crate) async fn peer_by_id(self: &Arc<Self>, id: usize) -> Option<Peer> {
        self.peers.read().await.get(&id).map(|v| v.clone())
    }

    /// Get a peer via it's source socket address
    pub(crate) async fn peer_by_src(self: &Arc<Self>, src: &SourceAddr) -> Option<Peer> {
        match self.src_to_id.read().await.get(src) {
            Some(id) => self.peers.read().await.get(&id).cloned(),
            None => None,
        }
    }

    /// Get a peer via it's destination socket address
    pub(crate) async fn peer_by_dst(self: &Arc<Self>, dst: &DstAddr) -> Option<Peer> {
        match self.dst_to_id.read().await.get(dst) {
            Some(id) => self.peers.read().await.get(&id).cloned(),
            None => None,
        }
    }

    /// Check if a peer is already stored via it's src or dst addr
    #[tracing::instrument(skip(self), level = "trace")]
    pub(crate) async fn filter_peer(self: &Arc<Self>, peer: &Peer) -> Result<(), PeerInsertError> {
        trace!("Attempting to filter peer");
        match peer.src {
            Some(src) if self.peer_by_src(&src).await.is_some() => {
                trace!("Peer with same source address exists");
                Err(PeerInsertError::SameSrcAddr)
            }
            _ => Ok(()),
        }?;

        match peer.dst {
            Some(dst) if self.peer_by_dst(&dst).await.is_some() => {
                trace!("Peer with same dst address exists");
                Err(PeerInsertError::SameDstAddr)
            }
            _ => Ok(()),
        }?;

        Ok(())
    }

    /// Add a new peer into the store
    #[tracing::instrument(skip(self), level = "trace")]
    pub(crate) async fn add_peer(
        self: &Arc<Self>,
        mut peer: Peer,
    ) -> Result<usize, PeerInsertError> {
        self.filter_peer(&peer).await?;
        let mut curr = self.curr.write().await;
        *curr += 1;

        peer.id = *curr;

        // insert to src-map if src addr is known
        if let Some(src) = peer.src {
            self.src_to_id.write().await.insert(src, *curr);
        }

        // insert to dst-map if dst addr is known
        if let Some(dst) = peer.dst {
            self.dst_to_id.write().await.insert(dst, *curr);
        }

        Ok(*curr)
    }

    /// Change the destination address on an existing peer connection
    pub(crate) async fn change_dst(
        self: &Arc<Self>,
        id: usize,
        dst: &DstAddr,
    ) -> Result<Peer, PeerInsertError> {
        // Get the peer by Id and change it's dst field
        let mut peer = self
            .peer_by_id(id)
            .await
            .map_or(Err(PeerInsertError::NoSuchPeer), |p| Ok(p))?;
        peer.dst = Some(dst.clone());

        // Get an existing peer with this destination
        let ghost = self.peer_by_dst(dst).await;

        // Copy the "known" status from the ghost identity
        // FIXME: Why? -- spacekookie
        if let Some(ghost) = ghost {
            peer.known = peer.known || ghost.known;
            self.del_peer(ghost.id).await;
        }

        // Update peer data in peer maps
        self.del_peer(peer.id).await;
        self.add_peer(peer.clone()).await?;
        Ok(peer)
    }

    /// Remove a peer by Id, and do nothing if the peer doens't exist
    #[tracing::instrument(skip(self), level = "trace")]
    pub(crate) async fn del_peer(self: &Arc<Self>, id: usize) {
        let mut peers = self.peers.write().await;
        let mut src_to_id = self.src_to_id.write().await;
        let mut dst_to_id = self.dst_to_id.write().await;

        if let Some(peer) = peers.remove(&id) {
            if let Some(src) = peer.src {
                src_to_id.remove(&src);
            }
            if let Some(dst) = peer.dst {
                dst_to_id.remove(&dst);
            }
        }
    }

    #[tracing::instrument(skip(self, peers), level = "trace")]
    pub(crate) async fn load<I: Into<SocketAddr>>(
        self: &Arc<Self>,
        peers: Vec<I>,
    ) -> Result<(), PeerErrs> {
        let new_peers: Vec<_> = peers.into_iter().map(Into::into).collect();

        // Lock all required data stores
        let mut peers = self.peers.write().await;
        let mut dst_to_id = self.dst_to_id.write().await;
        let mut curr = self.curr.write().await;

        new_peers.into_iter().fold(Ok(()), |prev, addr| {
            // Utility closure to insert a new peer
            macro_rules! insert_new_peer {
                () => {
                    dst_to_id.insert(addr.clone(), *curr);
                    peers.insert(
                        *curr,
                        Peer {
                            id: *curr,
                            src: None,
                            dst: Some(addr),
                            known: true,
                        },
                    );
                };
            };

            match prev {
                Ok(_) if dst_to_id.contains_key(&addr) => PeerErrs::new(addr),
                Err(e) if dst_to_id.contains_key(&addr) => Err(e.append(addr)),
                Ok(()) => {
                    insert_new_peer!();
                    *curr += 1;
                    Ok(())
                }
                err @ Err(_) => {
                    insert_new_peer!();
                    *curr += 1;
                    err
                }
            }
        })
    }
}

// #[async_std::test]
// async fn load_peers() {
//     use std::net::{Ipv4Addr, SocketAddrV4};

//     let a1 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000);
//     let a2 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);

//     let peers = Peers::new();
//     peers.load(vec![a1.clone(), a2.clone()]).await.unwrap();

//     let id = peers.id_by_dst(&a1.into()).await.unwrap();
//     assert_eq!(peers.dst_by_id(id).await, Some(a1.into()));
// }
