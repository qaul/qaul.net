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
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Instant,
};

use libp2p::{identity::Keypair, PeerId};
use tokio::sync::mpsc;
use tracing::error;

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{messages::ManifestEntry, CodecError},
        identity::Multikey,
        index::{
            IndexAllocator, IndexDictionary, MirrorIndexDictionary, ReintroductionTracker, Space,
            RESERVED_INDEX,
        },
        manifest::{ChunkAssembler, Manifest, ManifestLog},
        seq::SeqNum,
        table::{Node, Nodes, RoutingTable, User, Users},
    },
    storage::{configuration::RoutingV2Options, manifest_state::HostManifestState},
};

pub mod codec;
pub mod identity;
pub mod index;
pub mod init;
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
    PeerIdNotInlineKey(u64),
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
            RoutingV2Error::PeerIdNotInlineKey(code) => {
                write!(
                    f,
                    "peer id does not embed a public key (multihash code {code:#x})"
                )
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

/// Which kind of entry this node originates for itself (spec §3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropagationForm {
    User,
    Node,
}

impl PropagationForm {
    /// The index space this node originates its own entry in.
    pub const fn origin_space(self) -> Space {
        match self {
            PropagationForm::User => Space::User,
            PropagationForm::Node => Space::Node,
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
    pub host_keypair: Keypair,
    pub host_mk: Multikey,
    pub last_manifest_emission_ms: RwLock<u64>,
    /// The §3.2 form currently *in effect*. Compared against
    /// `desired_propagation_form()` each origin tick; a difference drives the
    /// §3.5 reserved-index transition.
    pub propagation_form: RwLock<PropagationForm>,
}

impl RouterV2State {
    pub fn new(
        host_keypair: Keypair,
        host_multikey: Multikey,
        options: RoutingV2Options,
    ) -> (Self, mpsc::UnboundedReceiver<OutboundMsg>) {
        let (tx, rx) = mpsc::unbounded_channel::<OutboundMsg>();
        let state = Self {
            options,
            user_dict: RwLock::new(IndexDictionary::new(None)),
            // Both spaces start with no self-binding. §3.5 ties each reserved
            // index to a propagation form, and `sync_propagation_form`
            // establishes whichever one applies.
            node_dict: RwLock::new(IndexDictionary::new(None)),
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
            host_keypair,
            host_mk: host_multikey,
            last_manifest_emission_ms: RwLock::new(0u64),
            // A fresh node hosts no users and has no neighbours, so it starts
            // in user form; the first origin tick reconciles it.
            propagation_form: RwLock::new(PropagationForm::User),
        };
        (state, rx)
    }

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
    }

    /// we're adding a locally hosted user in this node's user index space.
    pub fn register_hosted_user(&self, user_id: [u8; 8], profile_version: u32) {
        {
            let mut users = self.users.write().unwrap();
            match users.get(&user_id) {
                Some(existing) => {
                    let mut u = existing.write().unwrap();
                    u.profile_version = profile_version;
                    u.is_hosted = true;
                }
                None => users.insert(
                    user_id,
                    User {
                        id: user_id,
                        public_key: None,
                        profile_version,
                        routing_entry: None,
                        delegation_gateways: Vec::new(),
                        is_hosted: true,
                    },
                ),
            }
        }

        let newly_bound = {
            let mut dict = self.user_dict.write().unwrap();
            if dict.idx_of(&user_id).is_some() {
                // this means it has been added so we don't have anythint to rebind or reintroduce
                None
            } else if dict.id_of(RESERVED_INDEX).is_none() {
                dict.bind(RESERVED_INDEX, user_id);
                Some(RESERVED_INDEX)
            } else {
                let mut allocator = self.users_allocator.write().unwrap();
                match allocator.allocate() {
                    Some(idx) => {
                        dict.bind(idx, user_id);
                        Some(idx)
                    }
                    None => {
                        error!("user allocator exhausted registering hosted user {user_id:?}");
                        None
                    }
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
        let has_internet_peer = self
            .mirrors
            .read()
            .unwrap()
            .values()
            .any(|info| info.transports.contains(&ConnectionModule::Internet));
        if has_internet_peer {
            return PropagationForm::Node;
        }

        // TODO(Phase 13): holding a delegation from another user is the third
        // §3.2 trigger; cross-host delegation does not exist yet.
        PropagationForm::User
    }

    pub(crate) fn release_index(&self, space: Space, id: &[u8; 8]) -> Option<u16> {
        let (dict_lock, alloc_lock) = match space {
            Space::Node => (&self.node_dict, &self.node_allocator),
            Space::User => (&self.user_dict, &self.users_allocator),
        };

        let mut dict = dict_lock.write().unwrap();
        let idx = dict.idx_of(id)?;

        self.routing_table.write().unwrap().clear(space, idx);
        if idx != RESERVED_INDEX {
            alloc_lock.write().unwrap().release(idx, Instant::now());
        }
        dict.unbind(idx);

        self.reintroduction_tracker
            .write()
            .unwrap()
            .clear_mark(space, idx);

        Some(idx)
    }

    /// Makes sure this node has a [`Node`] record for itself.
    fn ensure_host_node_record(&self, host_node_id: [u8; 8]) {
        let manifest_version = self.manifest.read().unwrap().manifest_version;
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
                is_gateway: false,
                delegated_users: Vec::new(),
                manifest_signature: None,
                retained_chunks: None,
                learn_sphere: None,
                manifest_log: ManifestLog::default(),
            },
        );
    }

    pub fn sync_propagation_form(&self) -> PropagationForm {
        let desired = self.desired_propagation_form();
        let current = *self.propagation_form.read().unwrap();
        if desired == current {
            return current;
        }

        // §3.5: the reserved index of the form we are leaving is released
        match desired {
            PropagationForm::Node => {
                for user_id in self.hosted_user_ids() {
                    if let Some(idx) = self.release_index(Space::User, &user_id) {
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
                // Give up the node-space self-binding; nothing references it
                // while we originate user entries.
                self.release_index(Space::Node, &self.host_mk.to_id());

                if let Some(user_id) = self.hosted_user_ids().first().copied() {
                    self.release_index(Space::User, &user_id);
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
        desired
    }

    pub fn unregister_hosted_user(&self, user_id: [u8; 8]) {
        let held_reserved = self
            .user_dict
            .read()
            .unwrap()
            .idx_of(&user_id)
            .map(|idx| idx == RESERVED_INDEX)
            .unwrap_or(false);

        self.release_index(Space::User, &user_id);
        self.users.write().unwrap().remove(&user_id);

        if held_reserved {
            if let Some(next) = self.hosted_user_ids().first().copied() {
                self.release_index(Space::User, &next);
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
        // manifest_version (§10.5, §10.7). Symmetric with the gap on the
        // creation side — neither half touches the manifest yet.
        tracing::info!("router_v2: hosted user {user_id:?} unregistered");
    }

    /// Registers a neighbour as a routable node: ensures a [`Node`]
    pub fn register_neighbour_node(&self, node_id: [u8; 8], public_key: Option<Multikey>) {
        {
            let mut nodes = self.nodes.write().unwrap();
            match nodes.get(&node_id) {
                Some(existing) => {
                    if public_key.is_some() {
                        existing.write().unwrap().public_key = public_key.clone();
                    }
                }
                None => nodes.insert(
                    node_id,
                    Node {
                        id: node_id,
                        public_key: public_key.clone(),
                        manifest_version: 0,
                        advertised_version: 0,
                        is_gateway: false,
                        delegated_users: Vec::new(),
                        manifest_signature: None,
                        retained_chunks: None,
                        learn_sphere: None,
                        manifest_log: ManifestLog::default(),
                    },
                ),
            }
        }

        let mut dict = self.node_dict.write().unwrap();
        if dict.idx_of(&node_id).is_some() {
            return;
        }

        let mut allocator = self.node_allocator.write().unwrap();
        let Some(idx) = allocator.allocate() else {
            tracing::error!("node allocator exhausted registering neighbour {node_id:?}");
            return;
        };
        dict.bind(idx, node_id);

        self.reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::Node, idx);

        tracing::info!("router_v2: neighbour node {node_id:?} allocated node index {idx}");
    }

    /// saves `transport` as a way to reach `peer`.
    /// `true` when this transport was not already registered for peer
    pub fn add_neighbour_transport(
        &self,
        peer: PeerId,
        node_id: [u8; 8],
        transport: ConnectionModule,
    ) -> bool {
        let mut mirrors = self.mirrors.write().unwrap();
        match mirrors.entry(peer) {
            Entry::Occupied(mut existing) => existing.get_mut().transports.insert(transport),
            Entry::Vacant(slot) => {
                slot.insert(NeighbourInfo::new(node_id, transport));
                true
            }
        }
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
