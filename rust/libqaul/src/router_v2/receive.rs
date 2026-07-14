// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Receive-side handlers for router_v2.

use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

use libp2p::PeerId;
use tracing::{debug, error, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{
            messages::{Entry, Mapping, NodeManifest, RoutingUpdate},
            CodecError, Header, RoutingMessage,
        },
        identity::Multikey,
        index::Space,
        manifest::Manifest,
        metric::hop_cost,
        seq::{is_fresher_u32, Acceptance, SeqNum},
        table::{DelegatedUser, Node, RoutingEntry, TargetRef, User},
        Result, RouterV2State, RoutingV2Error,
    },
};

impl RouterV2State {
    pub fn translate_incoming(
        &self,
        neighbour: PeerId,
        space: Space,
        incoming_idx: u16,
    ) -> Result<u16> {
        let id = {
            let mirrors = self.mirrors.read().unwrap();
            let mirrors_for_neighbour = mirrors
                .get(&neighbour)
                .ok_or(RoutingV2Error::UnknownMapping(incoming_idx))?;
            let mirror_dict = match space {
                Space::Node => &mirrors_for_neighbour.nodes,
                Space::User => &mirrors_for_neighbour.users,
            };
            mirror_dict
                .id_of(incoming_idx)
                .ok_or(RoutingV2Error::UnknownMapping(incoming_idx))?
        };

        let (dict, alloc) = match space {
            Space::Node => (&self.node_dict, &self.node_allocator),
            Space::User => (&self.user_dict, &self.users_allocator),
        };

        let mut self_dict = dict.write().unwrap();
        if let Some(i) = self_dict.idx_of(&id) {
            return Ok(i);
        }

        let mut allocator = alloc.write().unwrap();
        let mut tracker = self.reintroduction_tracker.write().unwrap();

        let Some(allocated_idx) = allocator.allocate() else {
            return Err(RoutingV2Error::AllocatorExhausted);
        };
        self_dict.bind(allocated_idx, id);
        tracker.mark_first_time(space, allocated_idx);

        Ok(allocated_idx)
    }

    pub fn apply_mapping(&self, neighbour: PeerId, space: Space, mapping: Mapping) -> Result<()> {
        let mirror_id = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(neigbour_mirrors) = mirrors.get(&neighbour) else {
                debug!("neighbour vanished between mapping steps: {neighbour:?}");
                return Ok(());
            };
            let dict = match space {
                Space::Node => &neigbour_mirrors.nodes,
                Space::User => &neigbour_mirrors.users,
            };
            dict.id_of(mapping.abs_idx)
        };

        match mirror_id {
            Some(id) if id != mapping.target_id => {
                let mut rt = self.routing_table.write().unwrap();
                let (mut entry_dict, mut allocator) = match space {
                    Space::Node => (
                        self.node_dict.write().unwrap(),
                        self.node_allocator.write().unwrap(),
                    ),
                    Space::User => (
                        self.user_dict.write().unwrap(),
                        self.users_allocator.write().unwrap(),
                    ),
                };

                if let Some(idx) = entry_dict.idx_of(&id) {
                    rt.clear(space, idx);
                    allocator.release(idx, Instant::now());
                    entry_dict.unbind(idx);
                }
            }
            Some(_) => {}
            None => {}
        };

        // now, we can safely bind the mapping to the correspoding neighbor
        {
            let mut mirrors = self.mirrors.write().unwrap();
            let Some(neigbour_mirrors) = mirrors.get_mut(&neighbour) else {
                return Ok(());
            };
            let dict = match space {
                Space::Node => &mut neigbour_mirrors.nodes,
                Space::User => &mut neigbour_mirrors.users,
            };
            dict.bind(mapping.abs_idx, mapping.target_id);
        }

        match space {
            Space::Node => {
                let mut nodes = self.nodes.write().unwrap();
                match nodes.get(&mapping.target_id) {
                    Some(node) => {
                        let version = {
                            let n = node.read().unwrap();
                            n.manifest_version
                        };
                        if is_fresher_u32(mapping.version, version) {
                            let mut n = node.write().unwrap();
                            n.manifest_version = mapping.version;
                        } else if version == mapping.version {
                        } else {
                            // TODO
                            debug!(
                                "stale profile echo from {neighbour:?}: target={:?} stored_version={version} incoming={}",
                                mapping.target_id,
                                mapping.version
                            );
                        }
                    }
                    None => {
                        let n = Node {
                            id: mapping.target_id,
                            is_gateway: false,
                            delegated_users: Vec::new(),
                            manifest_version: mapping.version,
                            public_key: None,
                        };
                        nodes.insert(mapping.target_id, n);
                    }
                };
            }
            Space::User => {
                let mut users = self.users.write().unwrap();
                match users.get(&mapping.target_id) {
                    Some(user) => {
                        let version = {
                            let u = user.read().unwrap();
                            u.profile_version
                        };
                        if is_fresher_u32(mapping.version, version) {
                            let mut u = user.write().unwrap();
                            u.profile_version = mapping.version;
                        } else if version == mapping.version {
                        } else {
                            // TODO
                            debug!(
                                "stale profile echo from {neighbour:?}: target={:?} stored_version={version} incoming={}",
                                mapping.target_id,
                                mapping.version
                            );
                        }
                    }
                    None => {
                        let u = User {
                            id: mapping.target_id,
                            profile_version: mapping.version,
                            routing_entry: None,
                            delegation_gateways: Vec::new(),
                            public_key: None,
                        };
                        users.insert(mapping.target_id, u);
                    }
                };
            }
        }
        Ok(())
    }

    /// takes the wire-form entry from codec::messages and builds the stored form RoutingEntry
    pub fn apply_entry(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        rssi_dbm: Option<i8>,
        space: Space,
        entry: Entry,
        now: u64,
    ) -> Result<()> {
        // the hop count has exceeded the design limit
        if entry.hop_count >= 63 {
            return Ok(());
        }

        let metric = entry.metric.saturating_add(hop_cost(transport, rssi_dbm));
        let own_idx = match self.translate_incoming(neighbour, space, entry.abs_idx) {
            Ok(idx) => idx,
            Err(RoutingV2Error::UnknownMapping(_)) => return Ok(()),
            Err(e) => return Err(e),
        };

        let target = {
            match space {
                Space::Node => {
                    let dict = self.node_dict.read().unwrap();
                    let Some(id) = dict.id_of(own_idx) else {
                        debug!("receive-loop drop: own_idx {own_idx:?} has no dict binding (space={space:?})");
                        return Ok(());
                    };
                    let nodes = self.nodes.read().unwrap();
                    if let Some(node) = nodes.get(&id) {
                        TargetRef::Node(node)
                    } else {
                        debug!("failed to find mapping");
                        return Ok(());
                    }
                }
                Space::User => {
                    let dict = self.user_dict.read().unwrap();
                    let Some(id) = dict.id_of(own_idx) else {
                        debug!(
                            "receive-loop drop: user own_idx {own_idx} has no user_dict binding"
                        );
                        return Ok(());
                    };
                    let users = self.users.read().unwrap();
                    if let Some(user) = users.get(&id) {
                        TargetRef::User(user)
                    } else {
                        debug!("receive-loop drop: user_id {id:?} not found in users map (own_idx={own_idx})");
                        return Ok(());
                    }
                }
            }
        };

        let (accept_entry, local_entry) = {
            let rt = self.routing_table.read().unwrap();
            match rt.get(space, own_idx) {
                // that is, yhe slot is empty, so accept it.
                None => (true, entry.local_only),
                Some(e) => {
                    let stored_entry = e.read().unwrap();
                    let accept = match stored_entry.seq_num.acceptance(SeqNum::from(entry.seq)) {
                        Acceptance::Fresher | Acceptance::Reboot => true,
                        // that is, acceot only if the incoming metric is better
                        Acceptance::NoChange => metric < stored_entry.metric,
                    };
                    (accept, stored_entry.local_only && entry.local_only)
                }
            }
        };
        if !accept_entry {
            debug!(
                "receive-loop drop: not better than stored (own_idx={own_idx}, incoming_seq={}, incoming_metric={metric})",
                entry.seq,
            );
            return Ok(());
        }

        // get next_hop
        let neighbour_node_id = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(info) = mirrors.get(&neighbour) else {
                return Ok(());
            };
            info.node_id
        };
        let next_hop_idx = {
            let dict = self.node_dict.read().unwrap();
            if let Some(idx) = dict.idx_of(&neighbour_node_id) {
                idx
            } else {
                debug!("neighbour node_id has no node_dict entry: {neighbour_node_id:?}");
                return Ok(());
            }
        };

        let new_entry = Arc::new(RwLock::new(RoutingEntry {
            target,
            target_index: own_idx,
            seq_num: SeqNum::from(entry.seq),
            metric,
            next_hop: next_hop_idx,
            transport,
            last_update: now,
            hop_count: entry.hop_count.saturating_add(1),
            local_only: local_entry,
        }));

        if let TargetRef::User(user) = &new_entry.read().unwrap().target {
            user.write().unwrap().routing_entry = Some(Arc::downgrade(&new_entry));
        }

        self.routing_table
            .write()
            .unwrap()
            .set(space, own_idx, new_entry);

        self.relay_queue.write().unwrap().insert((space, own_idx));
        Ok(())
    }

    pub fn handle_routing_update(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        rssi_dbm: Option<i8>,
        msg: RoutingUpdate,
        now: u64,
    ) -> Result<()> {
        for mapping in msg.user_mappings {
            match self.apply_mapping(neighbour, Space::User, mapping) {
                Ok(_) => {}
                Err(e) => warn!("apply_mapping user failed: {e}"),
            };
        }

        for mapping in msg.node_mappings {
            match self.apply_mapping(neighbour, Space::Node, mapping) {
                Ok(_) => {}
                Err(e) => warn!("apply_mapping node failed: {e}"),
            };
        }

        for entry in msg.user_entries {
            match self.apply_entry(neighbour, transport, rssi_dbm, Space::User, entry, now) {
                Ok(_) => {}
                Err(e) => warn!("apply_entry user failed: {e}"),
            };
        }

        for entry in msg.node_entries {
            match self.apply_entry(neighbour, transport, rssi_dbm, Space::Node, entry, now) {
                Ok(_) => {}
                Err(e) => warn!("apply_entry node failed: {e}"),
            };
        }

        Ok(())
    }

    pub fn received(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        rssi_dbm: Option<i8>,
        mut buf: &[u8],
        now: u64,
    ) -> Result<()> {
        while !buf.is_empty() {
            let (header, body_slice) = match Header::decode(buf) {
                Ok(h) => h,
                Err(CodecError::BadVersion { payload_len, .. }) => {
                    let skip = 4 + payload_len;
                    if buf.len() < skip as usize {
                        break;
                    }
                    buf = &buf[skip as usize..];
                    continue;
                }
                Err(e) => {
                    warn!("failed to decode header: {e}");
                    return Ok(());
                }
            };

            let payload_len = header.payload_len as usize;
            if body_slice.len() < payload_len {
                warn!(
                    "received: truncated body, expected {payload_len} got {}",
                    body_slice.len()
                );
                return Ok(());
            }
            let payload = &body_slice[..payload_len];
            buf = &body_slice[payload_len..];

            match header.message_type {
                RoutingMessage::RoutingUpdate => match RoutingUpdate::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) =
                            self.handle_routing_update(neighbour, transport, rssi_dbm, msg, now)
                        {
                            error!("handle_routing_update failed: {e}");
                        }
                    }
                    Err(e) => error!("RoutingUpdate decode failed: {e}"),
                },
                RoutingMessage::NodeManifest => match NodeManifest::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_node_manifest(neighbour, msg, now) {
                            error!("handle_node_manifest failed: {e}");
                        }
                    }
                    Err(e) => error!("NodeManifest decode failed: {e}"),
                },
                _ => debug!("to be implemented"),
            }
        }
        Ok(())
    }

    fn get_resource_mk(&self, resouce_id: &[u8; 8], space: Space) -> Option<Multikey> {
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

    pub fn handle_node_manifest(
        &self,
        neighbour: PeerId,
        mut msg: NodeManifest,
        now: u64,
    ) -> Result<()> {
        let origin_node_id = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(info) = mirrors.get(&neighbour) else {
                debug!("node_manifest from unknown neighbour: {neighbour:?}");
                return Ok(());
            };

            let Some(id) = info.nodes.id_of(msg.origin_node_index) else {
                debug!(
                    "node_manifest origin_node_index {} unknown in mirror",
                    msg.origin_node_index
                );
                return Ok(());
            };

            id
        };

        let host_mk = {
            match self.get_resource_mk(&origin_node_id, Space::Node) {
                Some(mk) => mk,
                None => {
                    debug!("node_manifest received but origin's public_key is unknown — TODO(§11.5 ProfileFetch)");
                    return Ok(());
                }
            }
        };

        if Manifest::verify_chunk(&msg, &host_mk).is_err() {
            debug!("node_manifest chunk sig invalid for origin {origin_node_id:?}");
            return Ok(());
        };

        let mut filtered_entries = Vec::new();
        for entry in msg.entries {
            let Some(user_mk) = self.get_resource_mk(&entry.user_id, Space::User) else {
                continue;
            };

            if Manifest::verify_entry(&entry, &host_mk, &user_mk).is_err() || entry.timeout <= now {
                continue;
            }

            filtered_entries.push(entry);
        }
        msg.entries = filtered_entries;

        let completed_manifest = {
            let mut assembler = self.chunk_assembler.write().unwrap();
            let Some(completed) = assembler.insert(origin_node_id, msg) else {
                return Ok(());
            };
            completed
        };

        let is_gateway = (completed_manifest.flags & 0x01) != 0;
        let delegated_users: Vec<DelegatedUser> = {
            let mut users = self.users.write().unwrap();
            completed_manifest
                .entries
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
                                    profile_version: 0,
                                    routing_entry: None,
                                    delegation_gateways: Vec::new(),
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
                    }
                })
                .collect()
        };

        let nodes = self.nodes.read().unwrap();
        if let Some(node_arc) = nodes.get(&origin_node_id) {
            let mut node = node_arc.write().unwrap();
            node.manifest_version = completed_manifest.manifest_version;
            node.is_gateway = is_gateway;
            node.delegated_users = delegated_users;
        }

        //TODO(Phase 8): enqueue completed manifest for relay per §8.5.

        Ok(())
    }
}
