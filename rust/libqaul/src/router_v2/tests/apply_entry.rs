// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Per-entry processing: metric accrual, hop count, relay inclusion (spec §7.2, §8.8 step 3).

use crate::router_v2::*;
use crate::router_v2::{
    codec::messages::{NodeEntry, UserEntry},
    index::Space,
    receive::ReceiveCtx,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
};
use libp2p::PeerId;

/// Build a ReceiveCtx with defaults every test uses (Lan transport,
/// no RSSI). Callers vary neighbour and `now`.
fn default_ctx(peer: PeerId, now: u64) -> ReceiveCtx {
    ReceiveCtx {
        neighbour: peer,
        transport: ConnectionModule::Lan,
        rssi_dbm: None,
        now,
    }
}

const NEIGHBOUR_NODE_ID: [u8; 8] = [77; 8];
const NEIGHBOUR_IDX_IN_NODE_DICT: u16 = 500;

/// Wires everything a user-space `apply_entry` call needs:
/// - a neighbour with a distinct node_id, added to mirrors
/// - that node_id bound in node_dict (so next_hop resolution succeeds)
/// - the incoming `abs_idx` bound in the neighbour's mirror to `target_id`
/// - `target_id` pre-bound in our own user_dict at `own_idx`
///   (so translate_incoming hits the existing-binding fast path)
/// - a stub User record for `target_id`
fn setup_user_target(
    state: &RouterV2State,
    abs_idx: u16,
    own_idx: u16,
    target_id: [u8; 8],
) -> (PeerId, Arc<RwLock<crate::router_v2::table::User>>) {
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
    bind_mirror(state, peer, Space::User, abs_idx, target_id);
    bind_own_dict(state, Space::User, own_idx, target_id);
    bind_own_dict(
        state,
        Space::Node,
        NEIGHBOUR_IDX_IN_NODE_DICT,
        NEIGHBOUR_NODE_ID,
    );
    let user = install_user(state, target_id, 0);
    (peer, user)
}

fn setup_node_target(
    state: &RouterV2State,
    abs_idx: u16,
    own_idx: u16,
    target_id: [u8; 8],
) -> (PeerId, Arc<RwLock<crate::router_v2::table::Node>>) {
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
    bind_mirror(state, peer, Space::Node, abs_idx, target_id);
    bind_own_dict(state, Space::Node, own_idx, target_id);
    bind_own_dict(
        state,
        Space::Node,
        NEIGHBOUR_IDX_IN_NODE_DICT,
        NEIGHBOUR_NODE_ID,
    );
    let node = install_node(state, target_id, 0, false);
    (peer, node)
}

/// Preload a routing-table slot with a stored entry for §7.2
/// comparison tests.
fn preload_entry(
    state: &RouterV2State,
    space: Space,
    own_idx: u16,
    target: TargetRef,
    seq: u16,
    metric: u16,
    local_only: bool,
) {
    let entry = Arc::new(RwLock::new(RoutingEntry {
        target_index: own_idx,
        target,
        seq_num: SeqNum::from(seq),
        metric,
        next_hop: 0,
        transport: ConnectionModule::Lan,
        last_update: 0,
        hop_count: 0,
        local_only,
    }));
    state
        .routing_table
        .write()
        .unwrap()
        .set(space, own_idx, entry);
}

fn wire_user_entry(
    abs_idx: u16,
    seq: u16,
    metric: u16,
    hop_count: u8,
    local_only: bool,
) -> UserEntry {
    UserEntry {
        abs_idx,
        seq,
        metric,
        hop_count,
        local_only,
    }
}

fn wire_node_entry(
    abs_idx: u16,
    seq: u16,
    metric: u16,
    hop_count: u8,
    local_only: bool,
    manifest_version: u32,
) -> NodeEntry {
    NodeEntry {
        abs_idx,
        seq,
        metric,
        hop_count,
        local_only,
        manifest_version,
    }
}

// ---------- TTL / drops ----------

#[test]
fn ttl_drop_when_incoming_hop_count_is_63() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, _) = setup_user_target(&state, 5, 42, target_id);

    state
        .apply_user_entry(
            &default_ctx(peer, 1_000),
            wire_user_entry(5, 1, 10, 63, false),
        )
        .unwrap();

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .is_none());
}

#[test]
fn hop_count_62_is_accepted_and_stored_as_63() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, _) = setup_user_target(&state, 5, 42, target_id);

    state
        .apply_user_entry(
            &default_ctx(peer, 1_000),
            wire_user_entry(5, 1, 10, 62, false),
        )
        .unwrap();

    let rt = state.routing_table.read().unwrap();
    let stored = rt.get(Space::User, 42).unwrap();
    assert_eq!(stored.read().unwrap().hop_count, 63);
}

#[test]
fn unknown_mapping_drops_silently() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
    // Neighbour exists but has no mirror binding at abs_idx 5.
    bind_own_dict(
        &state,
        Space::Node,
        NEIGHBOUR_IDX_IN_NODE_DICT,
        NEIGHBOUR_NODE_ID,
    );

    state
        .apply_user_entry(
            &default_ctx(peer, 1_000),
            wire_user_entry(5, 1, 10, 1, false),
        )
        .unwrap();

    assert!(state.users.read().unwrap().len() == 0);
    assert!(state
        .routing_table
        .read()
        .unwrap()
        .user_entries
        .iter()
        .all(|s| s.is_none()));
}

/// The mapping section is required to create a stub User before entries
/// reference the target. If it hasn't, the entry must be dropped rather
/// than trigger a fabricated record.
#[test]
fn missing_user_target_record_drops() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let target_id = [1; 8];
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
    bind_mirror(&state, peer, Space::User, 5, target_id);
    bind_own_dict(&state, Space::User, 42, target_id);
    bind_own_dict(
        &state,
        Space::Node,
        NEIGHBOUR_IDX_IN_NODE_DICT,
        NEIGHBOUR_NODE_ID,
    );
    // NOTE: no install_user — the record is missing.

    state
        .apply_user_entry(
            &default_ctx(peer, 1_000),
            wire_user_entry(5, 1, 10, 1, false),
        )
        .unwrap();

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .is_none());
}

#[test]
fn neighbour_node_id_not_in_node_dict_drops() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let target_id = [1; 8];
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
    bind_mirror(&state, peer, Space::User, 5, target_id);
    bind_own_dict(&state, Space::User, 42, target_id);
    install_user(&state, target_id, 0);
    // NOTE: no bind_own_dict for NEIGHBOUR_NODE_ID — step 7 must fail.

    state
        .apply_user_entry(
            &default_ctx(peer, 1_000),
            wire_user_entry(5, 1, 10, 1, false),
        )
        .unwrap();

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .is_none());
}

// ---------- empty-slot accept ----------

/// Full happy path for a user-space entry into an empty slot: verifies
/// every RoutingEntry field, that the User's Weak back-edge is
/// attached, and that metric composition + hop-count increment applied.
#[test]
fn empty_slot_accept_stores_entry_and_attaches_user_back_edge() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);

    state
        .apply_user_entry(
            &default_ctx(peer, 1_234),
            wire_user_entry(5, 7, 10, 2, false),
        )
        .unwrap();

    let rt = state.routing_table.read().unwrap();
    let stored_arc = rt.get(Space::User, 42).expect("slot must be occupied");
    let stored = stored_arc.read().unwrap();

    assert_eq!(stored.target_index, 42);
    assert_eq!(stored.seq_num, SeqNum::from(7u16));
    // Lan weight is 10, no BLE RSSI → penalty 0 → metric = 10 + 10 = 20.
    assert_eq!(stored.metric, 20);
    assert_eq!(stored.next_hop, NEIGHBOUR_IDX_IN_NODE_DICT);
    assert_eq!(stored.transport, ConnectionModule::Lan);
    assert_eq!(stored.last_update, 1_234);
    assert_eq!(stored.hop_count, 3);
    assert!(!stored.local_only);

    // Weak back-edge on the User points at the stored entry.
    let weak = user.read().unwrap().routing_entry.clone().unwrap();
    let upgraded = weak.upgrade().unwrap();
    assert!(Arc::ptr_eq(&upgraded, &stored_arc));
}

#[test]
fn empty_slot_accept_for_node_target_stores_entry() {
    let (state, _rx) = fresh_state();
    let target_id = [2; 8];
    let (peer, _) = setup_node_target(&state, 5, 42, target_id);

    state
        .apply_node_entry(
            &default_ctx(peer, 500),
            wire_node_entry(5, 1, 10, 0, false, 0),
        )
        .unwrap();

    let rt = state.routing_table.read().unwrap();
    let stored = rt.get(Space::Node, 42).expect("slot must be occupied");
    // Just confirm the Node case doesn't panic or fail — Node has no
    // routing_entry field so there's no back-edge to verify.
    assert_eq!(stored.read().unwrap().target_index, 42);
}

// ---------- §7.2 relay-inclusion ----------

#[test]
fn fresher_seq_replaces_stored_entry() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(
        &state,
        Space::User,
        42,
        TargetRef::User(user),
        10,
        50,
        false,
    );

    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 20, 10, 1, false),
        )
        .unwrap();

    let stored = state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .unwrap();
    let e = stored.read().unwrap();
    assert_eq!(e.seq_num, SeqNum::from(20u16));
    assert_eq!(e.metric, 20); // 10 + hop_cost(Lan, None) = 10 + 10
}

/// Reboot: a huge forward gap under wrap arithmetic must still be
/// accepted per spec §6.3 / §7.2.
#[test]
fn reboot_gap_accepts_new_entry() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(
        &state,
        Space::User,
        42,
        TargetRef::User(user),
        200,
        50,
        false,
    );

    // Incoming seq=30, stored=200: forward gap under wrap is 65_366, > 100 → Reboot.
    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 30, 10, 1, false),
        )
        .unwrap();

    let stored = state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .unwrap();
    assert_eq!(stored.read().unwrap().seq_num, SeqNum::from(30u16));
}

/// Per §6.3, any gap > 100 (including large backward-looking jumps
/// under wrap arithmetic) is treated as a reboot and accepted. There
/// is no "older, drop me" bucket — a peer with a lower seq than we
/// have stored is presumed to have restarted with a fresh random seed.
/// Pins this behaviour so a future refactor of `acceptance` doesn't
/// silently drift into "naive greater-than" semantics.
#[test]
fn backward_looking_seq_is_treated_as_reboot_and_accepted() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(
        &state,
        Space::User,
        42,
        TargetRef::User(user),
        50,
        30,
        false,
    );

    // seq=40, stored=50: forward gap under wrap = 65_526 → Reboot bucket.
    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 40, 10, 1, false),
        )
        .unwrap();

    let stored = state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .unwrap();
    let e = stored.read().unwrap();
    assert_eq!(e.seq_num, SeqNum::from(40u16), "reboot bucket must replace");
    assert_eq!(e.metric, 20, "new metric = 10 + hop_cost(Lan, None)=10");
}

#[test]
fn same_seq_lower_metric_replaces_stored() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(
        &state,
        Space::User,
        42,
        TargetRef::User(user),
        10,
        50,
        false,
    );

    // Same seq, incoming metric 5 + hop_cost(Lan, None)=10 = 15 < stored 50.
    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 10, 5, 1, false),
        )
        .unwrap();

    let stored = state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .unwrap();
    assert_eq!(stored.read().unwrap().metric, 15);
}

/// §7.2 requires strict `<` on the metric tiebreak — equal metric must
/// not overwrite the incumbent (flapping protection).
#[test]
fn same_seq_equal_metric_drops() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(
        &state,
        Space::User,
        42,
        TargetRef::User(user),
        10,
        20,
        false,
    );

    // Same seq, new metric 10 + 10 = 20 = stored → strict < fails → drop.
    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 10, 10, 5, false),
        )
        .unwrap();

    let stored = state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 42)
        .unwrap();
    let e = stored.read().unwrap();
    assert_eq!(e.metric, 20);
    assert_eq!(e.hop_count, 0, "stored hop_count preserved (drop path)");
}

#[test]
fn same_seq_higher_metric_drops() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(
        &state,
        Space::User,
        42,
        TargetRef::User(user),
        10,
        20,
        false,
    );

    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 10, 30, 1, false),
        )
        .unwrap();

    assert_eq!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap()
            .read()
            .unwrap()
            .metric,
        20,
    );
}

// ---------- local_only monotonicity (§7.4) ----------

/// Sticky at zero: stored=false remains false even when incoming=true.
#[test]
fn local_only_sticky_when_stored_is_false() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(
        &state,
        Space::User,
        42,
        TargetRef::User(user),
        10,
        50,
        false,
    );

    // Fresher seq → accepted; local_only should stay false.
    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 20, 10, 1, true),
        )
        .unwrap();

    assert!(
        !state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap()
            .read()
            .unwrap()
            .local_only
    );
}

/// Transitions to false: stored=true is overridden by incoming=false.
#[test]
fn local_only_transitions_to_false_when_incoming_is_false() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, user) = setup_user_target(&state, 5, 42, target_id);
    preload_entry(&state, Space::User, 42, TargetRef::User(user), 10, 50, true);

    state
        .apply_user_entry(
            &default_ctx(peer, 2_000),
            wire_user_entry(5, 20, 10, 1, false),
        )
        .unwrap();

    assert!(
        !state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap()
            .read()
            .unwrap()
            .local_only
    );
}

#[test]
fn local_only_empty_slot_uses_incoming_value() {
    let (state, _rx) = fresh_state();
    let target_id = [1; 8];
    let (peer, _) = setup_user_target(&state, 5, 42, target_id);

    state
        .apply_user_entry(
            &default_ctx(peer, 1_000),
            wire_user_entry(5, 1, 10, 1, true),
        )
        .unwrap();

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap()
            .read()
            .unwrap()
            .local_only
    );
}
