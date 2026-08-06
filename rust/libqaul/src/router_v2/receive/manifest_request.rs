// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The requester side of manifest synchronization (spec §8.7, §10.8).

use libp2p::PeerId;
use tracing::{debug, error, info};

use crate::router_v2::{
    codec::{
        messages::{ManifestRequest, ManifestRequestItem},
        RoutingMessage,
    },
    metric::hop_cost,
    RouterV2State,
};

impl RouterV2State {
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

    pub(crate) fn request_full_manifest(
        &self,
        neighbour: PeerId,
        origin_node_id: [u8; 8],
        now: u64,
    ) {
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
}
