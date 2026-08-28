// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Translating a neighbour's index space into our own (spec §3.6).

use crate::router_v2::*;
use crate::router_v2::{index::Space, test_utils::*};

#[test]
fn translate_incoming_unknown_neighbour_returns_unknown_mapping() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let err = state
        .translate_incoming(peer, Space::User, 5, 1_000)
        .unwrap_err();
    assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
}

#[test]
fn translate_incoming_known_neighbour_unknown_idx_returns_unknown_mapping() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let err = state
        .translate_incoming(peer, Space::User, 5, 1_000)
        .unwrap_err();
    assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
}

/// If our own dict already has a binding for the ID, return the
/// existing own_idx; do not allocate, do not mark the tracker.
#[test]
fn translate_incoming_existing_own_binding_returns_existing_idx() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let id = [7; 8];

    bind_mirror(&state, peer, Space::User, 5, id);
    state.user_dict.write().unwrap().bind(99, id);

    let got = state
        .translate_incoming(peer, Space::User, 5, 1_000)
        .unwrap();
    assert_eq!(got, 99);

    let pending = state
        .reintroduction_tracker
        .write()
        .unwrap()
        .take_pending(Space::User);
    assert!(
        pending.is_empty(),
        "existing-binding path must not touch the tracker"
    );
}

#[test]
fn translate_incoming_fresh_allocates_binds_and_marks_tracker() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let id = [11; 8];
    bind_mirror(&state, peer, Space::User, 5, id);

    let allocated_idx = state
        .translate_incoming(peer, Space::User, 5, 1_000)
        .unwrap();

    let dict = state.user_dict.read().unwrap();
    assert_eq!(dict.idx_of(&id), Some(allocated_idx));
    assert_eq!(dict.id_of(allocated_idx), Some(id));
    drop(dict);

    let pending = state
        .reintroduction_tracker
        .write()
        .unwrap()
        .take_pending(Space::User);
    assert!(pending.contains(&allocated_idx));
}

#[test]
fn translate_incoming_is_idempotent_for_same_id() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let id = [13; 8];
    bind_mirror(&state, peer, Space::User, 5, id);

    let first = state
        .translate_incoming(peer, Space::User, 5, 1_000)
        .unwrap();
    let second = state
        .translate_incoming(peer, Space::User, 5, 1_000)
        .unwrap();
    assert_eq!(first, second);
}

#[test]
fn translate_incoming_spaces_are_independent() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let user_id = [21; 8];
    let node_id = [22; 8];

    bind_mirror(&state, peer, Space::User, 5, user_id);
    bind_mirror(&state, peer, Space::Node, 5, node_id);

    let user_idx = state
        .translate_incoming(peer, Space::User, 5, 1_000)
        .unwrap();
    let node_idx = state
        .translate_incoming(peer, Space::Node, 5, 1_000)
        .unwrap();

    assert_eq!(
        state.user_dict.read().unwrap().id_of(user_idx),
        Some(user_id)
    );
    assert_eq!(
        state.node_dict.read().unwrap().id_of(node_idx),
        Some(node_id)
    );
    assert_eq!(state.node_dict.read().unwrap().idx_of(&user_id), None);
    assert_eq!(state.user_dict.read().unwrap().idx_of(&node_id), None);
}

// ---------- pending_introductions ----------

#[test]
fn pending_introductions_empty_when_no_marks() {
    let (state, _rx) = fresh_state();
    assert!(state.pending_introductions(Space::User).is_empty());
    assert!(state.pending_introductions(Space::Node).is_empty());
}

#[test]
fn pending_introductions_returns_marked_user_with_correct_version() {
    let (state, _rx) = fresh_state();
    let id = [3; 8];
    install_user(&state, id, 42);
    state.user_dict.write().unwrap().bind(7, id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 7);

    let out = state.pending_introductions(Space::User);
    assert_eq!(out, vec![(7, id, 42)]);
}

#[test]
fn pending_introductions_returns_marked_node_with_correct_version() {
    let (state, _rx) = fresh_state();
    let id = [4; 8];
    install_node(&state, id, 99, false);
    state.node_dict.write().unwrap().bind(8, id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::Node, 8);

    let out = state.pending_introductions(Space::Node);
    assert_eq!(out, vec![(8, id, 99)]);
}

#[test]
fn pending_introductions_drains_only_requested_space() {
    let (state, _rx) = fresh_state();

    let user_id = [1; 8];
    install_user(&state, user_id, 5);
    state.user_dict.write().unwrap().bind(10, user_id);

    let node_id = [2; 8];
    install_node(&state, node_id, 6, false);
    state.node_dict.write().unwrap().bind(20, node_id);

    {
        let mut t = state.reintroduction_tracker.write().unwrap();
        t.mark_first_time(Space::User, 10);
        t.mark_first_time(Space::Node, 20);
    }

    let users = state.pending_introductions(Space::User);
    assert_eq!(users, vec![(10, user_id, 5)]);

    let nodes = state.pending_introductions(Space::Node);
    assert_eq!(nodes, vec![(20, node_id, 6)]);
}

#[test]
fn pending_introductions_second_call_returns_empty_after_drain() {
    let (state, _rx) = fresh_state();
    let id = [9; 8];
    install_user(&state, id, 1);
    state.user_dict.write().unwrap().bind(3, id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 3);

    let first = state.pending_introductions(Space::User);
    assert!(!first.is_empty(), "first call should drain the mark");

    let second = state.pending_introductions(Space::User);
    assert!(second.is_empty(), "second call should be empty after drain");
}

/// Phase 8's delta encoder requires ascending idx order.
#[test]
fn pending_introductions_results_sorted_by_index() {
    let (state, _rx) = fresh_state();
    let ids: Vec<[u8; 8]> = (1..=5).map(|i| [i as u8; 8]).collect();
    let idxs = [50u16, 10, 200, 30, 80];

    for (i, idx) in idxs.iter().enumerate() {
        install_user(&state, ids[i], i as u32);
        state.user_dict.write().unwrap().bind(*idx, ids[i]);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, *idx);
    }

    let out = state.pending_introductions(Space::User);
    let returned_idxs: Vec<u16> = out.iter().map(|(idx, _, _)| *idx).collect();
    let mut expected = idxs.to_vec();
    expected.sort();
    assert_eq!(returned_idxs, expected);
}

#[test]
fn pending_introductions_skips_orphan_with_no_dict_binding() {
    let (state, _rx) = fresh_state();
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 42);

    let out = state.pending_introductions(Space::User);
    assert!(
        out.is_empty(),
        "orphan mark with no dict binding must be skipped"
    );
}

#[test]
fn pending_introductions_skips_orphan_with_no_record() {
    let (state, _rx) = fresh_state();
    let id = [77; 8];
    state.user_dict.write().unwrap().bind(42, id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 42);

    let out = state.pending_introductions(Space::User);
    assert!(out.is_empty(), "missing user record must be skipped");
}

#[test]
fn pending_introductions_mixed_healthy_and_orphan() {
    let (state, _rx) = fresh_state();

    let good_id = [1; 8];
    install_user(&state, good_id, 7);
    state.user_dict.write().unwrap().bind(10, good_id);

    {
        let mut t = state.reintroduction_tracker.write().unwrap();
        t.mark_first_time(Space::User, 10);
        t.mark_first_time(Space::User, 99); // orphan
    }

    let out = state.pending_introductions(Space::User);
    assert_eq!(out, vec![(10, good_id, 7)]);
}
