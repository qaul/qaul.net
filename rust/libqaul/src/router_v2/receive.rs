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
            messages::{Mapping, NodeEntry, NodeManifest, RoutingUpdate, UserEntry},
            CodecError, Header, RoutingMessage,
        },
        identity::Multikey,
        index::Space,
        manifest::{Manifest, ManifestLog},
        metric::hop_cost,
        seq::{is_fresher_u32, Acceptance, SeqNum},
        table::{DelegatedUser, Node, RoutingEntry, TargetRef, User},
        Result, RouterV2State, RoutingV2Error, Sphere,
    },
};

pub(crate) struct ReceiveCtx {
    pub neighbour: PeerId,
    pub transport: ConnectionModule,
    pub rssi_dbm: Option<i8>,
    pub now: u64,
}

struct EntryArg {
    pub abs_idx: u16,
    pub seq: u16,
    pub metric: u16,
    pub hop_count: u8,
    pub local_only: bool,
}

impl From<&UserEntry> for EntryArg {
    fn from(e: &UserEntry) -> Self {
        Self {
            abs_idx: e.abs_idx,
            seq: e.seq,
            metric: e.metric,
            hop_count: e.hop_count,
            local_only: e.local_only,
        }
    }
}

impl From<&NodeEntry> for EntryArg {
    fn from(e: &NodeEntry) -> Self {
        Self {
            abs_idx: e.abs_idx,
            seq: e.seq,
            metric: e.metric,
            hop_count: e.hop_count,
            local_only: e.local_only,
        }
    }
}

struct AcceptedEntry {
    own_idx: u16,
    target: TargetRef,
    seq: u16,
    metric: u16,
    next_hop_idx: u16,
    hop_count: u8,
    local_only: bool,
}

enum EvaluateOutcome {
    Accept(AcceptedEntry),
    /// we successfly translated the target, but relay-inclusion rejected it.
    RejectedButTargetKnown {
        target_ref: TargetRef,
    },
    /// dropped before target resolution (unknown mapping, TTL, missing target).
    Dropped,
}

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
                        let advertised = {
                            let n = node.read().unwrap();
                            n.advertised_version
                        };

                        if is_fresher_u32(mapping.version, advertised) {
                            let mut n = node.write().unwrap();
                            n.advertised_version = mapping.version;
                            // TODO(Phase 10b): if mapping.version > n.manifest_version trigger a MANIFEST_REQUEST pull.
                        } else {
                            debug!("stale node advertisement from {neighbour:?}: target={:?} stored_advertised={advertised} incoming={}",
                                mapping.target_id, mapping.version);
                        }
                    }
                    None => {
                        let n = Node {
                            id: mapping.target_id,
                            public_key: None,
                            manifest_version: 0,
                            advertised_version: mapping.version,
                            is_gateway: false,
                            delegated_users: Vec::new(),
                            manifest_signature: None,
                            retained_chunks: None,
                            learn_sphere: None,
                            manifest_log: ManifestLog::default(),
                        };
                        nodes.insert(mapping.target_id, n);
                        // TODO(Phase 10b): if mapping.version > 0, enqueue a MANIFEST_REQUEST
                    }
                }
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

    fn lookup_target(&self, space: Space, own_idx: u16) -> Option<TargetRef> {
        match space {
            Space::User => {
                let dict = self.user_dict.read().unwrap();
                let id = dict.id_of(own_idx)?;
                drop(dict);
                let users = self.users.read().unwrap();
                users.get(&id).map(TargetRef::User)
            }
            Space::Node => {
                let dict = self.node_dict.read().unwrap();
                let id = dict.id_of(own_idx)?;
                drop(dict);
                let nodes = self.nodes.read().unwrap();
                nodes.get(&id).map(TargetRef::Node)
            }
        }
    }

    fn evaluate_entry(
        &self,
        ctx: &ReceiveCtx,
        space: Space,
        entry: EntryArg,
    ) -> Result<EvaluateOutcome> {
        if entry.hop_count >= 63 {
            return Ok(EvaluateOutcome::Dropped);
        }

        let metric = entry
            .metric
            .saturating_add(hop_cost(ctx.transport, ctx.rssi_dbm));

        let own_idx = match self.translate_incoming(ctx.neighbour, space, entry.abs_idx) {
            Ok(idx) => idx,
            Err(RoutingV2Error::UnknownMapping(_)) => return Ok(EvaluateOutcome::Dropped),
            Err(e) => return Err(e),
        };

        let Some(target) = self.lookup_target(space, own_idx) else {
            debug!("receive-loop drop: target lookup failed (space={space:?}, own_idx={own_idx})");
            return Ok(EvaluateOutcome::Dropped);
        };

        // §7.2 relay-inclusion + §7.4 local_only monotonicity.
        let (accept, local_only) = {
            let rt = self.routing_table.read().unwrap();
            match rt.get(space, own_idx) {
                None => (true, entry.local_only),
                Some(existing) => {
                    let stored = existing.read().unwrap();
                    let accept = match stored.seq_num.acceptance(SeqNum::from(entry.seq)) {
                        Acceptance::Fresher | Acceptance::Reboot => true,
                        Acceptance::NoChange => metric < stored.metric,
                    };
                    (accept, stored.local_only && entry.local_only)
                }
            }
        };
        if !accept {
            debug!(
                    "receive-loop drop: not better than stored (own_idx={own_idx}, seq={}, metric={metric})",
                    entry.seq
                );
            return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
        }

        let neighbour_node_id = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(info) = mirrors.get(&ctx.neighbour) else {
                debug!("neighbour vanished mid-receive: {:?}", ctx.neighbour);
                return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
            };
            info.node_id
        };
        let next_hop_idx = {
            let dict = self.node_dict.read().unwrap();
            match dict.idx_of(&neighbour_node_id) {
                Some(idx) => idx,
                None => {
                    debug!("neighbour node_id has no node_dict entry: {neighbour_node_id:?}");
                    return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
                }
            }
        };

        Ok(EvaluateOutcome::Accept(AcceptedEntry {
            own_idx,
            target,
            seq: entry.seq,
            metric,
            next_hop_idx,
            hop_count: entry.hop_count,
            local_only,
        }))
    }

    pub fn apply_user_entry(&self, ctx: &ReceiveCtx, entry: UserEntry) -> Result<()> {
        match self.evaluate_entry(&ctx, Space::User, (&entry).into())? {
            EvaluateOutcome::Accept(a) => {
                self.commit_routing_entry(ctx, Space::User, a);
            }
            _ => {}
        };
        Ok(())
    }

    pub fn apply_node_entry(&self, ctx: &ReceiveCtx, entry: NodeEntry) -> Result<()> {
        let outcome = self.evaluate_entry(ctx, Space::Node, (&entry).into())?;
        let target = match &outcome {
            EvaluateOutcome::Accept(a) => Some(&a.target),
            EvaluateOutcome::RejectedButTargetKnown { target_ref } => Some(target_ref),
            EvaluateOutcome::Dropped => None,
        };

        if let Some(TargetRef::Node(n)) = target {
            let mut node = n.write().unwrap();
            if is_fresher_u32(entry.manifest_version, node.advertised_version) {
                node.advertised_version = entry.manifest_version;
                // TODO(Phase 10b): if entry.manifest_version > node.manifest_version,
                //   maybe_enqueue_request(state, ctx.neighbour, target_id, entry.manifest_version).
            }
        }

        if let EvaluateOutcome::Accept(accepted) = outcome {
            self.commit_routing_entry(ctx, Space::Node, accepted);
        }

        Ok(())
    }

    fn commit_routing_entry(&self, ctx: &ReceiveCtx, space: Space, accepted: AcceptedEntry) {
        let new_entry = Arc::new(RwLock::new(RoutingEntry {
            target: accepted.target,
            target_index: accepted.own_idx,
            seq_num: SeqNum::from(accepted.seq),
            metric: accepted.metric,
            next_hop: accepted.next_hop_idx,
            transport: ctx.transport,
            last_update: ctx.now,
            hop_count: accepted.hop_count.saturating_add(1),
            local_only: accepted.local_only,
        }));

        if let TargetRef::User(user) = &new_entry.read().unwrap().target {
            user.write().unwrap().routing_entry = Some(Arc::downgrade(&new_entry));
        }

        self.routing_table
            .write()
            .unwrap()
            .set(space, accepted.own_idx, new_entry);

        self.relay_queue
            .write()
            .unwrap()
            .insert((space, accepted.own_idx));
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

        let ctx = ReceiveCtx {
            neighbour,
            transport,
            rssi_dbm,
            now,
        };
        for entry in msg.user_entries {
            if let Err(e) = self.apply_user_entry(&ctx, entry) {
                warn!("apply_user_entry failed: {e}");
            }
        }

        for entry in msg.node_entries {
            if let Err(e) = self.apply_node_entry(&ctx, entry) {
                warn!("apply_node_entry failed: {e}");
            }
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
                        if let Err(e) = self.handle_node_manifest(neighbour, msg, now, transport) {
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
        mut msg: NodeManifest,
        now: u64,
        transport: ConnectionModule,
    ) -> Result<()> {
        let origin_node_id = msg.origin_node_id;

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
                                    profile_version: entry.profile_version,
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
                        // TODO(Phase 3 codec update): once ManifestEntry
                        // carries profile_version on the wire (spec §10.1),
                        // read it from `entry.profile_version` here.
                        profile_version: 0,
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

        self.manifest_relay_queue.write().unwrap().insert(
            origin_node_id,
            (completed_manifest.chunks, Sphere::of(transport)),
        );

        Ok(())
    }
}
