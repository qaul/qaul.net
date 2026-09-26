// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Whole-message ROUTING_UPDATE processing, section ordering included (spec §8.3).

use crate::router_v2::*;
use crate::router_v2::{
    codec::messages::{Mapping, NodeEntry, RoutingUpdate, UserEntry},
    index::Space,
    seq::SeqNum,
    test_utils::*,
};
use libp2p::PeerId;

const NEIGHBOUR_NODE_ID: [u8; 8] = [77; 8];
const NEIGHBOUR_IDX_IN_NODE_DICT: u16 = 500;

/// Adds a neighbour and binds its node_id in node_dict so that any
/// entry processed downstream can resolve `next_hop`.
fn setup_neighbour(state: &RouterV2State) -> PeerId {
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
    bind_own_dict(
        state,
        Space::Node,
        NEIGHBOUR_IDX_IN_NODE_DICT,
        NEIGHBOUR_NODE_ID,
    );
    peer
}

fn empty_update() -> RoutingUpdate {
    RoutingUpdate {
        user_mappings: Vec::new(),
        node_mappings: Vec::new(),
        user_entries: Vec::new(),
        node_entries: Vec::new(),
    }
}

#[test]
fn empty_message_is_noop() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);

    state
        .handle_routing_update(peer, ConnectionModule::Lan, None, empty_update(), 1_000)
        .unwrap();

    assert_eq!(state.users.read().unwrap().len(), 0);
    assert_eq!(state.nodes.read().unwrap().len(), 0);
    assert!(state
        .routing_table
        .read()
        .unwrap()
        .user_entries
        .iter()
        .all(|s| s.is_none()));
}

/// The critical §8.7 ordering guarantee: a mapping and an entry for
/// the same target arriving in one message must both take effect.
/// This only works if the mapping section is processed before the
/// entry section (otherwise the entry would fail target lookup).
#[test]
fn mapping_then_entry_lands_full_route() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);

    let target_id = [1; 8];
    let msg = RoutingUpdate {
        user_mappings: vec![Mapping {
            abs_idx: 5,
            target_id,
            version: 3,
        }],
        node_mappings: Vec::new(),
        user_entries: vec![UserEntry {
            abs_idx: 5,
            seq: 7,
            metric: 10,
            hop_count: 2,
            local_only: false,
        }],
        node_entries: Vec::new(),
    };

    state
        .handle_routing_update(peer, ConnectionModule::Lan, None, msg, 5_000)
        .unwrap();

    // Mirror binding from the mapping section.
    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(target_id),);
    drop(mirrors);

    // User stub created by the mapping section, with the carried version.
    let users = state.users.read().unwrap();
    let user_arc = users.get(&target_id).expect("stub must exist");
    assert_eq!(user_arc.read().unwrap().profile_version, 3);
    drop(users);

    // Own idx allocated by translate_incoming (in the entry pass).
    let own_idx = state
        .user_dict
        .read()
        .unwrap()
        .idx_of(&target_id)
        .expect("target must be bound in own dict");

    // Routing entry stored at the allocated own_idx.
    let rt = state.routing_table.read().unwrap();
    let stored = rt.get(Space::User, own_idx).expect("entry must be stored");
    let e = stored.read().unwrap();
    assert_eq!(e.seq_num, SeqNum::from(7u16));
    assert_eq!(e.metric, 20); // 10 + hop_cost(Lan, None) = 20
    assert_eq!(e.hop_count, 3);
}

#[test]
fn both_spaces_processed_independently() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);

    let user_id = [1; 8];
    let node_id = [2; 8];
    let msg = RoutingUpdate {
        user_mappings: vec![Mapping {
            abs_idx: 5,
            target_id: user_id,
            version: 1,
        }],
        node_mappings: vec![Mapping {
            abs_idx: 6,
            target_id: node_id,
            version: 2,
        }],
        user_entries: vec![UserEntry {
            abs_idx: 5,
            seq: 1,
            metric: 10,
            hop_count: 1,
            local_only: false,
        }],
        node_entries: vec![NodeEntry {
            abs_idx: 6,
            seq: 1,
            metric: 15,
            hop_count: 1,
            local_only: false,
            manifest_version: 0,
        }],
    };

    state
        .handle_routing_update(peer, ConnectionModule::Lan, None, msg, 1_000)
        .unwrap();

    assert!(state.users.read().unwrap().get(&user_id).is_some());
    assert!(state.nodes.read().unwrap().get(&node_id).is_some());

    let user_idx = state.user_dict.read().unwrap().idx_of(&user_id).unwrap();
    let node_own_idx = state.node_dict.read().unwrap().idx_of(&node_id).unwrap();
    let rt = state.routing_table.read().unwrap();
    assert!(rt.get(Space::User, user_idx).is_some());
    assert!(rt.get(Space::Node, node_own_idx).is_some());
}

/// Unknown neighbour (mirrors doesn't have this peer). Each row's
/// apply_ call handles this internally with Ok; the orchestrator
/// finishes without side effects.
#[test]
fn unknown_neighbour_processes_without_side_effects() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer(); // never added to mirrors

    let msg = RoutingUpdate {
        user_mappings: vec![Mapping {
            abs_idx: 5,
            target_id: [1; 8],
            version: 1,
        }],
        node_mappings: Vec::new(),
        user_entries: Vec::new(),
        node_entries: Vec::new(),
    };

    state
        .handle_routing_update(peer, ConnectionModule::Lan, None, msg, 1_000)
        .unwrap();

    assert_eq!(state.users.read().unwrap().len(), 0);
    assert!(state.mirrors.read().unwrap().get(&peer).is_none());
}
