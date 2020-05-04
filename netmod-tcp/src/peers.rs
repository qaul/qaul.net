//! Peer tracking

use crate::error::PeerErrs;
use async_std::sync::{Arc, RwLock};
use std::collections::{BTreeMap, HashSet};
use std::net::{IpAddr, SocketAddr};

#[derive(Default)]
pub(crate) struct PeerList {
    map: RwLock<BTreeMap<usize, Arc<SocketAddr>>>,
    peers: RwLock<HashSet<SocketAddr>>,
    curr: RwLock<usize>,
}

impl PeerList {
    /// Create a new empty peer list
    pub(crate) fn new() -> Arc<Self> {
        Default::default()
    }

    pub(crate) async fn is_peer(self: &Arc<Self>, ip: &IpAddr) -> bool {
        self.peers
            .read()
            .await
            .iter()
            .map(|addr| addr.ip())
            .fold(false, |b, ip_| b || &ip_ == ip)
    }

    pub(crate) async fn get_id(self: &Arc<Self>, ip: &IpAddr) -> Option<usize> {
        self.map
            .read()
            .await
            .iter()
            .find(|(_, ip_)| &ip_.ip() == ip)
            .map(|(id, _)| *id)
    }

    pub(crate) async fn add(self: &Arc<Self>, addr: &SocketAddr) -> Option<usize> {
        let mut peers = self.peers.write().await;
        let mut map = self.map.write().await;
        let mut curr = self.curr.write().await;

        if !peers.contains(&addr) {
            let id = *curr;
            map.insert(id, Arc::new(addr.clone()));
            peers.insert(addr.clone());
            *curr += 1;
            Some(id)
        } else {
            None
        }
    }

    pub(crate) async fn load<I: Into<SocketAddr>>(
        self: &Arc<Self>,
        peers: Vec<I>,
    ) -> Result<(), PeerErrs> {
        let new_peers: Vec<_> = peers.into_iter().map(Into::into).collect();

        let mut peers = self.peers.write().await;
        let mut map = self.map.write().await;
        let mut curr = self.curr.write().await;

        new_peers.into_iter().fold(Ok(()), |prev, addr| match prev {
            Ok(_) if peers.contains(&addr) => PeerErrs::new(addr),
            Err(e) if peers.contains(&addr) => Err(e.append(addr)),
            Ok(()) => {
                map.insert(*curr, Arc::new(addr.clone()));
                peers.insert(addr);
                *curr += 1;
                Ok(())
            }
            Err(e) => {
                map.insert(*curr, Arc::new(addr.clone()));
                peers.insert(addr);
                *curr += 1;
                Err(e.append(addr))
            }
        })
    }
}
