// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Bootstrap behaviour when a neighbour appears (spec §8.4).

use crate::router_v2::*;
use crate::router_v2::{
    codec::{
        messages::{IndexDump, RoutingUpdate},
        Header, RoutingMessage,
    },
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

/// Decode a framed OutboundMsg body into a RoutingUpdate.
fn decode_update_body(bytes: &[u8]) -> RoutingUpdate {
    let (header, body_slice) = Header::decode(bytes).expect("frame header");
    assert_eq!(header.message_type, RoutingMessage::RoutingUpdate);
    let payload = &body_slice[..header.payload_len as usize];
    RoutingUpdate::decode(payload).expect("RoutingUpdate body")
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

    // The dump is followed by our own entry, so the neighbour does not wait
    // for the ten-second origin tick to hear about us.
    let origin = rx.try_recv().expect("an origin update follows the dump");
    let update = decode_update_body(&origin.bytes);
    assert_eq!(
        update.user_entries.len(),
        1,
        "user form originates a user entry"
    );
    assert_eq!(update.user_entries[0].metric, 0);
    assert_eq!(update.user_entries[0].hop_count, 0);
    assert!(
        update.user_mappings.is_empty(),
        "no self-binding exists yet, so there is nothing to introduce"
    );

    assert!(
        rx.try_recv().is_err(),
        "one dump and one origin per neighbour"
    );
}

/// Once a hosted user takes the user-space reserved slot, the dump carries
/// it — this is what lets a peer translate the origin's user entry at
/// index 0.
#[test]
fn user_form_dump_carries_the_hosted_user_self_binding() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    let user_id = [42; 8];
    state.register_hosted_user(user_id, 7, fresh_multikey());

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

/// §7.1's origin phase is every ten seconds and `spawn_origin_tick` eats its
/// first tick, so without this a neighbour that appeared at t=2s heard
/// nothing about us until t=10s.
#[test]
fn the_origin_update_introduces_our_own_entry() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    let user_id = [42; 8];
    state.register_hosted_user(user_id, 7, fresh_multikey());

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let _dump = rx.try_recv().expect("the dump comes first");
    let update = decode_update_body(&rx.try_recv().expect("then the origin update").bytes);

    assert_eq!(update.user_entries.len(), 1);
    assert_eq!(update.user_entries[0].abs_idx, 0);
    assert_eq!(update.user_entries[0].metric, 0, "§7.1: metric zero");
    assert_eq!(update.user_entries[0].hop_count, 0, "§7.1: hop count zero");

    // §8.3: the inline mapping is what lets the receiver resolve index 0
    assert_eq!(update.user_mappings.len(), 1);
    assert_eq!(update.user_mappings[0].abs_idx, 0);
    assert_eq!(update.user_mappings[0].target_id, user_id);
    assert_eq!(update.user_mappings[0].version, 7);
}

/// §6.1 ties the increment to the origin cycle: "The sequence number is
/// incremented by one at each origin update cycle (every ten seconds)". A
/// neighbour-connect emission is not a cycle, so it reuses the current value.
/// §7.2 then drops the duplicate at any neighbour that already holds it,
/// which is also what stops a flapping link driving a sequence-number storm.
#[test]
fn the_origin_update_does_not_increment_the_sequence_number() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.register_hosted_user([42; 8], 0, fresh_multikey());

    let before = state.seq_num.read().unwrap().value();

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let _dump = rx.try_recv().expect("the dump comes first");
    let update = decode_update_body(&rx.try_recv().expect("then the origin update").bytes);

    assert_eq!(
        update.user_entries[0].seq, before,
        "sent at the current seq"
    );
    assert_eq!(
        state.seq_num.read().unwrap().value(),
        before,
        "the origin cycle owns the increment, not this path"
    );
}

/// §2.3: the entry crossing into the Local sphere is marked local_only, the
/// same as the origin tick marks it.
#[test]
fn the_origin_update_carries_the_sphere_flag() {
    let (state, mut rx) = fresh_state();
    state.register_hosted_user([42; 8], 0, fresh_multikey());

    on_neighbour_connect(&state, fresh_peer(), ConnectionModule::Lan);
    let _ = rx.try_recv();
    let lan = decode_update_body(&rx.try_recv().expect("lan origin update").bytes);
    assert!(lan.user_entries[0].local_only, "LAN is the Local sphere");

    on_neighbour_connect(&state, fresh_peer(), ConnectionModule::Internet);
    let _ = rx.try_recv();
    let internet = decode_update_body(&rx.try_recv().expect("internet origin update").bytes);
    assert!(
        !internet.user_entries[0].local_only,
        "INTERNET is not the Local sphere"
    );
}

/// The complement of the connect-time emission. At startup the neighbour is
/// registered about 10ms *before* the hosted user reaches the reserved index,
/// so the connect-time update has no self-binding to introduce and the
/// receiver drops our entry as an unknown mapping (§8.3). Binding the user has
/// to originate as well, or nothing about us reaches the neighbour until the
/// ten-second origin tick.
#[test]
fn binding_the_hosted_user_originates_to_existing_neighbours() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [9; 8], ConnectionModule::Lan);
    while rx.try_recv().is_ok() {}

    let user_id = [42; 8];
    state.register_hosted_user(user_id, 3, fresh_multikey());

    let msg = rx
        .try_recv()
        .expect("binding the hosted user must originate");
    assert_eq!(msg.peer, peer);
    let update = decode_update_body(&msg.bytes);
    assert_eq!(update.user_entries.len(), 1);
    assert_eq!(update.user_entries[0].abs_idx, 0);
    assert_eq!(
        update.user_mappings.len(),
        1,
        "the freshly bound self-binding is what has to be introduced"
    );
    assert_eq!(update.user_mappings[0].abs_idx, 0);
    assert_eq!(update.user_mappings[0].target_id, user_id);
    assert_eq!(update.user_mappings[0].version, 3);
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

    // §8.4 forbids the dump, but §8.3 leaves inline introductions as BLE's
    // only mapping path, so the origin update still goes out.
    let msg = rx.try_recv().expect("BLE still gets an origin update");
    let (header, _) = Header::decode(&msg.bytes).expect("frame header");
    assert_eq!(
        header.message_type,
        RoutingMessage::RoutingUpdate,
        "BLE must not receive INDEX_DUMP"
    );
    assert!(rx.try_recv().is_err(), "nothing else goes over BLE");
}

#[test]
fn ble_coded_transport_skips_send() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();

    install_user(&state, [1; 8], 0);
    bind_own_dict(&state, Space::User, 7, [1; 8]);

    on_neighbour_connect(&state, peer, ConnectionModule::BleCoded);

    let msg = rx
        .try_recv()
        .expect("BLE-coded still gets an origin update");
    let (header, _) = Header::decode(&msg.bytes).expect("frame header");
    assert_eq!(
        header.message_type,
        RoutingMessage::RoutingUpdate,
        "BLE-coded must not receive INDEX_DUMP"
    );
    assert!(rx.try_recv().is_err(), "nothing else goes over BLE-coded");
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
