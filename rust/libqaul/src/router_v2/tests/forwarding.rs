// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Next-hop selection for outbound messages (spec §9.2, Primer Part I §4).

use crate::router_v2::*;
use crate::router_v2::{
    forwarding::ForwardingDecision,
    identity::Multikey,
    index::Space,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
};
use libp2p::identity::Keypair;

fn entry(
    target: TargetRef,
    next_hop: u16,
    metric: u16,
    transport: ConnectionModule,
) -> Arc<RwLock<RoutingEntry>> {
    Arc::new(RwLock::new(RoutingEntry {
        target_index: 0,
        target,
        seq_num: SeqNum::from(0u16),
        metric,
        next_hop,
        transport,
        last_update: 0,
        hop_count: 0,
        local_only: false,
    }))
}

/// A neighbour registered exactly as `ping_event` does it: node id derived
/// from the PeerId, bound in the node dictionary at `idx`.
fn neighbour(state: &RouterV2State, idx: u16, transport: ConnectionModule) -> (PeerId, [u8; 8]) {
    let kp = Keypair::generate_ed25519();
    let peer = kp.public().to_peer_id();
    let node_id = Multikey::from(kp.public()).to_id();
    state.add_neighbour_transport(peer, node_id, transport);
    bind_own_dict(state, Space::Node, idx, node_id);
    (peer, node_id)
}

/// A recipient PeerId together with the 8-byte id v2 keys it under.
fn recipient() -> (PeerId, [u8; 8]) {
    let kp = Keypair::generate_ed25519();
    (
        kp.public().to_peer_id(),
        Multikey::from(kp.public()).to_id(),
    )
}

// ---------- step 1: the recipient is directly known ----------

#[test]
fn a_known_user_forwards_to_the_neighbour_that_owns_the_route() {
    let (state, _rx) = fresh_state();
    let (peer, _) = neighbour(&state, 100, ConnectionModule::Lan);
    let (recipient_peer, recipient_id) = recipient();

    let user = install_user(&state, recipient_id, 0);
    let e = entry(
        TargetRef::User(user.clone()),
        100,
        10,
        ConnectionModule::Lan,
    );
    user.write().unwrap().routing_entry = Some(Arc::downgrade(&e));
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::User, 5, e.clone());

    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::Forward {
            peer,
            transport: ConnectionModule::Lan,
        }
    );
}

#[test]
fn the_entrys_transport_is_carried_through() {
    let (state, _rx) = fresh_state();
    let (peer, _) = neighbour(&state, 100, ConnectionModule::Internet);
    let (recipient_peer, recipient_id) = recipient();

    let user = install_user(&state, recipient_id, 0);
    let e = entry(
        TargetRef::User(user.clone()),
        100,
        10,
        ConnectionModule::Internet,
    );
    user.write().unwrap().routing_entry = Some(Arc::downgrade(&e));
    state.routing_table.write().unwrap().set(Space::User, 5, e);

    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::Forward {
            peer,
            transport: ConnectionModule::Internet,
        }
    );
}

// ---------- the seam between next_hop_for_user and peer_of_node ----------

/// `next_hop_for_user` resolves against the routing table, which can outlive
/// the neighbour it names. That must fall through to the default route, not
/// fail: a gateway may still reach the recipient.
#[test]
fn a_next_hop_that_is_no_longer_a_neighbour_falls_through() {
    let (state, _rx) = fresh_state();
    let (peer, _) = neighbour(&state, 100, ConnectionModule::Lan);
    let (recipient_peer, recipient_id) = recipient();

    let user = install_user(&state, recipient_id, 0);
    let e = entry(
        TargetRef::User(user.clone()),
        100,
        10,
        ConnectionModule::Lan,
    );
    user.write().unwrap().routing_entry = Some(Arc::downgrade(&e));
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::User, 5, e.clone());

    // The route still points at index 100, but the neighbour is gone.
    state.remove_neighbour_transport(peer, ConnectionModule::Lan);

    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::HandoffToDTN,
        "no gateway available, so the fall-through ends at DTN"
    );
}

// ---------- step 2: unknown recipient ----------

#[test]
fn an_unknown_recipient_on_a_leaf_with_no_gateway_hands_off_to_dtn() {
    let (state, _rx) = fresh_state();
    neighbour(&state, 100, ConnectionModule::Lan);
    let (recipient_peer, _) = recipient();

    assert!(!state.host_is_gateway());
    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::HandoffToDTN
    );
}

/// Anycast: a leaf defaults toward the lowest-metric reachable gateway.
#[test]
fn a_leaf_routes_an_unknown_recipient_to_the_nearest_gateway() {
    let (state, _rx) = fresh_state();
    let (near_peer, near_id) = neighbour(&state, 100, ConnectionModule::Lan);
    let (_far_peer, far_id) = neighbour(&state, 101, ConnectionModule::Lan);
    let (recipient_peer, _) = recipient();

    for (id, idx, metric) in [(near_id, 100u16, 10u16), (far_id, 101, 50)] {
        let node = install_node(&state, id, 1, true);
        let e = entry(TargetRef::Node(node), idx, metric, ConnectionModule::Lan);
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, idx, e);
    }

    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::Forward {
            peer: near_peer,
            transport: ConnectionModule::Lan,
        },
        "the metric-10 gateway wins over metric-50"
    );
}

/// Re-resolution on every call is what absorbs gateway loss — nothing is
/// cached, so dropping the best gateway promotes the next one immediately.
#[test]
fn losing_the_best_gateway_promotes_the_next_one() {
    let (state, _rx) = fresh_state();
    let (near_peer, near_id) = neighbour(&state, 100, ConnectionModule::Lan);
    let (far_peer, far_id) = neighbour(&state, 101, ConnectionModule::Lan);
    let (recipient_peer, _) = recipient();

    for (id, idx, metric) in [(near_id, 100u16, 10u16), (far_id, 101, 50)] {
        let node = install_node(&state, id, 1, true);
        let e = entry(TargetRef::Node(node), idx, metric, ConnectionModule::Lan);
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, idx, e);
    }
    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::Forward {
            peer: near_peer,
            transport: ConnectionModule::Lan
        }
    );

    state.routing_table.write().unwrap().clear(Space::Node, 100);

    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::Forward {
            peer: far_peer,
            transport: ConnectionModule::Lan,
        }
    );
}

/// `is_gateway` says what a node *is*, not that we can reach it. Without a
/// live routing entry it is not a candidate.
#[test]
fn a_gateway_without_a_routing_entry_is_not_a_candidate() {
    let (state, _rx) = fresh_state();
    let (_peer, node_id) = neighbour(&state, 100, ConnectionModule::Lan);
    let (recipient_peer, _) = recipient();

    install_node(&state, node_id, 1, true);
    // No routing-table entry installed for index 100.

    assert_eq!(state.nearest_gateway(), None);
    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::HandoffToDTN
    );
}

#[test]
fn a_non_gateway_node_is_never_a_default_route() {
    let (state, _rx) = fresh_state();
    let (_peer, node_id) = neighbour(&state, 100, ConnectionModule::Lan);
    let (recipient_peer, _) = recipient();

    let node = install_node(&state, node_id, 1, false);
    let e = entry(TargetRef::Node(node), 100, 10, ConnectionModule::Lan);
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, 100, e);

    assert_eq!(state.nearest_gateway(), None);
    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::HandoffToDTN
    );
}

/// §9.2: a gateway already holds the global directory, so a miss there is
/// authoritative — it must not default-route to another gateway.
#[test]
fn a_gateway_host_treats_an_unknown_recipient_as_unreachable() {
    let (state, _rx) = fresh_state();
    let (_peer, node_id) = neighbour(&state, 100, ConnectionModule::Lan);
    let (recipient_peer, _) = recipient();

    // Another reachable gateway exists...
    let node = install_node(&state, node_id, 1, true);
    let e = entry(TargetRef::Node(node), 100, 10, ConnectionModule::Lan);
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, 100, e);
    assert!(state.nearest_gateway().is_some());

    // ...but we are a gateway ourselves.
    state.manifest.write().unwrap().set_gateway(true);
    assert!(state.host_is_gateway());

    assert_eq!(
        state.resolve_forwarding(recipient_peer),
        ForwardingDecision::HandoffToDTN,
        "a gateway must not punt an unknown recipient to another gateway"
    );
}

// ---------- id derivation ----------

/// A PeerId with a hashed multihash has no recoverable key, so it has no v2
/// identity at all. That is an ordinary unreachable recipient, not an error.
#[test]
fn a_recipient_without_a_recoverable_key_hands_off_to_dtn() {
    use sha2::{Digest, Sha256};

    let (state, _rx) = fresh_state();
    let (_peer, node_id) = neighbour(&state, 100, ConnectionModule::Lan);

    // Give the node a reachable gateway, so DTN cannot be reached by accident.
    let node = install_node(&state, node_id, 1, true);
    let e = entry(TargetRef::Node(node), 100, 10, ConnectionModule::Lan);
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, 100, e);

    let digest = Sha256::digest(b"not a key");
    let multihash =
        libp2p::multihash::Multihash::<64>::wrap(0x12, &digest).expect("digest fits in 64 bytes");
    let hashed = PeerId::from_multihash(multihash).expect("sha2-256 is a valid peer id code");

    assert_eq!(
        state.resolve_forwarding(hashed),
        ForwardingDecision::HandoffToDTN,
        "an unrecoverable key must not fall through to the gateway default route"
    );
}
