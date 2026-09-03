// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! This file describes how messages are propagated between nodes.

use std::collections::HashSet;
use std::sync::Arc;

use libp2p::PeerId;
use std::sync::RwLock;
use tracing::{debug, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{
            messages::{IndexDump, Mapping, NodeEntry, RoutingUpdate, UserEntry},
            Header, RoutingMessage, PROTOCOL_VERSION,
        },
        index::Space,
        manifest,
        table::{RoutingEntry, TargetRef},
        OutboundKind, OutboundMsg, RouterV2State, Sphere,
    },
};

/// §8.4 body overhead: `chunk_index`, `chunk_count`, and the two 2-byte
/// section counts.
const DUMP_OVERHEAD: usize = 6;
const DUMP_MAPPING_BYTES: usize = 15;
/// how many mappings we can fit ine INDEX_BUMP
const MAPPINGS_PER_DUMP: usize = (manifest::MAX_BODY - DUMP_OVERHEAD) / DUMP_MAPPING_BYTES;

/// should this stored routing entry be withheld
/// from a neighbour we're about to send a ROUTING_UPDATE to.
pub fn blocked_by_split_horizon(
    state: &RouterV2State,
    entry: &RoutingEntry,
    outgoing_neighbour_id: [u8; 8],
) -> bool {
    let node_dict = state.node_dict.read().unwrap();
    let Some(next_hop_id) = node_dict.id_of(entry.next_hop) else {
        debug!(
            "split-horizon: entry.next_hop {} has no node_dict binding — blocking",
            entry.next_hop
        );
        return true;
    };
    next_hop_id == outgoing_neighbour_id
}

/// §2.3 gateway membrane. Local-outgoing propagates only entries learned
/// from the Local sphere; Internet-outgoing propagates only gateway node entries.
pub fn should_propagate(entry: &RoutingEntry, sphere: Sphere) -> bool {
    match sphere {
        Sphere::Local => Sphere::of(entry.transport) == Sphere::Local,
        Sphere::Internet => entry.target_is_gateway(),
    }
}

/// check if this node should be introduced as a user or carried in the manifest, per 2.3
pub fn should_introduce(
    state: &RouterV2State,
    space: Space,
    target_id: &[u8; 8],
    sphere: Sphere,
) -> bool {
    match (sphere, space) {
        (Sphere::Local, _) => true,
        (Sphere::Internet, Space::User) => false,
        (Sphere::Internet, Space::Node) => state
            .nodes
            .read()
            .unwrap()
            .get(target_id)
            .is_some_and(|node| node.read().unwrap().is_gateway),
    }
}

pub fn compute_outgoing_local_only(stored: bool, outgoing_sphere: Sphere) -> bool {
    match outgoing_sphere {
        Sphere::Internet => false,
        Sphere::Local => stored,
    }
}

/// §2.2: "the hop counter is sphere-local"
pub fn compute_outgoing_hop_count(
    stored_hop_count: u8,
    stored_transport: ConnectionModule,
    outgoing_sphere: Sphere,
) -> u8 {
    let crosses_membrane =
        outgoing_sphere == Sphere::Internet && Sphere::of(stored_transport) == Sphere::Local;

    if crosses_membrane {
        0
    } else {
        stored_hop_count
    }
}

fn build_origin_update(
    new_seq: u16,
    origin_space: Space,
    manifest_version: u32,
    local_only: bool,
    intros: Vec<Mapping>,
) -> RoutingUpdate {
    match origin_space {
        Space::Node => {
            let entry = NodeEntry {
                abs_idx: 0,
                seq: new_seq,
                metric: 0,
                local_only,
                hop_count: 0,
                manifest_version,
            };
            RoutingUpdate {
                user_mappings: Vec::new(),
                node_mappings: intros,
                user_entries: Vec::new(),
                node_entries: vec![entry],
            }
        }
        Space::User => {
            let entry = UserEntry {
                abs_idx: 0,
                seq: new_seq,
                metric: 0,
                local_only,
                hop_count: 0,
            };
            RoutingUpdate {
                user_mappings: intros,
                node_mappings: Vec::new(),
                user_entries: vec![entry],
                node_entries: Vec::new(),
            }
        }
    }
}

/// orchestrates sending routing updates per second
pub fn tick_origin(state: &RouterV2State, now_ms: u64) {
    let new_seq = {
        let mut seq = state.seq_num.write().unwrap();
        seq.increment();
        seq.value()
    };

    let origin_space = state.sync_propagation_form(now_ms).origin_space();

    let intros = state
        .pending_introductions(origin_space)
        .into_iter()
        .map(|intro| Mapping {
            abs_idx: intro.0,
            target_id: intro.1,
            version: intro.2,
        })
        .collect::<Vec<Mapping>>();

    let pairs: Vec<(PeerId, ConnectionModule)> = {
        let mirrors = state.mirrors.read().unwrap();
        mirrors
            .iter()
            .flat_map(|(peer, neighbour_info)| {
                let p = *peer;
                neighbour_info.transports.iter().map(move |t| (p, *t))
            })
            .collect()
    };
    let own_manifest_version = state.manifest.read().unwrap().manifest_version;
    for (peer, transport) in pairs {
        let sphere_outgoing = Sphere::of(transport);
        let local_only = sphere_outgoing == Sphere::Local;
        let msg = build_origin_update(
            new_seq,
            origin_space,
            own_manifest_version,
            local_only,
            intros.clone(),
        );

        let mut body = Vec::new();
        if let Err(e) = msg.encode(&mut body) {
            warn!("origin tick: encode failed for {peer:?}/{transport:?}: {e}");
            continue;
        }

        let header = Header {
            version: PROTOCOL_VERSION,
            message_type: RoutingMessage::RoutingUpdate,
            payload_len: body.len() as u16,
        };
        let mut header_bytes = Vec::with_capacity(body.len() + 4);
        header.encode(&mut header_bytes);
        header_bytes.extend(body);

        if let Err(e) = state.tx_outbound.send(OutboundMsg {
            kind: OutboundKind::Routing,
            peer,
            transport,
            bytes: header_bytes,
        }) {
            warn!("origin tick: outbound channel send failed for {peer:?}: {e}");
        }
    }
}

/// orchestrates outbound routing updates every 1s
pub fn tick_relay(state: &RouterV2State, now: u64) {
    //  remove expired slots
    state.sweep_expired(now);
    let relay_queue = std::mem::take(&mut *state.relay_queue.write().unwrap());
    let entries: Vec<(Space, u16, Arc<RwLock<RoutingEntry>>)> = {
        let rt = state.routing_table.read().unwrap();
        relay_queue
            .into_iter()
            .filter_map(|(space, idx)| rt.get(space, idx).map(|entry| (space, idx, entry)))
            .collect()
    };

    // any index that we have cleared in applu_entry, will be skipped here
    // so we can take a correct snapshot of the state
    let pairs: Vec<(PeerId, [u8; 8], ConnectionModule)> = {
        let mirrors = state.mirrors.read().unwrap();
        mirrors
            .iter()
            .flat_map(|(peer, info)| {
                let peer = *peer;
                let node_id = info.node_id;
                info.transports.iter().map(move |t| (peer, node_id, *t))
            })
            .collect()
    };

    // i just could not find a better name, i tried
    let map_to_intros = |t: (u16, [u8; 8], u32)| Mapping {
        abs_idx: t.0,
        target_id: t.1,
        version: t.2,
    };

    // Draining the tracker hands us the only outstanding copy of each mark,
    // so anything the sphere filter withholds from *every* peer has to be
    // put back — otherwise the index is never introduced to anyone and the
    // receiver drops each entry that references it as an unknown mapping,
    // permanently (§3.6).
    let user_intros = state.pending_introductions(Space::User);
    let node_intros = state.pending_introductions(Space::Node);
    let mut user_sent: HashSet<u16> = HashSet::new();
    let mut node_sent: HashSet<u16> = HashSet::new();

    for (peer, neigbour_id, transport) in pairs {
        let sphere_outbound = Sphere::of(transport);

        // §2.3: the membrane applies to index mappings as well as entries.
        let user_mappings: Vec<Mapping> = user_intros
            .iter()
            .filter(|t| should_introduce(state, Space::User, &t.1, sphere_outbound))
            .map(|t| map_to_intros(*t))
            .collect();
        let node_mappings: Vec<Mapping> = node_intros
            .iter()
            .filter(|t| should_introduce(state, Space::Node, &t.1, sphere_outbound))
            .map(|t| map_to_intros(*t))
            .collect();

        user_sent.extend(user_mappings.iter().map(|m| m.abs_idx));
        node_sent.extend(node_mappings.iter().map(|m| m.abs_idx));

        let mut user_out = Vec::new();
        let mut node_out = Vec::new();

        for (space, own_idx, entry) in &entries {
            let e = entry.read().unwrap();
            if blocked_by_split_horizon(state, &e, neigbour_id) {
                continue;
            }
            if !should_propagate(&e, sphere_outbound) {
                continue;
            }

            let local_only = compute_outgoing_local_only(e.local_only, sphere_outbound);
            let hop_count = compute_outgoing_hop_count(e.hop_count, e.transport, sphere_outbound);
            match space {
                Space::Node => {
                    let manifest_version = match &e.target {
                        TargetRef::Node(n) => n.read().unwrap().manifest_version,
                        TargetRef::User(_) => {
                            debug!("routing entry in node-space with User target — skipping");
                            continue;
                        }
                    };
                    node_out.push(NodeEntry {
                        abs_idx: *own_idx,
                        seq: e.seq_num.value(),
                        metric: e.metric,
                        hop_count,
                        local_only,
                        manifest_version,
                    })
                }
                Space::User => user_out.push(UserEntry {
                    abs_idx: *own_idx,
                    seq: e.seq_num.value(),
                    metric: e.metric,
                    hop_count,
                    local_only,
                }),
            }
        }

        user_out.sort_by_key(|e| e.abs_idx);
        node_out.sort_by_key(|e| e.abs_idx);

        // empty batch, save ourselves the stress of sending an empty message
        if user_out.is_empty()
            && node_out.is_empty()
            && user_mappings.is_empty()
            && node_mappings.is_empty()
        {
            continue;
        }
        let msg = RoutingUpdate {
            user_mappings,
            node_mappings,
            user_entries: user_out,
            node_entries: node_out,
        };

        let mut body = Vec::new();
        if let Err(e) = msg.encode(&mut body) {
            warn!("relay tick: encode failed for {peer:?}/{transport:?}: {e}");
            continue;
        }

        let header = Header {
            version: PROTOCOL_VERSION,
            message_type: RoutingMessage::RoutingUpdate,
            payload_len: body.len() as u16,
        };

        let mut frame = Vec::with_capacity(4 + body.len());
        header.encode(&mut frame);
        frame.extend(body);

        if let Err(e) = state.tx_outbound.send(OutboundMsg {
            kind: OutboundKind::Routing,
            peer,
            transport,
            bytes: frame,
        }) {
            warn!("relay tick: outbound channel send failed for {peer:?}: {e}");
        }
    }

    requeue_unsent_introductions(state, &user_intros, &user_sent, Space::User);
    requeue_unsent_introductions(state, &node_intros, &node_sent, Space::Node);
}

fn requeue_unsent_introductions(
    state: &RouterV2State,
    intros: &[(u16, [u8; 8], u32)],
    sent: &HashSet<u16>,
    space: Space,
) {
    let unsent: Vec<u16> = intros
        .iter()
        .map(|t| t.0)
        .filter(|idx| !sent.contains(idx))
        .collect();
    if unsent.is_empty() {
        return;
    }

    let mut tracker = state.reintroduction_tracker.write().unwrap();
    for idx in unsent {
        debug!("relay tick: {space:?} idx {idx} reached no peer, re-marking (§3.6)");
        tracker.mark_first_time(space, idx);
    }
}

/// Sends an INDEX_DUMP when a neighbour connects
pub fn on_neighbour_connect(state: &RouterV2State, neighbour: PeerId, transport: ConnectionModule) {
    if matches!(
        transport,
        ConnectionModule::Ble1m | ConnectionModule::BleCoded
    ) {
        // Per §8.4: no INDEX_DUMP on BLE.
        // TODO: NODE_MANIFEST send when identity plumbing exists.
        return;
    }

    let sphere_outbound = Sphere::of(transport);

    let user_mappings = {
        let dict = state.user_dict.read().unwrap();
        let users = state.users.read().unwrap();
        let mut mappings = Vec::new();
        for (&idx, &id) in &dict.forward_dir {
            if !should_introduce(state, Space::User, &id, sphere_outbound) {
                continue;
            }
            let version = users
                .get(&id)
                .map(|arc| arc.read().unwrap().profile_version)
                .unwrap_or(0);
            mappings.push(Mapping {
                abs_idx: idx,
                target_id: id,
                version,
            });
        }
        mappings.sort_by_key(|m| m.abs_idx);
        mappings
    };

    let node_mappings = {
        let dict = state.node_dict.read().unwrap();
        let nodes = state.nodes.read().unwrap();
        let mut mappings = Vec::new();
        for (&idx, &id) in &dict.forward_dir {
            if !should_introduce(state, Space::Node, &id, sphere_outbound) {
                continue;
            }
            let version = nodes
                .get(&id)
                .map(|arc| arc.read().unwrap().manifest_version)
                .unwrap_or(0);
            mappings.push(Mapping {
                abs_idx: idx,
                target_id: id,
                version,
            });
        }
        mappings.sort_by_key(|m| m.abs_idx);
        mappings
    };

    for dump in split_index_dump(user_mappings, node_mappings) {
        let mut body = Vec::new();
        if let Err(e) = dump.encode(&mut body) {
            warn!("bootstrap: INDEX_DUMP encode failed: {e}");
            return;
        }

        let payload_len: u16 = match body.len().try_into() {
            Ok(n) => n,
            Err(_) => {
                warn!("bootstrap: body exceeds u16 range");
                return;
            }
        };

        let header = Header {
            version: PROTOCOL_VERSION,
            message_type: RoutingMessage::IndexDump,
            payload_len,
        };
        let mut frame = Vec::with_capacity(4 + body.len());
        header.encode(&mut frame);
        frame.extend(&body);

        if let Err(e) = state.tx_outbound.send(OutboundMsg {
            kind: OutboundKind::Routing,
            peer: neighbour,
            transport,
            bytes: frame,
        }) {
            warn!("bootstrap: outbound send failed for {neighbour:?}: {e}");
            return;
        }
    }
}

/// §8.4: split a dictionary across as many `INDEX_DUMP` messages as it takes
pub fn split_index_dump(
    user_mappings: Vec<Mapping>,
    node_mappings: Vec<Mapping>,
) -> Vec<IndexDump> {
    let total = user_mappings.len() + node_mappings.len();
    let chunk_count = total.div_ceil(MAPPINGS_PER_DUMP).max(1);

    if chunk_count > u8::MAX as usize {
        warn!(
            "bootstrap: dictionary of {total} mappings needs {chunk_count} chunks, truncating to {}",
            u8::MAX
        );
    }
    let chunk_count = chunk_count.min(u8::MAX as usize);

    let mut users = user_mappings.into_iter().peekable();
    let mut nodes = node_mappings.into_iter().peekable();
    let mut chunks = Vec::with_capacity(chunk_count);

    for chunk_index in 0..chunk_count {
        let mut room = MAPPINGS_PER_DUMP;
        let mut user_chunk = Vec::new();
        let mut node_chunk = Vec::new();

        while room > 0 && users.peek().is_some() {
            user_chunk.push(users.next().unwrap());
            room -= 1;
        }
        while room > 0 && nodes.peek().is_some() {
            node_chunk.push(nodes.next().unwrap());
            room -= 1;
        }

        chunks.push(IndexDump {
            chunk_index: chunk_index as u8,
            chunk_count: chunk_count as u8,
            user_mappings: user_chunk,
            node_mappings: node_chunk,
        });
    }

    chunks
}
