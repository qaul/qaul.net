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
    router_v2::{identity::Multikey, index::Space},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

#[derive(Debug)]
pub struct User {
    pub id: [u8; 8],
    pub public_key: Multikey,
    pub profile_version: u32,
    pub routing_entry: Option<Weak<RwLock<RoutingEntry>>>,
    pub delegation_gateways: Vec<Weak<RwLock<Node>>>,
}

/// A node
#[derive(Debug)]
pub struct Node {
    pub id: [u8; 8],
    pub public_key: Multikey,
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
    pub seq_num: u16,
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
