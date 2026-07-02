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
        codec::{messages::Mapping, CodecError},
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_of_lan_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::Lan), Sphere::Local);
    }

    #[test]
    fn sphere_of_ble1m_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::Ble1m), Sphere::Local);
    }

    #[test]
    fn sphere_of_ble_coded_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::BleCoded), Sphere::Local);
    }

    #[test]
    fn sphere_of_internet_is_internet() {
        assert_eq!(Sphere::of(ConnectionModule::Internet), Sphere::Internet);
    }

    #[test]
    fn sphere_of_self_is_local() {
        // ConnectionModule::Local refers to this node itself, which is
        // part of its own Local sphere by definition
        assert_eq!(Sphere::of(ConnectionModule::Local), Sphere::Local);
    }

    #[test]
    fn sphere_of_none_currently_falls_through_to_local() {
        assert_eq!(Sphere::of(ConnectionModule::None), Sphere::Local);
    }
}

#[cfg(test)]
mod next_hop_tests {
    use super::*;
    use crate::router_v2::{
        identity::Multikey,
        index::Space,
        seq::SeqNum,
        table::{Node, RoutingEntry, TargetRef, User},
    };
    use libp2p::identity::Keypair;
    use std::sync::Weak;

    fn fresh_multikey() -> Multikey {
        Multikey::from(Keypair::generate_ed25519().public())
    }

    fn fresh_state() -> RouterV2State {
        RouterV2State::new([0; 8])
    }

    /// Inserts a user into `state.users` and returns the live Arc so the
    /// caller can downgrade it / mutate its routing fields.
    fn register_user(state: &RouterV2State, id: [u8; 8]) -> Arc<RwLock<User>> {
        let user = User {
            id,
            public_key: Some(fresh_multikey()),
            profile_version: 0,
            routing_entry: None,
            delegation_gateways: Vec::new(),
        };
        let mut users = state.users.write().unwrap();
        users.insert(id, user);
        users.get(&id).unwrap()
    }

    /// Inserts a node into `state.nodes` and returns the live Arc so the
    /// caller can downgrade it for gateway Weak refs.
    fn register_node(state: &RouterV2State, id: [u8; 8], is_gateway: bool) -> Arc<RwLock<Node>> {
        let node = Node {
            id,
            public_key: Some(fresh_multikey()),
            manifest_version: 0,
            is_gateway,
            delegated_users: Vec::new(),
        };
        let mut nodes = state.nodes.write().unwrap();
        nodes.insert(id, node);
        nodes.get(&id).unwrap()
    }

    fn make_entry(
        target: TargetRef,
        next_hop: u16,
        metric: u16,
        transport: ConnectionModule,
    ) -> Arc<RwLock<RoutingEntry>> {
        Arc::new(RwLock::new(RoutingEntry {
            target_index: 0,
            target,
            seq_num: SeqNum::from(0u16),
            metric,
            next_hop,
            transport,
            last_update: 0,
            hop_count: 0,
            local_only: false,
        }))
    }

    /// Reserves an index for `id` in the node-space dictionary so that
    /// `next_hop_node_id(idx)` can resolve it.
    fn bind_node(state: &RouterV2State, idx: u16, id: [u8; 8]) {
        state.node_dict.write().unwrap().bind(idx, id);
    }

    #[test]
    fn unknown_user_returns_none() {
        let state = fresh_state();
        assert_eq!(state.next_hop_for_user([99; 8]), None);
    }

    #[test]
    fn known_user_with_no_routing_data_returns_none() {
        let state = fresh_state();
        register_user(&state, [1; 8]);
        assert_eq!(state.next_hop_for_user([1; 8]), None);
    }

    /// Step 2: a direct routing entry whose next_hop resolves through the
    /// dictionary should produce that hop's node id and the entry's transport.
    #[test]
    fn direct_routing_entry_resolves_next_hop_and_transport() {
        let state = fresh_state();
        let user = register_user(&state, [1; 8]);

        let neighbour_id = [9; 8];
        let neighbour_idx = 100;
        bind_node(&state, neighbour_idx, neighbour_id);

        let entry = make_entry(
            TargetRef::User(user.clone()),
            neighbour_idx,
            42,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, 5, entry.clone());
        user.write().unwrap().routing_entry = Some(Arc::downgrade(&entry));

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((neighbour_id, ConnectionModule::Lan)),
        );
    }

    /// Step 3: no direct entry; two delegation gateways; the gateway with
    /// the lowest metric wins, and its routing entry's next_hop / transport
    /// determine the result.
    #[test]
    fn gateway_fallback_picks_lowest_metric() {
        let state = fresh_state();
        let user = register_user(&state, [1; 8]);

        let g_hi = register_node(&state, [10; 8], true);
        let g_lo = register_node(&state, [20; 8], true);

        // The gateway nodes must have routing entries in the node-space
        // table, indexed by their own node-space idx via the dictionary.
        bind_node(&state, 50, [10; 8]);
        bind_node(&state, 60, [20; 8]);

        // Each gateway's routing entry points at a different neighbour.
        let nbr_hi = [11; 8];
        let nbr_lo = [21; 8];
        bind_node(&state, 101, nbr_hi);
        bind_node(&state, 102, nbr_lo);

        let e_hi = make_entry(
            TargetRef::Node(g_hi.clone()),
            101,
            30,
            ConnectionModule::Lan,
        );
        let e_lo = make_entry(
            TargetRef::Node(g_lo.clone()),
            102,
            10,
            ConnectionModule::Internet,
        );
        {
            let mut rt = state.routing_table.write().unwrap();
            rt.set(Space::Node, 50, e_hi);
            rt.set(Space::Node, 60, e_lo);
        }

        user.write().unwrap().delegation_gateways =
            vec![Arc::downgrade(&g_hi), Arc::downgrade(&g_lo)];

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((nbr_lo, ConnectionModule::Internet)),
        );
    }

    /// A direct routing entry must be preferred over a delegation gateway,
    /// even when the gateway has a lower metric. The metric comparison only
    /// applies within the step 3 fallback.
    #[test]
    fn direct_entry_preferred_over_lower_metric_gateway() {
        let state = fresh_state();
        let user = register_user(&state, [1; 8]);

        // Direct entry, high metric.
        let direct_nbr = [50; 8];
        bind_node(&state, 200, direct_nbr);
        let direct = make_entry(
            TargetRef::User(user.clone()),
            200,
            100,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, 5, direct.clone());

        // Gateway path, much lower metric.
        let gw = register_node(&state, [10; 8], true);
        bind_node(&state, 50, [10; 8]);
        let gw_nbr = [11; 8];
        bind_node(&state, 101, gw_nbr);
        let gw_entry = make_entry(
            TargetRef::Node(gw.clone()),
            101,
            5,
            ConnectionModule::Internet,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, gw_entry);

        {
            let mut u = user.write().unwrap();
            u.routing_entry = Some(Arc::downgrade(&direct));
            u.delegation_gateways.push(Arc::downgrade(&gw));
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((direct_nbr, ConnectionModule::Lan)),
        );
    }

    /// If the direct entry's Weak is dangling (the table has dropped the
    /// strong), `next_hop_for_user` must fall through to the gateway
    /// fallback, not return None.
    #[test]
    fn dangling_direct_entry_falls_through_to_gateway() {
        let state = fresh_state();
        let user = register_user(&state, [1; 8]);

        // Build an entry but never put it in the table; the strong dies
        // immediately and the weak is dangling from the start.
        let orphan = make_entry(TargetRef::User(user.clone()), 0, 100, ConnectionModule::Lan);
        let dangling: Weak<RwLock<RoutingEntry>> = Arc::downgrade(&orphan);
        drop(orphan);

        // Working gateway path.
        let gw = register_node(&state, [10; 8], true);
        bind_node(&state, 50, [10; 8]);
        let gw_nbr = [11; 8];
        bind_node(&state, 101, gw_nbr);
        let gw_entry = make_entry(
            TargetRef::Node(gw.clone()),
            101,
            5,
            ConnectionModule::Internet,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, gw_entry);

        {
            let mut u = user.write().unwrap();
            u.routing_entry = Some(dangling);
            u.delegation_gateways.push(Arc::downgrade(&gw));
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((gw_nbr, ConnectionModule::Internet)),
        );
    }

    /// A gateway whose Weak<Node> is dangling must be silently skipped;
    /// any remaining live gateway should still produce a result.
    #[test]
    fn dangling_gateway_is_skipped() {
        let state = fresh_state();
        let user = register_user(&state, [1; 8]);

        // Live gateway with a usable routing entry.
        let live = register_node(&state, [10; 8], true);
        bind_node(&state, 50, [10; 8]);
        let live_nbr = [11; 8];
        bind_node(&state, 101, live_nbr);
        let live_entry = make_entry(
            TargetRef::Node(live.clone()),
            101,
            30,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, live_entry);

        // Dangling gateway: build a Node, take a Weak, drop the strong.
        let orphan = Arc::new(RwLock::new(Node {
            id: [20; 8],
            public_key: Some(fresh_multikey()),
            manifest_version: 0,
            is_gateway: true,
            delegated_users: Vec::new(),
        }));
        let dangling = Arc::downgrade(&orphan);
        drop(orphan);

        {
            let mut u = user.write().unwrap();
            u.delegation_gateways = vec![dangling, Arc::downgrade(&live)];
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((live_nbr, ConnectionModule::Lan)),
        );
    }

    /// A gateway that has been registered as a Node but has no routing
    /// entry of its own (e.g. unreachable) must be skipped, not produce an
    /// `(id, transport)` tuple with garbage data.
    #[test]
    fn gateway_with_no_routing_entry_is_skipped() {
        let state = fresh_state();
        let user = register_user(&state, [1; 8]);

        // Unreachable gateway: node exists, but no routing entry in the table.
        let unreachable = register_node(&state, [20; 8], true);
        bind_node(&state, 60, [20; 8]);

        // Reachable gateway.
        let reachable = register_node(&state, [10; 8], true);
        bind_node(&state, 50, [10; 8]);
        let r_nbr = [11; 8];
        bind_node(&state, 101, r_nbr);
        let r_entry = make_entry(
            TargetRef::Node(reachable.clone()),
            101,
            5,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, r_entry);

        {
            let mut u = user.write().unwrap();
            u.delegation_gateways = vec![Arc::downgrade(&unreachable), Arc::downgrade(&reachable)];
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((r_nbr, ConnectionModule::Lan)),
        );
    }

    /// `next_hop_node_id` is a thin pass-through over the node-space
    /// dictionary. A bound index resolves; an unbound one returns None.
    #[test]
    fn next_hop_node_id_resolves_bound_indices_and_misses_unbound() {
        let state = fresh_state();
        bind_node(&state, 77, [7; 8]);
        assert_eq!(state.next_hop_node_id(77), Some([7; 8]));
        assert_eq!(state.next_hop_node_id(78), None);
    }
}

#[cfg(test)]
mod sweep_tests {
    use super::*;
    use crate::router_v2::{
        identity::Multikey,
        index::Space,
        seq::SeqNum,
        table::{Node, RoutingEntry, TargetRef, User},
    };
    use libp2p::identity::Keypair;
    use std::sync::Weak;

    fn fresh_multikey() -> Multikey {
        Multikey::from(Keypair::generate_ed25519().public())
    }

    fn fresh_state() -> RouterV2State {
        RouterV2State::new([0; 8])
    }

    fn install_user(state: &RouterV2State, id: [u8; 8]) -> Arc<RwLock<User>> {
        let u = User {
            id,
            public_key: Some(fresh_multikey()),
            profile_version: 0,
            routing_entry: None,
            delegation_gateways: Vec::new(),
        };
        let mut users = state.users.write().unwrap();
        users.insert(id, u);
        users.get(&id).unwrap()
    }

    fn install_node(state: &RouterV2State, id: [u8; 8]) -> Arc<RwLock<Node>> {
        let n = Node {
            id,
            public_key: Some(fresh_multikey()),
            manifest_version: 0,
            is_gateway: false,
            delegated_users: Vec::new(),
        };
        let mut nodes = state.nodes.write().unwrap();
        nodes.insert(id, n);
        nodes.get(&id).unwrap()
    }

    /// Installs a routing entry at `(space, idx)`, binds the dictionary,
    /// and returns a Weak to the entry so tests can verify cycle
    /// discipline after sweep. The strong Arc is moved into the table,
    /// so the only strong reference lives in the routing table itself.
    fn install_entry(
        state: &RouterV2State,
        space: Space,
        idx: u16,
        target_id: [u8; 8],
        target: TargetRef,
        last_update: u64,
    ) -> Weak<RwLock<RoutingEntry>> {
        let arc = Arc::new(RwLock::new(RoutingEntry {
            target_index: idx,
            target,
            seq_num: SeqNum::from(0u16),
            metric: 0,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update,
            hop_count: 0,
            local_only: false,
        }));
        let weak = Arc::downgrade(&arc);
        state.routing_table.write().unwrap().set(space, idx, arc);
        match space {
            Space::User => state.user_dict.write().unwrap().bind(idx, target_id),
            Space::Node => state.node_dict.write().unwrap().bind(idx, target_id),
        }
        weak
    }

    /// Default-config expiry threshold (ms). Cached so tests don't fight
    /// over a magic number.
    fn expiry_ms(state: &RouterV2State) -> u64 {
        state.options.route_expiry_ms
    }

    #[test]
    fn entry_past_threshold_is_removed() {
        let state = fresh_state();
        let user = install_user(&state, [1; 8]);
        let now: u64 = 100_000;
        // last_update + expiry = (now - expiry - 1) + expiry = now - 1, < now → expired
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        state.sweep_expired(now);

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 5)
            .is_none());
    }

    #[test]
    fn entry_within_threshold_is_kept() {
        let state = fresh_state();
        let user = install_user(&state, [1; 8]);
        let now: u64 = 100_000;
        // last_update + expiry = now + 1 → not < now, kept
        let last_update = now - expiry_ms(&state) + 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        state.sweep_expired(now);

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 5)
            .is_some());
    }

    /// At exactly `last_update + expiry == now`, the strict `<` comparison
    /// keeps the entry. Pins the operator against an accidental `<=`.
    #[test]
    fn entry_at_exact_boundary_is_kept() {
        let state = fresh_state();
        let user = install_user(&state, [1; 8]);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state);
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        state.sweep_expired(now);

        assert!(
            state
                .routing_table
                .read()
                .unwrap()
                .get(Space::User, 5)
                .is_some(),
            "entry exactly at the threshold must survive (strict `<`)",
        );
    }

    #[test]
    fn expired_entry_unbinds_the_dictionary() {
        let state = fresh_state();
        let user = install_user(&state, [1; 8]);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        // Sanity: dict has the binding before sweep.
        assert_eq!(state.user_dict.read().unwrap().id_of(5), Some([1; 8]));

        state.sweep_expired(now);

        assert_eq!(state.user_dict.read().unwrap().id_of(5), None);
        assert_eq!(state.user_dict.read().unwrap().idx_of(&[1; 8]), None);
    }

    #[test]
    fn expired_entry_pushes_idx_into_allocator_cooldown() {
        let state = fresh_state();
        let user = install_user(&state, [1; 8]);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        // Sanity: idx not in cooldown before sweep.
        assert!(!state.users_allocator.read().unwrap().idx_in_cooldown(5));

        state.sweep_expired(now);

        assert!(
            state.users_allocator.read().unwrap().idx_in_cooldown(5),
            "released idx must enter cooldown so the allocator doesn't rebind it immediately",
        );
    }

    /// Cycle discipline (spec A.3): once the table drops its Arc, the
    /// User's back-edge Weak must resolve to None. The sweeper relies on
    /// this so it doesn't have to mutate the User itself.
    #[test]
    fn expired_entry_makes_user_weak_routing_entry_dangle() {
        let state = fresh_state();
        let user = install_user(&state, [1; 8]);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        let weak = install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user.clone()),
            last_update,
        );
        user.write().unwrap().routing_entry = Some(weak.clone());

        assert!(weak.upgrade().is_some(), "weak must upgrade before sweep");

        state.sweep_expired(now);

        assert!(
            weak.upgrade().is_none(),
            "weak must dangle after sweep drops the table's Arc",
        );
        // The User's Option is still Some — the sweeper doesn't reach in
        // and clear it; we rely on the Weak naturally resolving to None.
        assert!(user.read().unwrap().routing_entry.is_some());
    }

    #[test]
    fn node_space_expiry_is_independent_from_user_space() {
        let state = fresh_state();
        let node = install_node(&state, [9; 8]);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::Node,
            7,
            [9; 8],
            TargetRef::Node(node),
            last_update,
        );

        // Also install a fresh user-space entry that must survive.
        let user = install_user(&state, [1; 8]);
        install_entry(&state, Space::User, 3, [1; 8], TargetRef::User(user), now);

        state.sweep_expired(now);

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::Node, 7)
            .is_none());
        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 3)
            .is_some());
        assert!(state.node_allocator.read().unwrap().idx_in_cooldown(7));
        assert!(!state.users_allocator.read().unwrap().idx_in_cooldown(3));
    }

    #[test]
    fn mixed_entries_only_expired_are_removed() {
        let state = fresh_state();
        let now: u64 = 100_000;

        let old_user = install_user(&state, [1; 8]);
        let fresh_user = install_user(&state, [2; 8]);

        install_entry(
            &state,
            Space::User,
            10,
            [1; 8],
            TargetRef::User(old_user),
            now - expiry_ms(&state) - 1,
        );
        install_entry(
            &state,
            Space::User,
            11,
            [2; 8],
            TargetRef::User(fresh_user),
            now,
        );

        state.sweep_expired(now);

        let rt = state.routing_table.read().unwrap();
        assert!(rt.get(Space::User, 10).is_none(), "stale entry removed");
        assert!(rt.get(Space::User, 11).is_some(), "fresh entry untouched");
    }

    #[test]
    fn sweep_on_empty_state_is_a_noop() {
        let state = fresh_state();
        // Just verifying it doesn't panic walking 131k empty slots.
        state.sweep_expired(0);
        state.sweep_expired(u64::MAX);
    }
}

#[cfg(test)]
mod translate_tests {
    use super::*;
    use crate::router_v2::{
        identity::Multikey,
        index::Space,
        table::{Node, User},
    };
    use libp2p::identity::Keypair;

    fn fresh_multikey() -> Multikey {
        Multikey::from(Keypair::generate_ed25519().public())
    }

    fn fresh_state() -> RouterV2State {
        RouterV2State::new([0; 8])
    }

    fn fresh_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    fn add_neighbour(state: &RouterV2State) -> PeerId {
        let peer = fresh_peer();
        state.add_neighbour_mirror(peer);
        peer
    }

    fn bind_mirror(state: &RouterV2State, peer: PeerId, space: Space, idx: u16, id: [u8; 8]) {
        let mut mirrors = state.mirrors.write().unwrap();
        let nm = mirrors.get_mut(&peer).unwrap();
        match space {
            Space::Node => nm.nodes.bind(idx, id),
            Space::User => nm.users.bind(idx, id),
        }
    }

    fn install_user(state: &RouterV2State, id: [u8; 8], profile_version: u32) {
        let u = User {
            id,
            public_key: Some(fresh_multikey()),
            profile_version,
            routing_entry: None,
            delegation_gateways: Vec::new(),
        };
        state.users.write().unwrap().insert(id, u);
    }

    fn install_node(state: &RouterV2State, id: [u8; 8], manifest_version: u32) {
        let n = Node {
            id,
            public_key: Some(fresh_multikey()),
            manifest_version,
            is_gateway: false,
            delegated_users: Vec::new(),
        };
        state.nodes.write().unwrap().insert(id, n);
    }

    #[test]
    fn translate_incoming_unknown_neighbour_returns_unknown_mapping() {
        let state = fresh_state();
        let peer = fresh_peer();
        let err = state.translate_incoming(peer, Space::User, 5).unwrap_err();
        assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
    }

    #[test]
    fn translate_incoming_known_neighbour_unknown_idx_returns_unknown_mapping() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let err = state.translate_incoming(peer, Space::User, 5).unwrap_err();
        assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
    }

    /// If our own dict already has a binding for the ID, return the
    /// existing own_idx; do not allocate, do not mark the tracker.
    #[test]
    fn translate_incoming_existing_own_binding_returns_existing_idx() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [7; 8];

        bind_mirror(&state, peer, Space::User, 5, id);
        state.user_dict.write().unwrap().bind(99, id);

        let got = state.translate_incoming(peer, Space::User, 5).unwrap();
        assert_eq!(got, 99);

        let pending = state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(Space::User);
        assert!(
            pending.is_empty(),
            "existing-binding path must not touch the tracker"
        );
    }

    /// Fresh allocation: not in own dict yet → allocate, bind, mark.
    #[test]
    fn translate_incoming_fresh_allocates_binds_and_marks_tracker() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [11; 8];
        bind_mirror(&state, peer, Space::User, 5, id);

        let allocated_idx = state.translate_incoming(peer, Space::User, 5).unwrap();

        let dict = state.user_dict.read().unwrap();
        assert_eq!(dict.idx_of(&id), Some(allocated_idx));
        assert_eq!(dict.id_of(allocated_idx), Some(id));
        drop(dict);

        let pending = state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(Space::User);
        assert!(pending.contains(&allocated_idx));
    }

    /// Translating the same neighbour idx twice yields the same own idx
    /// — the second call hits the "existing own binding" path.
    #[test]
    fn translate_incoming_is_idempotent_for_same_id() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [13; 8];
        bind_mirror(&state, peer, Space::User, 5, id);

        let first = state.translate_incoming(peer, Space::User, 5).unwrap();
        let second = state.translate_incoming(peer, Space::User, 5).unwrap();
        assert_eq!(first, second);
    }

    /// The two index spaces use independent allocators and dicts. A user
    /// binding at neighbour idx 5 must not leak into the node space.
    #[test]
    fn translate_incoming_spaces_are_independent() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let user_id = [21; 8];
        let node_id = [22; 8];

        bind_mirror(&state, peer, Space::User, 5, user_id);
        bind_mirror(&state, peer, Space::Node, 5, node_id);

        let user_idx = state.translate_incoming(peer, Space::User, 5).unwrap();
        let node_idx = state.translate_incoming(peer, Space::Node, 5).unwrap();

        assert_eq!(
            state.user_dict.read().unwrap().id_of(user_idx),
            Some(user_id)
        );
        assert_eq!(
            state.node_dict.read().unwrap().id_of(node_idx),
            Some(node_id)
        );
        // Cross-checks: ids never bleed across spaces.
        assert_eq!(state.node_dict.read().unwrap().idx_of(&user_id), None);
        assert_eq!(state.user_dict.read().unwrap().idx_of(&node_id), None);
    }

    // ---------- pending_introductions ----------

    #[test]
    fn pending_introductions_empty_when_no_marks() {
        let state = fresh_state();
        assert!(state.pending_introductions(Space::User).is_empty());
        assert!(state.pending_introductions(Space::Node).is_empty());
    }

    #[test]
    fn pending_introductions_returns_marked_user_with_correct_version() {
        let state = fresh_state();
        let id = [3; 8];
        install_user(&state, id, 42);
        state.user_dict.write().unwrap().bind(7, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 7);

        let out = state.pending_introductions(Space::User);
        assert_eq!(out, vec![(7, id, 42)]);
    }

    #[test]
    fn pending_introductions_returns_marked_node_with_correct_version() {
        let state = fresh_state();
        let id = [4; 8];
        install_node(&state, id, 99);
        state.node_dict.write().unwrap().bind(8, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::Node, 8);

        let out = state.pending_introductions(Space::Node);
        assert_eq!(out, vec![(8, id, 99)]);
    }

    /// Marks in one space must not leak into the other space's drain.
    #[test]
    fn pending_introductions_drains_only_requested_space() {
        let state = fresh_state();

        let user_id = [1; 8];
        install_user(&state, user_id, 5);
        state.user_dict.write().unwrap().bind(10, user_id);

        let node_id = [2; 8];
        install_node(&state, node_id, 6);
        state.node_dict.write().unwrap().bind(20, node_id);

        {
            let mut t = state.reintroduction_tracker.write().unwrap();
            t.mark_first_time(Space::User, 10);
            t.mark_first_time(Space::Node, 20);
        }

        let users = state.pending_introductions(Space::User);
        assert_eq!(users, vec![(10, user_id, 5)]);

        let nodes = state.pending_introductions(Space::Node);
        assert_eq!(nodes, vec![(20, node_id, 6)]);
    }

    /// `take_pending` is destructive: a second call without new marks
    /// must return an empty Vec.
    #[test]
    fn pending_introductions_second_call_returns_empty_after_drain() {
        let state = fresh_state();
        let id = [9; 8];
        install_user(&state, id, 1);
        state.user_dict.write().unwrap().bind(3, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 3);

        let first = state.pending_introductions(Space::User);
        assert!(!first.is_empty(), "first call should drain the mark");

        let second = state.pending_introductions(Space::User);
        assert!(second.is_empty(), "second call should be empty after drain");
    }

    /// Phase 8's delta encoder requires ascending idx order. HashSet
    /// iteration is non-deterministic, so the sort is what gives the
    /// caller a stable contract.
    #[test]
    fn pending_introductions_results_sorted_by_index() {
        let state = fresh_state();
        let ids: Vec<[u8; 8]> = (1..=5).map(|i| [i as u8; 8]).collect();
        let idxs = [50u16, 10, 200, 30, 80];

        for (i, idx) in idxs.iter().enumerate() {
            install_user(&state, ids[i], i as u32);
            state.user_dict.write().unwrap().bind(*idx, ids[i]);
            state
                .reintroduction_tracker
                .write()
                .unwrap()
                .mark_first_time(Space::User, *idx);
        }

        let out = state.pending_introductions(Space::User);
        let returned_idxs: Vec<u16> = out.iter().map(|(idx, _, _)| *idx).collect();
        let mut expected = idxs.to_vec();
        expected.sort();
        assert_eq!(returned_idxs, expected);
    }

    /// Mark exists but no dict binding — must be silently skipped.
    #[test]
    fn pending_introductions_skips_orphan_with_no_dict_binding() {
        let state = fresh_state();
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 42);

        let out = state.pending_introductions(Space::User);
        assert!(
            out.is_empty(),
            "orphan mark with no dict binding must be skipped"
        );
    }

    /// Mark + dict binding exist, but the users map has no User record
    /// for that id — also skipped.
    #[test]
    fn pending_introductions_skips_orphan_with_no_record() {
        let state = fresh_state();
        let id = [77; 8];
        state.user_dict.write().unwrap().bind(42, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 42);

        let out = state.pending_introductions(Space::User);
        assert!(out.is_empty(), "missing user record must be skipped");
    }

    /// Healthy mark + orphan mark in the same drain: healthy survives,
    /// orphan is dropped, no panic.
    #[test]
    fn pending_introductions_mixed_healthy_and_orphan() {
        let state = fresh_state();

        let good_id = [1; 8];
        install_user(&state, good_id, 7);
        state.user_dict.write().unwrap().bind(10, good_id);

        {
            let mut t = state.reintroduction_tracker.write().unwrap();
            t.mark_first_time(Space::User, 10);
            t.mark_first_time(Space::User, 99); // orphan
        }

        let out = state.pending_introductions(Space::User);
        assert_eq!(out, vec![(10, good_id, 7)]);
    }
}

#[cfg(test)]
mod apply_mapping_tests {
    use super::*;
    use crate::router_v2::{
        codec::messages::Mapping,
        identity::Multikey,
        index::Space,
        table::{Node, RoutingEntry, TargetRef, User},
    };
    use libp2p::identity::Keypair;

    fn fresh_multikey() -> Multikey {
        Multikey::from(Keypair::generate_ed25519().public())
    }

    fn fresh_state() -> RouterV2State {
        RouterV2State::new([0; 8])
    }

    fn fresh_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    fn add_neighbour(state: &RouterV2State) -> PeerId {
        let peer = fresh_peer();
        state.add_neighbour_mirror(peer);
        peer
    }

    fn bind_mirror(state: &RouterV2State, peer: PeerId, space: Space, idx: u16, id: [u8; 8]) {
        let mut mirrors = state.mirrors.write().unwrap();
        let nm = mirrors.get_mut(&peer).unwrap();
        match space {
            Space::Node => nm.nodes.bind(idx, id),
            Space::User => nm.users.bind(idx, id),
        }
    }

    fn install_user(
        state: &RouterV2State,
        id: [u8; 8],
        profile_version: u32,
    ) -> Arc<RwLock<User>> {
        let u = User {
            id,
            public_key: Some(fresh_multikey()),
            profile_version,
            routing_entry: None,
            delegation_gateways: Vec::new(),
        };
        let mut users = state.users.write().unwrap();
        users.insert(id, u);
        users.get(&id).unwrap()
    }

    fn install_node(
        state: &RouterV2State,
        id: [u8; 8],
        manifest_version: u32,
    ) -> Arc<RwLock<Node>> {
        let n = Node {
            id,
            public_key: Some(fresh_multikey()),
            manifest_version,
            is_gateway: false,
            delegated_users: Vec::new(),
        };
        let mut nodes = state.nodes.write().unwrap();
        nodes.insert(id, n);
        nodes.get(&id).unwrap()
    }

    // ---------- unknown neighbour ----------

    #[test]
    fn apply_mapping_unknown_neighbour_is_noop() {
        let state = fresh_state();
        let peer = fresh_peer(); // never registered

        let result = state.apply_mapping(
            peer,
            Space::User,
            Mapping { abs_idx: 5, target_id: [1; 8], version: 42 },
        );

        assert!(result.is_ok());
        assert_eq!(state.users.read().unwrap().len(), 0);
        assert!(state.mirrors.read().unwrap().is_empty());
    }

    // ---------- fresh binding, first sight ----------

    #[test]
    fn apply_mapping_fresh_user_creates_stub_and_binds_mirror() {
        let state = fresh_state();
        let peer = add_neighbour(&state);

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping { abs_idx: 5, target_id: [1; 8], version: 42 },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some([1; 8]));
        drop(mirrors);

        let users = state.users.read().unwrap();
        let user_arc = users.get(&[1; 8]).unwrap();
        let user = user_arc.read().unwrap();
        assert_eq!(user.id, [1; 8]);
        assert_eq!(user.profile_version, 42);
        assert!(user.public_key.is_none(), "stub must not fabricate a key");
    }

    #[test]
    fn apply_mapping_fresh_node_creates_stub_and_binds_mirror() {
        let state = fresh_state();
        let peer = add_neighbour(&state);

        state
            .apply_mapping(
                peer,
                Space::Node,
                Mapping { abs_idx: 5, target_id: [2; 8], version: 99 },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().nodes.id_of(5), Some([2; 8]));
        drop(mirrors);

        let nodes = state.nodes.read().unwrap();
        let node = nodes.get(&[2; 8]).unwrap();
        let n = node.read().unwrap();
        assert_eq!(n.manifest_version, 99);
        assert!(!n.is_gateway, "stub node is not a gateway by default");
        assert!(n.public_key.is_none());
    }

    // ---------- same-id re-mapping ----------

    #[test]
    fn apply_mapping_same_id_updates_version_only() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [3; 8];

        bind_mirror(&state, peer, Space::User, 5, id);
        install_user(&state, id, 10);

        state
            .apply_mapping(peer, Space::User, Mapping { abs_idx: 5, target_id: id, version: 20 })
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(id));
        drop(mirrors);

        let users = state.users.read().unwrap();
        assert_eq!(users.get(&id).unwrap().read().unwrap().profile_version, 20);
    }

    // ---------- rebind: different target_id at existing idx ----------

    /// The critical §8.7-step-2 case: the mirror already has abs_idx bound
    /// to an OLD target_id. Applying a NEW target_id must clear the host's
    /// routing entry for the old target, release the old own_idx into
    /// cooldown, unbind the own dict, then bind the new mapping.
    #[test]
    fn apply_mapping_rebind_clears_old_routing_state() {
        let state = fresh_state();
        let peer = add_neighbour(&state);

        let old_id = [10; 8];
        let new_id = [20; 8];
        let own_idx: u16 = 7;

        bind_mirror(&state, peer, Space::User, 5, old_id);
        let old_user = install_user(&state, old_id, 1);
        state.user_dict.write().unwrap().bind(own_idx, old_id);

        let entry = Arc::new(RwLock::new(RoutingEntry {
            target_index: own_idx,
            target: TargetRef::User(old_user.clone()),
            seq_num: crate::router_v2::seq::SeqNum::from(0u16),
            metric: 5,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update: 0,
            hop_count: 1,
            local_only: false,
        }));
        let entry_weak = Arc::downgrade(&entry);
        state.routing_table.write().unwrap().set(Space::User, own_idx, entry);
        old_user.write().unwrap().routing_entry = Some(entry_weak.clone());

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping { abs_idx: 5, target_id: new_id, version: 1 },
            )
            .unwrap();

        // Old routing table slot cleared; cycle-discipline Weak dangles.
        assert!(state.routing_table.read().unwrap().get(Space::User, own_idx).is_none());
        assert!(entry_weak.upgrade().is_none(), "old routing entry Arc must be dropped");

        // Old own_idx unbound from own dict.
        assert_eq!(state.user_dict.read().unwrap().idx_of(&old_id), None);
        assert_eq!(state.user_dict.read().unwrap().id_of(own_idx), None);

        // Old own_idx released into cooldown.
        assert!(state.users_allocator.read().unwrap().idx_in_cooldown(own_idx));

        // Mirror now bound to NEW.
        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(new_id));
        drop(mirrors);

        // NEW user stub created.
        assert!(state.users.read().unwrap().get(&new_id).is_some());
    }

    // ---------- version comparison branches ----------

    #[test]
    fn apply_mapping_incoming_version_equal_is_noop() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [4; 8];
        install_user(&state, id, 42);

        state
            .apply_mapping(peer, Space::User, Mapping { abs_idx: 5, target_id: id, version: 42 })
            .unwrap();

        assert_eq!(
            state
                .users
                .read()
                .unwrap()
                .get(&id)
                .unwrap()
                .read()
                .unwrap()
                .profile_version,
            42,
        );
    }

    #[test]
    fn apply_mapping_incoming_version_older_preserves_stored() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [5; 8];
        install_user(&state, id, 100);

        state
            .apply_mapping(peer, Space::User, Mapping { abs_idx: 5, target_id: id, version: 50 })
            .unwrap();

        assert_eq!(
            state
                .users
                .read()
                .unwrap()
                .get(&id)
                .unwrap()
                .read()
                .unwrap()
                .profile_version,
            100,
            "stale-echo path must NOT overwrite the fresher stored version",
        );
    }

    #[test]
    fn apply_mapping_incoming_version_fresher_updates_node() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [6; 8];
        install_node(&state, id, 5);

        state
            .apply_mapping(peer, Space::Node, Mapping { abs_idx: 5, target_id: id, version: 12 })
            .unwrap();

        assert_eq!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&id)
                .unwrap()
                .read()
                .unwrap()
                .manifest_version,
            12,
        );
    }

    // ---------- spaces are independent ----------

    #[test]
    fn apply_mapping_user_and_node_spaces_are_independent() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let user_id = [11; 8];
        let node_id = [22; 8];

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping { abs_idx: 5, target_id: user_id, version: 1 },
            )
            .unwrap();
        state
            .apply_mapping(
                peer,
                Space::Node,
                Mapping { abs_idx: 5, target_id: node_id, version: 1 },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        let nm = mirrors.get(&peer).unwrap();
        assert_eq!(nm.users.id_of(5), Some(user_id));
        assert_eq!(nm.nodes.id_of(5), Some(node_id));
        drop(mirrors);

        assert!(state.users.read().unwrap().get(&user_id).is_some());
        assert!(state.users.read().unwrap().get(&node_id).is_none());
        assert!(state.nodes.read().unwrap().get(&node_id).is_some());
        assert!(state.nodes.read().unwrap().get(&user_id).is_none());
    }
}
