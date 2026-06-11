use std::{collections::HashMap, sync::RwLock};

use libp2p::PeerId;

use crate::{connections::ConnectionModule, router_v2::index::{IndexDictionary, MirrorIndexDictionary}};

pub mod identity;
pub mod index;

#[derive(Debug, thiserror::Error)]
pub enum RoutingV2Error {
    MultikeyErrror(#[from] libp2p::identity::DecodingError),
}

impl std::fmt::Display for RoutingV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingV2Error::MultikeyErrror(e) => write!(f, "{e}"),
        }
    }
}

pub type Result<T> = std::result::Result<T, RoutingV2Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sphere {
    Local,
    Internet,
}

impl Sphere {
    pub const fn of(module: ConnectionModule) -> Self {
        match module {
            ConnectionModule::Internet => Sphere::Internet,
            _ => Sphere::Local,
        }
    }
}

/// groups one user-space and one node-space mirror per neigbour
#[derive(Debug, Default)]
pub struct NeighbourMirrors {
    pub users: MirrorIndexDictionary,
    pub nodes: MirrorIndexDictionary,
}

/// Instance-based router state that owns all routing sub-state.
/// This is the major struct that will replace the current Router.
/// Each `RouterState` instance is fully independent, enabling multiple
/// nodes to run in the same process.
pub struct RouterV2State {
    /// Index space for each user this particular node knows
    pub user_dict: RwLock<IndexDictionary>,
    /// Index space for each node this particular node knows
    pub node_dict: RwLock<IndexDictionary>,
    /// Two mirrors per neighbour, one per index space.
    pub mirrors: RwLock<HashMap<PeerId, NeighbourMirrors>>
}

impl RouterV2State {
    pub fn new(host_node_id: [u8; 8]) -> Self {
        Self {
            user_dict: RwLock::new(IndexDictionary::new(None)),
            node_dict: RwLock::new(IndexDictionary::new(Some(host_node_id))),
            mirrors: RwLock::new(HashMap::new())
        }
    }

    /// Inserts a fresh empty mirror for the neighbour, replacing any prior mirror
    pub fn add_neighbour_mirror(&self, peer: PeerId) {
        let mut mirrors = self.mirrors.write().unwrap();
        mirrors.insert(peer, NeighbourMirrors::default());
    }

    pub fn remove_mirror(&self, peer: PeerId) {
        let mut mirrors = self.mirrors.write().unwrap();
        mirrors.remove_entry(&peer);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_of_lan_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::Lan), Sphere::Local);
    }

    #[test]
    fn sphere_of_ble1m_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::Ble1m), Sphere::Local);
    }

    #[test]
    fn sphere_of_ble_coded_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::BleCoded), Sphere::Local);
    }

    #[test]
    fn sphere_of_internet_is_internet() {
        assert_eq!(Sphere::of(ConnectionModule::Internet), Sphere::Internet);
    }

    #[test]
    fn sphere_of_self_is_local() {
        // ConnectionModule::Local refers to this node itself, which is
        // part of its own Local sphere by definition
        assert_eq!(Sphere::of(ConnectionModule::Local), Sphere::Local);
    }

    #[test]
    fn sphere_of_none_currently_falls_through_to_local() {
        assert_eq!(Sphere::of(ConnectionModule::None), Sphere::Local);
    }
}
