// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Route expiry and the index cooldown that follows it (spec §3.7, §7.5).

use crate::router_v2::*;
use crate::router_v2::{
    index::Space,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
};
use std::sync::Weak;

/// Installs a routing entry at `(space, idx)`, binds the dictionary,
/// and returns a Weak to the entry so tests can verify cycle
/// discipline after sweep.
fn install_entry(
    state: &RouterV2State,
    space: Space,
    idx: u16,
    target_id: [u8; 8],
    target: TargetRef,
    last_update: u64,
) -> Weak<RwLock<RoutingEntry>> {
    let arc = Arc::new(RwLock::new(RoutingEntry {
        target_index: idx,
        target,
        seq_num: SeqNum::from(0u16),
        metric: 0,
        next_hop: 0,
        transport: ConnectionModule::Lan,
        last_update,
        hop_count: 0,
        local_only: false,
    }));
    let weak = Arc::downgrade(&arc);
    state.routing_table.write().unwrap().set(space, idx, arc);
    bind_own_dict(state, space, idx, target_id);
    weak
}

fn expiry_ms(state: &RouterV2State) -> u64 {
    state.options.route_expiry_ms
}

#[test]
fn entry_past_threshold_is_removed() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) - 1;
    install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user),
        last_update,
    );

    state.sweep_expired(now);

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 5)
        .is_none());
}

#[test]
fn entry_within_threshold_is_kept() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) + 1;
    install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user),
        last_update,
    );

    state.sweep_expired(now);

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 5)
        .is_some());
}

/// At exactly `last_update + expiry == now`, the strict `<` comparison
/// keeps the entry. Pins the operator against an accidental `<=`.
#[test]
fn entry_at_exact_boundary_is_kept() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state);
    install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user),
        last_update,
    );

    state.sweep_expired(now);

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 5)
            .is_some(),
        "entry exactly at the threshold must survive (strict `<`)",
    );
}

#[test]
fn expired_entry_unbinds_the_dictionary() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) - 1;
    install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user),
        last_update,
    );

    assert_eq!(state.user_dict.read().unwrap().id_of(5), Some([1; 8]));

    state.sweep_expired(now);

    assert_eq!(state.user_dict.read().unwrap().id_of(5), None);
    assert_eq!(state.user_dict.read().unwrap().idx_of(&[1; 8]), None);
}

#[test]
fn expired_entry_pushes_idx_into_allocator_cooldown() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) - 1;
    install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user),
        last_update,
    );

    assert!(!state.users_allocator.read().unwrap().idx_in_cooldown(5));

    state.sweep_expired(now);

    assert!(
        state.users_allocator.read().unwrap().idx_in_cooldown(5),
        "released idx must enter cooldown so the allocator doesn't rebind it immediately",
    );
}

/// A mark that outlives its dictionary binding is an orphan: the next
/// `pending_introductions` pass finds no id for the index and drops it
/// with a warning. Retiring the index must retire the mark with it.
#[test]
fn expired_entry_clears_the_reintroduction_mark() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) - 1;
    install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user),
        last_update,
    );
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 5);

    state.sweep_expired(now);

    let pending = state
        .reintroduction_tracker
        .write()
        .unwrap()
        .take_pending(Space::User);
    assert!(
        !pending.contains(&5),
        "sweeping an index must clear its reintroduction mark, not leave an orphan",
    );
}

/// RESERVED_INDEX is bound directly rather than allocated (§3.2), so the
/// allocator has nothing to take back. Releasing it would put an index
/// the allocator never owned into cooldown.
#[test]
fn expired_reserved_index_is_not_returned_to_the_allocator() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) - 1;
    install_entry(
        &state,
        Space::User,
        index::RESERVED_INDEX,
        [1; 8],
        TargetRef::User(user),
        last_update,
    );

    state.sweep_expired(now);

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, index::RESERVED_INDEX)
            .is_none(),
        "the reserved slot still expires like any other",
    );
    assert!(
        !state
            .users_allocator
            .read()
            .unwrap()
            .idx_in_cooldown(index::RESERVED_INDEX),
        "RESERVED_INDEX must never enter allocator cooldown",
    );
}

/// The scan and the retirement are separate phases, so an entry
/// refreshed in between must survive. Simulated by re-installing the
/// entry with a current `last_update` at the same index.
#[test]
fn entry_refreshed_between_scan_and_retire_is_kept() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let stale = now - expiry_ms(&state) - 1;
    install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user.clone()),
        stale,
    );

    let expired = state.collect_expired(Space::User, now);
    assert_eq!(expired, vec![5], "scan must see the stale entry");

    // the refresh a concurrent commit would have performed
    install_entry(&state, Space::User, 5, [1; 8], TargetRef::User(user), now);
    state.retire_expired(Space::User, &expired, now);

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 5)
            .is_some(),
        "a route refreshed after the scan must not be retired",
    );
}

/// Cycle discipline (spec A.3): once the table drops its Arc, the
/// User's back-edge Weak must resolve to None.
#[test]
fn expired_entry_makes_user_weak_routing_entry_dangle() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) - 1;
    let weak = install_entry(
        &state,
        Space::User,
        5,
        [1; 8],
        TargetRef::User(user.clone()),
        last_update,
    );
    user.write().unwrap().routing_entry = Some(weak.clone());

    assert!(weak.upgrade().is_some(), "weak must upgrade before sweep");

    state.sweep_expired(now);

    assert!(
        weak.upgrade().is_none(),
        "weak must dangle after sweep drops the table's Arc",
    );
    assert!(user.read().unwrap().routing_entry.is_some());
}

#[test]
fn node_space_expiry_is_independent_from_user_space() {
    let (state, _rx) = fresh_state();
    let node = install_node(&state, [9; 8], 0, false);
    let now: u64 = 100_000;
    let last_update = now - expiry_ms(&state) - 1;
    install_entry(
        &state,
        Space::Node,
        7,
        [9; 8],
        TargetRef::Node(node),
        last_update,
    );

    let user = install_user(&state, [1; 8], 0);
    install_entry(&state, Space::User, 3, [1; 8], TargetRef::User(user), now);

    state.sweep_expired(now);

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::Node, 7)
        .is_none());
    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, 3)
        .is_some());
    assert!(state.node_allocator.read().unwrap().idx_in_cooldown(7));
    assert!(!state.users_allocator.read().unwrap().idx_in_cooldown(3));
}

#[test]
fn mixed_entries_only_expired_are_removed() {
    let (state, _rx) = fresh_state();
    let now: u64 = 100_000;

    let old_user = install_user(&state, [1; 8], 0);
    let fresh_user = install_user(&state, [2; 8], 0);

    install_entry(
        &state,
        Space::User,
        10,
        [1; 8],
        TargetRef::User(old_user),
        now - expiry_ms(&state) - 1,
    );
    install_entry(
        &state,
        Space::User,
        11,
        [2; 8],
        TargetRef::User(fresh_user),
        now,
    );

    state.sweep_expired(now);

    let rt = state.routing_table.read().unwrap();
    assert!(rt.get(Space::User, 10).is_none(), "stale entry removed");
    assert!(rt.get(Space::User, 11).is_some(), "fresh entry untouched");
}

#[test]
fn sweep_on_empty_state_is_a_noop() {
    let (state, _rx) = fresh_state();
    state.sweep_expired(0);
    state.sweep_expired(u64::MAX);
}
