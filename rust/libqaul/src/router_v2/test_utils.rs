//! Shared test scaffolding for the router_v2 test suite.
//!
//! Consolidates the fixture builders that every test module was
//! duplicating: fresh state/peer/key generators, mirror + dictionary
//! wiring, and default-populated User/Node inserters.

use crate::router_v2::{
    identity::Multikey,
    index::Space,
    table::{Node, User},
    RouterV2State,
};
use libp2p::{identity::Keypair, PeerId};
use std::sync::{Arc, RwLock};

pub fn fresh_multikey() -> Multikey {
    Multikey::from(Keypair::generate_ed25519().public())
}

pub fn fresh_state() -> RouterV2State {
    RouterV2State::new([0; 8])
}

pub fn fresh_peer() -> PeerId {
    Keypair::generate_ed25519().public().to_peer_id()
}

/// Registers a fresh neighbour mirror and returns the PeerId.
pub fn add_neighbour(state: &RouterV2State) -> PeerId {
    let peer = fresh_peer();
    state.add_neighbour_mirror(peer);
    peer
}

/// Binds a neighbour's mirror-dictionary entry in the given space.
pub fn bind_mirror(
    state: &RouterV2State,
    peer: PeerId,
    space: Space,
    idx: u16,
    id: [u8; 8],
) {
    let mut mirrors = state.mirrors.write().unwrap();
    let nm = mirrors.get_mut(&peer).unwrap();
    match space {
        Space::Node => nm.nodes.bind(idx, id),
        Space::User => nm.users.bind(idx, id),
    }
}

/// Binds an own-dictionary entry in the given space.
pub fn bind_own_dict(state: &RouterV2State, space: Space, idx: u16, id: [u8; 8]) {
    match space {
        Space::Node => state.node_dict.write().unwrap().bind(idx, id),
        Space::User => state.user_dict.write().unwrap().bind(idx, id),
    }
}

/// Installs a User with a real key and the given profile_version;
/// returns the live Arc so callers can downgrade / mutate it.
pub fn install_user(
    state: &RouterV2State,
    id: [u8; 8],
    profile_version: u32,
) -> Arc<RwLock<User>> {
    let u = User {
        id,
        public_key: Some(fresh_multikey()),
        profile_version,
        routing_entry: None,
        delegation_gateways: Vec::new(),
    };
    let mut users = state.users.write().unwrap();
    users.insert(id, u);
    users.get(&id).unwrap()
}

/// Installs a Node with a real key, the given manifest_version, and
/// the specified gateway flag; returns the live Arc.
pub fn install_node(
    state: &RouterV2State,
    id: [u8; 8],
    manifest_version: u32,
    is_gateway: bool,
) -> Arc<RwLock<Node>> {
    let n = Node {
        id,
        public_key: Some(fresh_multikey()),
        manifest_version,
        is_gateway,
        delegated_users: Vec::new(),
    };
    let mut nodes = state.nodes.write().unwrap();
    nodes.insert(id, n);
    nodes.get(&id).unwrap()
}
