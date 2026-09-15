// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! §11.4 forwarding: decide whether a management message is for this user, and if
//! not, pass it one hop closer.

use libp2p::PeerId;
use prost::Message;
use tracing::debug;

use crate::router_v2::{
    management::{profile::ManagementOutcome, Addressing, ForwardKey},
    RouterV2State,
};

use proto::{management_message::Body, ManagementMessage};
use qaul_proto::qaul_net_router_management as proto;

use super::MANAGEMENT_VERSION;

/// How long a forwarded [`ForwardKey`] is remembered
const FORWARD_MEMORY_MS: u64 = 5_000;

impl RouterV2State {
    /// what must the caller must still do
    pub fn on_management_received(
        &self,
        from_peer: PeerId,
        bytes: &[u8],
        now_ms: u64,
    ) -> ManagementOutcome {
        debug!(
            "management: received {} bytes from {from_peer}",
            bytes.len()
        );
        let envelope = match ManagementMessage::decode(bytes) {
            Ok(m) => m,
            Err(e) => {
                debug!("management: undecodable message from {from_peer}: {e}");
                return ManagementOutcome::None;
            }
        };

        // §8.8 step 2's rule, applied here: an unrecognised version is
        // discarded rather than guessed at.
        if envelope.version != MANAGEMENT_VERSION {
            debug!(
                "management: unrecognised version {} from {from_peer}, discarding",
                envelope.version
            );
            return ManagementOutcome::None;
        }

        let (Some(destination), Some(source)) =
            (as_id(&envelope.destination), as_id(&envelope.source))
        else {
            debug!("management: malformed source/destination from {from_peer}, discarding");
            return ManagementOutcome::None;
        };

        // §11.4 step 1: is it ours?
        if self.is_local_identity(destination, envelope.destination_is_node) {
            return self.dispatch_management_body(envelope, destination, source, now_ms);
        }

        // §11.4 step 2.
        let key: ForwardKey = (
            source,
            destination,
            envelope.request_id,
            is_response(&envelope.body),
        );
        if !self.remember_forward(key, now_ms) {
            debug!(
                "management: already forwarded request {} from {source:?} to {destination:?} \
                 (is_response={}), dropping to break a loop",
                envelope.request_id, key.3
            );
            return ManagementOutcome::None;
        }

        let destination_is_node = envelope.destination_is_node;
        self.forward_management(envelope, destination, destination_is_node);
        ManagementOutcome::None
    }

    /// `true` when this exact message has not been forwarded inside
    /// `FORWARD_MEMORY_MS`
    fn remember_forward(&self, key: ForwardKey, now_ms: u64) -> bool {
        let mut seen = self.management_recent_forwards.write().unwrap();
        seen.retain(|_, at| now_ms < at.saturating_add(FORWARD_MEMORY_MS));
        seen.insert(key, now_ms).is_none()
    }

    /// §11.4 step 1: the message is addressed to us, so act on its body.
    fn dispatch_management_body(
        &self,
        envelope: ManagementMessage,
        destination: [u8; 8],
        source: [u8; 8],
        now_ms: u64,
    ) -> ManagementOutcome {
        let addressing = Addressing {
            destination,
            destination_is_node: envelope.destination_is_node,
            source,
            source_is_node: envelope.source_is_node,
            request_id: envelope.request_id,
        };

        match envelope.body {
            Some(Body::ProfileRequest(req)) => {
                self.handle_profile_request(addressing, req);
                ManagementOutcome::None
            }
            Some(Body::ProfileResponse(resp)) => self.handle_profile_response(resp, now_ms),
            Some(Body::DelegationSubscribe(req)) => {
                self.handle_delegation_subscribe(addressing, req, now_ms);
                ManagementOutcome::None
            }
            Some(Body::DelegationSubscribeAck(ack)) => {
                self.handle_delegation_subscribe_ack(addressing, ack, now_ms);
                ManagementOutcome::None
            }
            Some(Body::DelegationRevoke(req)) => {
                self.handle_delegation_revoke(addressing, req, now_ms);
                ManagementOutcome::None
            }
            Some(Body::DelegationRevokeAck(ack)) => {
                self.handle_delegation_revoke_ack(addressing, ack);
                ManagementOutcome::None
            }
            None => {
                debug!(
                    "management: request {} from {source:?} carries no body we recognise, ignoring",
                    addressing.request_id
                );
                ManagementOutcome::None
            }
        }
    }
}

fn as_id(bytes: &[u8]) -> Option<[u8; 8]> {
    bytes.try_into().ok()
}

/// Whether this envelope answers a request rather than making one, per: 11.2
fn is_response(body: &Option<Body>) -> bool {
    matches!(
        body,
        Some(Body::ProfileResponse(_))
            | Some(Body::DelegationSubscribeAck(_))
            | Some(Body::DelegationRevokeAck(_))
    )
}
