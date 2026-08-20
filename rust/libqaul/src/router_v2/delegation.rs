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
        let node_ids: Vec<[u8; 8]> = self
            .nodes
            .read()
            .unwrap()
            .iter()
            .map(|(id, _)| *id)
            .collect();

        let mut candidates: Vec<GatewayCandidate> = node_ids
            .into_iter()
            .filter_map(|id| self.candidate_for(&id))
            .collect();

        candidates.sort_by_key(|c| c.metric);
        candidates
    }

    /// This node as a §10.3 candidate
    pub fn candidate_for(&self, node_id: &[u8; 8]) -> Option<GatewayCandidate> {
        // criterion 2: it must be a gateway, or its manifest never crosses
        // the Internet sphere and carrying the user achieves nothing.
        let multikey = {
            let nodes = self.nodes.read().unwrap();
            let node = nodes.get(node_id)?;
            let node = node.read().unwrap();
            if !node.is_gateway {
                return None;
            }
            node.public_key.clone()
        };

        // criterion 1: local-sphere reachable
        let idx = self.node_dict.read().unwrap().idx_of(node_id)?;
        let entry = self.routing_table.read().unwrap().get(Space::Node, idx)?;
        let entry = entry.read().unwrap();

        entry.local_only.then_some(GatewayCandidate {
            node_id: *node_id,
            multikey,
            metric: entry.metric,
        })
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

        // when the refresh window opens, a settled user is re-issued to the same gateway
        if let Some(current) = self.subscriptions.read().unwrap().get(user_id) {
            if !self.subscription_due_for_refresh(current, now_ms) {
                return None;
            }
            if !declined.contains(&current.target_node_id) {
                if let Some(candidate) = self.candidate_for(&current.target_node_id) {
                    return Some(candidate);
                }
            }
            // the target has stopped qualifying, or just refused, we can simply fall thru
        }

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

    /// inside TTL/2 of expiry, re-issue rather than wait for the gateway to drop us
    fn subscription_due_for_refresh(&self, subscription: &Subscription, now_ms: u64) -> bool {
        let window_ms = self.options.delegation_referesh.saturating_mul(1000);
        subscription.timeout <= now_ms.saturating_add(window_ms)
    }

    /// §10.3: drops subscriptions whose target has stopped qualifying.
    fn prune_broken_subscriptions(&self, now_ms: u64) {
        let broken: Vec<([u8; 8], [u8; 8])> = self
            .subscriptions
            .read()
            .unwrap()
            .iter()
            .filter(|(_, s)| self.candidate_for(&s.target_node_id).is_none())
            .map(|(user, s)| (*user, s.target_node_id))
            .collect();

        if broken.is_empty() {
            return;
        }

        let mut subscriptions = self.subscriptions.write().unwrap();
        for (user_id, target) in broken {
            tracing::info!(
                "router_v2 DELEGATION broken: target={target:?} no longer eligible for user={user_id:?}, will re-select (§10.3)"
            );
            subscriptions.remove(&user_id);
        }
        let _ = now_ms;
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

        self.prune_broken_subscriptions(now_ms);
    }
}
