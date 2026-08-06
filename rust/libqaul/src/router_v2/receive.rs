// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Receive-side handlers for router_v2.

use std::sync::{Arc, RwLock};

use libp2p::PeerId;
use tracing::{debug, error, info, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{
            messages::{
                IndexDump, ManifestDelta, ManifestEntry, ManifestRequest, ManifestRequestItem,
                Mapping, NodeEntry, NodeManifest, RoutingUpdate, UserEntry,
            },
            CodecError, Header, RoutingMessage,
        },
        identity::{ChunkSigningCtx, Multikey},
        index::Space,
        manifest::{
            canonical_entry_bytes, decide_serve, reconstruct_single_chunk_full, DeltaHeader,
            Manifest, ManifestLog, OriginServeState, ServeDecision,
        },
        metric::hop_cost,
        seq::{is_fresher_u32, Acceptance, SeqNum},
        table::{DelegatedUser, Node, RoutingEntry, TargetRef, User},
        OutboundMsg, Result, RouterV2State, RoutingV2Error, Sphere,
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
                self.release_index(space, &id);
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
                // `Nodes::get` hands back a cloned Arc, so the map guard is
                // released here. It must be: `maybe_request_manifest` reads
                // `self.nodes`, and holding a write guard across that call
                // deadlocks a std RwLock on the same thread.
                let existing = self.nodes.read().unwrap().get(&mapping.target_id);
                match existing {
                    Some(node) => {
                        let advertised = {
                            let n = node.read().unwrap();
                            n.advertised_version
                        };

                        if is_fresher_u32(mapping.version, advertised) {
                            {
                                let mut n = node.write().unwrap();
                                n.advertised_version = mapping.version;
                            }
                            // per 10.8. a fresher advertisement is the pull trigger
                            self.maybe_request_manifest(
                                neighbour,
                                mapping.target_id,
                                mapping.version,
                            );
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
                        self.nodes.write().unwrap().insert(mapping.target_id, n);
                        // new origin does not hold any state, so, we need to pull it since its version is fresher
                        self.maybe_request_manifest(neighbour, mapping.target_id, mapping.version);
                        return Ok(());
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
                            is_hosted: false,
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
            info!(
                "receive-loop drop: hop_count {} >= 63 (space={space:?}, peer={})",
                entry.hop_count, ctx.neighbour
            );
            return Ok(EvaluateOutcome::Dropped);
        }

        let metric = entry
            .metric
            .saturating_add(hop_cost(ctx.transport, ctx.rssi_dbm));

        let own_idx = match self.translate_incoming(ctx.neighbour, space, entry.abs_idx) {
            Ok(idx) => idx,
            Err(RoutingV2Error::UnknownMapping(idx)) => {
                info!(
                    "receive-loop drop: no mapping for incoming idx={idx} (space={space:?}, peer={})",
                    ctx.neighbour
                );
                return Ok(EvaluateOutcome::Dropped);
            }
            Err(e) => return Err(e),
        };

        let Some(target) = self.lookup_target(space, own_idx) else {
            info!("receive-loop drop: target lookup failed (space={space:?}, own_idx={own_idx})");
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
            info!(
                    "receive-loop drop: not better than stored (own_idx={own_idx}, seq={}, metric={metric})",
                    entry.seq
                );
            return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
        }

        let neighbour_node_id = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(neighbour_info) = mirrors.get(&ctx.neighbour) else {
                info!("neighbour vanished mid-receive: {:?}", ctx.neighbour);
                return Ok(EvaluateOutcome::RejectedButTargetKnown { target_ref: target });
            };
            neighbour_info.node_id
        };
        let next_hop_idx = {
            let dict = self.node_dict.read().unwrap();
            match dict.idx_of(&neighbour_node_id) {
                Some(idx) => idx,
                None => {
                    info!("neighbour node_id has no node_dict entry: {neighbour_node_id:?}");
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

    pub(crate) fn apply_user_entry(&self, ctx: &ReceiveCtx, entry: UserEntry) -> Result<()> {
        match self.evaluate_entry(&ctx, Space::User, (&entry).into())? {
            EvaluateOutcome::Accept(a) => {
                self.commit_routing_entry(ctx, Space::User, a);
            }
            _ => {}
        };
        Ok(())
    }

    pub(crate) fn apply_node_entry(&self, ctx: &ReceiveCtx, entry: NodeEntry) -> Result<()> {
        let outcome = self.evaluate_entry(ctx, Space::Node, (&entry).into())?;
        let target = match &outcome {
            EvaluateOutcome::Accept(a) => Some(&a.target),
            EvaluateOutcome::RejectedButTargetKnown { target_ref } => Some(target_ref),
            EvaluateOutcome::Dropped => None,
        };

        if let Some(TargetRef::Node(n)) = target {
            let target_id = {
                let mut node = n.write().unwrap();
                if is_fresher_u32(entry.manifest_version, node.advertised_version) {
                    node.advertised_version = entry.manifest_version;
                }
                node.id
            };
            // per 8.8: fire another pull trigger since the advertisement is recorded
            self.maybe_request_manifest(ctx.neighbour, target_id, entry.manifest_version);
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

        // TEMP(smoke test): positive confirmation that an entry landed. Without
        // this, acceptance is only inferable from the *absence* of a drop log.
        info!(
            "router_v2 ACCEPT ✓ space={space:?} own_idx={} metric={} hop_count={} next_hop={} local_only={} peer={}",
            accepted.own_idx,
            accepted.metric,
            accepted.hop_count.saturating_add(1),
            accepted.next_hop_idx,
            accepted.local_only,
            ctx.neighbour,
        );
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
        // TEMP(): first thing to check when nothing arrives — this
        // separates "never sent" from "sent but not dispatched here".
        info!(
            "router_v2 RECV ← peer={neighbour} transport={transport:?} type={:#04x} bytes={}",
            buf.get(1).copied().unwrap_or(0),
            buf.len(),
        );

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
                        if let Err(e) = self.handle_node_manifest(msg, now, transport) {
                            error!("handle_node_manifest failed: {e}");
                        }
                    }
                    Err(e) => error!("NodeManifest decode failed: {e}"),
                },
                RoutingMessage::IndexDump => match IndexDump::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_index_dump(neighbour, msg) {
                            error!("handle_index_dump failed: {e}");
                        }
                    }
                    Err(e) => error!("IndexDump decode failed: {e}"),
                },
                RoutingMessage::ManifestDelta => match ManifestDelta::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_manifest_delta(neighbour, msg, now, transport) {
                            error!("handle_manifest_delta failed: {e}");
                        }
                    }
                    Err(e) => error!("ManifestDelta decode failed: {e}"),
                },
                RoutingMessage::ManifestRequest => match ManifestRequest::decode(payload) {
                    Ok(msg) => {
                        if let Err(e) = self.handle_manifest_request(neighbour, transport, msg, now)
                        {
                            error!("handle_manifest_request failed: {e}");
                        }
                    }
                    Err(e) => error!("ManifestRequest decode failed: {e}"),
                },
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

    /// spec: 8.8 steps 5-6
    fn refresh_delegation_trust(&self, origin_node_id: &[u8; 8], now: u64) {
        let Some(node_arc) = self.nodes.read().unwrap().get(origin_node_id) else {
            return;
        };
        let Some(host_mk) = self.get_resource_mk(origin_node_id, Space::Node) else {
            return;
        };

        let (trusted, unverifiable): (Vec<[u8; 8]>, usize) = {
            let node = node_arc.read().unwrap();
            let mut trusted = Vec::new();
            let mut unverifiable = 0usize;

            for delegated in &node.delegated_users {
                // 10.4: an expired delegation is never trusted.
                if delegated.delegation_timeout <= now {
                    continue;
                }
                let Some(user_mk) = self.get_resource_mk(&delegated.user_id, Space::User) else {
                    // TODO(§11.5): fetch the subject's profile, then re-run this.
                    unverifiable += 1;
                    continue;
                };
                let entry = ManifestEntry {
                    user_id: delegated.user_id,
                    timeout: delegated.delegation_timeout,
                    entry_signature: delegated.entry_signature,
                    profile_version: delegated.profile_version,
                };
                if Manifest::verify_entry(&entry, &host_mk, &user_mk).is_ok() {
                    trusted.push(delegated.user_id);
                }
            }
            (trusted, unverifiable)
        };

        let weak_node = Arc::downgrade(&node_arc);
        let users = self.users.read().unwrap();
        for user_id in &trusted {
            let Some(user_arc) = users.get(user_id) else {
                continue;
            };
            let mut user = user_arc.write().unwrap();
            user.delegation_gateways.retain(|w| {
                w.upgrade()
                    .map(|n| n.read().unwrap().id != *origin_node_id)
                    .unwrap_or(false)
            });
            user.delegation_gateways.push(weak_node.clone());
        }

        info!(
            "router_v2 TRUST origin={origin_node_id:?} trusted={} unverifiable={} (awaiting §11.5 profiles)",
            trusted.len(),
            unverifiable,
        );
    }

    fn delegated_users_from_entries(&self, entries: &[ManifestEntry]) -> Vec<DelegatedUser> {
        let mut users = self.users.write().unwrap();
        entries
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
                                is_hosted: false,
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
                    profile_version: entry.profile_version,
                }
            })
            .collect()
    }

    pub fn handle_node_manifest(
        &self,
        msg: NodeManifest,
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

        // §8.8 step 5, byte-exact stored discipline
        let completed_manifest = {
            let mut assembler = self.chunk_assembler.write().unwrap();
            let Some(completed) = assembler.insert(origin_node_id, msg) else {
                return Ok(());
            };
            completed
        };

        let is_gateway = (completed_manifest.flags & 0x01) != 0;
        let delegated_users = self.delegated_users_from_entries(&completed_manifest.entries);

        let nodes = self.nodes.read().unwrap();
        if let Some(node_arc) = nodes.get(&origin_node_id) {
            let mut node = node_arc.write().unwrap();
            node.manifest_version = completed_manifest.manifest_version;
            node.is_gateway = is_gateway;
            node.delegated_users = delegated_users;
            // §2.3
            node.learn_sphere = Some(Sphere::of(transport));

            // we're retaining what the origin signed exactly byte by byte
            node.manifest_signature = if completed_manifest.chunks.len() == 1 {
                Some(completed_manifest.chunks[0].manifest_signature)
            } else {
                None
            };
            node.retained_chunks = Some(completed_manifest.chunks);
            // start the log afresh with thw base at the version we just commited.
            node.manifest_log
                .reset_to(completed_manifest.manifest_version);
        }
        drop(nodes);

        self.refresh_delegation_trust(&origin_node_id, now);

        Ok(())
    }

    /// per spec 8.8: handle MANIFEST_DELTA. the spec has 7 steps
    fn send_framed(
        &self,
        peer: PeerId,
        transport: ConnectionModule,
        message_type: RoutingMessage,
        body: Vec<u8>,
    ) {
        let payload_len = match u16::try_from(body.len()) {
            Ok(n) => n,
            Err(_) => {
                error!("send_framed: body of {} bytes exceeds u16", body.len());
                return;
            }
        };
        let header = Header {
            version: crate::router_v2::codec::PROTOCOL_VERSION,
            message_type,
            payload_len,
        };
        let mut frame = Vec::with_capacity(4 + body.len());
        header.encode(&mut frame);
        frame.extend(body);

        if let Err(e) = self.tx_outbound.send(OutboundMsg {
            peer,
            transport,
            bytes: frame,
        }) {
            warn!("send_framed: outbound channel closed for {peer:?}: {e}");
        }
    }

    /// frames and sends a batched `MANIFEST_REQUEST` to one neighbour
    pub fn send_manifest_request(&self, peer: PeerId, req: ManifestRequest) {
        let transport = {
            let mirrors = self.mirrors.read().unwrap();
            let Some(info) = mirrors.get(&peer) else {
                debug!("manifest request dropped: peer {peer} is no longer a neighbour");
                return;
            };
            let Some(t) = info
                .transports
                .iter()
                .copied()
                .min_by_key(|t| hop_cost(*t, None))
            else {
                return;
            };
            t
        };

        let mut body = Vec::new();
        if let Err(e) = req.encode(&mut body) {
            error!("manifest request encode failed for {peer}: {e}");
            return;
        }

        info!(
            "router_v2 PULL → peer={peer} transport={transport:?} items={}",
            req.items.len()
        );
        self.send_framed(peer, transport, RoutingMessage::ManifestRequest, body);
    }

    /// MANIFEST_REQUEST receiver processing (spec §8.8, steps 1-5).
    pub fn handle_manifest_request(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        msg: ManifestRequest,
        now: u64,
    ) -> Result<()> {
        let host_node_id = self.host_mk.to_id();
        let requester_sphere = Sphere::of(transport);

        for item in &msg.items {
            let origin_node_id = item.origin_node_id;
            let is_own = origin_node_id == host_node_id;

            // Step 1: no state for this origin means we never advertised it
            let Some(origin) = self.origin_serve_state(&origin_node_id, is_own) else {
                debug!("manifest_request: no state for origin {origin_node_id:?}, ignoring item");
                continue;
            };

            let decision = decide_serve(item, &origin, requester_sphere);
            info!(
                "router_v2 MANIFEST_REQUEST ← peer={neighbour} origin={origin_node_id:?} have={} have_none={} committed={} log_base={} → {decision:?}",
                item.have_version,
                item.have_none(),
                origin.committed,
                origin.log_base,
            );

            match decision {
                ServeDecision::Sealed | ServeDecision::Nothing => {}
                ServeDecision::Full | ServeDecision::Delta { .. }
                    if !self.allow_manifest_serve(neighbour, now) =>
                {
                    debug!(
                        "manifest_request: serve rate limit reached for peer={neighbour}, ignoring item for origin {origin_node_id:?}"
                    );
                }
                ServeDecision::Full => {
                    self.serve_full_manifest(neighbour, transport, origin_node_id, is_own)
                }
                ServeDecision::Delta { from_version } => {
                    self.serve_delta(neighbour, transport, origin_node_id, is_own, from_version)
                }
            }
        }

        Ok(())
    }

    /// looks for the serving view of an origin, or `None` when we hold nothing
    fn origin_serve_state(
        &self,
        origin_node_id: &[u8; 8],
        is_own: bool,
    ) -> Option<OriginServeState> {
        if is_own {
            return Some(OriginServeState {
                committed: self.manifest.read().unwrap().manifest_version,
                log_base: self.own_manifest_log.read().unwrap().log_base,
                learn_sphere: None,
            });
        }

        let node_arc = self.nodes.read().unwrap().get(origin_node_id)?;
        let node = node_arc.read().unwrap();
        // a stub created from a mapping has version 0 and no entries
        if node.manifest_version == 0 && node.delegated_users.is_empty() {
            return None;
        }
        Some(OriginServeState {
            committed: node.manifest_version,
            log_base: node.manifest_log.log_base,
            learn_sphere: node.learn_sphere,
        })
    }

    /// serve a full `NODE_MANIFEST`
    fn serve_full_manifest(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        origin_node_id: [u8; 8],
        is_own: bool,
    ) {
        let chunks: Vec<NodeManifest> = if is_own {
            let manifest = self.manifest.read().unwrap();

            if let Some(retained) = &manifest.retained_chunks {
                retained.clone()
            } else if let Some(signature) = manifest.manifest_signature {
                vec![reconstruct_single_chunk_full(
                    origin_node_id,
                    manifest.manifest_version,
                    manifest.is_gateway,
                    manifest.entries().to_vec(),
                    signature,
                )]
            } else {
                debug!("serve_full: own manifest not signed yet, cannot serve");
                return;
            }
        } else {
            let Some(node_arc) = self.nodes.read().unwrap().get(&origin_node_id) else {
                return;
            };
            let node = node_arc.read().unwrap();

            if let Some(retained) = &node.retained_chunks {
                retained.clone()
            } else if let Some(signature) = node.manifest_signature {
                vec![reconstruct_single_chunk_full(
                    origin_node_id,
                    node.manifest_version,
                    node.is_gateway,
                    node.delegated_users
                        .iter()
                        .map(|d| ManifestEntry {
                            user_id: d.user_id,
                            timeout: d.delegation_timeout,
                            entry_signature: d.entry_signature,
                            profile_version: d.profile_version,
                        })
                        .collect(),
                    signature,
                )]
            } else {
                debug!("serve_full: no signed bytes retained for {origin_node_id:?}, cannot serve");
                return;
            }
        };

        for chunk in chunks {
            let mut body = Vec::new();
            if let Err(e) = chunk.encode(&mut body) {
                error!("serve_full: encode failed for {origin_node_id:?}: {e}");
                return;
            }
            self.send_framed(neighbour, transport, RoutingMessage::NodeManifest, body);
        }
    }

    /// serve a MANIFEST_DELTA. respond with full manifest is body is above 60kb
    fn serve_delta(
        &self,
        neighbour: PeerId,
        transport: ConnectionModule,
        origin_node_id: [u8; 8],
        is_own: bool,
        from_version: u32,
    ) {
        let assembled = if is_own {
            let manifest = self.manifest.read().unwrap();
            // Cached at bump time, never computed here — see serve_full.
            let Some(signature) = manifest.manifest_signature else {
                debug!("serve_delta: own manifest not signed yet, serving full");
                drop(manifest);
                self.serve_full_manifest(neighbour, transport, origin_node_id, true);
                return;
            };
            let header = DeltaHeader {
                origin_node_id,
                from_version,
                to_version: manifest.manifest_version,
                is_gateway: manifest.is_gateway,
                manifest_signature: signature,
            };
            let records = self
                .own_manifest_log
                .read()
                .unwrap()
                .records_after(from_version);
            header.assemble(records)
        } else {
            let Some(node_arc) = self.nodes.read().unwrap().get(&origin_node_id) else {
                return;
            };
            let node = node_arc.read().unwrap();
            let Some(signature) = node.manifest_signature else {
                debug!(
                    "serve_delta: no whole-state signature for {origin_node_id:?}, serving full"
                );
                drop(node);
                self.serve_full_manifest(neighbour, transport, origin_node_id, false);
                return;
            };
            let header = DeltaHeader {
                origin_node_id,
                from_version,
                to_version: node.manifest_version,
                is_gateway: node.is_gateway,
                manifest_signature: signature,
            };
            let records = node.manifest_log.records_after(from_version);
            header.assemble(records)
        };

        // §8.6: a delta is never chunked, so an oversize range becomes a
        // full manifest
        let assembled = match assembled {
            Ok(msg) => msg,
            Err(e) => {
                debug!("serve_delta: {e} for {origin_node_id:?}");
                self.serve_full_manifest(neighbour, transport, origin_node_id, is_own);
                return;
            }
        };

        let mut body = Vec::new();
        if let Err(e) = assembled.encode(&mut body) {
            error!("serve_delta: encode failed for {origin_node_id:?}: {e}");
            return;
        }

        self.send_framed(neighbour, transport, RoutingMessage::ManifestDelta, body);
    }

    pub fn handle_manifest_delta(
        &self,
        neighbour: PeerId,
        msg: ManifestDelta,
        now: u64,
        transport: ConnectionModule,
    ) -> Result<()> {
        let origin_node_id = msg.origin_node_id;

        // Step 1: resolve the origin's key.
        let Some(origin_mk) = self.get_resource_mk(&origin_node_id, Space::Node) else {
            debug!(
                "manifest_delta from {origin_node_id:?} dropped: origin public_key unknown — TODO(§11.5 ProfileFetch)"
            );
            return Ok(());
        };

        // Step 2: the delta must build on exactly what we hold.
        let Some(node_arc) = self.nodes.read().unwrap().get(&origin_node_id) else {
            debug!("manifest_delta from {origin_node_id:?} dropped: no node record");
            return Ok(());
        };
        let (committed, stored): (u32, Vec<ManifestEntry>) = {
            let node = node_arc.read().unwrap();
            let entries = node
                .delegated_users
                .iter()
                .map(|d| ManifestEntry {
                    user_id: d.user_id,
                    timeout: d.delegation_timeout,
                    entry_signature: d.entry_signature,
                    profile_version: d.profile_version,
                })
                .collect();
            (node.manifest_version, entries)
        };
        if committed != msg.from_version {
            info!(
                "manifest_delta from {origin_node_id:?} dropped: committed {committed} != from_version {}",
                msg.from_version
            );
            return Ok(());
        }

        // Step 3: build the scratch set. removes first, then adds as upserts
        let mut scratch = stored;
        for remove in &msg.removes {
            scratch.retain(|e| e.user_id != remove.user_id);
        }
        for add in &msg.adds {
            match scratch.binary_search_by(|e| e.user_id.cmp(&add.entry.user_id)) {
                Ok(i) => scratch[i] = add.entry,
                Err(i) => scratch.insert(i, add.entry),
            }
        }

        // Step 4: verify the signature over the resulting state at to_version.
        let flags = msg.flags & 0x01;
        let scratch_bytes = canonical_entry_bytes(&scratch);
        let ctx = ChunkSigningCtx {
            origin_multikey: &origin_mk.encode(),
            manifest_version: msg.to_version,
            chunk_index: 0,
            chunk_count: 1,
            flags,
            canonical_entries: &scratch_bytes,
        };
        if !origin_mk.verify(&ctx.signing_input(), &msg.manifest_signature) {
            warn!(
                "manifest_delta from {origin_node_id:?} failed resulting-state verification; discarding scratch"
            );
            self.request_full_manifest(neighbour, origin_node_id, now);
            return Ok(());
        }

        // Step 5: commit.
        let is_gateway = flags != 0;
        let delegated_users = self.delegated_users_from_entries(&scratch);
        {
            let mut node = node_arc.write().unwrap();
            node.manifest_version = msg.to_version;
            node.is_gateway = is_gateway;
            node.delegated_users = delegated_users;
            node.learn_sphere = Some(Sphere::of(transport));
            node.manifest_signature = Some(msg.manifest_signature);
            node.retained_chunks = None;

            for add in &msg.adds {
                node.manifest_log.insert_add(add.record_version, add.entry);
            }
            for remove in &msg.removes {
                node.manifest_log
                    .insert_remove(remove.user_id, remove.record_version, now);
            }
            let tombstone_ttl_ms = self.options.delegation_ttl.saturating_mul(1000);
            node.manifest_log
                .compact(now, tombstone_ttl_ms, self.options.delta_log_cap);
        }

        info!(
            "router_v2 MANIFEST_DELTA ← origin={origin_node_id:?} {} → {} (+{} -{})",
            msg.from_version,
            msg.to_version,
            msg.adds.len(),
            msg.removes.len(),
        );

        // Step 6: re-evaluate the trusted subset. Applies to this path exactly
        // as it does to a full manifest — a delegation that arrives by delta
        // must be able to become trusted too.
        self.refresh_delegation_trust(&origin_node_id, now);

        // Step 7: the new committed version rides ordinary routing updates.
        // The delta is never relayed.
        Ok(())
    }

    /// per 8.8: sends a single MANIFEST_REQUEST setting have_none = 1 to trigger a full MANIFEST_NODE.
    fn request_full_manifest(&self, neighbour: PeerId, origin_node_id: [u8; 8], now: u64) {
        if !self.allow_manifest_request(neighbour, now) {
            debug!(
                "full re-request for {origin_node_id:?} suppressed by request rate limit; \
                 re-advertisement will retrigger"
            );
            return;
        }

        self.outstanding_manifest_requests
            .write()
            .unwrap()
            .insert((origin_node_id, neighbour), now);

        self.send_manifest_request(
            neighbour,
            ManifestRequest {
                items: vec![ManifestRequestItem {
                    origin_node_id,
                    have_version: 0,
                    item_flags: 0x01,
                }],
            },
        );
    }

    pub fn handle_index_dump(&self, neighbour: PeerId, msg: IndexDump) -> Result<()> {
        // TEMP(smoke test): an empty dump is expected from a fresh node —
        // dictionaries are only populated once the origin tick runs.
        info!(
            "router_v2 INDEX_DUMP ← peer={neighbour} user_mappings={} node_mappings={}",
            msg.user_mappings.len(),
            msg.node_mappings.len(),
        );

        for mapping in msg.user_mappings {
            if let Err(e) = self.apply_mapping(neighbour, Space::User, mapping) {
                warn!("index_dump: apply_mapping user failed: {e}");
            }
        }

        for mapping in msg.node_mappings {
            if let Err(e) = self.apply_mapping(neighbour, Space::Node, mapping) {
                warn!("index_dump: apply_mapping node failed: {e}");
            }
        }
        Ok(())
    }
}
