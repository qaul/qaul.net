// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Manifest pull scheduling and §14 rate limits (spec §8.7, §10.8).
//! A fresher advertised version is the pull trigger.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::RwLock;

use libp2p::PeerId;

use crate::router_v2::{
    codec::messages::{ManifestRequest, ManifestRequestItem},
    seq::is_fresher_u32,
    RouterV2State,
};

impl RouterV2State {
    /// per 10.8: queues manifest pull for origin_node_id against the neighbour that
    /// advertised the advertised_version.
    pub fn maybe_request_manifest(
        &self,
        neighbour: PeerId,
        origin_node_id: [u8; 8],
        advertised_version: u32,
    ) {
        // we can't pull our own manifest
        if origin_node_id == self.host_mk.to_id() {
            return;
        }

        let committed = self
            .nodes
            .read()
            .unwrap()
            .get(&origin_node_id)
            .map(|n| n.read().unwrap().manifest_version)
            .unwrap_or(0);

        if !is_fresher_u32(advertised_version, committed) {
            return;
        }

        {
            let outstanding = self.outstanding_manifest_requests.read().unwrap();
            if outstanding
                .keys()
                .any(|(origin, _)| *origin == origin_node_id)
            {
                return;
            }
        }

        let newly_queued = self
            .pending_manifest_requests
            .write()
            .unwrap()
            .entry(neighbour)
            .or_default()
            .insert(origin_node_id);

        if newly_queued {
            tracing::info!(
                "router_v2 PULL queued: origin={origin_node_id:?} advertised={advertised_version} committed={committed} via peer={neighbour}"
            );
        }
    }

    /// constructs one MANIFEST_REQUEST per neigjbour from the queue
    pub fn drain_manifest_reqs(&self, now_ms: u64) -> Vec<(PeerId, ManifestRequest)> {
        let queued: HashMap<PeerId, HashSet<[u8; 8]>> =
            std::mem::take(&mut *self.pending_manifest_requests.write().unwrap());

        let mut out = Vec::new();
        for (neighbour, origins) in queued {
            if !self.allow_manifest_request(neighbour, now_ms) {
                self.pending_manifest_requests
                    .write()
                    .unwrap()
                    .entry(neighbour)
                    .or_default()
                    .extend(origins);
                continue;
            }

            let mut items = Vec::new();
            for origin_node_id in origins {
                let (have_version, have_none) =
                    match self.nodes.read().unwrap().get(&origin_node_id) {
                        Some(node_arc) => {
                            let node = node_arc.read().unwrap();
                            if node.manifest_version == 0 && node.delegated_users.is_empty() {
                                (0, true)
                            } else {
                                (node.manifest_version, false)
                            }
                        }
                        None => (0, true),
                    };

                items.push(ManifestRequestItem {
                    origin_node_id,
                    have_version,
                    item_flags: if have_none { 0x01 } else { 0x00 },
                });

                self.outstanding_manifest_requests
                    .write()
                    .unwrap()
                    .insert((origin_node_id, neighbour), now_ms);

                // per 8.7: n_items is a single byte on the wire protocol
                if items.len() == 255 {
                    break;
                }
            }

            if !items.is_empty() {
                out.push((neighbour, ManifestRequest { items }));
            }
        }
        out
    }

    /// per 10.8: "A node SHALL NOT send more than `manifest_request_rate`
    /// requests per second per neighbour."
    pub fn allow_manifest_request(&self, peer: PeerId, now_ms: u64) -> bool {
        allow_in_window(
            &self.manifest_request_window,
            peer,
            now_ms,
            self.options.manifest_request_rate,
        )
    }

    /// per 8.8: "A node SHALL NOT emit more than `manifest_serve_rate`
    /// responses per second per neighbour; excess items are ignored and the
    /// requester retries."
    pub fn allow_manifest_serve(&self, peer: PeerId, now_ms: u64) -> bool {
        allow_in_window(
            &self.manifest_serve_window,
            peer,
            now_ms,
            self.options.manifest_serve_rate,
        )
    }

    /// drops requests that weren't answered and the time is past the
    /// confgured manifest_request_timeout. per 10.8
    pub fn drop_manifest_req_timeout(&self, now_ms: u64) {
        let timeout_ms = self.options.manifest_request_timeout.saturating_mul(1000);
        let mut outstanding = self.outstanding_manifest_requests.write().unwrap();
        outstanding.retain(|(origin, neighbour), sent_at| {
            let live = now_ms < sent_at.saturating_add(timeout_ms);
            if !live {
                tracing::debug!(
                    "manifest request for origin={origin:?} via peer={neighbour} timed out"
                );
            }
            live
        });
    }
}

/// 1 second sliding-window rate limiter as in section 14
fn allow_in_window(
    window: &RwLock<HashMap<PeerId, VecDeque<u64>>>,
    peer: PeerId,
    now_ms: u64,
    limit: u32,
) -> bool {
    let mut windows = window.write().unwrap();
    let samples = windows.entry(peer).or_default();

    while let Some(oldest) = samples.front() {
        if now_ms.saturating_sub(*oldest) >= 1_000 {
            samples.pop_front();
        } else {
            break;
        }
    }

    if samples.len() >= limit as usize {
        return false;
    }
    samples.push_back(now_ms);
    true
}
