// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The Phase 10 delta primitives: signing, assembly, and the reusable-signature property (spec §8.6).

use crate::router_v2::{
    codec::messages::{ManifestDelta, ManifestEntry, NodeManifest},
    identity::{delegation_signing_input, ChunkSigningCtx, Multikey},
    manifest::{canonical_entry_bytes, BuildError, DeltaHeader, LogRecord, Manifest, MAX_BODY},
};
use libp2p::identity::Keypair;

fn origin() -> (Keypair, Multikey) {
    let kp = Keypair::generate_ed25519();
    let mk = Multikey::from(kp.public());
    (kp, mk)
}

/// A delegation entry genuinely signed by a fresh user for `host_mk`.
fn signed_entry(host_mk: &Multikey, timeout: u64, profile_version: u32) -> ManifestEntry {
    let user_kp = Keypair::generate_ed25519();
    let user_mk = Multikey::from(user_kp.public());
    let signing_input = delegation_signing_input(&host_mk.encode(), timeout);
    let sig: [u8; 64] = user_kp.sign(&signing_input).unwrap().try_into().unwrap();
    ManifestEntry {
        user_id: user_mk.to_id(),
        timeout,
        entry_signature: sig,
        profile_version,
    }
}

fn manifest_at(host_mk: &Multikey, version: u32, is_gateway: bool, n: usize) -> Manifest {
    let mut m = Manifest::new();
    m.manifest_version = version;
    m.set_gateway(is_gateway);
    m.set_entries(
        (0..n)
            .map(|i| signed_entry(host_mk, 9_000 + i as u64, i as u32))
            .collect(),
    );
    m
}

/// Rebuilds a single-chunk NODE_MANIFEST around an already-stored
/// signature — no keys. This is the shape Phase 10 subtask 3's
/// `reconstruct_single_chunk_full` must produce, and the shape the
/// serve path must use instead of re-signing.
fn single_chunk_full(
    origin_node_id: [u8; 8],
    manifest_version: u32,
    is_gateway: bool,
    entries: &[ManifestEntry],
    stored_signature: [u8; 64],
) -> NodeManifest {
    NodeManifest {
        origin_node_id,
        manifest_version,
        chunk_index: 0,
        chunk_count: 1,
        flags: if is_gateway { 1 } else { 0 },
        manifest_signature: stored_signature,
        entries: entries.to_vec(),
    }
}

/// Mirrors `handle_manifest_delta` step 4: the receiver verifies the
/// message signature against the canonical bytes of the *resulting*
/// state at `to_version`, not against anything in the delta itself.
fn delta_verifies(msg: &ManifestDelta, mk: &Multikey, resulting: &[ManifestEntry]) -> bool {
    let bytes = canonical_entry_bytes(resulting);
    let ctx = ChunkSigningCtx {
        origin_multikey: &mk.encode(),
        manifest_version: msg.to_version,
        chunk_index: 0,
        chunk_count: 1,
        flags: msg.flags & 0x01,
        canonical_entries: &bytes,
    };
    mk.verify(&ctx.signing_input(), &msg.manifest_signature)
}

fn adds_for(entries: &[ManifestEntry], record_version: u32) -> Vec<LogRecord> {
    entries
        .iter()
        .map(|e| LogRecord::Add {
            record_version,
            entry: *e,
        })
        .collect()
}

#[test]
fn sign_state_round_trips_through_verify_chunk() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 7, false, 3);

    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();
    let full = single_chunk_full(mk.to_id(), 7, false, manifest.entries(), sig);

    assert!(Manifest::verify_chunk(&full, &mk).is_ok());
}

#[test]
fn sign_state_covers_the_gateway_flag() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 7, true, 2);
    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();

    // Same entries, same version, flags flipped: must not verify.
    let lying = single_chunk_full(mk.to_id(), 7, false, manifest.entries(), sig);
    assert!(Manifest::verify_chunk(&lying, &mk).is_err());
}

#[test]
fn stored_signature_verifies_an_assembled_delta() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 5, false, 3);
    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();

    let header = DeltaHeader {
        origin_node_id: mk.to_id(),
        from_version: 0,
        to_version: 5,
        is_gateway: false,
        manifest_signature: sig,
    };
    let msg = header.assemble(adds_for(manifest.entries(), 5)).unwrap();

    assert!(delta_verifies(&msg, &mk, manifest.entries()));
}

/// The property the whole sign-once/serve-many split rests on:
/// `from_version` is not signed content (§8.6), so one signature at
/// `to_version` serves a delta from *any* earlier base. This is what
/// lets a relay serve a foreign manifest without the origin's key.
#[test]
fn one_signature_serves_every_from_version() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 9, true, 4);
    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();

    let mut seen = Vec::new();
    for from in [0u32, 1, 5, 8] {
        let header = DeltaHeader {
            origin_node_id: mk.to_id(),
            from_version: from,
            to_version: 9,
            is_gateway: true,
            manifest_signature: sig,
        };
        let msg = header.assemble(adds_for(manifest.entries(), 9)).unwrap();

        assert_eq!(msg.from_version, from);
        assert_eq!(
            msg.manifest_signature, sig,
            "signature must not vary with from_version"
        );
        assert!(
            delta_verifies(&msg, &mk, manifest.entries()),
            "delta from {from} failed resulting-state verification"
        );
        seen.push(msg.manifest_signature);
    }

    // And the very same signature still verifies as a full manifest.
    let full = single_chunk_full(mk.to_id(), 9, true, manifest.entries(), sig);
    assert!(Manifest::verify_chunk(&full, &mk).is_ok());
    assert!(seen.iter().all(|s| *s == sig));
}

#[test]
fn tampering_with_flags_breaks_delta_verification() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 4, true, 2);
    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();

    let header = DeltaHeader {
        origin_node_id: mk.to_id(),
        from_version: 3,
        to_version: 4,
        is_gateway: true,
        manifest_signature: sig,
    };
    let mut msg = header.assemble(adds_for(manifest.entries(), 4)).unwrap();
    assert!(delta_verifies(&msg, &mk, manifest.entries()));

    // Clearing the gateway bit in flight must be detected.
    msg.flags = 0;
    assert!(!delta_verifies(&msg, &mk, manifest.entries()));
}

#[test]
fn tampering_with_to_version_breaks_delta_verification() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 4, false, 2);
    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();

    let header = DeltaHeader {
        origin_node_id: mk.to_id(),
        from_version: 3,
        to_version: 4,
        is_gateway: false,
        manifest_signature: sig,
    };
    let mut msg = header.assemble(adds_for(manifest.entries(), 4)).unwrap();

    msg.to_version = 5;
    assert!(!delta_verifies(&msg, &mk, manifest.entries()));
}

/// A version bump that leaves the entry set untouched is legitimate,
/// and produces a delta with no records. The receiver's scratch set
/// equals its stored set, so the signature must still verify.
#[test]
fn empty_records_still_verify_when_the_state_did_not_change() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 6, false, 2);
    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();

    let header = DeltaHeader {
        origin_node_id: mk.to_id(),
        from_version: 5,
        to_version: 6,
        is_gateway: false,
        manifest_signature: sig,
    };
    let msg = header.assemble(Vec::new()).unwrap();

    assert!(msg.adds.is_empty());
    assert!(msg.removes.is_empty());
    assert!(delta_verifies(&msg, &mk, manifest.entries()));
}

#[test]
fn assemble_splits_records_into_adds_and_removes() {
    let (_, mk) = origin();
    let kept = signed_entry(&mk, 9_000, 0);
    let dropped_id = [7u8; 8];

    let header = DeltaHeader {
        origin_node_id: mk.to_id(),
        from_version: 1,
        to_version: 3,
        is_gateway: false,
        manifest_signature: [0u8; 64],
    };
    let msg = header
        .assemble(vec![
            LogRecord::Add {
                record_version: 2,
                entry: kept,
            },
            LogRecord::Tombstone {
                user_id: dropped_id,
                record_version: 3,
                created_ms: 1_000,
            },
        ])
        .unwrap();

    assert_eq!(msg.adds.len(), 1);
    assert_eq!(msg.adds[0].record_version, 2);
    assert_eq!(msg.adds[0].entry.user_id, kept.user_id);
    assert_eq!(msg.removes.len(), 1);
    assert_eq!(msg.removes[0].user_id, dropped_id);
    assert_eq!(msg.removes[0].record_version, 3);
}

fn dummy_adds(n: u32) -> Vec<LogRecord> {
    (0..n)
        .map(|i| LogRecord::Add {
            record_version: 1,
            entry: ManifestEntry {
                user_id: (i as u64).to_be_bytes(),
                timeout: 0,
                entry_signature: [0u8; 64],
                profile_version: 0,
            },
        })
        .collect()
}

fn dummy_header() -> DeltaHeader {
    DeltaHeader {
        origin_node_id: [1u8; 8],
        from_version: 0,
        to_version: 1,
        is_gateway: false,
        manifest_signature: [0u8; 64],
    }
}

/// §8.6: a delta is never chunked, so an oversize range must be refused
/// here and served as a full NODE_MANIFEST instead. The check runs
/// before any encoding work.
#[test]
fn an_oversized_delta_is_refused_by_the_size_guard() {
    // 88 bytes per add + 89 overhead: 700 adds clears the 60 KiB body.
    let err = dummy_header().assemble(dummy_adds(700)).unwrap_err();
    assert!(matches!(err, BuildError::WouldExceedSizeLimit));
}

/// Boundary: (61_440 - 89) / 88 = 697.16, so 697 adds fit and 698 do not.
#[test]
fn the_size_guard_admits_the_largest_delta_that_fits() {
    let largest = dummy_header().assemble(dummy_adds(697)).unwrap();
    let mut body = Vec::new();
    largest.encode(&mut body).unwrap();
    assert!(body.len() <= MAX_BODY, "697 adds encoded to {}", body.len());

    assert!(matches!(
        dummy_header().assemble(dummy_adds(698)).unwrap_err(),
        BuildError::WouldExceedSizeLimit
    ));
}

#[test]
fn a_modest_delta_body_stays_under_the_wire_cap() {
    let (kp, mk) = origin();
    let manifest = manifest_at(&mk, 2, false, 8);
    let sig = manifest.sign_state(&kp, &mk.encode()).unwrap();

    let header = DeltaHeader {
        origin_node_id: mk.to_id(),
        from_version: 1,
        to_version: 2,
        is_gateway: false,
        manifest_signature: sig,
    };
    let msg = header.assemble(adds_for(manifest.entries(), 2)).unwrap();

    let mut body = Vec::new();
    msg.encode(&mut body).unwrap();
    assert!(body.len() < MAX_BODY);
}
