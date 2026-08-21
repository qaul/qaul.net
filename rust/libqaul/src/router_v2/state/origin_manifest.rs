// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! This origin's own manifest: delegation set, version bumps, persistence
//! (spec §10.1, §10.8, §10.9).

use std::collections::HashSet;

use crate::{
    router_v2::{
        codec::messages::ManifestEntry,
        identity::SelfDelegation,
        manifest::{DelegetedEntry, Manifest},
        BumpTrigger, RouterV2State,
    },
    storage::manifest_state::{DelegationEntry, HostManifestState},
};

/// Grace from the first unreachable observation to removal: the §10.7
/// budget of 95 s less the 35 s route expiry already spent making the
/// loss visible.
const UNREACHABLE_GRACE_MS: u64 = 60_000;

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

    /// §10.7: stop advertising delegated users we can no longer deliver to.
    ///
    /// Returns whether the manifest changed, so the caller can persist.
    ///
    /// The §10.7 budget is 95 s: up to 35 s for route expiry to make the
    /// loss visible, then the emission window. Route expiry is already
    /// spent by the time anything is observable here, so the grace measured
    /// from observation is the remainder — waiting the full 95 s from the
    /// first unreachable tick would overshoot the bound by the detection
    /// interval.
    pub fn sweep_unreachable_delegations(&self, now_ms: u64) -> bool {
        let carried: HashSet<[u8; 8]> = self
            .manifest
            .read()
            .unwrap()
            .entries()
            .iter()
            .map(|e| e.user_id)
            .collect();

        if carried.is_empty() {
            self.delegation_liveness.write().unwrap().clear();
            return false;
        }

        let expired: Vec<[u8; 8]> = {
            let mut liveness = self.delegation_liveness.write().unwrap();
            liveness.retain(|user_id, _| carried.contains(user_id));

            carried
                .iter()
                .filter(|user_id| {
                    if self.delegated_user_is_reachable(user_id) {
                        liveness.insert(**user_id, now_ms);
                        return false;
                    }
                    // First unreachable sighting starts the clock rather
                    // than dropping outright, so a single missed tick does
                    // not evict an entry.
                    let since = *liveness.entry(**user_id).or_insert(now_ms);
                    now_ms.saturating_sub(since) > UNREACHABLE_GRACE_MS
                })
                .copied()
                .collect()
        };

        if expired.is_empty() {
            return false;
        }

        {
            let mut liveness = self.delegation_liveness.write().unwrap();
            for user_id in &expired {
                tracing::info!(
                    "router_v2 DELEGATION dropped: {user_id:?} unreachable past the §10.7 bound"
                );
                liveness.remove(user_id);
            }
        }
        for user_id in &expired {
            self.remove_self_delegation(user_id);
        }

        // §10.8 exempts this from the 60 s rate limit: a manifest that keeps
        // advertising an undeliverable user black-holes traffic addressed to
        // them, which is exactly what §10.7 exists to prevent. Flapping is
        // bounded on the other side — a dropped user only comes back by
        // subscribing again (§11.6), and that re-add rides the ordinary
        // rate-limited accumulated bump.
        self.try_bump_manifest_version(now_ms, BumpTrigger::ForcedRemoval)
            .is_some()
    }

    /// Can we still deliver to this delegated user?
    ///
    /// A user we host is reachable by definition. It also never has a
    /// routing entry — `register_hosted_user` leaves it `None`, and under
    /// node form §3.2 gives hosted users no user index at all — so testing
    /// one the same way as a foreign entry would drop our own users from
    /// our own manifest on the first tick after startup.
    fn delegated_user_is_reachable(&self, user_id: &[u8; 8]) -> bool {
        let users = self.users.read().unwrap();
        let Some(user) = users.get(user_id) else {
            return false;
        };
        let user = user.read().unwrap();

        user.is_hosted
            || user
                .routing_entry
                .as_ref()
                .is_some_and(|weak| weak.upgrade().is_some())
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
