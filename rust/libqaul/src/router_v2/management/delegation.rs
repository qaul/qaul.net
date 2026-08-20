// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! This verifies a user's request to be carried in this node's manifest
//! and it answers it. as per 11.6

use tracing::debug;

use crate::router_v2::{
    index::Space,
    management::Addressing,
    manifest::{DelegetedEntry, Manifest},
    RouterV2State,
};

use proto::{management_message::Body, DelegationSubscribe, DelegationSubscribeAck, RejectReason};
use qaul_proto::qaul_net_router_management as proto;

/// we hold this while fetching the delegating user's key
#[derive(Debug, Clone)]
pub(crate) struct PendingSubscribe {
    pub addressing: Addressing,
    pub timeout: u64,
    pub entry_signature: [u8; 64],
    pub parked_at_ms: u64,
}

impl RouterV2State {
    /// §11.6: a user asks to be carried in our manifest.
    pub(crate) fn handle_delegation_subscribe(
        &self,
        addressing: Addressing,
        req: DelegationSubscribe,
        now_ms: u64,
    ) {
        let (Ok(user_id), Ok(entry_signature)) = (
            <[u8; 8]>::try_from(req.user_id.as_slice()),
            <[u8; 64]>::try_from(req.entry_signature.as_slice()),
        ) else {
            debug!(
                "delegation: malformed subscribe from {:?}, discarding",
                addressing.source
            );
            return;
        };

        // sign over the target's multikey: 10.1
        if !addressing.destination_is_node || addressing.destination != self.host_mk.to_id() {
            debug!(
                "delegation: subscribe addressed to {:?}, not us",
                addressing.destination
            );
            return;
        }

        if req.timeout <= now_ms {
            debug!("delegation: subscribe from {user_id:?} is already expired, rejecting");
            self.send_subscribe_ack(addressing, false, RejectReason::Policy);
            return;
        }

        match self.get_resource_mk(&user_id, Space::User) {
            Some(user_mk) => {
                self.settle_subscribe(addressing, user_id, req.timeout, entry_signature, &user_mk)
            }
            None => {
                debug!(
                    "delegation: no key for {user_id:?}, parking subscribe behind a §11.5 fetch"
                );
                self.pending_subscribes
                    .write()
                    .unwrap()
                    .entry(user_id)
                    .or_default()
                    .push(PendingSubscribe {
                        addressing,
                        timeout: req.timeout,
                        entry_signature,
                        parked_at_ms: now_ms,
                    });
                self.request_profile(user_id, false, now_ms);
            }
        }
    }

    /// verify and either record or reject
    fn settle_subscribe(
        &self,
        addressing: Addressing,
        user_id: [u8; 8],
        timeout: u64,
        entry_signature: [u8; 64],
        user_mk: &crate::router_v2::identity::Multikey,
    ) {
        let entry = DelegetedEntry {
            user_id,
            timeout,
            entry_signature,
            profile_version: self.cached_profile_version(&user_id),
        };

        if let Err(e) = Manifest::verify_entry(&entry, &self.host_mk, user_mk) {
            debug!("delegation: subscribe from {user_id:?} does not verify ({e:?}), rejecting");
            self.send_subscribe_ack(addressing, false, RejectReason::Policy);
            return;
        }

        let changed = self.record_delegation(entry);
        tracing::info!(
            "router_v2 DELEGATION accepted user={user_id:?} timeout={timeout} changed={changed} (§11.6)"
        );
        self.send_subscribe_ack(addressing, true, RejectReason::None);
    }

    /// Completes every subscribe that was waiting on this user's key.
    pub(crate) fn resume_pending_subscribes(&self, user_id: &[u8; 8], _now_ms: u64) {
        let parked = self
            .pending_subscribes
            .write()
            .unwrap()
            .remove(user_id)
            .unwrap_or_default();
        if parked.is_empty() {
            return;
        }

        let Some(user_mk) = self.get_resource_mk(user_id, Space::User) else {
            debug!(
                "delegation: {user_id:?} still has no key, dropping {} parked subscribe(s)",
                parked.len()
            );
            return;
        };

        for p in parked {
            self.settle_subscribe(
                p.addressing,
                *user_id,
                p.timeout,
                p.entry_signature,
                &user_mk,
            );
        }
    }

    /// drops subscriptions that their "fetch" never returned anything
    pub(crate) fn clear_pending_subscribes(&self, now_ms: u64) {
        let window_ms = self.options.manifest_request_timeout.saturating_mul(1000);
        let mut pending = self.pending_subscribes.write().unwrap();

        for parked in pending.values_mut() {
            parked.retain(|p| now_ms < p.parked_at_ms.saturating_add(window_ms));
        }
        pending.retain(|_, parked| !parked.is_empty());
    }

    fn cached_profile_version(&self, user_id: &[u8; 8]) -> u32 {
        self.users
            .read()
            .unwrap()
            .get(user_id)
            .map(|u| u.read().unwrap().profile_version)
            .unwrap_or(0)
    }

    fn send_subscribe_ack(&self, addressing: Addressing, accepted: bool, reason: RejectReason) {
        let body = Body::DelegationSubscribeAck(DelegationSubscribeAck {
            accepted,
            reason: reason as i32,
        });
        self.forward_management(
            addressing.reply(body),
            addressing.source,
            addressing.source_is_node,
        );
    }
}
