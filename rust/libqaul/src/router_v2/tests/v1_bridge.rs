// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Rebuilding v1's `RoutingTable` from v2 state
//! (`docs/proposals/v1-to-v2-cutover.md`).

use crate::connections::ConnectionModule;
use crate::router_v2::{
    identity::Multikey,
    index::Space,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
    RouterV2State,
};
use crate::utilities::qaul_id::QaulId;
use libp2p::identity::Keypair;
use std::sync::{Arc, RwLock};

const NOW: u64 = 1_000_000;

/// A neighbour we can actually resolve a next hop through: a mirror so
/// `peer_of_node` finds a peer, and a node-dict binding so `next_hop_node_id`
/// resolves the index.
fn install_next_hop(state: &RouterV2State, node_id: [u8; 8], idx: u16) -> libp2p::PeerId {
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, node_id, ConnectionModule::Lan);
    bind_own_dict(state, Space::Node, idx, node_id);
    peer
}

fn entry_via(
    next_hop: u16,
    transport: ConnectionModule,
    metric: u16,
    hop_count: u8,
) -> RoutingEntry {
    RoutingEntry {
        target_index: 1,
        target: TargetRef::User(Arc::new(RwLock::new(crate::router_v2::table::User {
            id: [0; 8],
            public_key: None,
            profile_version: 0,
            routing_entry: None,
            delegation_gateways: Vec::new(),
            is_hosted: false,
        }))),
        seq_num: SeqNum::from(0u16),
        metric,
        next_hop,
        transport,
        last_update: NOW,
        hop_count,
        local_only: true,
    }
}

/// Gives `user` a direct routing entry pointing at `next_hop`, wiring the
/// weak back-edge the way `commit_routing_entry` does.
fn give_direct_route(
    state: &RouterV2State,
    user_id: [u8; 8],
    idx: u16,
    next_hop: u16,
    transport: ConnectionModule,
    hop_count: u8,
) {
    let users = state.users.read().unwrap();
    let user = users.get(&user_id).unwrap();
    let entry = Arc::new(RwLock::new(entry_via(next_hop, transport, 10, hop_count)));
    user.write().unwrap().routing_entry = Some(Arc::downgrade(&entry));
    drop(users);
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::User, idx, entry);
}

/// Installs a user carrying a real key, and returns its v2 id plus the q8id
/// the bridge should file it under.
fn user_with_key(state: &RouterV2State) -> ([u8; 8], Vec<u8>) {
    let mk = Multikey::from(Keypair::generate_ed25519().public());
    let id = mk.to_id();
    let q8id = QaulId::to_q8id(mk.to_peer_id());
    state.users.write().unwrap().insert(
        id,
        crate::router_v2::table::User {
            id,
            public_key: Some(mk),
            profile_version: 0,
            routing_entry: None,
            delegation_gateways: Vec::new(),
            is_hosted: false,
        },
    );
    (id, q8id)
}

/// The two routers derive their 8-byte ids differently — v1 slices the
/// PeerId, v2 hashes the multikey — so the bridge has to translate rather
/// than reuse the key.
#[test]
fn the_bridge_keys_by_q8id_not_by_the_v2_id() {
    let (state, _rx) = fresh_state();
    let (v2_id, q8id) = user_with_key(&state);
    install_next_hop(&state, [9; 8], 100);
    give_direct_route(&state, v2_id, 40, 100, ConnectionModule::Lan, 3);

    let table = state.create_v1_routing_table(NOW);

    assert!(table.table.contains_key(&q8id), "filed under the q8id");
    assert!(
        !table.table.contains_key(v2_id.as_slice()),
        "the v2 id must not be used as a v1 key"
    );
}

#[test]
fn a_direct_entry_becomes_one_connection() {
    let (state, _rx) = fresh_state();
    let (v2_id, q8id) = user_with_key(&state);
    let peer = install_next_hop(&state, [9; 8], 100);
    give_direct_route(&state, v2_id, 40, 100, ConnectionModule::Lan, 3);

    let table = state.create_v1_routing_table(NOW);
    let entry = table.table.get(&q8id).expect("user present");

    assert_eq!(
        entry.connections.len(),
        1,
        "§9.2 holds one entry per target"
    );
    let connection = &entry.connections[0];
    assert_eq!(connection.module, ConnectionModule::Lan);
    assert_eq!(connection.node, peer);
    assert_eq!(connection.hc, 3);
    assert_eq!(connection.rtt, 0, "v2 has no rtt; it must not be invented");
    assert_eq!(connection.lq, 0);
}

/// §7.4 puts `local_only` in bit 7 and reserves bit 6, so the raw byte is not
/// a hop count.
#[test]
fn the_hop_count_is_masked_before_it_reaches_v1() {
    let (state, _rx) = fresh_state();
    let (v2_id, q8id) = user_with_key(&state);
    install_next_hop(&state, [9; 8], 100);
    // hop count 5 with both the local_only and reserved bits set
    give_direct_route(&state, v2_id, 40, 100, ConnectionModule::Lan, 0b1100_0101);

    let table = state.create_v1_routing_table(NOW);
    assert_eq!(table.table.get(&q8id).unwrap().connections[0].hc, 5);
}

#[test]
fn a_user_without_a_fetched_profile_is_skipped() {
    let (state, _rx) = fresh_state();
    // install_user mints a key; strip it to model a user we know only by id.
    let user = install_user(&state, [3; 8], 0);
    user.write().unwrap().public_key = None;
    install_next_hop(&state, [9; 8], 100);
    give_direct_route(&state, [3; 8], 40, 100, ConnectionModule::Lan, 1);

    let table = state.create_v1_routing_table(NOW);
    assert!(
        table.table.is_empty(),
        "no key means no PeerId and so no q8id to file under"
    );
}

/// §4.1: a user this node hosts is reachable over the Local transport, and
/// has no routing entry of its own to read.
#[test]
fn a_hosted_user_gets_a_local_connection() {
    let (state, _rx) = fresh_state();
    let mk = Multikey::from(Keypair::generate_ed25519().public());
    let q8id = QaulId::to_q8id(mk.to_peer_id());
    state.register_hosted_user(mk.to_id(), 1, mk.clone());

    let table = state.create_v1_routing_table(NOW);
    let connection = &table
        .table
        .get(&q8id)
        .expect("hosted user present")
        .connections[0];

    assert_eq!(connection.module, ConnectionModule::Local);
    assert_eq!(connection.node, state.host_mk.to_peer_id());
    assert_eq!(connection.hc, 0);
}

/// §9.2 step 3: with no direct entry, the user is reachable through the
/// lowest-metric gateway carrying them.
#[test]
fn a_delegated_user_resolves_through_the_best_gateway() {
    let (state, _rx) = fresh_state();
    let (v2_id, q8id) = user_with_key(&state);

    let far_peer = install_next_hop(&state, [1; 8], 101);
    let near_peer = install_next_hop(&state, [2; 8], 102);
    let far = install_node(&state, [1; 8], 1, true);
    let near = install_node(&state, [2; 8], 1, true);

    for (idx, next_hop, metric) in [(101u16, 101u16, 50u16), (102, 102, 10)] {
        let entry = Arc::new(RwLock::new(entry_via(
            next_hop,
            ConnectionModule::Lan,
            metric,
            2,
        )));
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, idx, entry);
    }

    {
        let users = state.users.read().unwrap();
        let user_arc = users.get(&v2_id).unwrap();
        user_arc.write().unwrap().delegation_gateways =
            vec![Arc::downgrade(&far), Arc::downgrade(&near)];
    }

    let table = state.create_v1_routing_table(NOW);
    let connection = &table.table.get(&q8id).unwrap().connections[0];

    assert_eq!(connection.node, near_peer, "metric 10 must beat metric 50");
    assert_ne!(connection.node, far_peer);
}

/// `resolve_forwarding` already treats a vanished next hop as no route; the
/// bridge must do the same rather than emit an unusable entry.
#[test]
fn an_entry_whose_next_hop_is_gone_is_dropped() {
    let (state, _rx) = fresh_state();
    let (v2_id, _q8id) = user_with_key(&state);
    // Bound in the dictionary but never a neighbour, so `peer_of_node` fails.
    bind_own_dict(&state, Space::Node, 100, [9; 8]);
    give_direct_route(&state, v2_id, 40, 100, ConnectionModule::Lan, 1);

    let table = state.create_v1_routing_table(NOW);
    assert!(table.table.is_empty());
}
