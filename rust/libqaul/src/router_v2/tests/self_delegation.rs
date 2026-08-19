// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Issuing and revoking this host's own delegations (spec §10.2).

use crate::router_v2::*;
use crate::{
    node::user_accounts::UserAccount,
    router_v2::{
        manifest::{LogRecord, Manifest},
        test_utils::*,
        BumpTrigger,
    },
    storage::manifest_state::HostManifestState,
};
use libp2p::{identity::Keypair, PeerId};

/// `manifest_rate_limit` defaults to 60 s (§14); the window is in ms.
const WINDOW_MS: u64 = 60_000;
const TTL_MS: u64 = 6 * 60 * 60 * 1000;

fn fresh_account() -> UserAccount {
    let keys = Keypair::generate_ed25519();
    UserAccount {
        id: PeerId::from(keys.public()),
        keys,
        name: "test".into(),
        password_hash: None,
        password_salt: None,
    }
}

/// Registers `account` as a hosted user with a self-delegation, the way
/// `UserAccounts::create` does. Returns its routing id.
fn delegate(state: &RouterV2State, account: &UserAccount, now_ms: u64) -> [u8; 8] {
    let id = account.routing_user_id();
    let delegation = account.issue_self_delegation(&state.host_mk, now_ms + TTL_MS);
    state.add_self_delegation(id, 0, delegation);
    id
}

// ----- the signing boundary -----

/// The round trip that proves the split works: libqaul signs with the
/// user's key, router_v2 stores the artefact, and a receiver reconstructs
/// the same input from `(host_mk, entry.timeout)` and verifies it against
/// the user's multikey. If the signing input ever drifts on either side,
/// this is the test that fails.
#[test]
fn stored_delegation_verifies_against_the_users_key() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = delegate(&state, &account, 0);

    let manifest = state.manifest.read().unwrap();
    let entry = manifest
        .entries()
        .iter()
        .find(|e| e.user_id == id)
        .expect("entry stored");

    assert!(
        Manifest::verify_entry(entry, &state.host_mk, &account.multikey()).is_ok(),
        "the stored entry must verify against the delegating user's key"
    );
}

/// The timeout is signed content (§10.1), so extending an entry's life
/// without a fresh signature must not verify.
#[test]
fn tampering_with_the_timeout_breaks_verification() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = delegate(&state, &account, 0);

    let mut entry = *state
        .manifest
        .read()
        .unwrap()
        .entries()
        .iter()
        .find(|e| e.user_id == id)
        .unwrap();
    entry.timeout = entry.timeout.saturating_add(1);

    assert!(
        Manifest::verify_entry(&entry, &state.host_mk, &account.multikey()).is_err(),
        "an altered timeout must invalidate the delegation"
    );
}

/// A delegation is bound to one host. Another host's key must not verify
/// it, or a node could claim to represent users that never authorised it.
#[test]
fn a_delegation_does_not_verify_for_a_different_host() {
    let (state, _rx) = fresh_state();
    let (other_host, _rx2) = fresh_state();
    let account = fresh_account();
    let id = delegate(&state, &account, 0);

    let manifest = state.manifest.read().unwrap();
    let entry = manifest.entries().iter().find(|e| e.user_id == id).unwrap();

    assert!(
        Manifest::verify_entry(entry, &other_host.host_mk, &account.multikey()).is_err(),
        "the signature binds the authorisation to one specific host"
    );
}

// ----- accumulate -----

/// §10.8: a change marks the manifest dirty but must not bump on its own —
/// timing belongs to the bump.
#[test]
fn adding_a_delegation_marks_dirty_without_bumping() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = delegate(&state, &account, 0);

    assert_eq!(state.manifest.read().unwrap().entries().len(), 1);
    assert_eq!(
        state.manifest.read().unwrap().manifest_version,
        0,
        "the add itself must not bump"
    );
    assert!(state.dirty_delegations.read().unwrap().contains(&id));
}

#[test]
fn accumulated_bump_inside_the_window_is_declined() {
    let (state, _rx) = fresh_state();
    delegate(&state, &fresh_account(), 0);

    assert_eq!(
        state.try_bump_manifest_version(1_000, BumpTrigger::Accumulated),
        None,
        "still inside the 60s window"
    );
    assert_eq!(state.manifest.read().unwrap().manifest_version, 0);
}

#[test]
fn accumulated_bump_with_nothing_dirty_is_declined() {
    let (state, _rx) = fresh_state();

    assert_eq!(
        state.try_bump_manifest_version(WINDOW_MS * 10, BumpTrigger::Accumulated),
        None,
        "no change to fold"
    );
}

#[test]
fn accumulated_bump_after_the_window_folds_and_logs() {
    let (state, _rx) = fresh_state();
    let id = delegate(&state, &fresh_account(), 0);

    assert_eq!(
        state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
        Some(1)
    );
    assert_eq!(state.manifest.read().unwrap().manifest_version, 1);
    assert!(
        state.dirty_delegations.read().unwrap().is_empty(),
        "the dirty set is consumed by the fold"
    );

    let records = state.own_manifest_log.read().unwrap().records_after(0);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].record_version(), 1);
    assert_eq!(records[0].user_id(), id);
    assert!(matches!(records[0], LogRecord::Add { .. }));
}

/// The core of §10.8: "changes that occur within a window accumulate and
/// fold into a single bump". Two adds produce one version carrying two
/// records, not two versions.
#[test]
fn two_adds_in_one_window_fold_into_a_single_bump() {
    let (state, _rx) = fresh_state();
    let a = delegate(&state, &fresh_account(), 0);
    let b = delegate(&state, &fresh_account(), 1_000);

    assert_eq!(
        state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
        Some(1),
        "both changes fold into one version"
    );

    let records = state.own_manifest_log.read().unwrap().records_after(0);
    assert_eq!(records.len(), 2);
    assert!(records.iter().all(|r| r.record_version() == 1));

    let mut ids: Vec<[u8; 8]> = records.iter().map(|r| r.user_id()).collect();
    ids.sort();
    let mut expected = vec![a, b];
    expected.sort();
    assert_eq!(ids, expected);
}

/// Folding from *current state* rather than replaying operations is what
/// makes this correct for free: a user added and removed inside one window
/// never appeared in a committed version, and collapses to one tombstone.
#[test]
fn add_then_remove_in_one_window_folds_to_a_tombstone() {
    let (state, _rx) = fresh_state();
    let id = delegate(&state, &fresh_account(), 0);
    assert!(state.remove_self_delegation(&id));

    assert_eq!(
        state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
        Some(1)
    );

    assert!(state.manifest.read().unwrap().entries().is_empty());
    let records = state.own_manifest_log.read().unwrap().records_after(0);
    assert_eq!(records.len(), 1);
    assert!(matches!(records[0], LogRecord::Tombstone { .. }));
}

/// Re-issuing an identical delegation is not a change. Without this the
/// TTL refresh (§10.3) would bump the version every cycle forever and make
/// every peer re-pull each time.
#[test]
fn identical_redelegation_is_not_a_change() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = account.routing_user_id();
    let delegation = account.issue_self_delegation(&state.host_mk, TTL_MS);

    assert!(state.add_self_delegation(id, 0, delegation));
    assert!(
        !state.add_self_delegation(id, 0, delegation),
        "an identical re-issue must report no change"
    );
    assert_eq!(state.manifest.read().unwrap().entries().len(), 1);
}

/// A fresh timeout means a fresh signature, which *is* a change.
#[test]
fn redelegating_with_a_new_timeout_is_a_change() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = account.routing_user_id();

    assert!(state.add_self_delegation(
        id,
        0,
        account.issue_self_delegation(&state.host_mk, TTL_MS)
    ));
    assert!(state.add_self_delegation(
        id,
        0,
        account.issue_self_delegation(&state.host_mk, TTL_MS + 1)
    ));
    assert_eq!(state.manifest.read().unwrap().entries().len(), 1);
}

/// The startup path uses this to tell "restored from disk" from "created
/// while v2 was disabled, so never issued at all".
#[test]
fn has_self_delegation_distinguishes_present_from_absent() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = account.routing_user_id();

    assert!(!state.has_self_delegation(&id));
    delegate(&state, &account, 1_000_000_000);
    assert!(state.has_self_delegation(&id));

    state.remove_self_delegation(&id);
    assert!(!state.has_self_delegation(&id));
}

// ----- §10.4 refresh window -----

/// `delegation_referesh` defaults to 3 h (TTL/2), in seconds.
const REFRESH_MS: u64 = 3 * 60 * 60 * 1000;

#[test]
fn a_fresh_delegation_is_not_due_for_refresh() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let now = 1_000_000_000u64;
    delegate(&state, &account, now);

    assert!(
        state.delegations_due_for_refresh(now).is_empty(),
        "a delegation with a full TTL ahead of it is not due"
    );
}

#[test]
fn a_delegation_inside_the_window_is_due() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let now = 1_000_000_000u64;
    let id = delegate(&state, &account, now);

    // one ms past the point where the remaining lifetime drops to TTL/2
    let due = state.delegations_due_for_refresh(now + REFRESH_MS + 1);

    assert_eq!(due, vec![(id, 0)]);
}

/// The boundary itself counts as due — waiting for strict expiry would
/// leave no margin for a missed tick.
#[test]
fn a_delegation_exactly_at_the_window_edge_is_due() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let now = 1_000_000_000u64;
    let id = delegate(&state, &account, now);

    let due = state.delegations_due_for_refresh(now + REFRESH_MS);

    assert_eq!(due, vec![(id, 0)]);
}

/// A node down for longer than the TTL comes back with expired entries.
/// They must still be reported, or the user is never rescued.
#[test]
fn an_already_expired_delegation_is_still_due() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let now = 1_000_000_000u64;
    let id = delegate(&state, &account, now);

    let due = state.delegations_due_for_refresh(now + TTL_MS + 1);

    assert_eq!(due, vec![(id, 0)]);
}

/// The refresh re-issues through `add_self_delegation`, which replaces the
/// whole entry — so the caller has to carry the stored `profile_version`
/// forward rather than assuming zero.
#[test]
fn due_entries_report_their_profile_version() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = account.routing_user_id();
    let now = 1_000_000_000u64;
    state.add_self_delegation(
        id,
        7,
        account.issue_self_delegation(&state.host_mk, now + TTL_MS),
    );

    let due = state.delegations_due_for_refresh(now + REFRESH_MS + 1);

    assert_eq!(due, vec![(id, 7)]);
}

/// A refresh is an ordinary change: it marks the user dirty and rides the
/// next accumulated bump rather than forcing one.
#[test]
fn refreshing_marks_dirty_and_bumps_with_the_window() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = account.routing_user_id();
    let now = 1_000_000_000u64;
    delegate(&state, &account, now);
    state.try_bump_manifest_version(now, BumpTrigger::Accumulated);

    let refresh_at = now + REFRESH_MS + 1;
    assert!(
        state.add_self_delegation(
            id,
            0,
            account.issue_self_delegation(&state.host_mk, refresh_at + TTL_MS)
        ),
        "a new timeout is a change"
    );

    let bumped = state.try_bump_manifest_version(refresh_at, BumpTrigger::Accumulated);
    assert!(bumped.is_some(), "the refresh must reach a version bump");
    assert!(
        state.delegations_due_for_refresh(refresh_at).is_empty(),
        "after refreshing, the entry leaves the window"
    );
}

#[test]
fn removing_an_absent_delegation_is_not_a_change() {
    let (state, _rx) = fresh_state();
    assert!(!state.remove_self_delegation(&[9; 8]));
    assert!(state.dirty_delegations.read().unwrap().is_empty());
}

// ----- rate-limit bypasses -----

/// §10.8 names the single↔multi transition as a bypass, and it is a trigger
/// in its own right — so it must bump inside the window *and* with nothing
/// dirty. A plain `force: bool` would get the second half wrong.
#[test]
fn form_transition_bumps_inside_the_window_with_nothing_dirty() {
    let (state, _rx) = fresh_state();

    assert_eq!(
        state.try_bump_manifest_version(0, BumpTrigger::FormTransition),
        Some(1)
    );
}

#[test]
fn forced_removal_bypasses_the_window() {
    let (state, _rx) = fresh_state();
    let id = delegate(&state, &fresh_account(), 0);
    state.remove_self_delegation(&id);

    assert_eq!(
        state.try_bump_manifest_version(1_000, BumpTrigger::ForcedRemoval),
        Some(1),
        "§10.7 removal takes effect in the next relay batch"
    );
}

/// A bump restarts the window, so a second accumulated change has to wait
/// out a fresh 60 s rather than riding the first bump's timestamp.
#[test]
fn a_bump_restarts_the_rate_limit_window() {
    let (state, _rx) = fresh_state();
    delegate(&state, &fresh_account(), 0);
    assert_eq!(
        state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
        Some(1)
    );

    delegate(&state, &fresh_account(), WINDOW_MS + 1);
    assert_eq!(
        state.try_bump_manifest_version(WINDOW_MS + 1_000, BumpTrigger::Accumulated),
        None,
        "the window restarted at the previous bump"
    );
    assert_eq!(
        state.try_bump_manifest_version(WINDOW_MS * 2, BumpTrigger::Accumulated),
        Some(2)
    );
}

// ----- persistence (§10.8 SHALL) -----

/// §10.8: "An origin SHALL persist its `manifest_version` across restarts
/// and resume from the persisted value… a regression would corrupt delta
/// selection." This covers the snapshot/restore round trip without touching
/// the filesystem.
#[test]
fn snapshot_round_trips_version_and_entries() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let id = delegate(&state, &account, 0);
    state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated);

    let snapshot = state.host_manifest_snapshot();
    assert_eq!(snapshot.manifest_version, 1);
    assert_eq!(snapshot.entries.len(), 1);
    assert_eq!(snapshot.entries[0].entry_signature.len(), 64);

    // A restarted node restores from it.
    let (restarted, _rx2) = fresh_state();
    restarted.restore_host_manifest(&snapshot);

    let manifest = restarted.manifest.read().unwrap();
    assert_eq!(manifest.manifest_version, 1, "version must not regress");
    assert_eq!(manifest.entries().len(), 1);
    assert_eq!(manifest.entries()[0].user_id, id);
}

/// A first startup with no file on disk must not look like a regression.
#[test]
fn default_host_state_restores_as_a_clean_origin() {
    let (state, _rx) = fresh_state();
    state.restore_host_manifest(&HostManifestState::default());

    assert_eq!(state.manifest.read().unwrap().manifest_version, 0);
    assert!(state.manifest.read().unwrap().entries().is_empty());
}
