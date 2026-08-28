// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! §3.4 staleness detection end to end: the advertised `profile_version`
//! reaching routing state (origin side), and §3.8 trigger 3 marking the
//! index so the new value actually goes out (relay side).
//!
//! §8.3 makes these two halves one mechanism — a user *entry* carries no
//! version field, so a `profile_version` travels only in an inline mapping.

use crate::router_v2::*;
use crate::router_v2::{
    codec::messages::Mapping,
    identity::Profile,
    index::{Space, RESERVED_INDEX},
    management::profile::{HostedProfile, SignedProfileBlob},
    test_utils::*,
};

fn hosted_profile(multikey: identity::Multikey, version: u32) -> HostedProfile {
    HostedProfile {
        profile: Profile {
            multikey,
            version,
            name: "tester".to_string(),
            self_signature: [0u8; 64],
        },
        signed: SignedProfileBlob::default(),
    }
}

// -------------------------------------------------------------- origin side

/// §3.4: `pending_introductions` and `INDEX_DUMP` both read the advertised
/// version off the `UsersMap` record, so publishing a profile has to write
/// through to it — not only into `hosted_profiles`.
#[test]
fn registering_a_hosted_profile_writes_the_version_through() {
    let (state, _rx) = fresh_state();
    let mk = fresh_multikey();
    state.register_hosted_user([1; 8], 0, mk.clone());

    state.register_hosted_profile([1; 8], hosted_profile(mk, 7));

    let users = state.users.read().unwrap();
    let user = users.get(&[1; 8]).expect("hosted user");
    assert_eq!(user.read().unwrap().profile_version, 7);
}

/// The whole point: a rename must make the next `ROUTING_UPDATE` carry a
/// fresh inline mapping, or no neighbour is ever told to re-fetch.
#[test]
fn a_profile_change_is_advertised_in_the_next_introduction() {
    let (state, _rx) = fresh_state();
    let mk = fresh_multikey();
    state.register_hosted_user([1; 8], 0, mk.clone());
    state.register_hosted_profile([1; 8], hosted_profile(mk.clone(), 1));
    // Drain the first-time mark from registration.
    let _ = state.pending_introductions(Space::User);

    state.register_hosted_profile([1; 8], hosted_profile(mk, 2));

    let intros = state.pending_introductions(Space::User);
    assert_eq!(
        intros,
        vec![(RESERVED_INDEX, [1; 8], 2)],
        "§3.8 trigger 3 must re-introduce the index with the new version"
    );
}

/// §3.4 counts up only. A stale re-publish must neither roll the advertised
/// value back nor spend an introduction.
#[test]
fn a_stale_profile_republish_changes_nothing() {
    let (state, _rx) = fresh_state();
    let mk = fresh_multikey();
    state.register_hosted_user([1; 8], 0, mk.clone());
    state.register_hosted_profile([1; 8], hosted_profile(mk.clone(), 5));
    let _ = state.pending_introductions(Space::User);

    state.register_hosted_profile([1; 8], hosted_profile(mk, 3));

    assert_eq!(
        state.hosted_profile_version(&[1; 8]),
        3,
        "the map is a cache"
    );
    let users = state.users.read().unwrap();
    assert_eq!(
        users.get(&[1; 8]).unwrap().read().unwrap().profile_version,
        5,
        "the advertised version never regresses"
    );
    drop(users);
    assert!(state.pending_introductions(Space::User).is_empty());
}

/// At account creation `publish_hosted_profile` runs *before*
/// `register_hosted_user`, so registration must not clobber the version the
/// profile publish just wrote.
#[test]
fn registering_a_hosted_user_does_not_roll_the_version_back() {
    let (state, _rx) = fresh_state();
    let mk = fresh_multikey();
    install_user(&state, [1; 8], 0);
    state.register_hosted_profile([1; 8], hosted_profile(mk.clone(), 9));

    state.register_hosted_user([1; 8], 0, mk);

    let users = state.users.read().unwrap();
    assert_eq!(
        users.get(&[1; 8]).unwrap().read().unwrap().profile_version,
        9
    );
}

/// §10.1's host-asserted `profile_version` on a manifest entry comes from
/// the same source, so a self-delegation issued after a profile change
/// carries the new value.
#[test]
fn hosted_profile_version_is_the_single_source() {
    let (state, _rx) = fresh_state();
    let mk = fresh_multikey();
    state.register_hosted_user([1; 8], 0, mk.clone());

    assert_eq!(
        state.hosted_profile_version(&[1; 8]),
        0,
        "nothing published"
    );
    state.register_hosted_profile([1; 8], hosted_profile(mk, 4));
    assert_eq!(state.hosted_profile_version(&[1; 8]), 4);
    assert_eq!(state.hosted_profile_version(&[9; 8]), 0, "unknown user");
}

// --------------------------------------------------------------- relay side

/// §8.3: when we learn a fresher version for a user we relay, our own index
/// for it must be re-introduced or the change stops dead at one hop.
#[test]
fn a_relayed_version_advance_marks_our_index() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    install_user(&state, [1; 8], 3);
    bind_own_dict(&state, Space::User, 20, [1; 8]);

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: [1; 8],
                version: 8,
            },
            1_000,
        )
        .expect("processed");

    assert_eq!(
        state.pending_introductions(Space::User),
        vec![(20, [1; 8], 8)],
        "the downstream neighbours have to hear the new version from us"
    );
}

/// A repeat of the version we already hold is not an advance and must not
/// spend an introduction slot every batch.
#[test]
fn a_repeated_version_does_not_mark_our_index() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);
    install_user(&state, [1; 8], 8);
    bind_own_dict(&state, Space::User, 20, [1; 8]);
    let _ = state.pending_introductions(Space::User);

    state
        .apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: [1; 8],
                version: 8,
            },
            1_000,
        )
        .expect("processed");

    assert!(state.pending_introductions(Space::User).is_empty());
}

/// The mark is keyed on our own dictionary. A user we hold no index for —
/// a hosted user under node form (§3.2), or one we have only just heard of
/// — has nothing to re-introduce, and marking would strand an orphan index
/// in the tracker.
#[test]
fn marking_a_user_with_no_index_is_a_noop() {
    let (state, _rx) = fresh_state();
    install_user(&state, [1; 8], 3);

    state.mark_profile_version_bump(&[1; 8]);

    assert!(state.pending_introductions(Space::User).is_empty());
}

/// §11.5: a profile fetch response is the other place a version advances.
#[test]
fn the_helper_marks_a_bound_index() {
    let (state, _rx) = fresh_state();
    install_user(&state, [1; 8], 3);
    bind_own_dict(&state, Space::User, 20, [1; 8]);

    state.mark_profile_version_bump(&[1; 8]);

    assert_eq!(
        state.pending_introductions(Space::User),
        vec![(20, [1; 8], 3)]
    );
}
