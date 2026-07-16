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
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Instant,
};

use libp2p::PeerId;
use tokio::sync::mpsc;

use crate::{
    connections::ConnectionModule, router_v2::{
        codec::{CodecError, messages::NodeManifest}, index::{
            IndexAllocator, IndexDictionary, MirrorIndexDictionary, ReintroductionTracker, Space,
        }, manifest::{ChunkAssembler, Manifest}, seq::SeqNum, table::{Nodes, RoutingTable, Users},
    }, storage::configuration::RoutingV2Options,
};

pub mod codec;
pub mod identity;
pub mod index;
pub mod manifest;
pub mod metric;
pub mod propagation;
pub mod receive;
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
pub struct NeighbourInfo {
    pub node_id: [u8; 8],
    pub users: MirrorIndexDictionary,
    pub nodes: MirrorIndexDictionary,
    pub transports: HashSet<ConnectionModule>,
}

impl NeighbourInfo {
    pub fn new(node_id: [u8; 8], transport: ConnectionModule) -> Self {
        let mut transports = HashSet::new();
        transports.insert(transport);
        NeighbourInfo {
            node_id,
            users: MirrorIndexDictionary::default(),
            nodes: MirrorIndexDictionary::default(),
            transports,
        }
    }
}

/// the shape for a message to be sent over the wire
#[derive(Debug, Clone)]
pub struct OutboundMsg {
    pub peer: PeerId,
    pub transport: ConnectionModule,
    pub bytes: Vec<u8>,
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
    pub mirrors: RwLock<HashMap<PeerId, NeighbourInfo>>,
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
    /// this node's sequence number
    pub seq_num: RwLock<SeqNum>,
    pub tx_outbound: mpsc::UnboundedSender<OutboundMsg>,
    /// pairs of entry to batched into the next 10s outbound
    pub relay_queue: RwLock<HashSet<(Space, u16)>>,
    /// the manifest for this node
    pub manifest: RwLock<Manifest>,
    /// chunk assembler
    pub chunk_assembler: RwLock<ChunkAssembler>,
    /// Manifests accepted at receive time, pending relay to other neighbours.
    pub manifest_relay_queue: RwLock<HashMap<[u8; 8], (Vec<NodeManifest>, Sphere)>>,
}

impl RouterV2State {
    pub fn new(host_node_id: [u8; 8]) -> (Self, mpsc::UnboundedReceiver<OutboundMsg>) {
        let (tx, rx) = mpsc::unbounded_channel::<OutboundMsg>();
        let state = Self {
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
            seq_num: RwLock::new(SeqNum::new()),
            tx_outbound: tx,
            relay_queue: RwLock::new(HashSet::new()),
            manifest: RwLock::new(Manifest::new()),
            chunk_assembler: RwLock::new(ChunkAssembler::new()),
            manifest_relay_queue: RwLock::new(HashMap::new())
        };
        (state, rx)
    }

    /// Spec 4.2
    pub fn add_neighbour_transport(
        &self,
        peer: PeerId,
        node_id: [u8; 8],
        transport: ConnectionModule,
    ) {
        let mut mirrors = self.mirrors.write().unwrap();
        mirrors
            .entry(peer)
            .and_modify(|info| {
                info.transports.insert(transport);
            })
            .or_insert_with(|| NeighbourInfo::new(node_id, transport));
    }

    pub fn remove_neighbour_transport(&self, peer: PeerId, transport: ConnectionModule) {
        let mut mirrors = self.mirrors.write().unwrap();
        let now_empty = if let Some(info) = mirrors.get_mut(&peer) {
            info.transports.remove(&transport);
            info.transports.is_empty()
        } else {
            false
        };
        if now_empty {
            mirrors.remove(&peer);
        }
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

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests;
