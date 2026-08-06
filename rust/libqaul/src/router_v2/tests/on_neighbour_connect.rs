// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Bootstrap behaviour when a neighbour appears (spec §8.4).

use crate::router_v2::*;
use crate::router_v2::{
    codec::{messages::IndexDump, Header, RoutingMessage},
    index::Space,
    propagation::on_neighbour_connect,
    test_utils::*,
};

/// Decode a framed OutboundMsg body into an IndexDump.
fn decode_dump_body(bytes: &[u8]) -> IndexDump {
    let (header, body_slice) = Header::decode(bytes).expect("frame header");
    assert_eq!(header.message_type, RoutingMessage::IndexDump);
    let payload = &body_slice[..header.payload_len as usize];
    IndexDump::decode(payload).expect("IndexDump body")
}

/// A node with nothing bound still emits a dump — the message itself is
/// the signal that the neighbour should introduce itself back.
///
/// Both sections are empty: §3.5 ties each reserved index to a propagation
/// form, and a fresh node hosts no users and holds no INTERNET connection,
/// so it is in user form with no hosted user yet. Neither space has a
/// self-binding to advertise.
#[test]
fn empty_state_still_sends_an_empty_dump() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let msg = rx.try_recv().expect("bootstrap must always emit");
    assert_eq!(msg.peer, peer);
    assert_eq!(msg.transport, ConnectionModule::Lan);
    let dump = decode_dump_body(&msg.bytes);

    assert!(dump.user_mappings.is_empty());
    assert!(
        dump.node_mappings.is_empty(),
        "no self-binding exists until a propagation form is established"
    );

    assert!(rx.try_recv().is_err(), "one dump per neighbour");
}

/// Once a hosted user takes the user-space reserved slot, the dump carries
/// it — this is what lets a peer translate the origin's user entry at
/// index 0.
#[test]
fn user_form_dump_carries_the_hosted_user_self_binding() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    let user_id = [42; 8];
    state.register_hosted_user(user_id, 7);

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let dump = decode_dump_body(&rx.try_recv().expect("one outbound").bytes);
    assert_eq!(dump.user_mappings.len(), 1);
    assert_eq!(dump.user_mappings[0].abs_idx, 0);
    assert_eq!(dump.user_mappings[0].target_id, user_id);
    assert_eq!(dump.user_mappings[0].version, 7);
    assert!(
        dump.node_mappings.is_empty(),
        "node-space reserved slot stays unbound in user form"
    );
}

#[test]
fn populated_dicts_produce_correct_mappings() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    // Set up a user binding + user record with a specific version.
    let user_id = [1; 8];
    install_user(&state, user_id, 42);
    bind_own_dict(&state, Space::User, 7, user_id);

    // And a node binding + node record.
    let node_id = [2; 8];
    install_node(&state, node_id, 99, false);
    bind_own_dict(&state, Space::Node, 8, node_id);

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let msg = rx.try_recv().expect("one outbound");
    let dump = decode_dump_body(&msg.bytes);

    // User side.
    assert_eq!(dump.user_mappings.len(), 1);
    assert_eq!(dump.user_mappings[0].abs_idx, 7);
    assert_eq!(dump.user_mappings[0].target_id, user_id);
    assert_eq!(dump.user_mappings[0].version, 42);

    // Node side: only our installed binding at idx 8. The node-space
    // reserved slot is unbound in user form (§3.5), so there is no
    // self-mapping alongside it.
    assert_eq!(dump.node_mappings.len(), 1);
    let installed = &dump.node_mappings[0];
    assert_eq!(installed.abs_idx, 8);
    assert_eq!(installed.target_id, node_id);
    assert_eq!(installed.version, 99);
}

/// Delta-encoding invariant: mappings must arrive sorted by abs_idx
/// on the wire. HashMap iteration is non-deterministic; the sort in
/// the code is what pins this contract.
#[test]
fn mappings_sorted_by_abs_idx() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    // Bind at unsorted indices.
    for (i, idx) in [100u16, 5, 50].iter().enumerate() {
        let id = [i as u8 + 1; 8];
        install_user(&state, id, 0);
        bind_own_dict(&state, Space::User, *idx, id);
    }

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let msg = rx.try_recv().expect("one outbound");
    let dump = decode_dump_body(&msg.bytes);
    let idxs: Vec<u16> = dump.user_mappings.iter().map(|m| m.abs_idx).collect();
    assert_eq!(idxs, vec![5, 50, 100]);
}

#[test]
fn ble1m_transport_skips_send() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    // Populate dicts so we'd otherwise have something to send.
    install_user(&state, [1; 8], 0);
    bind_own_dict(&state, Space::User, 7, [1; 8]);

    on_neighbour_connect(&state, peer, ConnectionModule::Ble1m);

    assert!(rx.try_recv().is_err(), "BLE must not receive INDEX_DUMP");
}

#[test]
fn ble_coded_transport_skips_send() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    install_user(&state, [1; 8], 0);
    bind_own_dict(&state, Space::User, 7, [1; 8]);

    on_neighbour_connect(&state, peer, ConnectionModule::BleCoded);

    assert!(
        rx.try_recv().is_err(),
        "BLE-coded must not receive INDEX_DUMP"
    );
}

/// dict has an idx→id binding but no matching User record — the
/// `unwrap_or(0)` fallback surfaces here.
#[test]
fn missing_user_record_defaults_version_to_zero() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    // Bind but don't install the User.
    bind_own_dict(&state, Space::User, 5, [77; 8]);

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let msg = rx.try_recv().expect("one outbound");
    let dump = decode_dump_body(&msg.bytes);
    assert_eq!(dump.user_mappings.len(), 1);
    assert_eq!(dump.user_mappings[0].abs_idx, 5);
    assert_eq!(dump.user_mappings[0].target_id, [77; 8]);
    assert_eq!(dump.user_mappings[0].version, 0);
}

#[test]
fn emits_indexdump_message_type() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let msg = rx.try_recv().expect("one outbound");
    let (header, _) = Header::decode(&msg.bytes).expect("frame header");
    assert_eq!(header.message_type, RoutingMessage::IndexDump);
}
