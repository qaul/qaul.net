// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Nodes manifest handling

#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("failed to sign the input")]
    SigningFailed,
    #[error("too many chunks to add to this batch")]
    TooManyChunks,
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("signature does not verify")]
    SignatureInvalid,
}

use std::{collections::HashMap, ops::Range};

use libp2p::identity::Keypair;
use tracing::debug;

use crate::router_v2::{
    codec::messages::{ManifestEntry, NodeManifest},
    identity::{delegation_signing_input, ChunkSigningCtx, Multikey},
    RouterV2State, Sphere,
};

const ENTRY_BYTES: usize = 80;
const HEADER_OVERHEAD: usize = 79;
const MAX_BODY: usize = 60 * 1024;

// DelegatedEntry is for the host-side manifest while ManifestEntry
// is for the wire codec. Since they have the same fields, we cna repurpose
pub type DelegetedEntry = ManifestEntry;

/// A node's manifest
#[derive(Debug)]
pub struct Manifest {
    pub manifest_version: u32,
    pub is_gateway: bool,
    // we have to keep it in ascending order by the user_id
    // any method that touches this must resort it in ascending orfer
    entries: Vec<DelegetedEntry>,
}

impl Manifest {
    pub fn new() -> Self {
        Manifest {
            manifest_version: 0,
            is_gateway: false,
            entries: Vec::new(),
        }
    }

    pub fn entries(&self) -> &[DelegetedEntry] {
        &self.entries
    }

    pub fn set_entries(&mut self, mut entries: Vec<DelegetedEntry>) {
        entries.sort_by(|a, b| a.user_id.cmp(&b.user_id));
        self.entries = entries;
    }

    pub fn set_gateway(&mut self, is_gateway: bool) {
        self.is_gateway = is_gateway;
    }

    pub fn bump_version(&mut self) {
        self.manifest_version = self.manifest_version.wrapping_add(1);
    }

    pub fn canonical_chunk_bytes(&self, chunk_range: Range<usize>) -> Vec<u8> {
        let slice = &self.entries()[chunk_range];
        let mut res = Vec::with_capacity(80 * slice.len());
        for entry in slice {
            entry.encode(&mut res);
        }
        res
    }

    pub fn build_chunks(
        &self,
        origin_node_idx: u16,
        host_keys: &Keypair,
        multikey: &[u8],
    ) -> Result<Vec<NodeManifest>, ManifestError> {
        let mut node_manifests = Vec::new();

        let total_entries = self.entries().len();
        let max_entry_per_chunk = (MAX_BODY - HEADER_OVERHEAD) / ENTRY_BYTES;

        let chunk_count = if total_entries == 0 {
            1
        } else {
            total_entries.div_ceil(max_entry_per_chunk)
        };
        if chunk_count > 256 {
            return Err(ManifestError::TooManyChunks);
        }

        let flags: u8 = if self.is_gateway { 1 } else { 0 };

        for chunk_idx in 0..chunk_count {
            let start = chunk_idx * max_entry_per_chunk;
            let end = (start + max_entry_per_chunk).min(total_entries);

            let chunk_bytes = self.canonical_chunk_bytes(start..end);
            let chunk_ctx = ChunkSigningCtx {
                origin_multikey: multikey,
                manifest_version: self.manifest_version,
                chunk_index: chunk_idx as u8,
                chunk_count: chunk_count as u8,
                flags,
                canonical_entries: &chunk_bytes,
            };

            let signing_input = chunk_ctx.signing_input();
            let signature = host_keys
                .sign(&signing_input)
                .map_err(|_| ManifestError::SigningFailed)?;
            let signature: [u8; 64] = signature.try_into().expect("ed25519 signature is 64 bytes");

            let entry_slice = self.entries()[start..end].to_vec();
            let nm = NodeManifest {
                origin_node_index: origin_node_idx,
                manifest_version: self.manifest_version,
                chunk_index: chunk_idx as u8,
                chunk_count: chunk_count as u8,
                flags,
                manifest_signature: signature,
                entries: entry_slice,
            };
            node_manifests.push(nm);
        }

        Ok(node_manifests)
    }

    /// at receive time, verify the received chunks
    pub fn verify_chunk(msg: &NodeManifest, multikey: &Multikey) -> Result<(), VerifyError> {
        let mut res = Vec::with_capacity(80 * msg.entries.len());
        for e in &msg.entries {
            e.encode(&mut res);
        }

        let sign_ctx = ChunkSigningCtx {
            origin_multikey: &multikey.encode(),
            manifest_version: msg.manifest_version,
            chunk_index: msg.chunk_index,
            chunk_count: msg.chunk_count,
            flags: msg.flags,
            canonical_entries: &res,
        };

        let input_sig = sign_ctx.signing_input();
        let verified = multikey.verify(&input_sig, &msg.manifest_signature);

        if verified {
            Ok(())
        } else {
            Err(VerifyError::SignatureInvalid)
        }
    }

    pub fn verify_entry(
        entry: &ManifestEntry,
        host_mk: &Multikey,
        user_mk: &Multikey,
    ) -> Result<(), VerifyError> {
        let signing_input = delegation_signing_input(&host_mk.encode(), entry.timeout);
        if user_mk.verify(&signing_input, &entry.entry_signature) {
            Ok(())
        } else {
            Err(VerifyError::SignatureInvalid)
        }
    }
}

/// a manifest that was assembled successfully from bytes over the wire
pub struct CompletedManifest {
    pub manifest_version: u32,
    pub flags: u8,
    pub entries: Vec<ManifestEntry>,
    pub chunks: Vec<NodeManifest>,
}

pub struct ChunkAssembler {
    partials: HashMap<[u8; 8], PartialManifest>,
}

struct PartialManifest {
    manifest_version: u32,
    chunk_count: u8,
    flags: u8,
    /// chunk_index → its entries.
    received: HashMap<u8, NodeManifest>,
}

impl ChunkAssembler {
    pub fn new() -> Self {
        Self {
            partials: HashMap::new(),
        }
    }

    /// Insert a verified chunk. Returns Some when the manifest is now
    /// complete; None while still accumulating.
    ///
    /// Callers must verify each chunk (see `verify_chunk`) before
    /// insertion — the assembler assumes signatures are already checked.
    pub fn insert(
        &mut self,
        origin_node_id: [u8; 8],
        chunk: NodeManifest,
    ) -> Option<CompletedManifest> {
        if chunk.chunk_index >= chunk.chunk_count {
            debug!("chunk index exceeds chunk_count");
            return None;
        }

        enum Action {
            Insert,
            Replace,
            Reset,
        }

        let action = match self.partials.get(&origin_node_id) {
            Some(partial) if partial.manifest_version == chunk.manifest_version => {
                if chunk.chunk_count != partial.chunk_count || chunk.flags != partial.flags {
                    Action::Reset
                } else {
                    Action::Insert
                }
            }
            _ => Action::Replace,
        };

        match action {
            Action::Insert => {
                self.partials
                    .get_mut(&origin_node_id)
                    .unwrap()
                    .received
                    .insert(chunk.chunk_index, chunk);
            }
            Action::Replace => {
                let manifest_version = chunk.manifest_version;
                let chunk_count = chunk.chunk_count;
                let flags = chunk.flags;
                let chunk_index = chunk.chunk_index;

                let mut received = HashMap::new();
                received.insert(chunk_index, chunk);

                self.partials.insert(
                    origin_node_id,
                    PartialManifest {
                        manifest_version,
                        chunk_count,
                        flags,
                        received,
                    },
                );
            }
            Action::Reset => {
                debug!("chunk set inconsistent; dropping partial for {origin_node_id:?}");
                self.partials.remove(&origin_node_id);
                return None;
            }
        }

        let partial = self.partials.get_mut(&origin_node_id)?;
        if (partial.received.len() as u8) < partial.chunk_count {
            return None;
        }

        let manifest_version = partial.manifest_version;
        let flags = partial.flags;
        let chunk_count = partial.chunk_count;

        let mut entries: Vec<ManifestEntry> = Vec::new();
        let mut chunks: Vec<NodeManifest> = Vec::new();

        for i in 0..chunk_count {
            if let Some(chunk) = partial.received.remove(&i) {
                entries.extend(chunk.entries.iter().cloned());
                chunks.push(chunk);
            }
        }

        self.partials.remove(&origin_node_id);

        Some(CompletedManifest {
            manifest_version,
            flags,
            entries,
            chunks,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    fn keypair_and_multikey() -> (Keypair, Multikey) {
        let kp = Keypair::generate_ed25519();
        let mk = Multikey::from(kp.public());
        (kp, mk)
    }

    /// Sign a delegation entry as `user_kp` for delegation-to `host_mk`.
    fn sign_entry(
        user_kp: &Keypair,
        host_mk: &Multikey,
        user_id: [u8; 8],
        timeout: u64,
    ) -> ManifestEntry {
        let signing_input = delegation_signing_input(&host_mk.encode(), timeout);
        let sig_bytes = user_kp.sign(&signing_input).unwrap();
        let entry_signature: [u8; 64] = sig_bytes.try_into().unwrap();
        ManifestEntry {
            user_id,
            timeout,
            entry_signature,
        }
    }

    fn synthetic_chunk(
        manifest_version: u32,
        chunk_index: u8,
        chunk_count: u8,
        entries: Vec<ManifestEntry>,
    ) -> NodeManifest {
        NodeManifest {
            origin_node_index: 0,
            manifest_version,
            chunk_index,
            chunk_count,
            flags: 0,
            manifest_signature: [0; 64],
            entries,
        }
    }

    fn dummy_entry(user_id_byte: u8) -> ManifestEntry {
        ManifestEntry {
            user_id: [user_id_byte; 8],
            timeout: 0,
            entry_signature: [0; 64],
        }
    }

    #[test]
    fn set_entries_sorts_out_of_order_input_by_user_id() {
        let mut manifest = Manifest::new();
        manifest.set_entries(vec![dummy_entry(3), dummy_entry(1), dummy_entry(2)]);
        let ids: Vec<[u8; 8]> = manifest.entries().iter().map(|e| e.user_id).collect();
        assert_eq!(ids, vec![[1; 8], [2; 8], [3; 8]]);
    }

    #[test]
    fn bump_version_wraps() {
        let mut manifest = Manifest::new();
        manifest.manifest_version = u32::MAX;
        manifest.bump_version();
        assert_eq!(manifest.manifest_version, 0);
    }

    // ---------- build_chunks ----------

    #[test]
    fn empty_manifest_produces_one_empty_chunk() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let manifest = Manifest::new();
        let chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].entries.is_empty());
        assert_eq!(chunks[0].chunk_index, 0);
        assert_eq!(chunks[0].chunk_count, 1);
    }

    #[test]
    fn small_manifest_produces_single_chunk() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, _) = keypair_and_multikey();
        let entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);

        let mut manifest = Manifest::new();
        manifest.set_entries(vec![entry]);

        let chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].entries.len(), 1);
        assert_eq!(chunks[0].chunk_index, 0);
        assert_eq!(chunks[0].chunk_count, 1);
    }

    #[test]
    fn is_gateway_flag_reflected_in_chunk_flags() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let mut manifest = Manifest::new();
        manifest.set_gateway(true);
        let chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();
        assert_eq!(chunks[0].flags, 1);
    }

    // ---------- round-trip verify ----------

    #[test]
    fn round_trip_verify_chunk_on_signed_manifest() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, _) = keypair_and_multikey();
        let entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);

        let mut manifest = Manifest::new();
        manifest.set_entries(vec![entry]);
        let chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();

        assert!(Manifest::verify_chunk(&chunks[0], &host_mk).is_ok());
    }

    #[test]
    fn round_trip_verify_entry_on_signed_delegation() {
        let (_host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, user_mk) = keypair_and_multikey();
        let entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);

        assert!(Manifest::verify_entry(&entry, &host_mk, &user_mk).is_ok());
    }

    // ---------- tamper detection ----------

    #[test]
    fn tampered_manifest_signature_fails_verify_chunk() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, _) = keypair_and_multikey();
        let entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);

        let mut manifest = Manifest::new();
        manifest.set_entries(vec![entry]);
        let mut chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();
        chunks[0].manifest_signature[0] ^= 0xFF;

        assert!(matches!(
            Manifest::verify_chunk(&chunks[0], &host_mk),
            Err(VerifyError::SignatureInvalid)
        ));
    }

    /// The "bug-2 regression test" from the plan: tampering with the
    /// `flags` byte must invalidate the whole-chunk signature.
    #[test]
    fn tampered_flags_fails_verify_chunk() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let mut manifest = Manifest::new();
        manifest.set_gateway(true);
        let mut chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();
        // Signed with flags=1; tamper to flags=0.
        chunks[0].flags = 0;
        assert!(matches!(
            Manifest::verify_chunk(&chunks[0], &host_mk),
            Err(VerifyError::SignatureInvalid)
        ));
    }

    #[test]
    fn tampered_manifest_version_fails_verify_chunk() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let manifest = Manifest::new();
        let mut chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();
        chunks[0].manifest_version = chunks[0].manifest_version.wrapping_add(1);
        assert!(matches!(
            Manifest::verify_chunk(&chunks[0], &host_mk),
            Err(VerifyError::SignatureInvalid)
        ));
    }

    #[test]
    fn tampered_entry_signature_fails_verify_entry() {
        let (_host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, user_mk) = keypair_and_multikey();
        let mut entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);
        entry.entry_signature[0] ^= 0xFF;

        assert!(matches!(
            Manifest::verify_entry(&entry, &host_mk, &user_mk),
            Err(VerifyError::SignatureInvalid)
        ));
    }

    #[test]
    fn tampered_entry_timeout_fails_verify_entry() {
        let (_host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, user_mk) = keypair_and_multikey();
        let mut entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);
        entry.timeout = 2_000;
        assert!(matches!(
            Manifest::verify_entry(&entry, &host_mk, &user_mk),
            Err(VerifyError::SignatureInvalid)
        ));
    }

    #[test]
    fn wrong_host_key_fails_verify_chunk() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let (_, wrong_mk) = keypair_and_multikey();
        let manifest = Manifest::new();
        let chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();
        assert!(matches!(
            Manifest::verify_chunk(&chunks[0], &wrong_mk),
            Err(VerifyError::SignatureInvalid)
        ));
    }

    #[test]
    fn wrong_user_key_fails_verify_entry() {
        let (_host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, _) = keypair_and_multikey();
        let (_, wrong_user_mk) = keypair_and_multikey();
        let entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);
        assert!(matches!(
            Manifest::verify_entry(&entry, &host_mk, &wrong_user_mk),
            Err(VerifyError::SignatureInvalid)
        ));
    }

    // ---------- ChunkAssembler ----------

    #[test]
    fn assembler_single_chunk_completes_immediately() {
        let mut assembler = ChunkAssembler::new();
        let entry = dummy_entry(7);
        let chunk = synthetic_chunk(1, 0, 1, vec![entry]);
        let completed = assembler.insert([1; 8], chunk).expect("completes");
        assert_eq!(completed.manifest_version, 1);
        assert_eq!(completed.entries.len(), 1);
    }

    #[test]
    fn assembler_partial_state_returns_none() {
        let mut assembler = ChunkAssembler::new();
        let chunk = synthetic_chunk(1, 0, 3, vec![dummy_entry(1)]);
        assert!(assembler.insert([1; 8], chunk).is_none());
    }

    #[test]
    fn assembler_out_of_order_chunks_still_complete() {
        let mut assembler = ChunkAssembler::new();
        let origin = [1; 8];

        // chunk_count = 3. Submit in order 2, 0, 1.
        let out_of_order = [
            (2, dummy_entry(30)),
            (0, dummy_entry(10)),
            (1, dummy_entry(20)),
        ];
        let mut completed_at = None;
        for (i, (chunk_index, entry)) in out_of_order.iter().enumerate() {
            let chunk = synthetic_chunk(1, *chunk_index, 3, vec![*entry]);
            if let Some(c) = assembler.insert(origin, chunk) {
                completed_at = Some((i, c));
            }
        }

        let (idx, completed) = completed_at.expect("must complete after last insert");
        assert_eq!(idx, 2, "completes only after the third insertion");
        // Reassembled entries follow chunk_index order (0, 1, 2), which maps
        // to canonical order because sender put them in that order per chunk.
        let user_ids: Vec<u8> = completed.entries.iter().map(|e| e.user_id[0]).collect();
        assert_eq!(user_ids, vec![10, 20, 30]);
    }

    #[test]
    fn assembler_version_change_drops_old_partial_and_starts_fresh() {
        let mut assembler = ChunkAssembler::new();
        let origin = [1; 8];

        // Start collecting v=1 (2 chunks total, only 1 arrives).
        assembler.insert(origin, synthetic_chunk(1, 0, 2, vec![dummy_entry(1)]));

        // v=2 arrives as a single-chunk manifest. Old partial is replaced.
        let completed = assembler
            .insert(origin, synthetic_chunk(2, 0, 1, vec![dummy_entry(2)]))
            .expect("v=2 single-chunk completes immediately");
        assert_eq!(completed.manifest_version, 2);
        assert_eq!(completed.entries[0].user_id, [2; 8]);
    }

    #[test]
    fn assembler_out_of_range_chunk_index_dropped() {
        let mut assembler = ChunkAssembler::new();
        // chunk_index=5 with chunk_count=3 → malformed.
        let chunk = synthetic_chunk(1, 5, 3, vec![]);
        assert!(assembler.insert([1; 8], chunk).is_none());
    }

    /// If two chunks for the same (origin, version) disagree on
    /// chunk_count or flags, the partial is reset.
    #[test]
    fn assembler_inconsistent_chunk_count_resets_partial() {
        let mut assembler = ChunkAssembler::new();
        let origin = [1; 8];

        // First chunk says chunk_count = 3.
        assembler.insert(origin, synthetic_chunk(1, 0, 3, vec![dummy_entry(1)]));

        // Second chunk (same version) says chunk_count = 5 — inconsistent.
        // Assembler drops the partial and returns None.
        assert!(assembler
            .insert(origin, synthetic_chunk(1, 1, 5, vec![dummy_entry(2)]))
            .is_none());

        // A fresh (chunk_index=0, chunk_count=1) for v=1 should start clean.
        let completed = assembler
            .insert(origin, synthetic_chunk(1, 0, 1, vec![dummy_entry(3)]))
            .expect("fresh single-chunk after reset completes");
        assert_eq!(completed.entries[0].user_id, [3; 8]);
    }

    /// End-to-end: build → verify → assemble on a single-chunk signed
    /// manifest with a real delegation entry.
    #[test]
    fn end_to_end_build_verify_assemble() {
        let (host_kp, host_mk) = keypair_and_multikey();
        let (user_kp, user_mk) = keypair_and_multikey();
        let entry = sign_entry(&user_kp, &host_mk, [7; 8], 1_000);

        let mut manifest = Manifest::new();
        manifest.set_entries(vec![entry]);
        let chunks = manifest
            .build_chunks(0, &host_kp, &host_mk.encode())
            .unwrap();

        // Verify the chunk signature.
        assert!(Manifest::verify_chunk(&chunks[0], &host_mk).is_ok());
        // Verify the per-entry signature.
        assert!(Manifest::verify_entry(&chunks[0].entries[0], &host_mk, &user_mk).is_ok());

        // Assemble.
        let mut assembler = ChunkAssembler::new();
        let completed = assembler
            .insert(host_mk.to_id(), chunks.into_iter().next().unwrap())
            .expect("completes");
        assert_eq!(completed.entries.len(), 1);
        assert_eq!(completed.entries[0].user_id, [7; 8]);
    }
}

pub fn enqueue_full_manifest(
    state: &RouterV2State,
    now: u64,
    bypass_rate_limit: bool,
) -> Result<(), ManifestError> {
    let mut manifest = state.manifest.write().unwrap();
    if !manifest.is_gateway && manifest.entries().is_empty() {
        return Ok(());
    }

    let mut last_manifest_emission_ms = state.last_manifest_emission_ms.write().unwrap();
    if !bypass_rate_limit && now.saturating_sub(*last_manifest_emission_ms) < 60_000 {
        return Ok(());
    }

    manifest.bump_version();
    let chunks = manifest.build_chunks(0, &state.host_keypair, &state.host_mk.encode())?;
    
    let mut manifest_relay = state.manifest_relay_queue.write().unwrap();
    manifest_relay.insert(state.host_mk.to_id(), (chunks, Sphere::Local));
    
    *last_manifest_emission_ms = now;
    Ok(())
}
