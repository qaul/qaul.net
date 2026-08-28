// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Locally hosted users and this node's propagation form (spec §3.2, §3.5).

use tracing::error;

use crate::router_v2::{
    identity::Multikey,
    index::{Space, RESERVED_INDEX},
    manifest::ManifestLog,
    seq::is_fresher_u32,
    table::{Node, User},
    BumpTrigger, PropagationForm, RouterV2State,
};

impl RouterV2State {
    pub fn register_hosted_user(
        &self,
        user_id: [u8; 8],
        profile_version: u32,
        public_key: Multikey,
    ) {
        {
            let mut users = self.users.write().unwrap();
            match users.get(&user_id) {
                Some(existing) => {
                    let mut u = existing.write().unwrap();
                    if is_fresher_u32(profile_version, u.profile_version) {
                        u.profile_version = profile_version;
                    }
                    u.is_hosted = true;
                    u.public_key = Some(public_key);
                }
                None => users.insert(
                    user_id,
                    User {
                        id: user_id,
                        public_key: Some(public_key),
                        profile_version,
                        routing_entry: None,
                        delegation_gateways: Vec::new(),
                        is_hosted: true,
                    },
                ),
            }
        }

        // per 3.2: a propagating node names its users through the manifest
        // not routing entries.
        if self.desired_propagation_form() == PropagationForm::Node {
            tracing::info!(
                "router_v2: hosted user {user_id:?} registered without a user index (node form, §3.2)"
            );
            return;
        }

        let newly_bound = {
            let mut dict = self.user_dict.write().unwrap();
            match dict.id_of(RESERVED_INDEX) {
                Some(existing) if existing == user_id => None,
                Some(existing) => {
                    // Only hosted users are ever bound here
                    error!(
                        "user RESERVED_INDEX held by {existing:?} while registering hosted user {user_id:?}; rebinding"
                    );
                    dict.bind(RESERVED_INDEX, user_id);
                    Some(RESERVED_INDEX)
                }
                None => {
                    dict.bind(RESERVED_INDEX, user_id);
                    Some(RESERVED_INDEX)
                }
            }
        };

        if let Some(idx) = newly_bound {
            self.reintroduction_tracker
                .write()
                .unwrap()
                .mark_first_time(Space::User, idx);

            tracing::info!(
                "router_v2: hosted user {user_id:?} bound at user index {idx} (profile_version={profile_version}, reserved={})",
                idx == RESERVED_INDEX
            );
        }
    }

    /// IDs of the users this node hosts locally.
    pub fn hosted_user_ids(&self) -> Vec<[u8; 8]> {
        self.users
            .read()
            .unwrap()
            .iter()
            .filter(|(_, arc)| arc.read().unwrap().is_hosted)
            .map(|(id, _)| *id)
            .collect()
    }

    /// The form this node *should* be propagating in right now (spec §3.2).
    pub fn desired_propagation_form(&self) -> PropagationForm {
        if self.hosted_user_ids().len() > 1 {
            return PropagationForm::Node;
        }
        if self.has_active_internet_transport() {
            return PropagationForm::Node;
        }
        if self.holds_foreign_delegation() {
            return PropagationForm::Node;
        }

        PropagationForm::User
    }

    /// the last trigger er 3.2: when a node carries a manifest entry for a user it
    /// does not host.
    fn holds_foreign_delegation(&self) -> bool {
        let hosted = self.hosted_user_ids();
        self.manifest
            .read()
            .unwrap()
            .entries()
            .iter()
            .any(|entry| !hosted.contains(&entry.user_id))
    }

    /// Makes sure this node has a [`Node`] record for itself.
    fn ensure_host_node_record(&self, host_node_id: [u8; 8]) {
        let (manifest_version, is_gateway) = {
            let manifest = self.manifest.read().unwrap();
            (manifest.manifest_version, manifest.is_gateway)
        };
        let mut nodes = self.nodes.write().unwrap();
        if nodes.get(&host_node_id).is_some() {
            return;
        }
        nodes.insert(
            host_node_id,
            Node {
                id: host_node_id,
                public_key: Some(self.host_mk.clone()),
                manifest_version,
                advertised_version: 0,
                is_gateway,
                delegated_users: Vec::new(),
                manifest_signature: None,
                retained_chunks: None,
                learn_sphere: None,
                manifest_log: ManifestLog::default(),
            },
        );
    }

    pub fn sync_propagation_form(&self, now_ms: u64) -> PropagationForm {
        let desired = self.desired_propagation_form();
        let current = *self.propagation_form.read().unwrap();
        if desired == current {
            return current;
        }

        // §3.5: the reserved index of the form we are leaving is released
        match desired {
            PropagationForm::Node => {
                for user_id in self.hosted_user_ids() {
                    if let Some(idx) = self.release_index(Space::User, &user_id, now_ms) {
                        tracing::info!(
                            "router_v2: released user index {idx} for hosted user {user_id:?} (→ node form)"
                        );
                    }
                }

                let host_node_id = self.host_mk.to_id();
                self.ensure_host_node_record(host_node_id);
                self.node_dict
                    .write()
                    .unwrap()
                    .bind(RESERVED_INDEX, host_node_id);
                self.reintroduction_tracker
                    .write()
                    .unwrap()
                    .mark_first_time(Space::Node, RESERVED_INDEX);
                tracing::info!(
                    "router_v2: host node {host_node_id:?} bound at node RESERVED_INDEX (→ node form)"
                );
            }
            PropagationForm::User => {
                self.release_index(Space::Node, &self.host_mk.to_id(), now_ms);

                if let Some(user_id) = self.hosted_user_ids().first().copied() {
                    self.release_index(Space::User, &user_id, now_ms);
                    self.user_dict
                        .write()
                        .unwrap()
                        .bind(RESERVED_INDEX, user_id);
                    self.reintroduction_tracker
                        .write()
                        .unwrap()
                        .mark_first_time(Space::User, RESERVED_INDEX);
                    tracing::info!(
                        "router_v2: hosted user {user_id:?} reclaimed RESERVED_INDEX (→ user form)"
                    );
                }
            }
        }

        *self.propagation_form.write().unwrap() = desired;
        tracing::info!("router_v2: propagation form {current:?} → {desired:?} (§3.2)");

        // §10.8: the single-user↔multi-user transition is a bump trigger
        self.try_bump_manifest_version(now_ms, BumpTrigger::FormTransition);

        desired
    }

    pub fn unregister_hosted_user(&self, user_id: [u8; 8], now_ms: u64) {
        let held_reserved = self
            .user_dict
            .read()
            .unwrap()
            .idx_of(&user_id)
            .map(|idx| idx == RESERVED_INDEX)
            .unwrap_or(false);

        self.release_index(Space::User, &user_id, now_ms);
        self.users.write().unwrap().remove(&user_id);

        if held_reserved {
            if let Some(next) = self.hosted_user_ids().first().copied() {
                self.release_index(Space::User, &next, now_ms);
                self.user_dict.write().unwrap().bind(RESERVED_INDEX, next);
                self.reintroduction_tracker
                    .write()
                    .unwrap()
                    .mark_rebind(Space::User, RESERVED_INDEX);
                tracing::info!(
                    "router_v2: hosted user {next:?} promoted to RESERVED_INDEX after removal"
                );
            }
        }

        // TODO(Phase 11): drop the user from the manifest and bump
        // manifest_version (§10.5, §10.7).
        tracing::info!("router_v2: hosted user {user_id:?} unregistered");
    }
}
