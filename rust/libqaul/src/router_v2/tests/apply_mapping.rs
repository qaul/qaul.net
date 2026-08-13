// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Applying inline mappings, including rebind cleanup and the §10.8 pull trigger.

use crate::router_v2::*;
use crate::router_v2::{
    codec::messages::Mapping,
    index::Space,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
};

#[test]
fn apply_mapping_unknown_neighbour_is_noop() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    let result = state.apply_mapping(
        peer,
        Space::User,
        Mapping {
            abs_idx: 5,
            target_id: [1; 8],
            version: 42,
        },
        1_000,
    );

    assert!(result.is_ok());
    assert_eq!(state.users.read().unwrap().len(), 0);
    assert!(state.mirrors.read().unwrap().is_empty());
}

#[test]
fn apply_mapping_fresh_user_creates_stub_and_binds_mirror() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: [1; 8],
                version: 42,
            },
            1_000,
        )
        .unwrap();

    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some([1; 8]));
    drop(mirrors);

    let users = state.users.read().unwrap();
    let user_arc = users.get(&[1; 8]).unwrap();
    let user = user_arc.read().unwrap();
    assert_eq!(user.id, [1; 8]);
    assert_eq!(user.profile_version, 42);
    assert!(user.public_key.is_none(), "stub must not fabricate a key");
}

#[test]
fn apply_mapping_fresh_node_creates_stub_and_binds_mirror() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    state
        .apply_mapping(
            peer,
            Space::Node,
            Mapping {
                abs_idx: 5,
                target_id: [2; 8],
                version: 99,
            },
            1_000,
        )
        .unwrap();

    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(mirrors.get(&peer).unwrap().nodes.id_of(5), Some([2; 8]));
    drop(mirrors);

    let nodes = state.nodes.read().unwrap();
    let node = nodes.get(&[2; 8]).unwrap();
    let n = node.read().unwrap();
    // Under the pull-based model (§10.8), a node mapping carries an
    // *advertisement* of the origin's manifest_version, not the
    // committed value. Stub nodes have manifest_version=0 (no
    // committed manifest yet); the mapping's version writes to
    // advertised_version and later drives the pull trigger.
    assert_eq!(n.manifest_version, 0, "stub node has no committed manifest");
    assert_eq!(
        n.advertised_version, 99,
        "mapping's version → advertised_version"
    );
    assert!(!n.is_gateway, "stub node is not a gateway by default");
    assert!(n.public_key.is_none());
}

#[test]
fn apply_mapping_same_id_updates_version_only() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let id = [3; 8];

    bind_mirror(&state, peer, Space::User, 5, id);
    install_user(&state, id, 10);

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: id,
                version: 20,
            },
            1_000,
        )
        .unwrap();

    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(id));
    drop(mirrors);

    let users = state.users.read().unwrap();
    assert_eq!(users.get(&id).unwrap().read().unwrap().profile_version, 20);
}

/// The critical §8.7-step-2 case: mirror already has abs_idx bound to
/// OLD; applying NEW must clear old routing entry, release own_idx to
/// cooldown, unbind own dict, then bind new mapping.
#[test]
fn apply_mapping_rebind_clears_old_routing_state() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    let old_id = [10; 8];
    let new_id = [20; 8];
    let own_idx: u16 = 7;

    bind_mirror(&state, peer, Space::User, 5, old_id);
    let old_user = install_user(&state, old_id, 1);
    state.user_dict.write().unwrap().bind(own_idx, old_id);

    let entry = Arc::new(RwLock::new(RoutingEntry {
        target_index: own_idx,
        target: TargetRef::User(old_user.clone()),
        seq_num: SeqNum::from(0u16),
        metric: 5,
        next_hop: 0,
        transport: ConnectionModule::Lan,
        last_update: 0,
        hop_count: 1,
        local_only: false,
    }));
    let entry_weak = Arc::downgrade(&entry);
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::User, own_idx, entry);
    old_user.write().unwrap().routing_entry = Some(entry_weak.clone());

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: new_id,
                version: 1,
            },
            1_000,
        )
        .unwrap();

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, own_idx)
        .is_none());
    assert!(
        entry_weak.upgrade().is_none(),
        "old routing entry Arc must be dropped"
    );

    assert_eq!(state.user_dict.read().unwrap().idx_of(&old_id), None);
    assert_eq!(state.user_dict.read().unwrap().id_of(own_idx), None);

    assert!(state
        .users_allocator
        .read()
        .unwrap()
        .idx_in_cooldown(own_idx));

    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(new_id));
    drop(mirrors);

    assert!(state.users.read().unwrap().get(&new_id).is_some());
}

#[test]
fn apply_mapping_incoming_version_equal_is_noop() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let id = [4; 8];
    install_user(&state, id, 42);

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: id,
                version: 42,
            },
            1_000,
        )
        .unwrap();

    assert_eq!(
        state
            .users
            .read()
            .unwrap()
            .get(&id)
            .unwrap()
            .read()
            .unwrap()
            .profile_version,
        42,
    );
}

#[test]
fn apply_mapping_incoming_version_older_preserves_stored() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let id = [5; 8];
    install_user(&state, id, 100);

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: id,
                version: 50,
            },
            1_000,
        )
        .unwrap();

    assert_eq!(
        state
            .users
            .read()
            .unwrap()
            .get(&id)
            .unwrap()
            .read()
            .unwrap()
            .profile_version,
        100,
        "stale-echo path must NOT overwrite the fresher stored version",
    );
}

/// A fresher advertised version updates Node.advertised_version
/// (the hint), not the committed manifest_version. The committed
/// value only advances when a verified manifest lands (§10.8).
#[test]
fn apply_mapping_incoming_version_fresher_updates_advertised_only() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let id = [6; 8];
    // install_node sets manifest_version=5 (committed); advertised_version=0.
    install_node(&state, id, 5, false);

    state
        .apply_mapping(
            peer,
            Space::Node,
            Mapping {
                abs_idx: 5,
                target_id: id,
                version: 12,
            },
            1_000,
        )
        .unwrap();

    let nodes = state.nodes.read().unwrap();
    let node = nodes.get(&id).unwrap();
    let n = node.read().unwrap();
    // Committed value stays at 5 — we haven't verified a manifest at 12.
    assert_eq!(
        n.manifest_version, 5,
        "committed manifest_version must not change from a mapping"
    );
    // The hint updates so the pull trigger can compare.
    assert_eq!(
        n.advertised_version, 12,
        "advertised_version records the incoming hint"
    );
}

#[test]
fn apply_mapping_user_and_node_spaces_are_independent() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    let user_id = [11; 8];
    let node_id = [22; 8];

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: user_id,
                version: 1,
            },
            1_000,
        )
        .unwrap();
    state
        .apply_mapping(
            peer,
            Space::Node,
            Mapping {
                abs_idx: 5,
                target_id: node_id,
                version: 1,
            },
            1_000,
        )
        .unwrap();

    let mirrors = state.mirrors.read().unwrap();
    let nm = mirrors.get(&peer).unwrap();
    assert_eq!(nm.users.id_of(5), Some(user_id));
    assert_eq!(nm.nodes.id_of(5), Some(node_id));
    drop(mirrors);

    assert!(state.users.read().unwrap().get(&user_id).is_some());
    assert!(state.users.read().unwrap().get(&node_id).is_none());
    assert!(state.nodes.read().unwrap().get(&node_id).is_some());
    assert!(state.nodes.read().unwrap().get(&user_id).is_none());
}
