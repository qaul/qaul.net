// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! This file describes how messages are propagated between nodes.

use libp2p::PeerId;
use tracing::{debug, warn};

use crate::{
    connections::ConnectionModule,
    router_v2::{
        codec::{
            messages::{Entry, Mapping, RoutingUpdate},
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
