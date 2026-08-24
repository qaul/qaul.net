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

use proto::{
    management_message::Body, DelegationRevoke, DelegationRevokeAck, DelegationSubscribe,
    DelegationSubscribeAck, RejectReason,
};
use qaul_proto::qaul_net_router_management as proto;

/// which message the delegation action camde from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DelegationAction {
    Subscribe,
    Revoke,
}

/// awaiting verification
#[derive(Debug, Clone, Copy)]
pub(crate) struct PendingDelegation {
    pub addressing: Addressing,
    pub action: DelegationAction,
    pub timeout: u64,
    pub signature: [u8; 64],
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
        let (Ok(user_id), Ok(signature)) = (
            <[u8; 8]>::try_from(req.user_id.as_slice()),
            <[u8; 64]>::try_from(req.entry_signature.as_slice()),
        ) else {
            debug!(
                "delegation: malformed subscribe from {:?}, discarding",
                addressing.source
            );
            return;
        };

        if !self.addressed_to_us(&addressing) {
            return;
        }

        if req.timeout <= now_ms {
            debug!("delegation: subscribe from {user_id:?} is already expired, rejecting");
            self.send_subscribe_ack(addressing, false, RejectReason::Policy);
            return;
        }

        self.admit_delegation(
            PendingDelegation {
                addressing,
                action: DelegationAction::Subscribe,
                timeout: req.timeout,
                signature,
                parked_at_ms: now_ms,
            },
            user_id,
            now_ms,
        );
    }

    /// §11.7: a user cancels a delegation we hold for it.
    pub(crate) fn handle_delegation_revoke(
        &self,
        addressing: Addressing,
        req: DelegationRevoke,
        now_ms: u64,
    ) {
        let (Ok(user_id), Ok(signature)) = (
            <[u8; 8]>::try_from(req.user_id.as_slice()),
            <[u8; 64]>::try_from(req.revoke_signature.as_slice()),
        ) else {
            debug!(
                "delegation: malformed revoke from {:?}, discarding",
                addressing.source
            );
            return;
        };

        if !self.addressed_to_us(&addressing) {
            return;
        }

        self.admit_delegation(
            PendingDelegation {
                addressing,
                action: DelegationAction::Revoke,
                timeout: req.timeout,
                signature,
                parked_at_ms: now_ms,
            },
            user_id,
            now_ms,
        );
    }

    fn addressed_to_us(&self, addressing: &Addressing) -> bool {
        let ours = addressing.destination_is_node && addressing.destination == self.host_mk.to_id();
        if !ours {
            debug!(
                "delegation: addressed to {:?}, not us",
                addressing.destination
            );
        }
        ours
    }

    /// Verify now if we hold the user's key, otherwise park behind a §11.5
    /// fetch and finish when it lands.
    fn admit_delegation(&self, pending: PendingDelegation, user_id: [u8; 8], now_ms: u64) {
        match self.get_resource_mk(&user_id, Space::User) {
            Some(user_mk) => self.settle_delegation(pending, user_id, &user_mk),
            None => {
                debug!(
                    "delegation: no key for {user_id:?}, parking {:?} behind a §11.5 fetch",
                    pending.action
                );
                self.pending_subscribes
                    .write()
                    .unwrap()
                    .entry(user_id)
                    .or_default()
                    .push(pending);
                self.request_profile(user_id, false, now_ms);
            }
        }
    }

    /// Verify, then apply the action the message asked for.
    fn settle_delegation(
        &self,
        pending: PendingDelegation,
        user_id: [u8; 8],
        user_mk: &crate::router_v2::identity::Multikey,
    ) {
        let entry = DelegetedEntry {
            user_id,
            timeout: pending.timeout,
            entry_signature: pending.signature,
            profile_version: self.cached_profile_version(&user_id),
        };
        let verified = Manifest::verify_entry(&entry, &self.host_mk, user_mk).is_ok();

        match pending.action {
            DelegationAction::Subscribe => {
                if !verified {
                    debug!("delegation: subscribe from {user_id:?} does not verify, rejecting");
                    self.send_subscribe_ack(pending.addressing, false, RejectReason::Policy);
                    return;
                }
                let changed = self.record_delegation(entry);
                tracing::info!(
                    "router_v2 DELEGATION accepted user={user_id:?} timeout={} changed={changed} (§11.6)",
                    pending.timeout
                );
                self.send_subscribe_ack(pending.addressing, true, RejectReason::None);
            }
            DelegationAction::Revoke => {
                if !verified {
                    debug!("delegation: revoke from {user_id:?} does not verify, refusing");
                    self.send_revoke_ack(pending.addressing, false);
                    return;
                }

                let removed = self
                    .stored_delegation_timeout(&user_id)
                    .is_some_and(|stored| stored == pending.timeout)
                    && self.remove_self_delegation(&user_id);

                tracing::info!(
                    "router_v2 DELEGATION revoked user={user_id:?} timeout={} removed={removed} (§11.7)",
                    pending.timeout
                );
                self.send_revoke_ack(pending.addressing, true);
            }
        }
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
                "delegation: {user_id:?} still has no key, dropping {} parked delegation action(s)",
                parked.len()
            );
            return;
        };

        for pending in parked {
            self.settle_delegation(pending, *user_id, &user_mk);
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

    /// The timeout of the entry we currently hold for this user, if any.
    fn stored_delegation_timeout(&self, user_id: &[u8; 8]) -> Option<u64> {
        self.manifest
            .read()
            .unwrap()
            .entries()
            .iter()
            .find(|e| e.user_id == *user_id)
            .map(|e| e.timeout)
    }

    fn send_revoke_ack(&self, addressing: Addressing, done: bool) {
        self.forward_management(
            addressing.reply(Body::DelegationRevokeAck(DelegationRevokeAck { done })),
            addressing.source,
            addressing.source_is_node,
        );
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
