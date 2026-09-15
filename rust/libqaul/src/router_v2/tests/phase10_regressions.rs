// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Proofs for the two defects fixed in Phase 10: serve-time signing and the unrestored log base.

use crate::router_v2::{
    codec::messages::{ManifestEntry, ManifestRequestItem},
    identity::{ChunkSigningCtx, SelfDelegation},
    manifest::{canonical_entry_bytes, decide_serve, OriginServeState, ServeDecision},
    test_utils::*,
    BumpTrigger, Sphere,
};
use crate::storage::manifest_state::{DelegationEntry, HostManifestState};

fn delegation(timeout: u64, marker: u8) -> SelfDelegation {
    SelfDelegation {
        timeout,
        entry_signature: [marker; 64],
    }
}

/// Verifies a whole-state signature the way `handle_manifest_delta` and
/// `verify_chunk` both do: over the canonical entry bytes at a claimed
/// version, chunk 0 of 1.
fn verifies_as_state_at(
    state: &crate::router_v2::RouterV2State,
    signature: &[u8; 64],
    entries: &[ManifestEntry],
    version: u32,
    is_gateway: bool,
) -> bool {
    let bytes = canonical_entry_bytes(entries);
    let ctx = ChunkSigningCtx {
        origin_multikey: &state.host_mk.encode(),
        manifest_version: version,
        chunk_index: 0,
        chunk_count: 1,
        flags: if is_gateway { 1 } else { 0 },
        canonical_entries: &bytes,
    };
    state.host_mk.verify(&ctx.signing_input(), signature)
}

/// THE BUG STEP 3 FIXES.
///
/// `add_self_delegation` mutates the entry set immediately and only marks
/// the user dirty; the version bump that commits it is rate-limited to
/// once per 60 s (§10.8). So between bumps the entry set is *ahead* of
/// `manifest_version`. Signing at serve time — as `serve_delta` and
/// `serve_full_manifest` both used to — therefore signs uncommitted
/// state under the committed version number we advertise, and a receiver
/// commits state the origin never committed at that version.
#[test]
fn signing_at_serve_time_would_cover_uncommitted_state() {
    let (state, _rx) = fresh_state();

    // Commit one delegation at v1.
    state.add_self_delegation([1u8; 8], 0, delegation(9_000, 0xA1));
    let v1 = state
        .try_bump_manifest_version(1_000_000, BumpTrigger::Accumulated)
        .expect("first bump");

    let (cached_at_v1, entries_at_v1) = {
        let m = state.manifest.read().unwrap();
        (m.manifest_signature.unwrap(), m.entries().to_vec())
    };
    assert_eq!(entries_at_v1.len(), 1);

    // A second delegation lands 500 ms later. The entry set changes
    // immediately, but the 60 s rate limit blocks the bump.
    state.add_self_delegation([2u8; 8], 0, delegation(9_000, 0xB2));
    assert!(
        state
            .try_bump_manifest_version(1_000_500, BumpTrigger::Accumulated)
            .is_none(),
        "§10.8 rate limit should suppress a bump 500ms after the last"
    );

    let m = state.manifest.read().unwrap();

    // The divergence that makes serve-time signing unsound.
    assert_eq!(m.manifest_version, v1, "still advertising v1");
    assert_eq!(m.entries().len(), 2, "but the entry set already moved on");

    // What the old serve path computed, on demand, from current state.
    let serve_time_sig = m
        .sign_state(&state.host_keypair, &state.host_mk.encode())
        .unwrap();

    assert_ne!(
        serve_time_sig, cached_at_v1,
        "serve-time signature differs from the committed one — this is the bug"
    );

    // The cached signature is the correct one for the version we
    // advertise; the serve-time one is not.
    assert!(
        verifies_as_state_at(&state, &cached_at_v1, &entries_at_v1, v1, false),
        "cached signature must verify against the state committed at v1"
    );
    assert!(
        !verifies_as_state_at(&state, &serve_time_sig, &entries_at_v1, v1, false),
        "serve-time signature must NOT verify against v1's committed state"
    );

    // And it is not salvageable by claiming the next version either:
    // nothing has committed v1+1, so no receiver would accept it.
    assert!(
        !verifies_as_state_at(&state, &serve_time_sig, &entries_at_v1, v1 + 1, false),
        "serve-time signature covers neither the committed nor any advertised version"
    );
}

/// The other half: after the fix the cached signature survives a
/// subsequent bump correctly — it tracks the new committed state, not
/// the stale one.
#[test]
fn the_cached_signature_follows_each_commit() {
    let (state, _rx) = fresh_state();

    state.add_self_delegation([1u8; 8], 0, delegation(9_000, 0xA1));
    let v1 = state
        .try_bump_manifest_version(1_000_000, BumpTrigger::Accumulated)
        .unwrap();
    let sig_v1 = state.manifest.read().unwrap().manifest_signature.unwrap();

    // Past the 60 s window, so this one commits.
    state.add_self_delegation([2u8; 8], 0, delegation(9_000, 0xB2));
    let v2 = state
        .try_bump_manifest_version(1_070_000, BumpTrigger::Accumulated)
        .expect("bump past the rate-limit window");
    assert_eq!(v2, v1 + 1);

    let m = state.manifest.read().unwrap();
    let sig_v2 = m.manifest_signature.unwrap();
    assert_ne!(sig_v1, sig_v2);
    assert!(
        verifies_as_state_at(&state, &sig_v2, m.entries(), v2, false),
        "cached signature must cover the newly committed state"
    );
}

/// End-to-end proof for Step 3: drive a real MANIFEST_REQUEST through
/// `handle_manifest_request` and check the emitted MANIFEST_DELTA the way
/// a receiver does — apply the records to the state at `from_version`,
/// then verify the signature over the resulting state at `to_version`.
///
/// With serve-time signing this fails: the signature covers whatever the
/// entry set happens to be at serve time, not the committed state at
/// `to_version`.
#[test]
fn a_served_delta_verifies_against_the_state_it_claims_to_produce() {
    use crate::connections::ConnectionModule;
    use crate::router_v2::codec::{
        messages::{ManifestDelta, ManifestRequest},
        Header, RoutingMessage,
    };

    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [7u8; 8], ConnectionModule::Lan);

    // Commit two versions so a ranged delta (1 -> 2) is servable.
    state.add_self_delegation([1u8; 8], 0, delegation(9_000, 0xA1));
    state
        .try_bump_manifest_version(1_000_000, BumpTrigger::Accumulated)
        .unwrap();
    state.add_self_delegation([2u8; 8], 0, delegation(9_000, 0xB2));
    let v2 = state
        .try_bump_manifest_version(1_070_000, BumpTrigger::Accumulated)
        .unwrap();

    let entries_at_v2 = state.manifest.read().unwrap().entries().to_vec();
    assert_eq!(entries_at_v2.len(), 2);

    // A third delegation lands but cannot commit — the rate limit holds.
    // The entry set is now ahead of the version we still advertise.
    state.add_self_delegation([3u8; 8], 0, delegation(9_000, 0xC3));
    assert!(state
        .try_bump_manifest_version(1_070_500, BumpTrigger::Accumulated)
        .is_none());
    assert_eq!(state.manifest.read().unwrap().entries().len(), 3);
    assert_eq!(state.manifest.read().unwrap().manifest_version, v2);

    // Serve a request from a neighbour sitting at version 1.
    let req = ManifestRequest {
        items: vec![ManifestRequestItem {
            origin_node_id: state.host_mk.to_id(),
            have_version: 1,
            item_flags: 0x00,
        }],
    };
    state
        .handle_manifest_request(peer, ConnectionModule::Lan, req, 1_070_500)
        .unwrap();

    let out = rx.try_recv().expect("a response should have been emitted");
    let (header, body) = Header::decode(&out.bytes).unwrap();
    assert_eq!(header.message_type, RoutingMessage::ManifestDelta);
    let delta = ManifestDelta::decode(body).unwrap();

    assert_eq!(delta.from_version, 1);
    assert_eq!(delta.to_version, v2);

    // Replay the receiver: state at from_version, apply, then verify.
    let mut scratch: Vec<ManifestEntry> = entries_at_v2
        .iter()
        .copied()
        .filter(|e| e.user_id == [1u8; 8])
        .collect();
    for r in &delta.removes {
        scratch.retain(|e| e.user_id != r.user_id);
    }
    for a in &delta.adds {
        match scratch.binary_search_by(|e| e.user_id.cmp(&a.entry.user_id)) {
            Ok(i) => scratch[i] = a.entry,
            Err(i) => scratch.insert(i, a.entry),
        }
    }
    let ids = |v: &[ManifestEntry]| v.iter().map(|e| e.user_id).collect::<Vec<_>>();
    assert_eq!(
        ids(&scratch),
        ids(&entries_at_v2),
        "delta must reproduce v2's entry set"
    );

    assert!(
        verifies_as_state_at(&state, &delta.manifest_signature, &scratch, v2, false),
        "the served delta must verify against the state it claims to produce"
    );
}

/// THE BUG STEP 1 FIXES.
///
/// §10.9: the delta log is not persisted, so on restart `log_base` must
/// be set to the committed version. Left at 0, `decide_serve` picks the
/// delta branch for any requester below the committed version — and
/// serves a delta assembled from an empty log, which cannot verify.
#[test]
fn after_restart_a_stale_requester_gets_a_full_manifest_not_an_empty_delta() {
    let (state, _rx) = fresh_state();

    let persisted = HostManifestState {
        manifest_version: 5,
        is_gateway: false,
        entries: vec![DelegationEntry {
            user_id: [9u8; 8],
            timeout: 9_000,
            entry_signature: vec![0xC3; 64],
            profile_version: 0,
        }],
        last_bump_ms_reserved: None,
    };
    state.restore_host_manifest(&persisted);

    let log_base = state.own_manifest_log.read().unwrap().log_base;
    assert_eq!(log_base, 5, "§10.9 restart: log_base tracks committed");

    // Restore must also leave the origin able to serve at all.
    assert!(
        state.manifest.read().unwrap().manifest_signature.is_some(),
        "a restored origin must be servable without waiting for a bump"
    );

    // A neighbour two versions behind.
    let item = ManifestRequestItem {
        origin_node_id: state.host_mk.to_id(),
        have_version: 3,
        item_flags: 0x00,
    };

    let fixed = OriginServeState {
        committed: 5,
        log_base,
        learn_sphere: None,
    };
    assert_eq!(
        decide_serve(&item, &fixed, Sphere::Local),
        ServeDecision::Full,
        "history before the restart is unknown, so only a full manifest is honest"
    );

    // The counterfactual: log_base left at 0, as before the fix.
    let broken = OriginServeState {
        committed: 5,
        log_base: 0,
        learn_sphere: None,
    };
    assert_eq!(
        decide_serve(&item, &broken, Sphere::Local),
        ServeDecision::Delta { from_version: 3 },
        "the old state chose a delta ..."
    );
    assert!(
        state
            .own_manifest_log
            .read()
            .unwrap()
            .records_after(3)
            .is_empty(),
        "... assembled from a log with no records — a 3→5 delta claiming no changes"
    );
}
