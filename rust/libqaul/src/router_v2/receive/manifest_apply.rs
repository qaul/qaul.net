// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Applying received manifests (spec §8.8: NODE_MANIFEST and MANIFEST_DELTA).
//! Follows the rule in section 8.8, step 5 which states that: a completed entry set is
//! retained **byte-for-byte as signed**.

use std::sync::Arc;

use libp2p::PeerId;
use tracing::{debug, info, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::messages::{ManifestDelta, ManifestEntry, NodeManifest},
        identity::{ChunkSigningCtx, Multikey},
        index::Space,
        manifest::{canonical_entry_bytes, Manifest},
        table::{DelegatedUser, User},
        Result, RouterV2State, Sphere,
    },
};

impl RouterV2State {
    pub(crate) fn get_resource_mk(&self, resouce_id: &[u8; 8], space: Space) -> Option<Multikey> {
        match space {
            Space::Node => {
                let nodes = self.nodes.read().unwrap();
                let Some(node_arc) = nodes.get(&resouce_id) else {
                    debug!("node_manifest for unknown origin node {resouce_id:?}");
                    return None;
                };
                let node = node_arc.read().unwrap();
                node.public_key.clone()
            }
            Space::User => {
                let users = self.users.read().unwrap();
                let Some(user_arc) = users.get(&resouce_id) else {
                    debug!("user for unknown origin node {resouce_id:?}");
                    return None;
                };
                let user = user_arc.read().unwrap();
                user.public_key.clone()
            }
        }
    }

    /// spec: 8.8 steps 5-6
    pub(crate) fn refresh_delegation_trust(&self, origin_node_id: &[u8; 8], now: u64) {
        let Some(node_arc) = self.nodes.read().unwrap().get(origin_node_id) else {
            return;
        };
        let Some(host_mk) = self.get_resource_mk(origin_node_id, Space::Node) else {
            return;
        };

        let (trusted, unverifiable): (Vec<[u8; 8]>, Vec<[u8; 8]>) = {
            let node = node_arc.read().unwrap();
            let mut trusted = Vec::new();
            let mut unverifiable = Vec::new();

            for delegated in &node.delegated_users {
                // 10.4: an expired delegation is never trusted.
                if delegated.delegation_timeout <= now {
                    continue;
                }
                let Some(user_mk) = self.get_resource_mk(&delegated.user_id, Space::User) else {
                    // §8.8 step 5: the entry stays stored and servable, it
                    // simply cannot be trusted until the key arrives.
                    // Collected rather than fetched here — see below.
                    unverifiable.push(delegated.user_id);
                    continue;
                };
                let entry = ManifestEntry {
                    user_id: delegated.user_id,
                    timeout: delegated.delegation_timeout,
                    entry_signature: delegated.entry_signature,
                    profile_version: delegated.profile_version,
                };
                if Manifest::verify_entry(&entry, &host_mk, &user_mk).is_ok() {
                    trusted.push(delegated.user_id);
                }
            }
            (trusted, unverifiable)
        };

        {
            let weak_node = Arc::downgrade(&node_arc);
            let users = self.users.read().unwrap();
            for user_id in &trusted {
                let Some(user_arc) = users.get(user_id) else {
                    continue;
                };
                let mut user = user_arc.write().unwrap();
                user.delegation_gateways.retain(|w| {
                    w.upgrade()
                        .map(|n| n.read().unwrap().id != *origin_node_id)
                        .unwrap_or(false)
                });
                user.delegation_gateways.push(weak_node.clone());
            }
        }

        info!(
            "router_v2 TRUST origin={origin_node_id:?} trusted={} unverifiable={}",
            trusted.len(),
            unverifiable.len(),
        );

        // §11.5: fetch the keys we are missing, which is what lets these
        // entries become trusted on a later pass.
        //
        // Deliberately after every guard above has been dropped —
        // `request_profile` takes `users` and, through `next_hop_for_user`,
        // reads `nodes` again. Calling it inside the loop that found these
        // ids would nest those locks.
        drop(node_arc);
        for user_id in unverifiable {
            self.request_profile(user_id, false, now);
        }
    }

    fn delegated_users_from_entries(&self, entries: &[ManifestEntry]) -> Vec<DelegatedUser> {
        let mut users = self.users.write().unwrap();
        entries
            .iter()
            .map(|entry| {
                let user_arc = match users.get(&entry.user_id) {
                    Some(arc) => arc,
                    None => {
                        users.insert(
                            entry.user_id,
                            User {
                                id: entry.user_id,
                                public_key: None,
                                profile_version: entry.profile_version,
                                routing_entry: None,
                                delegation_gateways: Vec::new(),
                                is_hosted: false,
                            },
                        );
                        users.get(&entry.user_id).expect("just inserted")
                    }
                };
                DelegatedUser {
                    user_id: entry.user_id,
                    user: user_arc,
                    delegation_timeout: entry.timeout,
                    entry_signature: entry.entry_signature,
                    profile_version: entry.profile_version,
                }
            })
            .collect()
    }

    pub fn handle_node_manifest(
        &self,
        msg: NodeManifest,
        now: u64,
        transport: ConnectionModule,
    ) -> Result<()> {
        let origin_node_id = msg.origin_node_id;

        let host_mk = {
            match self.get_resource_mk(&origin_node_id, Space::Node) {
                Some(mk) => mk,
                None => {
                    debug!(
                        "node_manifest from {origin_node_id:?} deferred: origin key unknown, \
                         fetching its profile (§11.5)"
                    );
                    self.request_profile(origin_node_id, true, now);
                    return Ok(());
                }
            }
        };

        if Manifest::verify_chunk(&msg, &host_mk).is_err() {
            debug!("node_manifest chunk sig invalid for origin {origin_node_id:?}");
            return Ok(());
        };

        // §8.8 step 5, byte-exact stored discipline
        let completed_manifest = {
            let mut assembler = self.chunk_assembler.write().unwrap();
            let Some(completed) = assembler.insert(origin_node_id, msg) else {
                return Ok(());
            };
            completed
        };

        let is_gateway = (completed_manifest.flags & 0x01) != 0;
        let delegated_users = self.delegated_users_from_entries(&completed_manifest.entries);

        let nodes = self.nodes.read().unwrap();
        if let Some(node_arc) = nodes.get(&origin_node_id) {
            let mut node = node_arc.write().unwrap();
            node.manifest_version = completed_manifest.manifest_version;
            node.is_gateway = is_gateway;
            node.delegated_users = delegated_users;
            // §2.3
            node.learn_sphere = Some(Sphere::of(transport));

            // we're retaining what the origin signed exactly byte by byte
            node.manifest_signature = if completed_manifest.chunks.len() == 1 {
                Some(completed_manifest.chunks[0].manifest_signature)
            } else {
                None
            };
            node.retained_chunks = Some(completed_manifest.chunks);
            // start the log afresh with thw base at the version we just commited.
            node.manifest_log
                .reset_to(completed_manifest.manifest_version);
        }
        drop(nodes);

        self.refresh_delegation_trust(&origin_node_id, now);

        Ok(())
    }

    pub fn handle_manifest_delta(
        &self,
        neighbour: PeerId,
        msg: ManifestDelta,
        now: u64,
        transport: ConnectionModule,
    ) -> Result<()> {
        let origin_node_id = msg.origin_node_id;

        // Step 1: resolve the origin's key.
        let Some(origin_mk) = self.get_resource_mk(&origin_node_id, Space::Node) else {
            debug!(
                "manifest_delta from {origin_node_id:?} dropped: origin key unknown, \
                 fetching its profile (§11.5)"
            );
            self.request_profile(origin_node_id, true, now);
            return Ok(());
        };

        // Step 2: the delta must build on exactly what we hold.
        let Some(node_arc) = self.nodes.read().unwrap().get(&origin_node_id) else {
            debug!("manifest_delta from {origin_node_id:?} dropped: no node record");
            return Ok(());
        };
        let (committed, stored): (u32, Vec<ManifestEntry>) = {
            let node = node_arc.read().unwrap();
            let entries = node
                .delegated_users
                .iter()
                .map(|d| ManifestEntry {
                    user_id: d.user_id,
                    timeout: d.delegation_timeout,
                    entry_signature: d.entry_signature,
                    profile_version: d.profile_version,
                })
                .collect();
            (node.manifest_version, entries)
        };
        if committed != msg.from_version {
            info!(
                "manifest_delta from {origin_node_id:?} dropped: committed {committed} != from_version {}",
                msg.from_version
            );
            return Ok(());
        }

        // Step 3: build the scratch set. removes first, then adds as upserts
        let mut scratch = stored;
        for remove in &msg.removes {
            scratch.retain(|e| e.user_id != remove.user_id);
        }
        for add in &msg.adds {
            match scratch.binary_search_by(|e| e.user_id.cmp(&add.entry.user_id)) {
                Ok(i) => scratch[i] = add.entry,
                Err(i) => scratch.insert(i, add.entry),
            }
        }

        // Step 4: verify the signature over the resulting state at to_version.
        let flags = msg.flags & 0x01;
        let scratch_bytes = canonical_entry_bytes(&scratch);
        let ctx = ChunkSigningCtx {
            origin_multikey: &origin_mk.encode(),
            manifest_version: msg.to_version,
            chunk_index: 0,
            chunk_count: 1,
            flags,
            canonical_entries: &scratch_bytes,
        };
        if !origin_mk.verify(&ctx.signing_input(), &msg.manifest_signature) {
            warn!(
                "manifest_delta from {origin_node_id:?} failed resulting-state verification; discarding scratch"
            );
            self.request_full_manifest(neighbour, origin_node_id, now);
            return Ok(());
        }

        // Step 5: commit.
        let is_gateway = flags != 0;
        let delegated_users = self.delegated_users_from_entries(&scratch);
        {
            let mut node = node_arc.write().unwrap();
            node.manifest_version = msg.to_version;
            node.is_gateway = is_gateway;
            node.delegated_users = delegated_users;
            node.learn_sphere = Some(Sphere::of(transport));
            node.manifest_signature = Some(msg.manifest_signature);
            node.retained_chunks = None;

            for add in &msg.adds {
                node.manifest_log.insert_add(add.record_version, add.entry);
            }
            for remove in &msg.removes {
                node.manifest_log
                    .insert_remove(remove.user_id, remove.record_version, now);
            }
            let tombstone_ttl_ms = self.options.delegation_ttl.saturating_mul(1000);
            node.manifest_log
                .compact(now, tombstone_ttl_ms, self.options.delta_log_cap);
        }

        info!(
            "router_v2 MANIFEST_DELTA ← origin={origin_node_id:?} {} → {} (+{} -{})",
            msg.from_version,
            msg.to_version,
            msg.adds.len(),
            msg.removes.len(),
        );

        // Step 6: re-evaluate the trusted subset
        self.refresh_delegation_trust(&origin_node_id, now);

        Ok(())
    }
}
