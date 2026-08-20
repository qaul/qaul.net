// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Cross-host delegation, issuing side: choosing a gateway to delegate to
//! (spec §10.3). The answering side lives in `management/delegation.rs`.

use crate::router_v2::{
    identity::{Multikey, SelfDelegation},
    index::Space,
    management::{Addressing, MANAGEMENT_VERSION},
    RouterV2State,
};

use proto::{
    management_message::Body, DelegationSubscribe, DelegationSubscribeAck, ManagementMessage,
};
use qaul_proto::qaul_net_router_management as proto;

/// A node that satisfies both §10.3 selection criteria.
#[derive(Debug, Clone)]
pub struct GatewayCandidate {
    pub node_id: [u8; 8],
    pub multikey: Option<Multikey>,
    pub metric: u16,
}

impl RouterV2State {
    /// delegation targets in order. prioritize best metric
    pub fn eligible_gateways(&self) -> Vec<GatewayCandidate> {
        let gateways: Vec<([u8; 8], Option<Multikey>)> = {
            let nodes = self.nodes.read().unwrap();
            nodes
                .iter()
                .filter_map(|(id, node)| {
                    let node = node.read().unwrap();
                    node.is_gateway.then(|| (*id, node.public_key.clone()))
                })
                .collect()
        };

        let mut candidates: Vec<GatewayCandidate> = gateways
            .into_iter()
            .filter_map(|(node_id, multikey)| {
                let idx = self.node_dict.read().unwrap().idx_of(&node_id)?;
                let entry = self.routing_table.read().unwrap().get(Space::Node, idx)?;
                let entry = entry.read().unwrap();

                entry.local_only.then_some(GatewayCandidate {
                    node_id,
                    multikey,
                    metric: entry.metric,
                })
            })
            .collect();

        candidates.sort_by_key(|c| c.metric);
        candidates
    }
}

/// A cross-host delegation a gateway has accepted for one of our users.
#[derive(Debug, Clone)]
pub struct Subscription {
    pub target_node_id: [u8; 8],
    pub timeout: u64,
    pub acked_at_ms: u64,
}

/// this is when a user asks a gateway to carry it
#[derive(Debug, Clone, Copy)]
pub struct DelegationRequest {
    pub user_id: [u8; 8],
    pub target_node_id: [u8; 8],
    pub delegation: SelfDelegation,
}

/// A [`DelegationRequest`] that has gone out and is waiting for its ack.
#[derive(Debug, Clone, Copy)]
pub struct OutstandingSubscribe {
    pub request: DelegationRequest,
    pub sent_at_ms: u64,
}

/// How long a refusal keeps a gateway out of the running
const DECLINE_MEMORY_MS: u64 = 5 * 60 * 1000;

impl RouterV2State {
    /// The gateway this user should subscribe to next
    pub fn select_delegation_target(
        &self,
        user_id: &[u8; 8],
        now_ms: u64,
    ) -> Option<GatewayCandidate> {
        if self.has_live_subscription(user_id, now_ms) {
            return None;
        }
        if self
            .outstanding_subscribes
            .read()
            .unwrap()
            .values()
            .any(|o| o.request.user_id == *user_id)
        {
            return None;
        }

        let declined: Vec<[u8; 8]> = {
            let d = self.declined_targets.read().unwrap();
            d.keys()
                .filter(|(u, _)| u == user_id)
                .map(|(_, node)| *node)
                .collect()
        };

        let best = self
            .eligible_gateways()
            .into_iter()
            .find(|c| !declined.contains(&c.node_id))?;

        if best.multikey.is_none() {
            tracing::debug!(
                "delegation: no key for candidate {:?}, fetching before subscribing",
                best.node_id
            );
            self.request_profile(best.node_id, true, now_ms);
            return None;
        }

        Some(best)
    }

    pub fn has_live_subscription(&self, user_id: &[u8; 8], now_ms: u64) -> bool {
        self.subscriptions
            .read()
            .unwrap()
            .get(user_id)
            .is_some_and(|s| s.timeout > now_ms)
    }

    /// 11.6 says to send a subscription, we send it and remember it
    /// so that the ack can be matched
    pub fn send_delegation_subscribe(&self, req: DelegationRequest, now_ms: u64) -> bool {
        let request_id = self
            .next_request_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let envelope = ManagementMessage {
            version: MANAGEMENT_VERSION,
            destination: req.target_node_id.to_vec(),
            destination_is_node: true,
            source: req.user_id.to_vec(),
            source_is_node: false,
            request_id,
            body: Some(Body::DelegationSubscribe(DelegationSubscribe {
                user_id: req.user_id.to_vec(),
                timeout: req.delegation.timeout,
                entry_signature: req.delegation.entry_signature.to_vec(),
            })),
        };

        if !self.forward_management(envelope, req.target_node_id, true) {
            return false;
        }

        self.outstanding_subscribes.write().unwrap().insert(
            request_id,
            OutstandingSubscribe {
                request: req,
                sent_at_ms: now_ms,
            },
        );
        tracing::info!(
            "router_v2 DELEGATION → user={:?} target={:?} request={request_id} (§11.6)",
            req.user_id,
            req.target_node_id,
        );
        true
    }

    /// the gateway's verdict on a subscribe we sent
    pub(crate) fn handle_delegation_subscribe_ack(
        &self,
        addressing: Addressing,
        ack: DelegationSubscribeAck,
        now_ms: u64,
    ) {
        let matches = self
            .outstanding_subscribes
            .read()
            .unwrap()
            .get(&addressing.request_id)
            .is_some_and(|o| {
                addressing.source_is_node && addressing.source == o.request.target_node_id
            });

        if !matches {
            tracing::debug!(
                "delegation: ack {} from {:?} matches no outstanding subscribe, discarding",
                addressing.request_id,
                addressing.source
            );
            return;
        }

        let pending = self
            .outstanding_subscribes
            .write()
            .unwrap()
            .remove(&addressing.request_id)
            .expect("checked just above");

        if ack.accepted {
            tracing::info!(
                "router_v2 DELEGATION accepted by {:?} for user={:?} (§11.6)",
                pending.request.target_node_id,
                pending.request.user_id
            );
            self.subscriptions.write().unwrap().insert(
                pending.request.user_id,
                Subscription {
                    target_node_id: pending.request.target_node_id,
                    timeout: pending.request.delegation.timeout,
                    acked_at_ms: now_ms,
                },
            );
        } else {
            //try a different acceptable gateway
            tracing::debug!(
                "delegation: {:?} refused user={:?} (reason {}), trying another gateway",
                pending.request.target_node_id,
                pending.request.user_id,
                ack.reason
            );
            self.declined_targets.write().unwrap().insert(
                (pending.request.user_id, pending.request.target_node_id),
                now_ms,
            );
        }
    }

    /// Expires unanswered subscribes, stale refusals and lapsed
    /// subscriptions.
    pub fn clear_delegation_state(&self, now_ms: u64) {
        let timeout_ms = self.options.manifest_request_timeout.saturating_mul(1000);

        let timed_out: Vec<([u8; 8], [u8; 8])> = {
            let mut outstanding = self.outstanding_subscribes.write().unwrap();
            let mut lapsed = Vec::new();
            outstanding.retain(|_, o| {
                let live = now_ms < o.sent_at_ms.saturating_add(timeout_ms);
                if !live {
                    lapsed.push((o.request.user_id, o.request.target_node_id));
                }
                live
            });
            lapsed
        };

        if !timed_out.is_empty() {
            // A silent target is as unusable as one that refused, and §11.2
            // gives us no other signal. Treat it the same so the next tick
            // moves on instead of retrying the same node forever.
            let mut declined = self.declined_targets.write().unwrap();
            for key in timed_out {
                tracing::debug!("delegation: subscribe to {:?} timed out", key.1);
                declined.insert(key, now_ms);
            }
        }

        self.declined_targets
            .write()
            .unwrap()
            .retain(|_, at| now_ms < at.saturating_add(DECLINE_MEMORY_MS));

        self.subscriptions
            .write()
            .unwrap()
            .retain(|_, s| s.timeout > now_ms);
    }
}
