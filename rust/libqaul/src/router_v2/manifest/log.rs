// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The per-origin delta log (spec §10.9).

use std::collections::BTreeMap;

use crate::router_v2::{codec::messages::ManifestEntry, seq::is_fresher_u32};

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
