// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Network management sub-protocol (spec §11).

use tracing::debug;

use crate::router_v2::{OutboundKind, OutboundMsg, RouterV2State};

use prost::Message;
use proto::{management_message::Body, ManagementMessage, ProfileRequest};
use qaul_proto::qaul_net_router_management as proto;

/// Sub-protocol version we speak and accept (§11.3).
pub const MANAGEMENT_VERSION: u32 = 1;

impl RouterV2State {
    /// schedules a profile fetch for `subject` per: 11.5
    pub fn request_profile(&self, subject: [u8; 8], is_node: bool, now_ms: u64) {
        if self
            .management_in_flight
            .read()
            .unwrap()
            .contains_key(&(subject, is_node))
        {
            return;
        }

        let cached_version = if is_node {
            // since a node's profile version isn't tracked separately, we can simple ask for anhthing
            0
        } else {
            self.users
                .read()
                .unwrap()
                .get(&subject)
                .map(|u| u.read().unwrap().profile_version)
                .unwrap_or(0)
        };

        let request_id = self
            .next_request_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let envelope = ManagementMessage {
            version: MANAGEMENT_VERSION,
            destination: subject.to_vec(),
            destination_is_node: is_node,
            source: self.host_mk.to_id().to_vec(),
            source_is_node: true,
            request_id,
            body: Some(Body::ProfileRequest(ProfileRequest { cached_version })),
        };

        if self.forward_management(envelope, subject, is_node) {
            self.management_in_flight
                .write()
                .unwrap()
                .insert((subject, is_node), now_ms);
        }
    }

    /// Sends an envelope one hop toward its destination (§11.4 step 2).
    pub(crate) fn forward_management(
        &self,
        envelope: ManagementMessage,
        destination: [u8; 8],
        destination_is_node: bool,
    ) -> bool {
        let hop = if destination_is_node {
            self.next_hop_for_node(destination)
        } else {
            self.next_hop_for_user(destination)
        };

        let Some((next_hop_node, transport)) = hop else {
            debug!("management: no route to {destination:?}, dropping (§11.2 best-effort)");
            return false;
        };

        let Some(peer) = self.peer_of_node(&next_hop_node) else {
            debug!("management: next hop {next_hop_node:?} is no longer a neighbour, dropping");
            return false;
        };

        let bytes = envelope.encode_to_vec();
        if let Err(e) = self.tx_outbound.send(OutboundMsg {
            peer,
            transport,
            kind: OutboundKind::Management,
            bytes,
        }) {
            debug!("management: outbound channel closed for {peer}: {e}");
            return false;
        }
        true
    }

    /// drops messages that were not answered: 11.2 and 14
    pub fn clear_management_msgs(&self, now_ms: u64) {
        let timeout_ms = self.options.manifest_request_timeout.saturating_mul(1000);
        let mut in_flight = self.management_in_flight.write().unwrap();
        in_flight.retain(|(subject, is_node), sent_at| {
            let live = now_ms < sent_at.saturating_add(timeout_ms);
            if !live {
                debug!("management: profile fetch for {subject:?} (is_node={is_node}) timed out");
            }
            live
        });
    }
}
