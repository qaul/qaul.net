use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use libp2p::PeerId;

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::CodecError,
        index::{IndexDictionary, MirrorIndexDictionary},
        table::{Nodes, RoutingTable, Users},
    },
};

pub mod codec;
pub mod identity;
pub mod index;
pub mod table;
pub mod seq;

#[derive(Debug, thiserror::Error)]
pub enum RoutingV2Error {
    MultikeyErrror(#[from] libp2p::identity::DecodingError),
    CodecError(#[from] CodecError),
}

impl std::fmt::Display for RoutingV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingV2Error::MultikeyErrror(e) => write!(f, "{e}"),
            RoutingV2Error::CodecError(e) => write!(f, "{e}"),
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
}

impl RouterV2State {
    pub fn new(host_node_id: [u8; 8]) -> Self {
        Self {
            user_dict: RwLock::new(IndexDictionary::new(None)),
            node_dict: RwLock::new(IndexDictionary::new(Some(host_node_id))),
            mirrors: RwLock::new(HashMap::new()),
            routing_table: Arc::new(RwLock::new(RoutingTable::new())),
            users: Arc::new(RwLock::new(Users::new())),
            nodes: Arc::new(RwLock::new(Nodes::new())),
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
            public_key: fresh_multikey(),
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
    fn register_node(
        state: &RouterV2State,
        id: [u8; 8],
        is_gateway: bool,
    ) -> Arc<RwLock<Node>> {
        let node = Node {
            id,
            public_key: fresh_multikey(),
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
            seq_num: 0,
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
        let orphan = make_entry(
            TargetRef::User(user.clone()),
            0,
            100,
            ConnectionModule::Lan,
        );
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
            public_key: fresh_multikey(),
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
            u.delegation_gateways =
                vec![Arc::downgrade(&unreachable), Arc::downgrade(&reachable)];
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
