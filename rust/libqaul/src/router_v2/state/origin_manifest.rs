// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! This origin's own manifest: delegation set, version bumps, persistence
//! (spec §10.1, §10.8, §10.9).

use crate::{
    router_v2::{
        codec::messages::ManifestEntry,
        identity::SelfDelegation,
        manifest::{DelegetedEntry, Manifest},
        BumpTrigger, RouterV2State,
    },
    storage::manifest_state::{DelegationEntry, HostManifestState},
};

impl RouterV2State {
    pub fn restore_host_manifest(&self, persisted: &HostManifestState) {
        let mut manifest = self.manifest.write().unwrap();
        manifest.manifest_version = persisted.manifest_version;
        manifest.set_gateway(persisted.is_gateway);
        manifest.set_entries(
            persisted
                .entries
                .iter()
                .map(|e| ManifestEntry {
                    user_id: e.user_id,
                    timeout: e.timeout,
                    entry_signature: {
                        let mut arr = [0u8; 64];
                        arr.copy_from_slice(&e.entry_signature);
                        arr
                    },
                    profile_version: e.profile_version,
                })
                .collect(),
        );

        self.own_manifest_log
            .write()
            .unwrap()
            .reset_to(persisted.manifest_version);
        self.resign_own_manifest(&mut manifest);
    }

    fn resign_own_manifest(&self, manifest: &mut Manifest) {
        let origin_multikey = self.host_mk.encode();

        manifest.manifest_signature =
            match manifest.sign_state(&self.host_keypair, &origin_multikey) {
                Ok(sig) => Some(sig),
                Err(e) => {
                    tracing::error!("router_v2: signing own manifest state failed: {e}");
                    None
                }
            };

        manifest.retained_chunks =
            match manifest.build_chunks(self.host_mk.to_id(), &self.host_keypair, &origin_multikey)
            {
                Ok(chunks) => Some(chunks),
                Err(e) => {
                    tracing::error!("router_v2: building own manifest chunks failed: {e}");
                    None
                }
            };
    }

    /// 10.3 says: a user's self-delegation in this host's manifest is recoreded
    pub fn add_self_delegation(
        &self,
        user_id: [u8; 8],
        profile_version: u32,
        delegation: SelfDelegation,
    ) -> bool {
        self.record_delegation(DelegetedEntry {
            user_id,
            timeout: delegation.timeout,
            entry_signature: delegation.entry_signature,
            profile_version,
        })
    }

    /// Puts a verified delegation entry into our own manifest, whoever
    /// signed it (§10.1). Self-delegations and accepted cross-host
    /// subscribes (§11.6) are the same thing once verified, and both ride
    /// the accumulated bump rather than forcing one.
    pub(crate) fn record_delegation(&self, entry: DelegetedEntry) -> bool {
        let user_id = entry.user_id;
        let changed = self.manifest.write().unwrap().upsert_entry(entry);

        if changed {
            self.dirty_delegations.write().unwrap().insert(user_id);
        }
        changed
    }

    /// whether this user already has an entry in our own manifest
    pub fn has_self_delegation(&self, user_id: &[u8; 8]) -> bool {
        self.manifest
            .read()
            .unwrap()
            .entries()
            .iter()
            .any(|e| e.user_id == *user_id)
    }

    /// entries that are in the refresh window and have their profile_version
    pub fn delegations_due_for_refresh(&self, now_ms: u64) -> Vec<([u8; 8], u32)> {
        let window_ms = self.options.delegation_referesh.saturating_mul(1000);
        let deadline = now_ms.saturating_add(window_ms);

        self.manifest
            .read()
            .unwrap()
            .entries()
            .iter()
            .filter(|e| e.timeout <= deadline)
            .map(|e| (e.user_id, e.profile_version))
            .collect()
    }

    pub fn remove_self_delegation(&self, user_id: &[u8; 8]) -> bool {
        let removed = self.manifest.write().unwrap().remove_entry(user_id);
        if removed {
            self.dirty_delegations.write().unwrap().insert(*user_id);
        }
        removed
    }

    pub fn try_bump_manifest_version(&self, now_ms: u64, trigger: BumpTrigger) -> Option<u32> {
        let dirty: Vec<[u8; 8]> = self
            .dirty_delegations
            .read()
            .unwrap()
            .iter()
            .copied()
            .collect();

        if trigger == BumpTrigger::Accumulated {
            if dirty.is_empty() {
                return None;
            }
            // `manifest_rate_limit` is seconds; everything else here is ms.
            let window_ms = self.options.manifest_rate_limit.saturating_mul(1000);
            let last = *self.last_manifest_bump_ms.read().unwrap();
            if now_ms < last.saturating_add(window_ms) {
                return None;
            }
        }

        let mut manifest = self.manifest.write().unwrap();
        let new_version = manifest.manifest_version.wrapping_add(1);

        {
            let mut log = self.own_manifest_log.write().unwrap();
            for user_id in &dirty {
                match manifest.entries().iter().find(|e| e.user_id == *user_id) {
                    Some(entry) => log.insert_add(new_version, entry.clone()),
                    None => log.insert_remove(*user_id, new_version, now_ms),
                }
            }
        }

        manifest.manifest_version = new_version;

        self.resign_own_manifest(&mut manifest);
        drop(manifest);

        self.dirty_delegations.write().unwrap().clear();
        *self.last_manifest_bump_ms.write().unwrap() = now_ms;

        tracing::info!(
            "router_v2: manifest_version → {new_version} ({trigger:?}, {} record(s) folded)",
            dirty.len()
        );
        Some(new_version)
    }

    pub fn host_manifest_snapshot(&self) -> HostManifestState {
        let manifest = self.manifest.read().unwrap();
        HostManifestState {
            manifest_version: manifest.manifest_version,
            is_gateway: manifest.is_gateway,
            entries: manifest
                .entries()
                .iter()
                .map(|e| DelegationEntry {
                    user_id: e.user_id,
                    timeout: e.timeout,
                    entry_signature: e.entry_signature.to_vec(),
                    profile_version: e.profile_version,
                })
                .collect(),
            last_bump_ms_reserved: None,
        }
    }
}
