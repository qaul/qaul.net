// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! §11.5 profile fetch: answering for our own identities, and applying a
//! response.

use tracing::debug;

use crate::router_v2::{
    identity::{Multikey, Profile},
    management::Addressing,
    RouterV2State,
};

use proto::{management_message::Body, Profile as ProtoProfile, ProfileResponse};
use qaul_proto::qaul_net_router_management as proto;

/// What the caller must do after a management message was processed.
#[derive(Debug, Clone)]
pub enum ManagementOutcome {
    /// Nothing for the caller to do
    None,
    /// A user profile was verified and cached in the router
    UserProfileLearned {
        user_id: [u8; 8],
        multikey: Multikey,
        name: String,
        profile_version: u32,
        /// this is just an extension, not in the spec
        capabilities: u32,
    },
}

impl RouterV2State {
    /// register the signed profile for a user that this node is a host for
    pub fn register_hosted_profile(&self, user_id: [u8; 8], profile: Profile) {
        self.hosted_profiles
            .write()
            .unwrap()
            .insert(user_id, profile);
    }

    /// Answers a `ProfileRequest` addressed to one of our identities
    pub(crate) fn handle_profile_request(&self, addressing: Addressing, _cached_version: u32) {
        let subject = addressing.destination;

        let profile = if addressing.destination_is_node {
            self.sign_node_profile()
        } else {
            let hosted = self.hosted_profiles.read().unwrap();
            hosted.get(&subject).cloned()
        };

        let Some(profile) = profile else {
            debug!(
                "management: no profile available for {subject:?}, cannot answer request {}",
                addressing.request_id
            );
            return;
        };

        let body = Body::ProfileResponse(ProfileResponse {
            found: true,
            profile: Some(ProtoProfile {
                multikey: profile.multikey.encode(),
                profile_version: profile.version,
                name: profile.name,
                self_signature: profile.self_signature.to_vec(),
                capabilities: self.local_capabilities(),
            }),
        });

        self.forward_management(
            addressing.reply(body),
            addressing.source,
            addressing.source_is_node,
        );
    }

    /// Builds and signs this node's own profile.
    fn sign_node_profile(&self) -> Option<Profile> {
        let mut profile = Profile {
            multikey: self.host_mk.clone(),
            version: 0,
            name: String::new(),
            self_signature: [0u8; 64],
        };

        let signature = self
            .host_keypair
            .sign(&profile.sign_input())
            .inspect_err(|e| debug!("management: signing our node profile failed: {e}"))
            .ok()?;

        profile.self_signature = <[u8; 64]>::try_from(signature)
            .inspect_err(|_| debug!("management: host signature was not 64 bytes"))
            .ok()?;

        Some(profile)
    }

    /// handle a `ProfileResponse` as per 11.5
    pub(crate) fn handle_profile_response(
        &self,
        response: ProfileResponse,
        now_ms: u64,
    ) -> ManagementOutcome {
        let _ = now_ms;

        if !response.found {
            debug!("management: profile not found; leaving the fetch to expire");
            return ManagementOutcome::None;
        }

        let Some(p) = response.profile else {
            debug!("management: found=true with no profile, discarding");
            return ManagementOutcome::None;
        };

        let Ok(multikey) = Multikey::decode(&p.multikey) else {
            debug!("management: undecodable multikey in profile response, discarding");
            return ManagementOutcome::None;
        };

        // §11.5 SHALL
        let subject = multikey.to_id();

        let Ok(signature) = <[u8; 64]>::try_from(p.self_signature.as_slice()) else {
            debug!("management: self_signature for {subject:?} is not 64 bytes, discarding");
            return ManagementOutcome::None;
        };

        let rebuilt = Profile {
            multikey: multikey.clone(),
            version: p.profile_version,
            name: p.name.clone(),
            self_signature: signature,
        };

        // §11.5 SHALL: self_signature must verify against multikey.
        if !multikey.verify(&rebuilt.sign_input(), &signature) {
            debug!("management: self_signature for {subject:?} does not verify, discarding");
            return ManagementOutcome::None;
        };

        // just accept profiles we actually asked for.
        let asked_as_user = self
            .management_in_flight
            .write()
            .unwrap()
            .remove(&(subject, false))
            .is_some();
        let asked_as_node = self
            .management_in_flight
            .write()
            .unwrap()
            .remove(&(subject, true))
            .is_some();

        if !asked_as_user && !asked_as_node {
            debug!("management: unsolicited profile for {subject:?}, discarding");
            return ManagementOutcome::None;
        }

        if asked_as_node {
            if let Some(node) = self.nodes.read().unwrap().get(&subject) {
                node.write().unwrap().public_key = Some(multikey.clone());
            }
        }

        if asked_as_user {
            if let Some(user) = self.users.read().unwrap().get(&subject) {
                let mut u = user.write().unwrap();
                u.public_key = Some(multikey.clone());
                u.profile_version = p.profile_version;
            }
        }

        self.refresh_trust_for_subject(&subject, now_ms);

        if asked_as_user {
            ManagementOutcome::UserProfileLearned {
                user_id: subject,
                multikey,
                name: p.name,
                profile_version: p.profile_version,
                capabilities: p.capabilities,
            }
        } else {
            ManagementOutcome::None
        }
    }

    fn refresh_trust_for_subject(&self, subject: &[u8; 8], now_ms: u64) {
        let origins: Vec<[u8; 8]> = {
            let nodes = self.nodes.read().unwrap();
            nodes
                .iter()
                .filter(|(_, node)| {
                    node.read()
                        .unwrap()
                        .delegated_users
                        .iter()
                        .any(|d| d.user_id == *subject)
                })
                .map(|(id, _)| *id)
                .collect()
        };

        for origin in origins {
            self.refresh_delegation_trust(&origin, now_ms);
        }
    }

    fn local_capabilities(&self) -> u32 {
        crate::router::users::Capabilities::LOCAL
    }
}
