// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The Qaul Routing Protocol is a distance-vector routing protocol for the
//! qaul.net mesh. It carries reachability information for users and for
//! nodes across heterogeneous transports including LAN, Internet, and
//! Bluetooth Low Energy. The protocol scales from village-sized deployments
//! of a few dozen nodes to networks on the order of one hundred thousand
//! nodes connected across many regions. It tolerates partitioned operation
//! and supports gateway-based delegation across network boundaries.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Instant,
};

use libp2p::PeerId;
use tracing::debug;

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{messages::Mapping, CodecError, messages::Entry},
        index::{
            IndexAllocator, IndexDictionary, MirrorIndexDictionary, ReintroductionTracker, Space,
        },
        seq::is_fresher_u32,
        table::{Node, Nodes, RoutingTable, User, Users},
    },
    storage::configuration::RoutingV2Options,
};

pub mod codec;
pub mod identity;
pub mod index;
pub mod metric;
pub mod seq;
pub mod table;

#[derive(Debug, thiserror::Error)]
pub enum RoutingV2Error {
    MultikeyErrror(#[from] libp2p::identity::DecodingError),
    CodecError(#[from] CodecError),
    UnknownMapping(u16),
    AllocatorExhausted,
}

impl std::fmt::Display for RoutingV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingV2Error::MultikeyErrror(e) => write!(f, "{e}"),
            RoutingV2Error::CodecError(e) => write!(f, "{e}"),
            RoutingV2Error::UnknownMapping(idx) => {
                write!(f, "could not find a reference for index: {idx}")
            }
            RoutingV2Error::AllocatorExhausted => {
                write!(f, "internal allocator has been exhausted")
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, RoutingV2Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sphere {
    Local,
    Internet,
}

impl Sphere {
    pub const fn of(module: ConnectionModule) -> Self {
        match module {
            ConnectionModule::Internet => Sphere::Internet,
            _ => Sphere::Local,
        }
    }
}

/// groups one user-space and one node-space mirror per neigbour
#[derive(Debug, Default)]
pub struct NeighbourMirrors {
    pub users: MirrorIndexDictionary,
    pub nodes: MirrorIndexDictionary,
}

/// Instance-based router state that owns all routing sub-state.
/// This is the major struct that will replace the current Router.
/// Each `RouterState` instance is fully independent, enabling multiple
/// nodes to run in the same process.
pub struct RouterV2State {
    /// the default options
    pub options: RoutingV2Options,
    /// Index space for each user this particular node knows
    pub user_dict: RwLock<IndexDictionary>,
    /// Index space for each node this particular node knows
    pub node_dict: RwLock<IndexDictionary>,
    /// Two mirrors per neighbour, one per index space.
    pub mirrors: RwLock<HashMap<PeerId, NeighbourMirrors>>,
    /// the nodes that this node knows about
    pub nodes: Arc<RwLock<Nodes>>,
    /// the users
    pub users: Arc<RwLock<Users>>,
    /// the routing table for this node
    pub routing_table: Arc<RwLock<RoutingTable>>,
    /// the index allocators
    pub users_allocator: RwLock<IndexAllocator>,
    pub node_allocator: RwLock<IndexAllocator>,
    /// tracks the indices that needs to be resent over the wire
    pub reintroduction_tracker: RwLock<ReintroductionTracker>,
}

impl RouterV2State {
    pub fn new(host_node_id: [u8; 8]) -> Self {
        Self {
            options: RoutingV2Options::default(),
            user_dict: RwLock::new(IndexDictionary::new(None)),
            node_dict: RwLock::new(IndexDictionary::new(Some(host_node_id))),
            mirrors: RwLock::new(HashMap::new()),
            routing_table: Arc::new(RwLock::new(RoutingTable::new())),
            users: Arc::new(RwLock::new(Users::new())),
            nodes: Arc::new(RwLock::new(Nodes::new())),
            users_allocator: RwLock::new(IndexAllocator::new()),
            node_allocator: RwLock::new(IndexAllocator::new()),
            reintroduction_tracker: RwLock::new(ReintroductionTracker::new()),
        }
    }

    /// Inserts a fresh empty mirror for the neighbour, replacing any prior mirror
    pub fn add_neighbour_mirror(&self, peer: PeerId) {
        let mut mirrors = self.mirrors.write().unwrap();
        mirrors.insert(peer, NeighbourMirrors::default());
    }

    pub fn remove_mirror(&self, peer: PeerId) {
        let mut mirrors = self.mirrors.write().unwrap();
        mirrors.remove_entry(&peer);
    }

    pub fn next_hop_node_id(&self, next_hop: u16) -> Option<[u8; 8]> {
        let node_entries = &self.node_dict.read().unwrap();
        node_entries.id_of(next_hop)
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

    /// gets expired indexes
    pub fn sweep_expired(&self, now: u64) {
        let expiry_ms = self.options.route_expiry_ms;
        let mut rt = self.routing_table.write().unwrap();

        {
            let mut users_dict = self.user_dict.write().unwrap();
            let mut allocator = self.users_allocator.write().unwrap();
            let user_entries = &mut rt.user_entries;

            for idx in 0..user_entries.len() {
                // skip empty entries
                let Some(e) = &user_entries[idx] else {
                    continue;
                };
                let expired = {
                    let entry = e.read().unwrap();
                    entry.last_update.saturating_add(expiry_ms) < now
                };
                if expired {
                    user_entries[idx] = None;
                    users_dict.unbind(idx as u16);
                    allocator.release(idx as u16, Instant::now());
                }
            }
        }

        {
            let mut nodes_dict = self.node_dict.write().unwrap();
            let mut allocator = self.node_allocator.write().unwrap();
            let node_entries = &mut rt.node_entries;

            for idx in 0..node_entries.len() {
                // skip empty entries
                let Some(e) = &node_entries[idx] else {
                    continue;
                };
                let expired = {
                    let entry = e.read().unwrap();
                    entry.last_update.saturating_add(expiry_ms) < now
                };
                if expired {
                    node_entries[idx] = None;
                    nodes_dict.unbind(idx as u16);
                    allocator.release(idx as u16, Instant::now());
                }
            }
        }
    }

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
        rssi_dbm: Option<i8>,
        space: Space,
        entry: Entry,
        now: u64,
    ) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests;
