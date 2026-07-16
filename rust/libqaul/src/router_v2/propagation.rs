// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! This file describes how messages are propagated between nodes.

use std::{collections::HashMap, sync::Arc};

use libp2p::PeerId;
use std::sync::RwLock;
use tracing::{debug, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{
            messages::{Entry, Mapping, NodeManifest, RoutingUpdate},
            Header, RoutingMessage, PROTOCOL_VERSION,
        },
        index::Space,
        table::RoutingEntry,
        OutboundMsg, RouterV2State, Sphere,
    },
};

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

pub fn compute_outgoing_local_only(stored: bool, outgoing_sphere: Sphere) -> bool {
    match outgoing_sphere {
        Sphere::Internet => false,
        Sphere::Local => stored,
    }
}

fn build_origin_update(
    new_seq: u16,
    origin_space: Space,
    local_only: bool,
    intros: Vec<Mapping>,
) -> RoutingUpdate {
    let entry = Entry {
        abs_idx: 0,
        seq: new_seq,
        metric: 0,
        local_only,
        hop_count: 0,
    };
    match origin_space {
        Space::Node => RoutingUpdate {
            user_mappings: Vec::new(),
            node_mappings: intros,
            user_entries: Vec::new(),
            node_entries: vec![entry],
        },
        Space::User => RoutingUpdate {
            user_mappings: intros,
            node_mappings: Vec::new(),
            user_entries: vec![entry],
            node_entries: Vec::new(),
        },
    }
}

/// orchestrates sending routing updates per second
pub fn tick_origin(state: &RouterV2State) {
    let new_seq = {
        let mut seq = state.seq_num.write().unwrap();
        seq.increment();
        seq.value()
    };

    // TODO: switch to Space::Node for multi-user hosts and gateways per §3.2.
    let origin_space = Space::User;

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

    for (peer, transport) in pairs {
        let sphere_outgoing = Sphere::of(transport);
        let local_only = sphere_outgoing == Sphere::Local;
        let msg = build_origin_update(new_seq, origin_space, local_only, intros.clone());

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

    let user_intros: Vec<Mapping> = state
        .pending_introductions(Space::User)
        .into_iter()
        .map(map_to_intros)
        .collect();
    let node_intros: Vec<Mapping> = state
        .pending_introductions(Space::Node)
        .into_iter()
        .map(map_to_intros)
        .collect();

    for (peer, neigbour_id, transport) in pairs {
        let sphere_outbound = Sphere::of(transport);

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

            let wire_entry = Entry {
                abs_idx: *own_idx,
                seq: e.seq_num.value(),
                metric: e.metric,
                hop_count: e.hop_count,
                local_only: compute_outgoing_local_only(e.local_only, sphere_outbound),
            };

            match space {
                Space::Node => node_out.push(wire_entry),
                Space::User => user_out.push(wire_entry),
            }
        }

        user_out.sort_by_key(|e| e.abs_idx);
        node_out.sort_by_key(|e| e.abs_idx);

        // empty batch, save ourselves the stress of sending an empty message
        if user_out.is_empty()
            && node_out.is_empty()
            && user_intros.is_empty()
            && node_intros.is_empty()
        {
            continue;
        }
        let msg = RoutingUpdate {
            user_mappings: user_intros.clone(),
            node_mappings: node_intros.clone(),
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
            peer,
            transport,
            bytes: frame,
        }) {
            warn!("relay tick: outbound channel send failed for {peer:?}: {e}");
        }
    }
}

pub fn tick_relay_manifests(state: &RouterV2State) {
    let relay_queue: HashMap<[u8; 8], (Vec<NodeManifest>, Sphere)> =
        std::mem::take(&mut *state.manifest_relay_queue.write().unwrap());

    let pairs: Vec<(PeerId, ConnectionModule)> = {
        let mirrors = state.mirrors.read().unwrap();
        mirrors
            .iter()
            .flat_map(|(peer, info)| {
                let peer = *peer;
                info.transports.iter().map(move |t| (peer, *t))
            })
            .collect()
    };

    for (&origin_id, (chunks, learn_sphere)) in &relay_queue {
        let is_gateway = chunks
            .first()
            .map(|c| (c.flags & 0x01) != 0)
            .unwrap_or(false);

        let recipients: Vec<(PeerId, ConnectionModule)> = pairs
            .iter()
            .copied()
            .filter(|(_, transport)| {
                let outgoing_sphere = Sphere::of(*transport);
                let downward_seal_hit =
                    *learn_sphere == Sphere::Internet && outgoing_sphere == Sphere::Local;
                let non_gateway_upward = outgoing_sphere == Sphere::Internet && !is_gateway;
                !downward_seal_hit && !non_gateway_upward
            })
            .collect();
        if recipients.is_empty() {
            continue;
        }

        let mut encoded_chunks: Vec<Vec<u8>> = Vec::with_capacity(chunks.len());
        for chunk in chunks {
            let mut body = Vec::new();
            if let Err(e) = chunk.encode(&mut body) {
                warn!("manifest relay: encode failed for origin {origin_id:?}: {e}");
                continue;
            }
            let header = Header {
                version: PROTOCOL_VERSION,
                message_type: RoutingMessage::NodeManifest,
                payload_len: body.len() as u16,
            };
            let mut frame = Vec::with_capacity(4 + body.len());
            header.encode(&mut frame);
            frame.extend(body);
            encoded_chunks.push(frame);
        }

        for (peer, transport) in &recipients {
            for frame in &encoded_chunks {
                if let Err(e) = state.tx_outbound.send(OutboundMsg {
                    peer: *peer,
                    transport: *transport,
                    bytes: frame.clone(),
                }) {
                    warn!("manifest relay: outbound send failed for {peer:?}: {e}");
                }
            }
        }
    }
}
