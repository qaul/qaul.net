// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Origin and relay tick output (spec §7.1).

use crate::router_v2::*;
use crate::router_v2::{
    codec::{messages::RoutingUpdate, Header, RoutingMessage},
    index::Space,
    propagation::{
        blocked_by_split_horizon, compute_outgoing_local_only, should_propagate, tick_origin,
    },
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
    Sphere,
};

/// Build a RoutingEntry with the fields the caller cares about.
/// Other fields get harmless defaults.
fn make_entry(
    target: TargetRef,
    next_hop: u16,
    transport: ConnectionModule,
    local_only: bool,
) -> RoutingEntry {
    RoutingEntry {
        target_index: 0,
        target,
        seq_num: SeqNum::from(0u16),
        metric: 0,
        next_hop,
        transport,
        last_update: 0,
        hop_count: 0,
        local_only,
    }
}

/// Decode a framed OutboundMsg body back into a RoutingUpdate for
/// tick_origin/tick_relay tests to inspect the wire output.
fn decode_frame(bytes: &[u8]) -> RoutingUpdate {
    let (header, body_slice) = Header::decode(bytes).expect("frame header");
    assert_eq!(header.message_type, RoutingMessage::RoutingUpdate);
    let payload = &body_slice[..header.payload_len as usize];
    RoutingUpdate::decode(payload).expect("routing-update body")
}

// ---------- blocked_by_split_horizon ----------

#[test]
fn split_horizon_blocks_when_next_hop_is_outgoing_neighbour() {
    let (state, _rx) = fresh_state();
    let target = install_user(&state, [1; 8], 0);
    let neighbour_id = [42; 8];
    bind_own_dict(&state, Space::Node, 7, neighbour_id);

    let entry = make_entry(TargetRef::User(target), 7, ConnectionModule::Lan, false);
    assert!(blocked_by_split_horizon(&state, &entry, neighbour_id));
}

#[test]
fn split_horizon_allows_when_next_hop_is_different_neighbour() {
    let (state, _rx) = fresh_state();
    let target = install_user(&state, [1; 8], 0);
    bind_own_dict(&state, Space::Node, 7, [42; 8]);

    let entry = make_entry(TargetRef::User(target), 7, ConnectionModule::Lan, false);
    assert!(!blocked_by_split_horizon(&state, &entry, [99; 8]));
}

/// Defensive: an entry pointing at an unresolvable next_hop is blocked
/// rather than sprayed onto every neighbour.
#[test]
fn split_horizon_blocks_when_next_hop_unresolvable() {
    let (state, _rx) = fresh_state();
    let target = install_user(&state, [1; 8], 0);

    let entry = make_entry(TargetRef::User(target), 99, ConnectionModule::Lan, false);
    assert!(blocked_by_split_horizon(&state, &entry, [42; 8]));
}

// ---------- should_propagate ----------

#[test]
fn should_propagate_local_outgoing_allows_local_learned() {
    let (state, _rx) = fresh_state();
    let target = install_user(&state, [1; 8], 0);
    let entry = make_entry(TargetRef::User(target), 0, ConnectionModule::Lan, false);
    assert!(should_propagate(&entry, Sphere::Local));
}

#[test]
fn should_propagate_local_outgoing_blocks_internet_learned() {
    let (state, _rx) = fresh_state();
    let target = install_user(&state, [1; 8], 0);
    let entry = make_entry(
        TargetRef::User(target),
        0,
        ConnectionModule::Internet,
        false,
    );
    assert!(!should_propagate(&entry, Sphere::Local));
}

#[test]
fn should_propagate_internet_outgoing_allows_gateway_node() {
    let (state, _rx) = fresh_state();
    let target = install_node(&state, [1; 8], 0, true);
    let entry = make_entry(TargetRef::Node(target), 0, ConnectionModule::Lan, false);
    assert!(should_propagate(&entry, Sphere::Internet));
}

#[test]
fn should_propagate_internet_outgoing_blocks_non_gateway_node() {
    let (state, _rx) = fresh_state();
    let target = install_node(&state, [1; 8], 0, false);
    let entry = make_entry(TargetRef::Node(target), 0, ConnectionModule::Lan, false);
    assert!(!should_propagate(&entry, Sphere::Internet));
}

/// User targets never cross the membrane upward, regardless of where
/// they were learned.
#[test]
fn should_propagate_internet_outgoing_blocks_user_targets() {
    let (state, _rx) = fresh_state();
    let target = install_user(&state, [1; 8], 0);
    let entry = make_entry(
        TargetRef::User(target),
        0,
        ConnectionModule::Internet,
        false,
    );
    assert!(!should_propagate(&entry, Sphere::Internet));
}

// ---------- compute_outgoing_local_only ----------

#[test]
fn outgoing_local_only_internet_always_false() {
    assert!(!compute_outgoing_local_only(false, Sphere::Internet));
    assert!(!compute_outgoing_local_only(true, Sphere::Internet));
}

#[test]
fn outgoing_local_only_local_passes_stored_through() {
    assert!(!compute_outgoing_local_only(false, Sphere::Local));
    assert!(compute_outgoing_local_only(true, Sphere::Local));
}

// ---------- tick_origin ----------

#[test]
fn tick_origin_with_no_neighbours_pushes_nothing() {
    let (state, mut rx) = fresh_state();

    let before = state.seq_num.read().unwrap().value();
    tick_origin(&state, 0);
    let after = state.seq_num.read().unwrap().value();

    // seq_num always increments once per tick, even with no neighbours.
    assert_eq!(after, before.wrapping_add(1));
    assert!(rx.try_recv().is_err(), "no neighbours → no messages");
}

/// One Lan neighbour → one message pushed with local_only=1 (§7.4
/// origin rule for Local-outgoing).
#[test]
fn tick_origin_one_lan_neighbour_pushes_one_message_with_local_only_true() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

    tick_origin(&state, 0);

    let msg = rx.try_recv().expect("one outbound expected");
    assert_eq!(msg.peer, peer);
    assert_eq!(msg.transport, ConnectionModule::Lan);
    assert!(rx.try_recv().is_err(), "no more outbounds");

    let update = decode_frame(&msg.bytes);
    assert_eq!(update.user_entries.len(), 1);
    assert!(update.node_entries.is_empty());
    let entry = &update.user_entries[0];
    assert_eq!(entry.abs_idx, 0, "origin uses RESERVED_INDEX");
    assert_eq!(entry.metric, 0);
    assert_eq!(entry.hop_count, 0);
    assert!(entry.local_only, "Local-outgoing → wire local_only=1");
}

/// One Internet neighbour → one message with local_only=0 (§7.4
/// origin rule for Internet-outgoing).
///
/// An INTERNET neighbour is also §3.2's gateway trigger, so the node is in
/// *node* form here and originates a node entry rather than a user entry.
/// Setting up a neighbour transport in a test therefore decides which entry
/// section gets populated — `desired_propagation_form` reads `mirrors`.
#[test]
fn tick_origin_one_internet_neighbour_pushes_message_with_local_only_false() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

    tick_origin(&state, 0);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    assert!(
        update.user_entries.is_empty(),
        "an INTERNET neighbour puts the node in node form (§3.2)"
    );
    assert_eq!(update.node_entries.len(), 1);
    assert!(!update.node_entries[0].local_only);
}

/// A neighbour reachable on two transports gets *two* outbound
/// messages this tick — one per (peer, transport) pair (§4.2).
#[test]
fn tick_origin_multi_transport_neighbour_pushes_one_per_transport() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

    tick_origin(&state, 0);

    let mut got_transports = Vec::new();
    while let Ok(msg) = rx.try_recv() {
        assert_eq!(msg.peer, peer);
        got_transports.push(msg.transport);
    }
    got_transports.sort_by_key(|t| format!("{t:?}"));
    assert_eq!(got_transports.len(), 2);
    assert!(got_transports.contains(&ConnectionModule::Lan));
    assert!(got_transports.contains(&ConnectionModule::Internet));
}

/// Pending introductions must be attached to every neighbour's message
/// in the mapping section corresponding to the origin space.
#[test]
fn tick_origin_attaches_pending_introductions_to_mapping_section() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

    // Set up an introduction: install a user, bind dict, mark tracker.
    let user_id = [11; 8];
    install_user(&state, user_id, 3);
    state.user_dict.write().unwrap().bind(5, user_id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 5);

    tick_origin(&state, 0);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    assert_eq!(update.user_mappings.len(), 1);
    assert!(update.node_mappings.is_empty());
    assert_eq!(update.user_mappings[0].abs_idx, 5);
    assert_eq!(update.user_mappings[0].target_id, user_id);
    assert_eq!(update.user_mappings[0].version, 3);
}

/// Two neighbours + one introduction → the *same* mapping section
/// appears in *both* outbound messages. Drain the tracker only once,
/// but attach to all neighbours (§3.8).
#[test]
fn tick_origin_same_intros_attached_to_all_neighbours() {
    let (state, mut rx) = fresh_state();
    let peer_a = fresh_peer();
    let peer_b = fresh_peer();
    state.add_neighbour_transport(peer_a, [10; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(peer_b, [20; 8], ConnectionModule::Lan);

    let user_id = [1; 8];
    install_user(&state, user_id, 7);
    state.user_dict.write().unwrap().bind(3, user_id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 3);

    tick_origin(&state, 0);

    let m1 = rx.try_recv().expect("outbound 1");
    let m2 = rx.try_recv().expect("outbound 2");
    assert!(rx.try_recv().is_err());

    let u1 = decode_frame(&m1.bytes);
    let u2 = decode_frame(&m2.bytes);
    assert_eq!(u1.user_mappings.len(), 1);
    assert_eq!(u2.user_mappings.len(), 1);
    assert_eq!(u1.user_mappings[0].target_id, user_id);
    assert_eq!(u2.user_mappings[0].target_id, user_id);
}

/// tick_origin increments seq_num by exactly one per invocation. The
/// wire entry's `seq` equals the new value after the increment.
#[test]
fn tick_origin_wire_seq_matches_incremented_seq_num() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

    let before = state.seq_num.read().unwrap().value();
    tick_origin(&state, 0);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    assert_eq!(update.user_entries[0].seq, before.wrapping_add(1));
}
