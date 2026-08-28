// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! User-form versus node-form selection and the transitions between them (spec §3.2).

use crate::router_v2::*;
use crate::router_v2::{index::Space, index::RESERVED_INDEX, test_utils::*, PropagationForm};

/// One hosted user, LAN-only: the §3.2 default.
#[test]
fn single_user_no_internet_is_user_form() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Lan);

    assert_eq!(state.desired_propagation_form(), PropagationForm::User);
}

/// Remote users must not count toward the multi-user trigger. Every
/// neighbour's user gets a stub in `users`, so counting the map instead of
/// the `is_hosted` flag would flip a plain two-node LAN into node form.
#[test]
fn remote_users_do_not_trigger_node_form() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    install_user(&state, [2; 8], 0); // learned from a neighbour
    install_user(&state, [3; 8], 0);

    assert_eq!(state.users.read().unwrap().len(), 3);
    assert_eq!(state.hosted_user_ids().len(), 1);
    assert_eq!(state.desired_propagation_form(), PropagationForm::User);
}

/// A user can be seen through a neighbour before the local account loads,
/// leaving a stub with `is_hosted: false`. Registering it must upgrade the
/// existing record rather than only setting the flag on a fresh insert.
#[test]
fn registering_an_existing_remote_stub_marks_it_hosted() {
    let (state, _rx) = fresh_state();
    install_user(&state, [1; 8], 5); // remote stub

    state.register_hosted_user([1; 8], 6, fresh_multikey());

    assert_eq!(state.hosted_user_ids(), vec![[1; 8]]);
    assert_eq!(state.users.read().unwrap().len(), 1, "no duplicate record");
}

#[test]
fn second_hosted_user_triggers_node_form() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    assert_eq!(state.desired_propagation_form(), PropagationForm::User);

    state.register_hosted_user([2; 8], 0, fresh_multikey());
    assert_eq!(state.desired_propagation_form(), PropagationForm::Node);
}

/// Spec line 206 keys this on an active INTERNET *connection*, which is
/// what a neighbour entry represents — not on a bound listener.
#[test]
fn internet_neighbour_triggers_node_form() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);

    assert_eq!(state.desired_propagation_form(), PropagationForm::Node);
}

/// §3.5 + reading (a): in node form the hosted users are named by the
/// manifest, not by routing entries, so they hold no user index at all.
#[test]
fn switching_to_node_form_releases_hosted_user_indexes() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([1; 8])
    );

    state.register_hosted_user([2; 8], 0, fresh_multikey());
    assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

    let dict = state.user_dict.read().unwrap();
    assert!(
        dict.id_of(RESERVED_INDEX).is_none(),
        "user-space reserved slot is released on entering node form"
    );
    assert!(dict.idx_of(&[1; 8]).is_none());
    assert!(dict.idx_of(&[2; 8]).is_none());
    drop(dict);

    assert!(
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(Space::Node)
            .contains(&RESERVED_INDEX),
        "the node self-binding must be introduced on entering node form"
    );
}

/// §3.5: exactly one space holds a self-binding at a time. Entering node
/// form must bind node-space 0x0000 to the host, and it needs a `Node`
/// record for itself or `pending_introductions` discards the mark as an
/// orphan and neighbours never learn the binding.
#[test]
fn entering_node_form_binds_the_node_reserved_index() {
    let (state, _rx) = fresh_state();
    let host_node_id = state.host_mk.to_id();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());

    assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

    assert_eq!(
        state.node_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some(host_node_id),
        "host takes the node-space reserved slot in node form"
    );
    assert!(
        state.nodes.read().unwrap().get(&host_node_id).is_some(),
        "a Node record for the host must exist for the introduction to resolve"
    );

    let intros = state.pending_introductions(Space::Node);
    assert!(
        intros
            .iter()
            .any(|(idx, id, _)| *idx == RESERVED_INDEX && *id == host_node_id),
        "the node self-binding must resolve into a real introduction, not an orphan mark"
    );
}

/// The inverse: leaving node form releases the node-space self-binding, so
/// the two reserved slots are never bound simultaneously.
#[test]
fn leaving_node_form_releases_the_node_reserved_index() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());
    assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

    state.unregister_hosted_user([2; 8], 1_000);
    assert_eq!(state.sync_propagation_form(0), PropagationForm::User);

    assert!(
        state
            .node_dict
            .read()
            .unwrap()
            .id_of(RESERVED_INDEX)
            .is_none(),
        "node-space reserved slot is released on returning to user form"
    );
    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([1; 8])
    );
}

/// Dropping back to a single hosted user returns the node to user form and
/// puts that user back at RESERVED_INDEX.
#[test]
fn switching_back_to_user_form_rebinds_reserved_index() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());
    assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

    state.unregister_hosted_user([2; 8], 1_000);
    assert_eq!(state.sync_propagation_form(0), PropagationForm::User);

    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([1; 8]),
        "the surviving hosted user reclaims the reserved slot"
    );
}

/// Reconciling an unchanged form must be a no-op — it runs every origin
/// tick, so churning indexes here would reintroduce bindings forever.
#[test]
fn sync_is_a_noop_when_the_form_has_not_changed() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    assert_eq!(state.sync_propagation_form(0), PropagationForm::User);

    // drain marks from registration
    let _ = state.pending_introductions(Space::User);

    assert_eq!(state.sync_propagation_form(0), PropagationForm::User);
    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([1; 8])
    );
    assert!(
        state.pending_introductions(Space::User).is_empty(),
        "an unchanged form must not queue new introductions"
    );
}

/// Releasing an index must also drop its pending introduction. Otherwise
/// the mark outlives the binding and `pending_introductions` reports an
/// "orphan mark" — and worse, the index can be reallocated to a different
/// target while a stale mark still points at it.
#[test]
fn releasing_an_index_clears_its_pending_mark() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());

    // The first registration queued an introduction at the reserved slot;
    // the form switch releases that index, so the mark must go with it.
    assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

    let pending = state
        .reintroduction_tracker
        .write()
        .unwrap()
        .take_pending(Space::User);
    assert!(
        !pending.contains(&RESERVED_INDEX),
        "reserved slot's mark must be cleared with its binding"
    );
    assert!(pending.is_empty(), "no user-space marks should survive");
}

// ----- unregister_hosted_user -----

#[test]
fn unregister_releases_index_and_record() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());

    state.unregister_hosted_user([1; 8], 1_000);

    assert!(state
        .user_dict
        .read()
        .unwrap()
        .id_of(RESERVED_INDEX)
        .is_none());
    assert!(state.users.read().unwrap().get(&[1; 8]).is_none());
    assert_eq!(state.hosted_user_ids().len(), 0);
}

/// §3.5 keeps exactly one hosted user at 0x0000. Removing the one that
/// holds it must promote a survivor, not leave the slot empty.
#[test]
fn unregister_promotes_a_survivor_into_the_reserved_slot() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());
    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([1; 8])
    );

    state.unregister_hosted_user([1; 8], 1_000);

    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([2; 8]),
        "the remaining hosted user takes the reserved slot"
    );
}

/// Removing an *un-indexed* hosted user — the normal case in node form,
/// where only the reserved slot is ever bound — must leave that slot alone.
#[test]
fn unregister_unindexed_user_leaves_reserved_slot_intact() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());
    assert!(state.user_dict.read().unwrap().idx_of(&[2; 8]).is_none());

    state.unregister_hosted_user([2; 8], 1_000);

    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([1; 8]),
        "the reserved slot is untouched by removing an un-indexed user"
    );
    assert_eq!(state.hosted_user_ids(), vec![[1; 8]]);
}

/// Unregistering something we never hosted must not panic or disturb state.
#[test]
fn unregister_unknown_user_is_a_noop() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());

    state.unregister_hosted_user([9; 8], 1_000);

    assert_eq!(
        state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some([1; 8])
    );
    assert_eq!(state.hosted_user_ids().len(), 1);
}

// ------------------------------------------------- §3.2 trigger 3

/// Carrying a delegation for a user this node does not host is §3.2's third
/// trigger for node form, independent of user count and connectivity.
#[test]
fn holding_a_foreign_delegation_forces_node_form() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    assert_eq!(state.desired_propagation_form(), PropagationForm::User);

    state.record_delegation(manifest::DelegetedEntry {
        user_id: [9; 8],
        timeout: u64::MAX,
        entry_signature: [0u8; 64],
        profile_version: 0,
    });

    assert_eq!(state.desired_propagation_form(), PropagationForm::Node);
}

/// Our own users in our own manifest are self-delegations, not delegations
/// "from any other user" — they must not trip the trigger, or a single-user
/// host would leave user form the moment it published its own entry.
#[test]
fn a_self_delegation_is_not_a_foreign_delegation() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());

    state.record_delegation(manifest::DelegetedEntry {
        user_id: [1; 8],
        timeout: u64::MAX,
        entry_signature: [0u8; 64],
        profile_version: 0,
    });

    assert_eq!(state.desired_propagation_form(), PropagationForm::User);
}

/// The race this closes: a host that has just lost INTERNET is no longer a
/// gateway, but a requester holding its stale manifest can still subscribe,
/// and `settle_delegation` admits on signature alone. Trigger 3 keeps that
/// host in node form so its manifest still reaches the wire (§8.3).
#[test]
fn a_host_that_stopped_being_a_gateway_stays_in_node_form_while_it_carries_others() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);
    state.sync_gateway_role();
    assert_eq!(state.desired_propagation_form(), PropagationForm::Node);

    // A subscribe lands while it is still a gateway.
    state.record_delegation(manifest::DelegetedEntry {
        user_id: [9; 8],
        timeout: u64::MAX,
        entry_signature: [0u8; 64],
        profile_version: 0,
    });

    // The INTERNET transport drops.
    state.remove_neighbour_transport(peer, ConnectionModule::Internet);
    state.sync_gateway_role();
    assert!(!state.host_is_gateway());

    assert_eq!(
        state.desired_propagation_form(),
        PropagationForm::Node,
        "a host carrying someone else's delegation must keep advertising a manifest"
    );
}

/// Once the foreign delegation is gone the host falls back to user form.
#[test]
fn dropping_the_last_foreign_delegation_returns_to_user_form() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.record_delegation(manifest::DelegetedEntry {
        user_id: [9; 8],
        timeout: u64::MAX,
        entry_signature: [0u8; 64],
        profile_version: 0,
    });
    assert_eq!(state.desired_propagation_form(), PropagationForm::Node);

    state.remove_self_delegation(&[9; 8]);

    assert_eq!(state.desired_propagation_form(), PropagationForm::User);
}
