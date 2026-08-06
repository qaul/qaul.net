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
