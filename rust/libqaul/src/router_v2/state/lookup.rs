// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Routing lookups over the committed table (spec §8.3, §9.2).

use crate::{
    connections::ConnectionModule,
    router_v2::{index, index::Space, RouterV2State},
};

impl RouterV2State {
    /// do we know about this ID?
    pub(crate) fn is_local_identity(&self, id: [u8; 8], is_node: bool) -> bool {
        if is_node {
            id == self.host_mk.to_id()
        } else {
            self.users
                .read()
                .unwrap()
                .get(&id)
                .is_some_and(|u| u.read().unwrap().is_hosted)
        }
    }

    pub(crate) fn mark_profile_version_bump(&self, user_id: &[u8; 8]) {
        let Some(idx) = self.user_dict.read().unwrap().idx_of(user_id) else {
            return;
        };
        self.reintroduction_tracker
            .write()
            .unwrap()
            .mark_version_bump(Space::User, idx);
        tracing::debug!(
            "router_v2: user {user_id:?} idx {idx} marked for re-introduction (§3.8 trigger 3)"
        );
    }

    /// The `profile_version` we advertise for a user we host (§3.4).
    pub fn hosted_profile_version(&self, user_id: &[u8; 8]) -> u32 {
        self.hosted_profiles
            .read()
            .unwrap()
            .get(user_id)
            .map(|hosted| hosted.profile.version)
            .unwrap_or(0)
    }

    pub fn next_hop_node_id(&self, next_hop: u16) -> Option<[u8; 8]> {
        let node_entries = &self.node_dict.read().unwrap();
        node_entries.id_of(next_hop)
    }

    /// Resolves a node to a next hop (§9.2)
    pub fn next_hop_for_node(&self, target: [u8; 8]) -> Option<([u8; 8], ConnectionModule)> {
        if let Some(transport) = self.neighbour_transport(&target) {
            return Some((target, transport));
        }

        let idx = self.node_dict.read().unwrap().idx_of(&target)?;
        let entry = self.routing_table.read().unwrap().get(Space::Node, idx)?;
        let entry = entry.read().unwrap();
        let next_hop = self.next_hop_node_id(entry.next_hop)?;
        Some((next_hop, entry.transport))
    }

    pub fn next_hop_for_user(&self, recipient: [u8; 8]) -> Option<([u8; 8], ConnectionModule)> {
        let users = self.users.read().unwrap();
        if let Some(user) = users.get(&recipient) {
            let user = user.read().unwrap();

            // we try to get the direct routing entry, if it fails, then we checck the gateways
            if let Some(weak) = &user.routing_entry {
                if let Some(entry) = weak.upgrade() {
                    let e = entry.read().unwrap();
                    if let Some(id) = self.next_hop_node_id(e.next_hop) {
                        return Some((id, e.transport));
                    }
                }
            }

            // check the delegation gateways, the best one, that is lowest metric
            // then get the index for it
            let mut gateway_entries: Vec<(u16, u16, ConnectionModule)> = Vec::new();
            for gateway in &user.delegation_gateways {
                match gateway.upgrade() {
                    Some(n) => {
                        let node = n.read().unwrap();
                        let id = node.id;
                        let node_dict = self.node_dict.read().unwrap();
                        match node_dict.idx_of(&id) {
                            Some(node_idx) => {
                                let rt = self.routing_table.read().unwrap();
                                match rt.get(index::Space::Node, node_idx) {
                                    Some(e) => {
                                        let entry = e.read().unwrap();
                                        gateway_entries.push((
                                            entry.metric,
                                            entry.next_hop,
                                            entry.transport,
                                        ));
                                    }
                                    None => continue,
                                }
                            }
                            None => continue,
                        }
                    }
                    None => continue,
                }
            }

            // thus pick the lowest-metric gateway.
            // also `?` handles when the vec is empty returns none, then exits
            let best = gateway_entries.iter().min_by_key(|e| e.0)?;
            // the finally, at last, get the 8 byte node id
            let id = self.next_hop_node_id(best.1)?;
            Some((id, best.2))
        } else {
            None
        }
    }

    /// get the actual indeces that need to be reintroduced
    pub fn pending_introductions(&self, space: Space) -> Vec<(u16, [u8; 8], u32)> {
        let pending = {
            let mut tracker = self.reintroduction_tracker.write().unwrap();
            tracker.take_pending(space)
        };

        let mut res: Vec<(u16, [u8; 8], u32)> = Vec::with_capacity(pending.len());

        match space {
            Space::Node => {
                let dict = self.node_dict.read().unwrap();
                let nodes = self.nodes.read().unwrap();
                for idx in &pending {
                    let Some(id) = dict.id_of(*idx) else {
                        tracing::warn!("orphan mark in node space: idx {idx} has no dict binding");
                        continue;
                    };

                    let Some(arc) = nodes.get(&id) else {
                        tracing::warn!("orphan mark in node space: id {id:?} has no node record");
                        continue;
                    };
                    let version = arc.read().unwrap().manifest_version;
                    res.push((*idx, id, version));
                }
            }
            Space::User => {
                let dict = self.user_dict.read().unwrap();
                let users = self.users.read().unwrap();
                for idx in &pending {
                    let Some(id) = dict.id_of(*idx) else {
                        tracing::warn!("orphan mark in user space: idx {idx} has no dict binding");
                        continue;
                    };

                    let Some(arc) = users.get(&id) else {
                        tracing::warn!("orphan mark in user space: id {id:?} has no user record");
                        continue;
                    };
                    let version = arc.read().unwrap().profile_version;
                    res.push((*idx, id, version));
                }
            }
        };

        res.sort_by_key(|(idx, _, _)| *idx);
        res
    }
}
