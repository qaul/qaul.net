// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Registering hosted users and neighbour nodes into the index spaces (spec §3.5).

use crate::router_v2::*;
use crate::router_v2::{index::Space, index::RESERVED_INDEX, test_utils::*, PropagationForm};

/// Drains the pending introduction marks for `space`.
fn take_marks(state: &RouterV2State, space: Space) -> std::collections::HashSet<u16> {
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .take_pending(space)
}

// ----- register_hosted_user -----

/// All three effects must land together. `pending_introductions` resolves a
/// mark through both the dictionary and the users map and silently discards
/// it if either is missing, so a partial write binds the index but never
/// introduces it — leaving peers unable to translate index 0.
#[test]
fn register_hosted_user_binds_record_and_marks() {
    let (state, _rx) = fresh_state();
    let user_id = [42; 8];

    state.register_hosted_user(user_id, 7, fresh_multikey());

    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some(user_id),
        "hosted user must occupy RESERVED_INDEX in the user space"
    );

    let users = state.users.read().unwrap();
    let user_arc = users.get(&user_id).expect("User record created");
    assert_eq!(user_arc.read().unwrap().profile_version, 7);
    drop(users);

    assert!(
        take_marks(&state, Space::User).contains(&RESERVED_INDEX),
        "the binding must be queued for introduction"
    );
}

/// The whole point of the binding: `pending_introductions` must resolve it
/// into a real mapping. This is the assertion that would have caught the
/// original bug, where index 0 was never introduced and every peer dropped
/// the origin's user entry.
#[test]
fn register_hosted_user_produces_a_resolvable_introduction() {
    let (state, _rx) = fresh_state();
    let user_id = [42; 8];

    state.register_hosted_user(user_id, 7, fresh_multikey());

    let intros = state.pending_introductions(Space::User);
    assert_eq!(
        intros,
        vec![(RESERVED_INDEX, user_id, 7)],
        "introduction must carry index, id and profile_version"
    );
}

/// Called on every startup, so repeats must not churn state.
#[test]
fn register_hosted_user_is_idempotent() {
    let (state, _rx) = fresh_state();
    let user_id = [42; 8];

    state.register_hosted_user(user_id, 1, fresh_multikey());
    state.register_hosted_user(user_id, 1, fresh_multikey());

    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some(user_id)
    );
    assert_eq!(state.users.read().unwrap().len(), 1, "no duplicate record");
}

/// A later call with a fresher profile_version updates the existing record
/// rather than being ignored or inserting a second one.
#[test]
fn register_hosted_user_updates_profile_version_on_existing_record() {
    let (state, _rx) = fresh_state();
    let user_id = [42; 8];

    state.register_hosted_user(user_id, 1, fresh_multikey());
    state.register_hosted_user(user_id, 9, fresh_multikey());

    let users = state.users.read().unwrap();
    let user_arc = users.get(&user_id).unwrap();
    assert_eq!(user_arc.read().unwrap().profile_version, 9);
    assert_eq!(users.len(), 1);
}

/// §3.2 reading (a): a second hosted user puts the node in *node* form,
/// where users are named by the manifest rather than by routing entries —
/// so the newcomer gets no user index at all, and the first keeps the
/// reserved slot until the form transition releases it.
///
/// The eviction this guards against is real: `IndexDictionary::bind`
/// replaces whatever occupies an index, so binding every hosted user at
/// RESERVED_INDEX would silently repoint peers' index 0 at a different user.
#[test]
fn second_hosted_user_does_not_evict_the_first() {
    let (state, _rx) = fresh_state();
    let first = [1; 8];
    let second = [2; 8];

    state.register_hosted_user(first, 1, fresh_multikey());
    state.register_hosted_user(second, 1, fresh_multikey());

    let dict = state.user_dict.read().unwrap();
    assert_eq!(
        dict.id_of(RESERVED_INDEX),
        Some(first),
        "the first hosted user keeps the reserved slot"
    );
    assert!(
        dict.idx_of(&second).is_none(),
        "a node-form host assigns its users no user index"
    );
    drop(dict);

    assert_eq!(state.hosted_user_ids().len(), 2);
    assert_eq!(
        state.desired_propagation_form(),
        PropagationForm::Node,
        "two hosted users is the §3.2 node-form trigger"
    );
}

/// Only an indexed user can be introduced. The second has no index, so the
/// user space has exactly one introduction pending — and it is the first
/// user at the reserved slot.
#[test]
fn only_the_indexed_hosted_user_is_introduced() {
    let (state, _rx) = fresh_state();
    let first = [1; 8];
    let second = [2; 8];

    state.register_hosted_user(first, 3, fresh_multikey());
    state.register_hosted_user(second, 4, fresh_multikey());

    assert_eq!(
        state.pending_introductions(Space::User),
        vec![(RESERVED_INDEX, first, 3)]
    );
}

/// Re-registering must not disturb the reserved slot or duplicate records,
/// whichever form the node is in.
#[test]
fn repeat_registration_is_stable_across_the_form_boundary() {
    let (state, _rx) = fresh_state();
    let first = [1; 8];
    let second = [2; 8];

    state.register_hosted_user(first, 1, fresh_multikey());
    state.register_hosted_user(second, 1, fresh_multikey());

    state.register_hosted_user(first, 2, fresh_multikey());
    state.register_hosted_user(second, 2, fresh_multikey());

    let dict = state.user_dict.read().unwrap();
    assert_eq!(dict.id_of(RESERVED_INDEX), Some(first));
    assert!(dict.idx_of(&second).is_none());
    drop(dict);
    assert_eq!(state.users.read().unwrap().len(), 2);
    assert_eq!(state.hosted_user_ids().len(), 2);
}

// ----- register_neighbour_node -----

/// A neighbour must get an own-side node index: a routing entry's next_hop
/// is a node index, and `translate_incoming` only allocates for targets
/// named by incoming *entries*. Without this a neighbour that is only ever
/// a next hop is never allocated one and every entry it sends is rejected.
#[test]
fn register_neighbour_node_allocates_index_record_and_mark() {
    let (state, _rx) = fresh_state();
    let node_id = [77; 8];
    let key = fresh_multikey();

    state.register_neighbour_node(node_id, Some(key));

    let idx = state
        .node_dict
        .read()
        .unwrap()
        .idx_of(&node_id)
        .expect("neighbour allocated a node index");
    assert_ne!(
        idx, RESERVED_INDEX,
        "the allocator must never hand out the reserved self index"
    );

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&node_id).expect("Node record created");
    assert!(
        node_arc.read().unwrap().public_key.is_some(),
        "the key from try_from_peer_id must be retained for §8.8 verification"
    );
    drop(nodes);

    assert!(
        take_marks(&state, Space::Node).contains(&idx),
        "binding must be queued for introduction"
    );
}

/// `ping_event` gates on `add_neighbour_transport`, but registration must be
/// safe under repeats regardless — and must not consume a fresh index each
/// time, which would exhaust the allocator on a flapping link.
#[test]
fn register_neighbour_node_is_idempotent() {
    let (state, _rx) = fresh_state();
    let node_id = [77; 8];

    state.register_neighbour_node(node_id, Some(fresh_multikey()));
    let first = state.node_dict.read().unwrap().idx_of(&node_id).unwrap();

    for _ in 0..5 {
        state.register_neighbour_node(node_id, Some(fresh_multikey()));
    }

    assert_eq!(
        state.node_dict.read().unwrap().idx_of(&node_id),
        Some(first),
        "repeat registration must not reallocate"
    );
    assert_eq!(state.nodes.read().unwrap().len(), 1, "no duplicate record");
}

/// Stubs built by `apply_mapping` carry `public_key: None`. Registering with
/// a key must upgrade such a stub in place rather than leaving it keyless —
/// §8.8 cannot verify a manifest without it.
#[test]
fn register_neighbour_node_upgrades_a_keyless_stub() {
    let (state, _rx) = fresh_state();
    let node_id = [77; 8];

    state.register_neighbour_node(node_id, None);
    {
        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&node_id).unwrap();
        assert!(node_arc.read().unwrap().public_key.is_none());
    }

    state.register_neighbour_node(node_id, Some(fresh_multikey()));

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&node_id).unwrap();
    assert!(
        node_arc.read().unwrap().public_key.is_some(),
        "a later call carrying a key must fill in the stub"
    );
}

/// The inverse: a call without a key must never clear one we already hold.
#[test]
fn register_neighbour_node_never_downgrades_a_known_key() {
    let (state, _rx) = fresh_state();
    let node_id = [77; 8];

    state.register_neighbour_node(node_id, Some(fresh_multikey()));
    state.register_neighbour_node(node_id, None);

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&node_id).unwrap();
    assert!(
        node_arc.read().unwrap().public_key.is_some(),
        "None must not overwrite a known key"
    );
}

/// Distinct neighbours get distinct indexes, and neither collides with the
/// host's own node binding at RESERVED_INDEX.
#[test]
fn distinct_neighbours_get_distinct_indexes() {
    let (state, _rx) = fresh_state();

    state.register_neighbour_node([10; 8], None);
    state.register_neighbour_node([20; 8], None);

    let dict = state.node_dict.read().unwrap();
    let a = dict.idx_of(&[10; 8]).unwrap();
    let b = dict.idx_of(&[20; 8]).unwrap();
    assert_ne!(a, b);
    assert_ne!(a, RESERVED_INDEX);
    assert_ne!(b, RESERVED_INDEX);
}

/// §3.5 ties each reserved index to a propagation form. In user form the
/// hosted user holds user-space 0x0000 and node-space 0x0000 is *unbound* —
/// the node self-binding only exists while propagating as a node entry.
#[test]
fn only_the_active_form_holds_a_reserved_binding() {
    let (state, _rx) = fresh_state();
    let user_id = [42; 8];

    state.register_hosted_user(user_id, 1, fresh_multikey());
    state.register_neighbour_node([77; 8], None);

    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some(user_id)
    );
    assert!(
        state
            .node_dict
            .read()
            .unwrap()
            .id_of(RESERVED_INDEX)
            .is_none(),
        "node-space reserved index stays unbound while in user form"
    );
}

/// Neighbours must never be handed the reserved slot, whichever form the
/// node is in.
#[test]
fn neighbour_registration_never_touches_the_reserved_slot() {
    let (state, _rx) = fresh_state();

    state.register_neighbour_node([77; 8], None);
    state.register_neighbour_node([88; 8], None);

    let dict = state.node_dict.read().unwrap();
    assert!(dict.id_of(RESERVED_INDEX).is_none());
    assert_ne!(dict.idx_of(&[77; 8]).unwrap(), RESERVED_INDEX);
    assert_ne!(dict.idx_of(&[88; 8]).unwrap(), RESERVED_INDEX);
}
