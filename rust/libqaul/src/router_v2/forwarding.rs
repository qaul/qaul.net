// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Next-hop selection for outbound data messages (spec §9.2).

use libp2p::PeerId;
use tracing::debug;

use crate::{
    connections::ConnectionModule,
    router_v2::{identity::id_from_peer_id, index::Space, RouterV2State},
};

/// how to handle a packet addressed to a receipient
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForwardingDecision {
    /// forward to neighbour over this transport
    Forward {
        peer: PeerId,
        transport: ConnectionModule,
    },
    /// no usable route
    HandoffToDTN,
}

impl RouterV2State {
    /// resolves where a message for `recipient` should go next: spec 9.2
    pub fn resolve_forwarding(&self, recipient: PeerId) -> ForwardingDecision {
        let Some(recipient_id) = id_from_peer_id(&recipient) else {
            debug!("v2 forwarding: {recipient} has no recoverable key, handing off to DTN");
            return ForwardingDecision::HandoffToDTN;
        };

        // Step 1: the recipient is known
        if let Some((next_hop_node, transport)) = self.next_hop_for_user(recipient_id) {
            match self.peer_of_node(&next_hop_node) {
                Some(peer) => return ForwardingDecision::Forward { peer, transport },
                None => {
                    debug!(
                        "v2 forwarding: next hop {next_hop_node:?} for {recipient_id:?} \
                         is no longer a neighbour"
                    );
                }
            }
        }

        // Step 2: unknown recipient.
        if self.host_is_gateway() {
            // We hold the global directory. Not being in it is authoritative.
            debug!(
                "v2 forwarding: gateway holds no route for {recipient_id:?}, handing off to DTN"
            );
            return ForwardingDecision::HandoffToDTN;
        }

        match self.nearest_gateway() {
            Some((peer, transport)) => ForwardingDecision::Forward { peer, transport },
            None => {
                debug!(
                    "v2 forwarding: no reachable gateway for {recipient_id:?}, handing off to DTN"
                );
                ForwardingDecision::HandoffToDTN
            }
        }
    }

    /// spec 10.1
    pub fn host_is_gateway(&self) -> bool {
        self.manifest.read().unwrap().is_gateway
    }

    /// get the gateway with lowest metric
    pub fn nearest_gateway(&self) -> Option<(PeerId, ConnectionModule)> {
        let gateway_ids: Vec<[u8; 8]> = {
            let nodes = self.nodes.read().unwrap();
            nodes
                .iter()
                .filter(|(_, node)| node.read().unwrap().is_gateway)
                .map(|(id, _)| *id)
                .collect()
        };

        let mut best: Option<(u16, u16, ConnectionModule)> = None;
        for id in gateway_ids {
            let Some(idx) = self.node_dict.read().unwrap().idx_of(&id) else {
                continue;
            };
            let Some(entry) = self.routing_table.read().unwrap().get(Space::Node, idx) else {
                continue;
            };
            let entry = entry.read().unwrap();
            if best.is_none_or(|(metric, _, _)| entry.metric < metric) {
                best = Some((entry.metric, entry.next_hop, entry.transport));
            }
        }

        let (_, next_hop_idx, transport) = best?;
        let next_hop_node = self.next_hop_node_id(next_hop_idx)?;
        let peer = self.peer_of_node(&next_hop_node)?;
        Some((peer, transport))
    }
}
