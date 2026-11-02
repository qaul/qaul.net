// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Relay-inclusion decisions and split horizon (spec §7.2, §7.3).

use crate::router_v2::*;
use crate::router_v2::{
    codec::{messages::RoutingUpdate, Header, RoutingMessage},
    index::Space,
    propagation::tick_relay,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
};

fn decode_frame(bytes: &[u8]) -> RoutingUpdate {
    let (header, body_slice) = Header::decode(bytes).expect("frame header");
    assert_eq!(header.message_type, RoutingMessage::RoutingUpdate);
    let payload = &body_slice[..header.payload_len as usize];
    RoutingUpdate::decode(payload).expect("routing-update body")
}

/// Installs a routing entry at `(space, own_idx)`, binds the own dict
/// for the target, and pushes into the relay queue.
fn queue_entry(
    state: &RouterV2State,
    space: Space,
    own_idx: u16,
    target: TargetRef,
    target_id: [u8; 8],
    next_hop_idx: u16,
    transport: ConnectionModule,
    seq: u16,
    metric: u16,
    local_only: bool,
) {
    bind_own_dict(state, space, own_idx, target_id);
    let arc = Arc::new(RwLock::new(RoutingEntry {
        target_index: own_idx,
        target,
        seq_num: SeqNum::from(seq),
        metric,
        next_hop: next_hop_idx,
        transport,
        last_update: 1_000,
        hop_count: 2,
        local_only,
    }));
    state
        .routing_table
        .write()
        .unwrap()
        .set(space, own_idx, arc);
    state.relay_queue.write().unwrap().insert((space, own_idx));
}

// ---------- empty cases ----------

#[test]
fn empty_queue_pushes_nothing() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

    tick_relay(&state, 5_000);

    assert!(rx.try_recv().is_err());
}

#[test]
fn no_neighbours_pushes_nothing() {
    let (state, mut rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        500,
        ConnectionModule::Lan,
        1,
        10,
        false,
    );

    tick_relay(&state, 5_000);

    assert!(rx.try_recv().is_err());
}

// ---------- happy path ----------

/// One queued entry, one neighbour → one outbound with the correct
/// wire fields.
#[test]
fn queued_entry_routed_to_neighbour() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

    // Entry's next_hop resolves to a phantom neighbour (not the
    // outgoing one), so split-horizon allows.
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        501,
        ConnectionModule::Lan,
        3,
        20,
        false,
    );

    tick_relay(&state, 5_000);

    let msg = rx.try_recv().expect("one outbound");
    assert_eq!(msg.peer, peer);
    assert_eq!(msg.transport, ConnectionModule::Lan);

    let update = decode_frame(&msg.bytes);
    assert_eq!(update.user_entries.len(), 1);
    assert!(update.node_entries.is_empty());
    let wire = &update.user_entries[0];
    assert_eq!(wire.abs_idx, 5);
    assert_eq!(wire.seq, 3);
    assert_eq!(wire.metric, 20);
    assert_eq!(wire.hop_count, 2);
}

// ---------- split horizon ----------

/// Split-horizon: the neighbour whose id equals the entry's resolved
/// next_hop must NOT receive this entry. A second neighbour still does.
#[test]
fn split_horizon_blocks_return_to_source_neighbour() {
    let (state, mut rx) = fresh_state();

    let peer_source = fresh_peer();
    let peer_other = fresh_peer();
    state.add_neighbour_transport(peer_source, [77; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(peer_other, [88; 8], ConnectionModule::Lan);

    // next_hop resolves to [77;8] — the source neighbour.
    bind_own_dict(&state, Space::Node, 500, [77; 8]);

    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        500,
        ConnectionModule::Lan,
        1,
        10,
        false,
    );

    tick_relay(&state, 5_000);

    // Only peer_other should receive; peer_source is split-horizon blocked.
    let mut recipients = Vec::new();
    while let Ok(msg) = rx.try_recv() {
        recipients.push(msg.peer);
    }
    assert_eq!(recipients, vec![peer_other]);
}

// ---------- sphere filter ----------

/// A user-target entry must not cross the Internet membrane (§2.3).
#[test]
fn sphere_filter_drops_user_target_on_internet_outgoing() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

    // next_hop points at a different node so split-horizon allows.
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        501,
        ConnectionModule::Lan,
        1,
        10,
        false,
    );

    tick_relay(&state, 5_000);

    // No outbound: the only survived entry would be user-space, which
    // gets sphere-filtered before send, and no intros exist to save
    // the batch. Empty-batch shortcut kicks in.
    assert!(rx.try_recv().is_err());
}

/// A gateway-node entry DOES cross the Internet membrane.
#[test]
fn sphere_filter_allows_gateway_node_on_internet_outgoing() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    let gateway = install_node(&state, [9; 8], 0, true); // is_gateway = true
    queue_entry(
        &state,
        Space::Node,
        5,
        TargetRef::Node(gateway),
        [9; 8],
        501,
        ConnectionModule::Lan,
        1,
        10,
        false,
    );

    tick_relay(&state, 5_000);

    let msg = rx.try_recv().expect("gateway entry must propagate");
    let update = decode_frame(&msg.bytes);
    assert_eq!(update.node_entries.len(), 1);
}

// ---------- local_only wire rewrite ----------

/// Stored `local_only = true` → Internet-outgoing wire flag becomes
/// `false` (§7.4 sender rule). Uses a gateway node so the entry
/// survives the sphere filter.
#[test]
fn local_only_stripped_for_internet_outgoing() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    let gateway = install_node(&state, [9; 8], 0, true);
    queue_entry(
        &state,
        Space::Node,
        5,
        TargetRef::Node(gateway),
        [9; 8],
        501,
        ConnectionModule::Lan,
        1,
        10,
        true, // stored local_only
    );

    tick_relay(&state, 5_000);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    assert!(
        !update.node_entries[0].local_only,
        "Internet-outgoing must strip local_only",
    );
}

/// Stored `local_only = true` → Local-outgoing wire flag equals the
/// stored value (pass through).
#[test]
fn local_only_preserved_for_local_outgoing() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        501,
        ConnectionModule::Lan,
        1,
        10,
        true,
    );

    tick_relay(&state, 5_000);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    assert!(update.user_entries[0].local_only);
}

// ---------- delta-encoding invariant ----------

/// Wire entries must be sorted by abs_idx per space. HashSet iteration
/// is non-deterministic, so this pins the sort.
#[test]
fn wire_entries_sorted_by_abs_idx() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    // Queue three entries at unsorted indices.
    for (i, own_idx) in [50u16, 10, 200].iter().enumerate() {
        let user = install_user(&state, [i as u8 + 1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            *own_idx,
            TargetRef::User(user),
            [i as u8 + 1; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            false,
        );
    }

    tick_relay(&state, 5_000);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    let idxs: Vec<u16> = update.user_entries.iter().map(|e| e.abs_idx).collect();
    assert_eq!(idxs, vec![10, 50, 200]);
}

// ---------- introductions ----------

/// Pending introductions must be attached to the outbound message
/// alongside any surviving entries.
#[test]
fn pending_introductions_attached_to_message() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    // Set up an introduction.
    let intro_id = [11; 8];
    install_user(&state, intro_id, 3);
    state.user_dict.write().unwrap().bind(7, intro_id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 7);

    // Also queue a routing entry so the batch isn't empty on the
    // entry side (empty-batch shortcut wouldn't fire since intros
    // exist, but this exercises the mixed case).
    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        501,
        ConnectionModule::Lan,
        1,
        10,
        false,
    );

    tick_relay(&state, 5_000);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    assert_eq!(update.user_mappings.len(), 1);
    assert_eq!(update.user_mappings[0].abs_idx, 7);
    assert_eq!(update.user_mappings[0].target_id, intro_id);
    assert_eq!(update.user_mappings[0].version, 3);
}

/// Introductions alone are enough to send — even with no entries.
#[test]
fn introductions_alone_produce_outbound() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

    let intro_id = [11; 8];
    install_user(&state, intro_id, 3);
    state.user_dict.write().unwrap().bind(7, intro_id);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 7);

    tick_relay(&state, 5_000);

    let msg = rx.try_recv().expect("one outbound");
    let update = decode_frame(&msg.bytes);
    assert_eq!(update.user_mappings.len(), 1);
    assert!(update.user_entries.is_empty());
}

// ---------- empty-batch shortcut ----------

/// When every queued entry is filtered out AND no introductions
/// exist, the tick must not emit anything for that neighbour.
#[test]
fn empty_batch_shortcut_suppresses_purely_empty_message() {
    let (state, mut rx) = fresh_state();

    // One neighbour, entry destined for split-horizon block.
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    bind_own_dict(&state, Space::Node, 500, [77; 8]);

    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        500, // → [77;8], split-horizon blocks
        ConnectionModule::Lan,
        1,
        10,
        false,
    );

    tick_relay(&state, 5_000);

    assert!(
        rx.try_recv().is_err(),
        "empty batch shortcut must suppress the send",
    );
}

// ---------- queue drain ----------

#[test]
fn relay_queue_drained_after_tick() {
    let (state, mut _rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    let user = install_user(&state, [1; 8], 0);
    queue_entry(
        &state,
        Space::User,
        5,
        TargetRef::User(user),
        [1; 8],
        501,
        ConnectionModule::Lan,
        1,
        10,
        false,
    );

    assert_eq!(state.relay_queue.read().unwrap().len(), 1);

    tick_relay(&state, 5_000);

    assert!(
        state.relay_queue.read().unwrap().is_empty(),
        "queue must be empty after tick",
    );
}

// ---------- multi-transport neighbour ----------

/// A neighbour on both LAN and Internet receives two messages, one
/// per transport, each with the correct local_only rewrite for its
/// outgoing sphere. Uses a gateway node so the entry survives on both.
#[test]
fn multi_transport_neighbour_gets_one_message_per_transport() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

    // next_hop points at a different node.
    bind_own_dict(&state, Space::Node, 501, [88; 8]);

    let gateway = install_node(&state, [9; 8], 0, true);
    queue_entry(
        &state,
        Space::Node,
        5,
        TargetRef::Node(gateway),
        [9; 8],
        501,
        ConnectionModule::Lan,
        1,
        10,
        true, // stored local_only
    );

    tick_relay(&state, 5_000);

    let mut by_transport: HashMap<ConnectionModule, bool> = HashMap::new();
    while let Ok(msg) = rx.try_recv() {
        let update = decode_frame(&msg.bytes);
        let wire_local_only = update.node_entries[0].local_only;
        by_transport.insert(msg.transport, wire_local_only);
    }
    assert_eq!(by_transport.len(), 2, "one message per transport");
    assert_eq!(
        by_transport[&ConnectionModule::Lan],
        true,
        "Local passes stored=true through"
    );
    assert_eq!(
        by_transport[&ConnectionModule::Internet],
        false,
        "Internet strips local_only regardless of stored",
    );
}
