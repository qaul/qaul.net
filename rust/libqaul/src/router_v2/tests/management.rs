// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Network management sub-protocol: envelope, profile fetch scheduling
//! (spec §11.3, §11.5, §14).

use crate::connections::ConnectionModule;
use crate::router_v2::{
    identity::{Multikey, Profile},
    index::Space,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
    RouterV2State,
};
use libp2p::identity::Keypair;
use prost::Message;
use std::sync::{Arc, RwLock};

use proto::{
    management_message::Body, ManagementMessage, Profile as ProtoProfile, ProfileRequest,
    ProfileResponse,
};
use qaul_proto::qaul_net_router_management as proto;

// ---------- §11.3 envelope ----------

mod envelope {
    use super::*;

    fn request_envelope() -> ManagementMessage {
        ManagementMessage {
            version: 1,
            destination: vec![1, 2, 3, 4, 5, 6, 7, 8],
            destination_is_node: false,
            source: vec![9, 10, 11, 12, 13, 14, 15, 16],
            source_is_node: true,
            request_id: 42,
            body: Some(Body::ProfileRequest(ProfileRequest { cached_version: 7 })),
        }
    }

    #[test]
    fn a_profile_request_round_trips() {
        let msg = request_envelope();
        let decoded = ManagementMessage::decode(&msg.encode_to_vec()[..]).unwrap();

        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.destination, msg.destination);
        assert!(!decoded.destination_is_node);
        assert!(decoded.source_is_node);
        assert_eq!(decoded.request_id, 42);
        match decoded.body {
            Some(Body::ProfileRequest(r)) => assert_eq!(r.cached_version, 7),
            other => panic!("expected a ProfileRequest body, got {other:?}"),
        }
    }

    #[test]
    fn a_profile_response_round_trips() {
        let msg = ManagementMessage {
            version: 1,
            destination: vec![9, 10, 11, 12, 13, 14, 15, 16],
            destination_is_node: true,
            source: vec![1, 2, 3, 4, 5, 6, 7, 8],
            source_is_node: false,
            request_id: 42,
            body: Some(Body::ProfileResponse(ProfileResponse {
                found: true,
                profile: Some(ProtoProfile {
                    multikey: vec![0xAA; 36],
                    profile_version: 3,
                    name: "alice".into(),
                    self_signature: vec![0xBB; 64],
                    capabilities: 1,
                }),
            })),
        };
        let decoded = ManagementMessage::decode(&msg.encode_to_vec()[..]).unwrap();

        match decoded.body {
            Some(Body::ProfileResponse(r)) => {
                assert!(r.found);
                let p = r.profile.expect("profile present");
                assert_eq!(p.profile_version, 3);
                assert_eq!(p.name, "alice");
                assert_eq!(p.capabilities, 1);
            }
            other => panic!("expected a ProfileResponse body, got {other:?}"),
        }
    }

    /// §11.5 permits `found: false`; the profile is then absent rather than
    /// empty, and a receiver must not treat a default-constructed profile as
    /// a real one.
    #[test]
    fn a_not_found_response_carries_no_profile() {
        let msg = ManagementMessage {
            version: 1,
            destination: vec![0; 8],
            destination_is_node: false,
            source: vec![1; 8],
            source_is_node: true,
            request_id: 1,
            body: Some(Body::ProfileResponse(ProfileResponse {
                found: false,
                profile: None,
            })),
        };
        let decoded = ManagementMessage::decode(&msg.encode_to_vec()[..]).unwrap();

        match decoded.body {
            Some(Body::ProfileResponse(r)) => {
                assert!(!r.found);
                assert!(r.profile.is_none());
            }
            other => panic!("expected a ProfileResponse body, got {other:?}"),
        }
    }

    /// Phase 13 will add field numbers 9-12 (subscribe/revoke). A message
    /// carrying one must decode without error today, with an unrecognised
    /// body, so a newer peer cannot break an older one.
    #[test]
    fn an_unknown_body_variant_decodes_without_error() {
        let mut bytes = request_envelope().encode_to_vec();
        // field 9, wire type 2 (length-delimited), empty payload
        bytes.extend_from_slice(&[(9 << 3) | 2, 0]);

        let decoded = ManagementMessage::decode(&bytes[..])
            .expect("a reserved-field body must not break decoding");
        assert_eq!(decoded.request_id, 42);
    }

    /// The wire `Profile` and the internal one must agree on what is signed:
    /// §11.5 fixes the input at `multikey || profile_version || name`, and
    /// `capabilities` is deliberately outside it.
    #[test]
    fn the_wire_profile_and_the_signed_input_agree() {
        let kp = Keypair::generate_ed25519();
        let mk = Multikey::from(kp.public());

        let internal = Profile {
            multikey: mk.clone(),
            version: 5,
            name: "alice".into(),
            self_signature: [0u8; 64],
        };
        let signing_input = internal.sign_input();
        let signature: [u8; 64] = kp.sign(&signing_input).unwrap().try_into().unwrap();

        let wire = ProtoProfile {
            multikey: mk.encode(),
            profile_version: 5,
            name: "alice".into(),
            self_signature: signature.to_vec(),
            capabilities: 0xFF, // must not affect verification
        };

        let rebuilt = Profile {
            multikey: Multikey::decode(&wire.multikey).unwrap(),
            version: wire.profile_version,
            name: wire.name.clone(),
            self_signature: signature,
        };
        assert_eq!(rebuilt.sign_input(), signing_input);
        assert!(
            mk.verify(&rebuilt.sign_input(), &signature),
            "a profile rebuilt from the wire must verify"
        );
    }

    /// Changing a signed field breaks verification; changing `capabilities`
    /// does not. This is the whole reason capabilities sit outside the
    /// signature — and the known exposure it carries.
    #[test]
    fn capabilities_are_outside_the_signature() {
        let kp = Keypair::generate_ed25519();
        let mk = Multikey::from(kp.public());

        let profile = Profile {
            multikey: mk.clone(),
            version: 5,
            name: "alice".into(),
            self_signature: [0u8; 64],
        };
        let sig: [u8; 64] = kp.sign(&profile.sign_input()).unwrap().try_into().unwrap();

        // capabilities is not an input, so the same signature verifies
        // whatever a forwarder sets it to.
        assert!(mk.verify(&profile.sign_input(), &sig));

        // a signed field, by contrast, invalidates it
        let tampered = Profile {
            name: "mallory".into(),
            ..profile
        };
        assert!(!mk.verify(&tampered.sign_input(), &sig));
    }
}

// ---------- §11.5 fetch scheduling ----------
//
// Per decision 12 there are no futures: a trigger schedules a fetch and
// returns, and the response handler caches and re-runs trust evaluation. So
// the observable behaviour is "a message went out" plus the in-flight
// bookkeeping that deduplicates and expires.

mod request_profile {
    use super::*;

    fn entry(
        target: TargetRef,
        next_hop: u16,
        transport: ConnectionModule,
    ) -> Arc<RwLock<RoutingEntry>> {
        Arc::new(RwLock::new(RoutingEntry {
            target_index: 0,
            target,
            seq_num: SeqNum::from(0u16),
            metric: 10,
            next_hop,
            transport,
            last_update: 0,
            hop_count: 0,
            local_only: false,
        }))
    }

    /// A subject reachable in user space, with a neighbour to send through.
    fn reachable_user(state: &RouterV2State) -> [u8; 8] {
        let peer = fresh_peer();
        let neighbour_id = [9u8; 8];
        state.add_neighbour_transport(peer, neighbour_id, ConnectionModule::Lan);
        bind_own_dict(state, Space::Node, 100, neighbour_id);

        let subject = [3u8; 8];
        let user = install_user(state, subject, 0);
        let e = entry(TargetRef::User(user.clone()), 100, ConnectionModule::Lan);
        user.write().unwrap().routing_entry = Some(Arc::downgrade(&e));
        state.routing_table.write().unwrap().set(Space::User, 40, e);
        subject
    }

    #[test]
    fn a_fetch_for_a_reachable_subject_is_sent() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        state.request_profile(subject, false, 1_000);

        let out = rx.try_recv().expect("a request should have been sent");
        let decoded = ManagementMessage::decode(&out.bytes[..]).unwrap();
        assert_eq!(decoded.destination, subject.to_vec());
        assert!(!decoded.destination_is_node);
        assert_eq!(decoded.source, state.host_mk.to_id().to_vec());
        assert!(decoded.source_is_node);
        match decoded.body {
            Some(Body::ProfileRequest(r)) => assert_eq!(r.cached_version, 0),
            other => panic!("expected a ProfileRequest, got {other:?}"),
        }
    }

    /// The cached version is what the responder uses to decide whether it has
    /// anything fresher to send (§11.5).
    #[test]
    fn the_cached_profile_version_is_carried() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);
        state
            .users
            .read()
            .unwrap()
            .get(&subject)
            .unwrap()
            .write()
            .unwrap()
            .profile_version = 4;

        state.request_profile(subject, false, 1_000);

        let out = rx.try_recv().unwrap();
        match ManagementMessage::decode(&out.bytes[..]).unwrap().body {
            Some(Body::ProfileRequest(r)) => assert_eq!(r.cached_version, 4),
            other => panic!("expected a ProfileRequest, got {other:?}"),
        }
    }

    /// Several triggers can fire for one subject at once — the unverifiable
    /// branch of trust evaluation, a manifest with an unknown origin, and a
    /// fresher mapping version. Only one request should go out.
    #[test]
    fn repeated_triggers_produce_one_in_flight_request() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        state.request_profile(subject, false, 1_000);
        state.request_profile(subject, false, 1_050);
        state.request_profile(subject, false, 1_100);

        assert!(rx.try_recv().is_ok(), "the first request goes out");
        assert!(
            rx.try_recv().is_err(),
            "the second and third must be deduplicated"
        );
    }

    /// The two index spaces are distinct subjects (§3.5), so an in-flight
    /// user fetch must not suppress a node fetch for the same 8 bytes.
    #[test]
    fn the_index_space_is_part_of_the_dedup_key() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        // make the same id reachable as a node too
        let node = install_node(&state, subject, 1, false);
        bind_own_dict(&state, Space::Node, 41, subject);
        let e = entry(TargetRef::Node(node), 100, ConnectionModule::Lan);
        state.routing_table.write().unwrap().set(Space::Node, 41, e);

        state.request_profile(subject, false, 1_000);
        state.request_profile(subject, true, 1_000);

        assert!(rx.try_recv().is_ok(), "user-space fetch");
        assert!(
            rx.try_recv().is_ok(),
            "node-space fetch is a separate subject"
        );
    }

    /// No route means nothing is sent — and nothing is recorded, so a later
    /// trigger can retry once the subject becomes reachable. Recording an
    /// unsent request would strand the subject until the sweeper ran.
    #[test]
    fn an_unreachable_subject_is_not_recorded_as_in_flight() {
        let (state, mut rx) = fresh_state();

        state.request_profile([42u8; 8], false, 1_000);
        assert!(rx.try_recv().is_err(), "nothing to send it through");

        // now make it reachable; the retry must go out
        let subject = reachable_user(&state);
        state.request_profile(subject, false, 1_100);
        assert!(rx.try_recv().is_ok(), "a later trigger must not be blocked");
    }

    /// §11.2 is best-effort with no retransmission at this layer, so a lost
    /// response must not pin the subject forever.
    #[test]
    fn a_stale_in_flight_entry_is_swept_and_the_subject_retried() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        state.request_profile(subject, false, 1_000);
        assert!(rx.try_recv().is_ok());

        // still in flight: suppressed
        state.request_profile(subject, false, 2_000);
        assert!(rx.try_recv().is_err());

        // past the timeout, the sweep clears it
        let timeout_ms = state.options.manifest_request_timeout * 1_000;
        state.clear_management_msgs(1_000 + timeout_ms + 1);

        state.request_profile(subject, false, 1_000 + timeout_ms + 2);
        assert!(rx.try_recv().is_ok(), "a swept subject can be retried");
    }
}
