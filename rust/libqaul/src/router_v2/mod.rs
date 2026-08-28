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
    collections::{HashMap, HashSet, VecDeque},
    sync::{atomic::AtomicU32, Arc, RwLock},
};

use libp2p::{identity::Keypair, PeerId};
use tokio::sync::mpsc;

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::CodecError,
        delegation::{OutstandingSubscribe, PendingRevocation, Subscription},
        identity::Multikey,
        index::{
            IndexAllocator, IndexDictionary, MirrorIndexDictionary, ReintroductionTracker, Space,
        },
        management::{delegation::PendingDelegation, profile::HostedProfile},
        manifest::{ChunkAssembler, Manifest, ManifestLog},
        seq::SeqNum,
        table::{Nodes, RoutingTable, Users},
    },
    storage::configuration::RoutingV2Options,
};

pub mod codec;
pub mod delegation;
pub mod forwarding;
pub mod identity;
pub mod index;
pub mod init;
pub mod management;
pub mod manifest;
pub mod metric;
pub mod propagation;
pub mod receive;
pub mod seq;
pub mod state;
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
            // per 4.1
            ConnectionModule::Internet => Sphere::Internet,
            ConnectionModule::Lan => Sphere::Local,
            ConnectionModule::Ble1m => Sphere::Local,
            ConnectionModule::BleCoded => Sphere::Local,
            ConnectionModule::Local => Sphere::Local,
            ConnectionModule::None => Sphere::Local,
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

/// spec 10.8 says why a `manifest_version` bump is being attempted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BumpTrigger {
    Accumulated,
    FormTransition,
    /// removal forced by loss of routing reachability to a delegated user
    ForcedRemoval,
}

/// groups one user-space and one node-space mirror per neigbour
#[derive(Debug, Default)]
pub struct NeighbourInfo {
    pub node_id: [u8; 8],
    pub users: MirrorIndexDictionary,
    pub nodes: MirrorIndexDictionary,
    pub transports: HashSet<ConnectionModule>,
    pub dump_stale: DumpStaleIndexes,
}

/// Per-space leftovers of an `INDEX_DUMP` in progress (§8.4).
#[derive(Debug, Default)]
pub struct DumpStaleIndexes {
    pub users: HashSet<u16>,
    pub nodes: HashSet<u16>,
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
            dump_stale: DumpStaleIndexes::default(),
        }
    }
}

/// Which libp2p behaviour carries an outbound frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutboundKind {
    /// A routing protocol frame (§8), over the qaul_info pipe.
    Routing,
    /// A `ManagementMessage` envelope (§11.3), over qaul_management.
    Management,
}

/// the shape for a message to be sent over the wire
#[derive(Debug, Clone)]
pub struct OutboundMsg {
    pub peer: PeerId,
    pub transport: ConnectionModule,
    pub kind: OutboundKind,
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
    pub last_manifest_bump_ms: RwLock<u64>,
    /// This origin's own delta log (§10.9).
    pub own_manifest_log: RwLock<ManifestLog>,
    /// User ids whose delegation entry has changed since the last bump
    pub dirty_delegations: RwLock<HashSet<[u8; 8]>>,
    pub dirty_manifest_flags: RwLock<bool>,
    /// per 8.7: the origins we intend to ask our next neighbour about
    pub pending_manifest_requests: RwLock<HashMap<PeerId, HashSet<[u8; 8]>>>,
    /// current requests that are in flight
    pub outstanding_manifest_requests: RwLock<HashMap<([u8; 8], PeerId), u64>>,
    /// profiles this node hosts
    pub hosted_profiles: RwLock<HashMap<[u8; 8], HostedProfile>>,
    /// `(source, request_id)` of recently forwarded management messages,
    /// with the time forwarded.
    pub management_recent_forwards: RwLock<HashMap<([u8; 8], u32), u64>>,
    /// §11.5 profile fetches that are waiting for response.
    /// key is (subject, is_node)
    pub management_in_flight: RwLock<HashMap<([u8; 8], bool), u64>>,
    /// subscribes parked until the delegating user's key arrives per 11.6
    pub(crate) pending_subscribes: RwLock<HashMap<[u8; 8], Vec<PendingDelegation>>>,
    /// subscribes we sent, keyed by request_id so the ack can be matched, per 11.6
    pub(crate) outstanding_subscribes: RwLock<HashMap<u32, OutstandingSubscribe>>,
    /// cross-host delegations a gateway has accepted
    pub(crate) subscriptions: RwLock<HashMap<[u8; 8], Subscription>>,
    /// (user, node) pairs that refused or went silent and when
    pub(crate) declined_targets: RwLock<HashMap<([u8; 8], [u8; 8]), u64>>,
    /// §10.7: when each carried user was last seen reachable
    pub(crate) delegation_liveness: RwLock<HashMap<[u8; 8], u64>>,
    /// §10.5 revocations waiting for the layer above to sign and send
    pub(crate) pending_revocations: RwLock<Vec<PendingRevocation>>,
    /// §11.3 request_id source
    pub next_request_id: AtomicU32,
    /// spec section 14 sliding windows
    pub manifest_request_window: RwLock<HashMap<PeerId, VecDeque<u64>>>,
    pub manifest_serve_window: RwLock<HashMap<PeerId, VecDeque<u64>>>,
    /// spec 3.5
    pub propagation_form: RwLock<PropagationForm>,
}

impl RouterV2State {
    pub fn new(
        host_keypair: Keypair,
        host_multikey: Multikey,
        options: RoutingV2Options,
    ) -> (Self, mpsc::UnboundedReceiver<OutboundMsg>) {
        let (tx, rx) = mpsc::unbounded_channel::<OutboundMsg>();
        // §3.7: config carries the cooldown in seconds, the allocator runs on
        // the same epoch-millisecond clock as the rest of the router.
        let idx_cooldown_ms = options.idx_cooldown.saturating_mul(1000);
        let state = Self {
            options,
            user_dict: RwLock::new(IndexDictionary::new(None)),
            node_dict: RwLock::new(IndexDictionary::new(None)),
            mirrors: RwLock::new(HashMap::new()),
            routing_table: Arc::new(RwLock::new(RoutingTable::new())),
            users: Arc::new(RwLock::new(Users::new())),
            nodes: Arc::new(RwLock::new(Nodes::new())),
            users_allocator: RwLock::new(IndexAllocator::new(idx_cooldown_ms)),
            node_allocator: RwLock::new(IndexAllocator::new(idx_cooldown_ms)),
            reintroduction_tracker: RwLock::new(ReintroductionTracker::new()),
            seq_num: RwLock::new(SeqNum::new()),
            tx_outbound: tx,
            relay_queue: RwLock::new(HashSet::new()),
            manifest: RwLock::new(Manifest::new()),
            chunk_assembler: RwLock::new(ChunkAssembler::new()),
            host_keypair,
            host_mk: host_multikey,
            last_manifest_bump_ms: RwLock::new(0u64),
            own_manifest_log: RwLock::new(ManifestLog::default()),
            dirty_delegations: RwLock::new(HashSet::new()),
            dirty_manifest_flags: RwLock::new(false),
            pending_manifest_requests: RwLock::new(HashMap::new()),
            outstanding_manifest_requests: RwLock::new(HashMap::new()),
            hosted_profiles: RwLock::new(HashMap::new()),
            management_recent_forwards: RwLock::new(HashMap::new()),
            management_in_flight: RwLock::new(HashMap::new()),
            pending_subscribes: RwLock::new(HashMap::new()),
            outstanding_subscribes: RwLock::new(HashMap::new()),
            subscriptions: RwLock::new(HashMap::new()),
            declined_targets: RwLock::new(HashMap::new()),
            delegation_liveness: RwLock::new(HashMap::new()),
            pending_revocations: RwLock::new(Vec::new()),
            next_request_id: AtomicU32::new(1),
            manifest_request_window: RwLock::new(HashMap::new()),
            manifest_serve_window: RwLock::new(HashMap::new()),
            propagation_form: RwLock::new(PropagationForm::User),
        };
        (state, rx)
    }
}

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests;
