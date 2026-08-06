// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Nodes manifest handling

use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

use libp2p::identity::Keypair;
use tracing::debug;

use crate::router_v2::{
    codec::messages::{
        DeltaAdd, DeltaRemove, ManifestDelta, ManifestEntry, ManifestRequestItem, NodeManifest,
    },
    identity::{delegation_signing_input, ChunkSigningCtx, Multikey},
    seq::is_fresher_u32,
    Sphere,
};

const ENTRY_BYTES: usize = 84;
const HEADER_OVERHEAD: usize = 85;
pub(crate) const MAX_BODY: usize = 60 * 1024;
// §8.6 MANIFEST_DELTA sizes.
const DELTA_ADD_BYTES: usize = 88;
const DELTA_REMOVE_BYTES: usize = 12;
const DELTA_OVERHEAD: usize = 89;

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

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("delta body would exceed the 60 KiB bound; serve a full manifest instead")]
    WouldExceedSizeLimit,
}

/// Records the most-recent transition per delegated user. Serving a
/// ranged MANIFEST_DELTA is exactly `records_after(from_version)`.
#[derive(Debug, Clone)]
pub enum LogRecord {
    Add {
        record_version: u32,
        entry: ManifestEntry,
    },
    Tombstone {
        user_id: [u8; 8],
        record_version: u32,
        created_ms: u64,
    },
}

impl LogRecord {
    pub fn record_version(&self) -> u32 {
        match self {
            LogRecord::Add { record_version, .. } => *record_version,
            LogRecord::Tombstone { record_version, .. } => *record_version,
        }
    }

    pub fn user_id(&self) -> [u8; 8] {
        match self {
            LogRecord::Add { entry, .. } => entry.user_id,
            LogRecord::Tombstone { user_id, .. } => *user_id,
        }
    }
}

/// Per-origin delta log. One instance per Node record for
/// foreign manifests; one on the host's own Manifest for our origin.
#[derive(Debug, Default, Clone)]
pub struct ManifestLog {
    /// Oldest servable version.
    pub log_base: u32,
    /// Keyed by user_id. we're storing at most one record per user.
    records: BTreeMap<[u8; 8], LogRecord>,
}

impl ManifestLog {
    /// simply upsert instead of replace
    pub fn insert_add(&mut self, record_version: u32, entry: ManifestEntry) {
        self.records.insert(
            entry.user_id,
            LogRecord::Add {
                record_version,
                entry,
            },
        );
    }

    /// instead of removing, we'll replace any prior Add for the same user that has a Tombstone.
    /// if the tombstone exists, just refresh the version+timestamp so we can track the latest event.
    pub fn insert_remove(&mut self, user_id: [u8; 8], record_version: u32, now_ms: u64) {
        self.records.insert(
            user_id,
            LogRecord::Tombstone {
                user_id,
                record_version,
                created_ms: now_ms,
            },
        );
    }

    /// get the records where record_version > from_version.
    /// in ascending order of of record_version
    pub fn records_after(&self, from_version: u32) -> Vec<LogRecord> {
        let mut recs: Vec<LogRecord> = self
            .records
            .values()
            .filter(|r| is_fresher_u32(r.record_version(), from_version))
            .cloned()
            .collect();
        recs.sort_by_key(|r| r.record_version());
        recs
    }

    /// sets the oldest observable version and drops records below or equal to it.
    pub fn set_log_base(&mut self, v: u32) {
        self.log_base = v;
        self.records
            .retain(|_, r| is_fresher_u32(r.record_version(), v));
    }

    /// this fn is governed by section 10.9 in the spec
    pub fn compact(&mut self, now_ms: u64, tombstone_ttl_ms: u64, cap: usize) {
        // first: make old tobstomes expired
        let ttl_cutoff = now_ms.saturating_sub(tombstone_ttl_ms);
        let expired: Vec<[u8; 8]> = self
            .records
            .iter()
            .filter_map(|(id, r)| match r {
                LogRecord::Tombstone { created_ms, .. } if *created_ms <= ttl_cutoff => Some(*id),
                _ => None,
            })
            .collect();

        let mut max_discarded_version = 0u32;
        let mut has_discarded = false;
        for uid in expired {
            if let Some(r) = self.records.remove(&uid) {
                if !has_discarded || is_fresher_u32(r.record_version(), max_discarded_version) {
                    max_discarded_version = r.record_version();
                }
                has_discarded = true;
            }
        }

        if self.records.len() > cap {
            let excess = self.records.len() - cap;
            let mut by_version: Vec<([u8; 8], u32)> = self
                .records
                .iter()
                .map(|(uid, r)| (*uid, r.record_version()))
                .collect();
            by_version.sort_by_key(|(_, v)| *v);
            for (uid, v) in by_version.into_iter().take(excess) {
                self.records.remove(&uid);
                if !has_discarded || is_fresher_u32(v, max_discarded_version) {
                    max_discarded_version = v;
                }
                has_discarded = true;
            }
        }

        if has_discarded && is_fresher_u32(max_discarded_version, self.log_base) {
            self.log_base = max_discarded_version;
        }
    }

    pub fn reset_to(&mut self, version: u32) {
        self.records.clear();
        self.log_base = version;
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

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
    pub manifest_signature: Option<[u8; 64]>,
    pub retained_chunks: Option<Vec<NodeManifest>>,
}

impl Manifest {
    pub fn new() -> Self {
        Manifest {
            manifest_version: 0,
            is_gateway: false,
            entries: Vec::new(),
            manifest_signature: None,
            retained_chunks: None,
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

    /// Inserts or replaces one user's delegation entry,
    /// Returns whether the stored state actually changed.
    pub fn upsert_entry(&mut self, entry: DelegetedEntry) -> bool {
        match self
            .entries
            .binary_search_by(|e| e.user_id.cmp(&entry.user_id))
        {
            Ok(i) => {
                let existing = &self.entries[i];
                if existing.timeout == entry.timeout
                    && existing.entry_signature == entry.entry_signature
                    && existing.profile_version == entry.profile_version
                {
                    return false;
                }
                self.entries[i] = entry;
                true
            }
            Err(i) => {
                self.entries.insert(i, entry);
                true
            }
        }
    }

    pub fn remove_entry(&mut self, user_id: &[u8; 8]) -> bool {
        match self.entries.binary_search_by(|e| e.user_id.cmp(user_id)) {
            Ok(i) => {
                self.entries.remove(i);
                true
            }
            Err(_) => false,
        }
    }

    pub fn bump_version(&mut self) {
        self.manifest_version = self.manifest_version.wrapping_add(1);
    }

    pub fn canonical_chunk_bytes(&self, chunk_range: Range<usize>) -> Vec<u8> {
        canonical_entry_bytes(&self.entries()[chunk_range])
    }

    /// sign the whole entry as one and it is the sig `MANIFEST_DELTA` carries.
    /// the `from_version` is absent per 8.6
    pub fn sign_state(
        &self,
        host_keys: &Keypair,
        origin_multikey: &[u8],
    ) -> Result<[u8; 64], ManifestError> {
        let entry_bytes = canonical_entry_bytes(self.entries());
        let ctx = ChunkSigningCtx {
            origin_multikey,
            manifest_version: self.manifest_version,
            chunk_index: 0,
            chunk_count: 1,
            flags: if self.is_gateway { 1 } else { 0 },
            canonical_entries: &entry_bytes,
        };
        let signature = host_keys
            .sign(&ctx.signing_input())
            .map_err(|_| ManifestError::SigningFailed)?;
        Ok(signature.try_into().expect("ed25519 signature is 64 bytes"))
    }

    pub fn build_chunks(
        &self,
        origin_node_id: [u8; 8],
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
                origin_node_id,
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

/// encodes slice entry. the caller IS responsible for ordering the user_id
pub fn canonical_entry_bytes(entries: &[ManifestEntry]) -> Vec<u8> {
    let mut res = Vec::with_capacity(ENTRY_BYTES * entries.len());
    for entry in entries {
        entry.encode(&mut res);
    }
    res
}


pub fn reconstruct_single_chunk_full(
    origin_node_id: [u8; 8],
    manifest_version: u32,
    is_gateway: bool,
    entries: Vec<ManifestEntry>,
    stored_signature: [u8; 64],
) -> NodeManifest {
    NodeManifest {
        origin_node_id,
        manifest_version,
        chunk_index: 0,
        chunk_count: 1,
        flags: if is_gateway { 1 } else { 0 },
        manifest_signature: stored_signature,
        entries,
    }
}

pub struct DeltaHeader {
    pub origin_node_id: [u8; 8],
    /// version the delta builds upon: remember, this is not signed
    pub from_version: u32,
    pub to_version: u32,
    pub is_gateway: bool,
    pub manifest_signature: [u8; 64],
}

impl DeltaHeader {
    pub fn assemble(self, records: Vec<LogRecord>) -> std::result::Result<ManifestDelta, BuildError> {
        let (n_adds, n_removes) =
            records
                .iter()
                .fold((0usize, 0usize), |(a, r), record| match record {
                    LogRecord::Add { .. } => (a + 1, r),
                    LogRecord::Tombstone { .. } => (a, r + 1),
                });
        let estimated = DELTA_ADD_BYTES * n_adds + DELTA_REMOVE_BYTES * n_removes + DELTA_OVERHEAD;
        if estimated > MAX_BODY {
            return Err(BuildError::WouldExceedSizeLimit);
        }

        let mut adds = Vec::new();
        let mut removes = Vec::new();

        for record in records {
            match record {
                LogRecord::Add {
                    record_version,
                    entry,
                } => adds.push(DeltaAdd {
                    record_version,
                    entry,
                }),
                LogRecord::Tombstone {
                    user_id,
                    record_version,
                    ..
                } => removes.push(DeltaRemove {
                    user_id,
                    record_version,
                }),
            }
        }

        Ok(ManifestDelta {
            origin_node_id: self.origin_node_id,
            from_version: self.from_version,
            to_version: self.to_version,
            flags: if self.is_gateway { 1 } else { 0 },
            manifest_signature: self.manifest_signature,
            adds,
            removes,
        })
    }
}

/// the things known about an origin to respond to it.
pub struct OriginServeState {
    pub committed: u32,
    pub log_base: u32,
    /// Sphere the manifest was learned over.
    pub learn_sphere: Option<Sphere>,
}

/// per 8.8 how a MANIFEST_REQUEST should be answered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServeDecision {
    /// Full `NODE_MANIFEST`
    /// which means the requester needs to bootstrap, that is, its base is far behind the current log
    Full,
    Delta {
        from_version: u32,
    },
    /// versions are equal
    Nothing,
    /// per 2.3: if we learnt about it over internet
    Sealed,
}

/// per 8.8: steps 2 to 5: we decide how to answer one request item
pub fn decide_serve(
    item: &ManifestRequestItem,
    origin: &OriginServeState,
    requester_sphere: Sphere,
) -> ServeDecision {
    if origin.learn_sphere == Some(Sphere::Internet) && requester_sphere == Sphere::Local {
        return ServeDecision::Sealed;
    }

    if item.have_none() {
        return ServeDecision::Full;
    }

    if !is_fresher_u32(item.have_version, origin.log_base) {
        return ServeDecision::Full;
    }

    if item.have_version == origin.committed {
        return ServeDecision::Nothing;
    }

    if is_fresher_u32(origin.committed, item.have_version) {
        return ServeDecision::Delta {
            from_version: item.have_version,
        };
    }

    // anything else we cannot decipher, sending back full manifest
    // is the best decision, i think
    ServeDecision::Full
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
            profile_version: 0,
        }
    }

    fn synthetic_chunk(
        manifest_version: u32,
        chunk_index: u8,
        chunk_count: u8,
        entries: Vec<ManifestEntry>,
    ) -> NodeManifest {
        NodeManifest {
            origin_node_id: [0; 8],
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
            profile_version: 0,
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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
            .build_chunks(host_mk.to_id(), &host_kp, &host_mk.encode())
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

    // ---------- ManifestLog (Phase 10b subtask 1) ----------

    fn entry_at(user_id_byte: u8, timeout: u64) -> ManifestEntry {
        ManifestEntry {
            user_id: [user_id_byte; 8],
            timeout,
            entry_signature: [0; 64],
            profile_version: 0,
        }
    }

    /// `insert_add` upserts by user_id — later add replaces earlier.
    #[test]
    fn insert_add_replaces_prior_add_for_same_user() {
        let mut log = ManifestLog::default();
        log.insert_add(1, entry_at(7, 100));
        log.insert_add(5, entry_at(7, 999));
        assert_eq!(log.len(), 1);
        let recs = log.records_after(0);
        match &recs[0] {
            LogRecord::Add {
                record_version,
                entry,
            } => {
                assert_eq!(*record_version, 5);
                assert_eq!(entry.timeout, 999);
            }
            other => panic!("expected Add, got {other:?}"),
        }
    }

    /// `insert_add` after a tombstone for the same user erases the
    /// tombstone (re-add collapses history to just the Add).
    #[test]
    fn insert_add_replaces_prior_tombstone_for_same_user() {
        let mut log = ManifestLog::default();
        log.insert_remove([7; 8], 3, 0);
        log.insert_add(5, entry_at(7, 42));
        assert_eq!(log.len(), 1);
        let recs = log.records_after(0);
        assert!(matches!(
            recs[0],
            LogRecord::Add {
                record_version: 5,
                ..
            }
        ));
    }

    /// `insert_remove` after an Add collapses to a Tombstone at the
    /// remove's version — the earlier Add is gone.
    #[test]
    fn insert_remove_collapses_prior_add() {
        let mut log = ManifestLog::default();
        log.insert_add(2, entry_at(9, 100));
        log.insert_remove([9; 8], 4, 12_345);
        assert_eq!(log.len(), 1);
        let recs = log.records_after(0);
        match &recs[0] {
            LogRecord::Tombstone {
                record_version,
                created_ms,
                user_id,
            } => {
                assert_eq!(*record_version, 4);
                assert_eq!(*created_ms, 12_345);
                assert_eq!(*user_id, [9; 8]);
            }
            other => panic!("expected Tombstone, got {other:?}"),
        }
    }

    /// A second `insert_remove` for the same user refreshes version +
    /// timestamp so retention age tracks the latest event.
    #[test]
    fn insert_remove_refreshes_existing_tombstone() {
        let mut log = ManifestLog::default();
        log.insert_remove([1; 8], 3, 100);
        log.insert_remove([1; 8], 8, 500);
        let recs = log.records_after(0);
        assert_eq!(recs.len(), 1);
        match &recs[0] {
            LogRecord::Tombstone {
                record_version,
                created_ms,
                ..
            } => {
                assert_eq!(*record_version, 8);
                assert_eq!(*created_ms, 500);
            }
            other => panic!("expected Tombstone, got {other:?}"),
        }
    }

    /// `records_after(v)` is a strict-greater filter — records at
    /// exactly `v` are NOT included (the caller already holds V, so
    /// records AT V shouldn't ride the delta).
    #[test]
    fn records_after_excludes_boundary_record() {
        let mut log = ManifestLog::default();
        log.insert_add(5, entry_at(1, 0));
        log.insert_add(6, entry_at(2, 0));
        let out = log.records_after(5);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].user_id(), [2; 8]);
    }

    /// `records_after` returns records sorted ascending by
    /// `record_version` — this is the wire order the receiver applies
    /// against their scratch set (§8.6).
    #[test]
    fn records_after_returns_sorted_by_record_version() {
        let mut log = ManifestLog::default();
        // Insert out of version order (by different user_ids so nothing collapses).
        log.insert_add(10, entry_at(3, 0));
        log.insert_add(2, entry_at(1, 0));
        log.insert_add(7, entry_at(2, 0));
        let out = log.records_after(0);
        let versions: Vec<u32> = out.iter().map(|r| r.record_version()).collect();
        assert_eq!(versions, vec![2, 7, 10]);
    }

    /// Empty log → empty vec. Records-below-`from_version` → empty vec.
    #[test]
    fn records_after_empty_and_all_below() {
        let mut log = ManifestLog::default();
        assert!(log.records_after(0).is_empty());

        log.insert_add(3, entry_at(1, 0));
        log.insert_add(5, entry_at(2, 0));
        assert!(
            log.records_after(100).is_empty(),
            "all records below from_version"
        );
    }

    /// Under circular arithmetic, `is_fresher_u32(new, old)` treats
    /// wrap correctly (§6.2 scaled to 32 bits). A record at version 1
    /// IS fresher than a from_version of u32::MAX (wrap gives distance 2).
    #[test]
    fn records_after_handles_wrap_around_versions() {
        let mut log = ManifestLog::default();
        log.insert_add(1, entry_at(9, 0));
        let out = log.records_after(u32::MAX);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].record_version(), 1);
    }

    /// `set_log_base(v)` drops records at or below v (strict-greater
    /// retention) and sets `log_base` for serve-side "old requester →
    /// full manifest" fallback.
    #[test]
    fn set_log_base_drops_at_or_below_and_sets_field() {
        let mut log = ManifestLog::default();
        log.insert_add(1, entry_at(1, 0));
        log.insert_add(2, entry_at(2, 0));
        log.insert_add(3, entry_at(3, 0));
        log.insert_add(4, entry_at(4, 0));

        log.set_log_base(2);

        assert_eq!(log.log_base, 2);
        let out = log.records_after(0);
        let versions: Vec<u32> = out.iter().map(|r| r.record_version()).collect();
        assert_eq!(
            versions,
            vec![3, 4],
            "records at or below log_base must be gone"
        );
    }

    /// `compact` drops tombstones older than TTL, leaves adds alone
    /// (adds have their own expiry via `entry.timeout`), and advances
    /// `log_base` past the highest discarded version.
    #[test]
    fn compact_drops_expired_tombstones_and_advances_log_base() {
        let mut log = ManifestLog::default();
        log.insert_add(1, entry_at(1, 0)); // add — untouched by TTL
        log.insert_remove([2; 8], 3, 100); // old tombstone
        log.insert_remove([3; 8], 7, 5_000); // fresh tombstone
        log.insert_add(9, entry_at(4, 0)); // add — untouched

        // now_ms=1_000, ttl=500 → cutoff=500 → tombstone at t=100 expires,
        // tombstone at t=5_000 stays.
        log.compact(1_000, 500, 100);

        let versions: Vec<u32> = log
            .records_after(0)
            .iter()
            .map(|r| r.record_version())
            .collect();
        assert_eq!(
            versions,
            vec![1, 7, 9],
            "one tombstone expired, adds untouched"
        );
        assert_eq!(
            log.log_base, 3,
            "log_base advances past highest discarded version"
        );
    }

    /// `compact` enforces the cap by dropping lowest-version records.
    /// Advances `log_base` to the highest dropped version so the serve
    /// path knows to answer with a full manifest for old requesters.
    #[test]
    fn compact_cap_drops_lowest_versions_first() {
        let mut log = ManifestLog::default();
        log.insert_add(1, entry_at(1, 100));
        log.insert_add(2, entry_at(2, 100));
        log.insert_add(3, entry_at(3, 100));
        log.insert_add(4, entry_at(4, 100));
        log.insert_add(5, entry_at(5, 100));

        // now_ms far in future, huge TTL → no tombstone expiry path
        // triggers. Cap of 2 forces 3 drops.
        log.compact(u64::MAX, u64::MAX, 2);

        let versions: Vec<u32> = log
            .records_after(0)
            .iter()
            .map(|r| r.record_version())
            .collect();
        assert_eq!(versions, vec![4, 5], "lowest 3 versions dropped");
        assert_eq!(
            log.log_base, 3,
            "log_base advances to highest dropped (version 3)"
        );
    }

    /// Both retention rules fire in one call — TTL first, cap second.
    /// The cap operates on the post-TTL set, so the numbers compose
    /// deterministically.
    #[test]
    fn compact_ttl_then_cap_in_one_pass() {
        let mut log = ManifestLog::default();
        // Two old tombstones + three adds.
        log.insert_remove([1; 8], 1, 0);
        log.insert_remove([2; 8], 2, 0);
        log.insert_add(3, entry_at(3, 0));
        log.insert_add(4, entry_at(4, 0));
        log.insert_add(5, entry_at(5, 0));

        // TTL drops the two tombstones (versions 1, 2); cap=2 then
        // drops the lowest add (version 3).
        log.compact(10_000, 1_000, 2);

        let versions: Vec<u32> = log
            .records_after(0)
            .iter()
            .map(|r| r.record_version())
            .collect();
        assert_eq!(versions, vec![4, 5]);
        assert_eq!(log.log_base, 3, "highest dropped version was 3");
    }

    /// `compact` with nothing to do → log unchanged, `log_base` unchanged.
    #[test]
    fn compact_noop_when_nothing_expires_or_overflows() {
        let mut log = ManifestLog::default();
        log.log_base = 42;
        log.insert_add(50, entry_at(1, 0));
        log.insert_add(60, entry_at(2, 0));

        log.compact(u64::MAX, u64::MAX, 100); // huge TTL, huge cap

        assert_eq!(
            log.log_base, 42,
            "log_base must not move when nothing dropped"
        );
        assert_eq!(log.len(), 2);
    }

    /// `reset_to(v)` clears every record and sets `log_base = v`.
    /// Called from `handle_node_manifest` on full-commit per §10.9
    /// "history before a full sync is unknown".
    #[test]
    fn reset_to_clears_records_and_sets_base() {
        let mut log = ManifestLog::default();
        log.insert_add(1, entry_at(1, 0));
        log.insert_add(2, entry_at(2, 0));
        log.insert_remove([3; 8], 3, 100);

        log.reset_to(99);

        assert_eq!(log.log_base, 99);
        assert!(log.is_empty(), "reset must clear all records");
        assert!(log.records_after(0).is_empty());
    }

    /// `LogRecord::user_id()` returns the right id regardless of variant.
    #[test]
    fn log_record_user_id_accessor() {
        let add = LogRecord::Add {
            record_version: 5,
            entry: entry_at(7, 0),
        };
        assert_eq!(add.user_id(), [7; 8]);

        let tombstone = LogRecord::Tombstone {
            user_id: [9; 8],
            record_version: 10,
            created_ms: 0,
        };
        assert_eq!(tombstone.user_id(), [9; 8]);
    }
}
