// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The routing table and everything regarding routing
//! Cycle discipline for in-memory routing state (spec A.3):
//!
//!   RoutingEntry.target (User or Node) → strong
//!   User.routing_entry                  → weak
//!   Node.delegated_users[].user         → strong
//!   User.delegation_gateways[]          → weak
//!
//! Forward edges (entry → target, manifest → user) own lifetimes.
//! Back edges resolve to None when the strong side disappears.

use crate::{
    connections::ConnectionModule,
    router_v2::{identity::Multikey, index::Space, seq::SeqNum},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

#[derive(Debug)]
pub struct User {
    pub id: [u8; 8],
    pub public_key: Option<Multikey>,
    pub profile_version: u32,
    pub routing_entry: Option<Weak<RwLock<RoutingEntry>>>,
    pub delegation_gateways: Vec<Weak<RwLock<Node>>>,
}

/// A node
#[derive(Debug)]
pub struct Node {
    pub id: [u8; 8],
    pub public_key: Option<Multikey>,
    pub manifest_version: u32,
    pub is_gateway: bool,
    pub delegated_users: Vec<DelegatedUser>,
}

/// a user that has delegated its global reachability to thus node
#[derive(Debug)]
pub struct DelegatedUser {
    pub user_id: [u8; 8],
    pub user: Arc<RwLock<User>>,
    pub delegation_timeout: u64,
    pub entry_signature: [u8; 64],
}

#[derive(Debug)]
pub enum TargetRef {
    User(Arc<RwLock<User>>),
    Node(Arc<RwLock<Node>>),
}

/// An entry in a routing table
#[derive(Debug)]
pub struct RoutingEntry {
    /// resolved absolute index of the target in its index space
    pub target_index: u16,
    /// strong reference to the User or Node this entry describes a route to
    pub target: TargetRef,
    /// origin's current sequence number
    pub seq_num: SeqNum,
    pub metric: u16,
    /// neighbour's node-space index in our dictionary which is the next forwarding hop to this curent target
    pub next_hop: u16,
    /// the transport on which this entry was propagated
    pub transport: ConnectionModule,
    /// receipt timestamp (ms since epoch)
    pub last_update: u64,
    ///
    pub hop_count: u8,
    pub local_only: bool,
}

impl RoutingEntry {
    pub fn target_kind(&self) -> Space {
        match &self.target {
            TargetRef::Node(_) => Space::Node,
            TargetRef::User(_) => Space::User,
        }
    }

    pub fn target_is_gateway(&self) -> bool {
        match &self.target {
            TargetRef::User(_) => false,
            TargetRef::Node(n) => n.read().unwrap().is_gateway,
        }
    }
}

/// map of every Node identity we have routing state for keyed by 8-byte ID
#[derive(Debug)]
pub struct Nodes(HashMap<[u8; 8], Arc<RwLock<Node>>>);

/// map of every User identity we have routing state for keyed by 8-byte ID
#[derive(Debug)]
pub struct Users(HashMap<[u8; 8], Arc<RwLock<User>>>);

impl Nodes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, id: &[u8; 8]) -> Option<Arc<RwLock<Node>>> {
        self.0.get(id).cloned()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, id: [u8; 8], n: Node) {
        self.0.insert(id, Arc::new(RwLock::new(n)));
    }

    pub fn remove(&mut self, id: &[u8; 8]) -> Option<Arc<RwLock<Node>>> {
        self.0.remove(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&[u8; 8], &Arc<RwLock<Node>>)> {
        self.0.iter()
    }
}

impl Users {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, id: &[u8; 8]) -> Option<Arc<RwLock<User>>> {
        self.0.get(id).cloned()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, id: [u8; 8], u: User) {
        self.0.insert(id, Arc::new(RwLock::new(u)));
    }

    pub fn remove(&mut self, id: &[u8; 8]) -> Option<Arc<RwLock<User>>> {
        self.0.remove(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&[u8; 8], &Arc<RwLock<User>>)> {
        self.0.iter()
    }
}

/// The actual routing table
pub struct RoutingTable {
    pub user_entries: Vec<Option<Arc<RwLock<RoutingEntry>>>>,
    pub node_entries: Vec<Option<Arc<RwLock<RoutingEntry>>>>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            // because the max entires a table can hold at a time is 65k
            user_entries: vec![None; 65_536],
            node_entries: vec![None; 65_536],
        }
    }

    pub fn get(&self, space: Space, idx: u16) -> Option<Arc<RwLock<RoutingEntry>>> {
        match space {
            Space::Node => self.node_entries[idx as usize].clone(),
            Space::User => self.user_entries[idx as usize].clone(),
        }
    }

    pub fn set(&mut self, space: Space, idx: u16, entry: Arc<RwLock<RoutingEntry>>) {
        match space {
            Space::Node => self.node_entries[idx as usize] = Some(entry),
            Space::User => self.user_entries[idx as usize] = Some(entry),
        }
    }

    pub fn clear(&mut self, space: Space, idx: u16) -> Option<Arc<RwLock<RoutingEntry>>> {
        match space {
            Space::Node => self.node_entries[idx as usize].take(),
            Space::User => self.user_entries[idx as usize].take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    fn fresh_multikey() -> Multikey {
        Multikey::from(Keypair::generate_ed25519().public())
    }

    fn make_user(id: [u8; 8]) -> User {
        User {
            id,
            public_key: Some(fresh_multikey()),
            profile_version: 0,
            routing_entry: None,
            delegation_gateways: Vec::new(),
        }
    }

    fn make_node(id: [u8; 8]) -> Node {
        Node {
            id,
            public_key: Some(fresh_multikey()),
            manifest_version: 0,
            is_gateway: false,
            delegated_users: Vec::new(),
        }
    }

    fn make_entry(target: TargetRef) -> Arc<RwLock<RoutingEntry>> {
        Arc::new(RwLock::new(RoutingEntry {
            target_index: 0,
            target,
            seq_num: SeqNum::from(0u16),
            metric: 0,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update: 0,
            hop_count: 0,
            local_only: false,
        }))
    }

    fn dummy_user_arc() -> Arc<RwLock<User>> {
        Arc::new(RwLock::new(make_user([0; 8])))
    }

    #[test]
    fn routing_table_new_pre_allocates_full_index_space() {
        let table = RoutingTable::new();
        assert_eq!(table.user_entries.len(), 65_536);
        assert_eq!(table.node_entries.len(), 65_536);
        assert!(table.user_entries.iter().all(|s| s.is_none()));
        assert!(table.node_entries.iter().all(|s| s.is_none()));
    }

    #[test]
    fn routing_table_set_then_get_round_trips_in_user_space() {
        let mut table = RoutingTable::new();
        let entry = make_entry(TargetRef::User(dummy_user_arc()));
        table.set(Space::User, 42, entry.clone());
        let got = table.get(Space::User, 42).unwrap();
        assert!(Arc::ptr_eq(&got, &entry));
    }

    #[test]
    fn routing_table_user_and_node_spaces_are_independent() {
        let mut table = RoutingTable::new();
        let entry = make_entry(TargetRef::User(dummy_user_arc()));
        table.set(Space::User, 42, entry);
        // Same idx in the other space must still be empty.
        assert!(table.get(Space::Node, 42).is_none());
    }

    #[test]
    fn routing_table_get_on_empty_slot_returns_none() {
        let table = RoutingTable::new();
        assert!(table.get(Space::User, 1234).is_none());
        assert!(table.get(Space::Node, 1234).is_none());
    }

    #[test]
    fn routing_table_set_overwrites_existing_entry() {
        let mut table = RoutingTable::new();
        let e1 = make_entry(TargetRef::User(dummy_user_arc()));
        let e2 = make_entry(TargetRef::User(dummy_user_arc()));
        table.set(Space::User, 7, e1);
        table.set(Space::User, 7, e2.clone());
        let got = table.get(Space::User, 7).unwrap();
        assert!(Arc::ptr_eq(&got, &e2));
    }

    #[test]
    fn routing_table_clear_returns_old_entry_and_empties_slot() {
        let mut table = RoutingTable::new();
        let entry = make_entry(TargetRef::User(dummy_user_arc()));
        table.set(Space::User, 99, entry.clone());

        let cleared = table.clear(Space::User, 99).unwrap();
        assert!(Arc::ptr_eq(&cleared, &entry));
        assert!(table.get(Space::User, 99).is_none());
    }

    #[test]
    fn routing_table_clear_on_empty_slot_returns_none() {
        let mut table = RoutingTable::new();
        assert!(table.clear(Space::User, 0).is_none());
        assert!(table.clear(Space::Node, 65_535).is_none());
    }

    #[test]
    fn routing_table_extreme_indices_work() {
        let mut table = RoutingTable::new();
        let entry = make_entry(TargetRef::User(dummy_user_arc()));
        table.set(Space::Node, 0, entry.clone());
        table.set(Space::Node, 65_535, entry.clone());
        assert!(table.get(Space::Node, 0).is_some());
        assert!(table.get(Space::Node, 65_535).is_some());
    }

    #[test]
    fn nodes_new_is_empty() {
        let nodes = Nodes::new();
        assert_eq!(nodes.len(), 0);
    }

    #[test]
    fn nodes_insert_then_get_round_trips() {
        let mut nodes = Nodes::new();
        let id = [1; 8];
        nodes.insert(id, make_node(id));
        let got = nodes.get(&id).unwrap();
        assert_eq!(got.read().unwrap().id, id);
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn nodes_get_on_missing_returns_none() {
        let nodes = Nodes::new();
        assert!(nodes.get(&[1; 8]).is_none());
    }

    #[test]
    fn nodes_remove_drops_node_and_returns_it() {
        let mut nodes = Nodes::new();
        let id = [2; 8];
        nodes.insert(id, make_node(id));
        let removed = nodes.remove(&id).unwrap();
        assert_eq!(removed.read().unwrap().id, id);
        assert_eq!(nodes.len(), 0);
        assert!(nodes.get(&id).is_none());
    }

    #[test]
    fn nodes_iter_yields_every_entry() {
        let mut nodes = Nodes::new();
        for i in 0..5u8 {
            let id = [i; 8];
            nodes.insert(id, make_node(id));
        }
        let collected: Vec<[u8; 8]> = nodes.iter().map(|(id, _)| *id).collect();
        assert_eq!(collected.len(), 5);
    }

    #[test]
    fn users_insert_then_get_round_trips() {
        let mut users = Users::new();
        let id = [3; 8];
        users.insert(id, make_user(id));
        let got = users.get(&id).unwrap();
        assert_eq!(got.read().unwrap().id, id);
    }

    #[test]
    fn users_remove_drops_user() {
        let mut users = Users::new();
        let id = [4; 8];
        users.insert(id, make_user(id));
        users.remove(&id);
        assert!(users.get(&id).is_none());
        assert_eq!(users.len(), 0);
    }

    /// Routing entries are owned by the routing table. When the table
    /// drops its strong reference, the User's back-edge Weak must resolve
    /// to None.
    #[test]
    fn weak_routing_entry_resolves_to_none_after_table_clears_slot() {
        let mut table = RoutingTable::new();
        let user = Arc::new(RwLock::new(make_user([5; 8])));
        let entry = make_entry(TargetRef::User(user.clone()));
        let weak = Arc::downgrade(&entry);

        user.write().unwrap().routing_entry = Some(weak.clone());
        table.set(Space::User, 1, entry);
        // After `set`, the only remaining strong ref is inside the table.

        assert!(
            weak.upgrade().is_some(),
            "weak must still upgrade while table holds the strong",
        );

        table.clear(Space::User, 1);
        assert!(
            weak.upgrade().is_none(),
            "weak must resolve to None after the table drops the strong",
        );
    }

    /// Gateways are owned by the Nodes map. When the map drops its strong
    /// reference, the User's `delegation_gateways` Weak entries must
    /// resolve to None.
    #[test]
    fn weak_node_resolves_to_none_after_nodes_map_drops_strong() {
        let mut nodes = Nodes::new();
        let id = [6; 8];
        nodes.insert(id, make_node(id));

        let weak = {
            let arc = nodes.get(&id).unwrap();
            Arc::downgrade(&arc)
        };
        // The cloned Arc from `get` dropped at the end of the block; only
        // the map's strong remains.
        assert!(weak.upgrade().is_some());

        nodes.remove(&id);
        assert!(weak.upgrade().is_none());
    }
}
