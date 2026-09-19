// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # DTN v2 Custody Integration Tests
//!
//! Exercises the redesigned custody wire format end-to-end at the boundaries
//! an external crate can reach:
//! - the signed, immutable `DtnV2Container` / `DtnRoute` through the full
//!   envelope chain and the `Dtn` oneof,
//! - the signed `DtnResponseV2`,
//! - sled storage of the custody entry (with its admission `tier`),
//! - duplicate detection and per-sender quota bookkeeping.
//!
//! The crypto/admission logic itself (grant + proof-of-work verification,
//! stateless traversal, signed-response handling) lives in the unit tests
//! inside `services/dtn/mod.rs`, which can reach the private helpers.

use libp2p::identity::{Keypair, PublicKey};
use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Protobuf types from qaul-proto (public crate)
use qaul_proto::qaul_net_messaging as proto;

/// Mirror of `DtnRoutedV2Entry` from libqaul (not publicly exported), so we can
/// exercise the sled storage layer independently.
#[derive(Serialize, Deserialize, Clone)]
struct DtnRoutedV2Entry {
    container_v2_bytes: Vec<u8>,
    sender_public_key: Vec<u8>,
    size: u32,
    accepted_at: u64,
    receiver_id: Vec<u8>,
    tier: u8,
}

/// Mirror of `SenderQuotaEntry`.
#[derive(Default, Serialize, Deserialize, Clone)]
struct SenderQuotaEntry {
    used_bytes: u64,
    message_count: u32,
}

const TIER_GRANT: u8 = 2;

fn random_peer() -> PeerId {
    PeerId::from(Keypair::generate_ed25519().public())
}

/// Build a signed inner Container for `receiver`.
/// Returns (sender_keys, container_bytes, original_signature).
fn signed_inner(receiver: &PeerId) -> (Keypair, Vec<u8>, Vec<u8>) {
    let keys = Keypair::generate_ed25519();
    let sender = PeerId::from(keys.public());
    let envelope = proto::Envelope {
        sender_id: sender.to_bytes(),
        receiver_id: receiver.to_bytes(),
        payload: vec![1, 2, 3],
    };
    let mut buf = Vec::with_capacity(envelope.encoded_len());
    envelope.encode(&mut buf).unwrap();
    let signature = keys.sign(&buf).unwrap();
    let container = proto::Container {
        signature: signature.clone(),
        envelope: Some(envelope),
    };
    (keys, container.encode_to_vec(), signature)
}

/// Build a signed `DtnV2Container` (one RouteHop per custodian).
fn build_container_v2(
    receiver: &PeerId,
    custodians: Vec<PeerId>,
    expires_at: Option<u64>,
) -> (proto::DtnV2Container, Keypair) {
    let (keys, container_bytes, signature) = signed_inner(receiver);
    let route_hop = custodians
        .iter()
        .map(|c| proto::RouteHop {
            route_entry: vec![proto::RouteEntry { id: c.to_bytes() }],
        })
        .collect();
    let route = proto::DtnRoute {
        original_signature: signature,
        route_hop,
        sender_public_key: keys.public().encode_protobuf(),
        expires_at,
    };
    let dtn_route = route.encode_to_vec();
    let dtn_route_sig = keys.sign(&dtn_route).unwrap();
    (
        proto::DtnV2Container {
            dtn_route,
            dtn_route_sig,
            envelope: container_bytes,
            custody_grant: None,
            pow: None,
        },
        keys,
    )
}

// ── Wire round-trips ──

#[test]
fn v2_container_survives_full_envelope_chain() {
    let receiver = random_peer();
    let sender = random_peer();
    let custodian = random_peer();
    let (container_v2, _keys) = build_container_v2(&receiver, vec![custodian], None);

    let payload = proto::EnvelopPayload {
        payload: Some(proto::envelop_payload::Payload::DtnV2(container_v2)),
    };
    let envelope = proto::Envelope {
        sender_id: sender.to_bytes(),
        receiver_id: receiver.to_bytes(),
        payload: payload.encode_to_vec(),
    };
    let container = proto::Container {
        signature: vec![0xDE, 0xAD],
        envelope: Some(envelope),
    };
    let bytes = container.encode_to_vec();

    let decoded_container = proto::Container::decode(&bytes[..]).unwrap();
    let decoded_envelope = decoded_container.envelope.unwrap();
    let decoded_payload = proto::EnvelopPayload::decode(&decoded_envelope.payload[..]).unwrap();
    match decoded_payload.payload {
        Some(proto::envelop_payload::Payload::DtnV2(c)) => {
            let route = proto::DtnRoute::decode(&c.dtn_route[..]).unwrap();
            assert_eq!(route.route_hop.len(), 1);
            assert_eq!(route.route_hop[0].route_entry[0].id, custodian.to_bytes());
            // inner receiver survives
            let inner = proto::Container::decode(&c.envelope[..]).unwrap();
            let inner_recv = PeerId::from_bytes(&inner.envelope.unwrap().receiver_id).unwrap();
            assert_eq!(inner_recv, receiver);
        }
        _ => panic!("Expected DtnV2 payload"),
    }
}

#[test]
fn v2_container_in_dtn_oneof() {
    let receiver = random_peer();
    let (container_v2, _keys) = build_container_v2(&receiver, vec![random_peer()], None);
    let dtn = proto::Dtn {
        message: Some(proto::dtn::Message::ContainerV2(container_v2)),
    };
    let decoded = proto::Dtn::decode(&dtn.encode_to_vec()[..]).unwrap();
    match decoded.message {
        Some(proto::dtn::Message::ContainerV2(c)) => {
            assert!(!c.dtn_route_sig.is_empty());
        }
        _ => panic!("Expected ContainerV2 variant"),
    }
}

// The route is signed by the sender and must verify against the embedded key —
// this is what makes the route immutable in transit.
#[test]
fn v2_route_signature_verifies() {
    let receiver = random_peer();
    let (container_v2, keys) = build_container_v2(&receiver, vec![random_peer()], None);
    let route = proto::DtnRoute::decode(&container_v2.dtn_route[..]).unwrap();
    let key = PublicKey::try_decode_protobuf(&route.sender_public_key).unwrap();
    assert_eq!(key, keys.public());
    assert!(key.verify(&container_v2.dtn_route, &container_v2.dtn_route_sig));

    // tampering the route breaks the signature
    let mut tampered = container_v2.dtn_route.clone();
    tampered[0] ^= 0xFF;
    assert!(!key.verify(&tampered, &container_v2.dtn_route_sig));
}

// A signed DtnResponseV2 round-trips through the Dtn oneof and verifies.
#[test]
fn v2_signed_response_round_trip() {
    let keys = Keypair::generate_ed25519();
    let responder = PeerId::from(keys.public());
    let mut resp = proto::DtnResponseV2 {
        kind: proto::dtn_response_v2::Kind::Delivery as i32,
        response_type: proto::dtn_response_v2::ResponseType::Accepted as i32,
        reason: proto::dtn_response_v2::Reason::None as i32,
        original_signature: vec![0xAB],
        responder_public_key: keys.public().encode_protobuf(),
        signature: Vec::new(),
    };
    resp.signature = keys.sign(&resp.encode_to_vec()).unwrap();

    let dtn = proto::Dtn {
        message: Some(proto::dtn::Message::ResponseV2(resp.clone())),
    };
    let decoded = proto::Dtn::decode(&dtn.encode_to_vec()[..]).unwrap();
    match decoded.message {
        Some(proto::dtn::Message::ResponseV2(r)) => {
            let key = PublicKey::try_decode_protobuf(&r.responder_public_key).unwrap();
            let mut unsigned = r.clone();
            unsigned.signature = Vec::new();
            assert!(key.verify(&unsigned.encode_to_vec(), &r.signature));
            assert_eq!(PeerId::from_public_key(&key), responder);
        }
        _ => panic!("Expected ResponseV2 variant"),
    }
}

#[test]
fn v2_route_expiry_is_optional() {
    let receiver = random_peer();
    let (with_expiry, _) = build_container_v2(&receiver, vec![random_peer()], Some(1234));
    let (no_expiry, _) = build_container_v2(&receiver, vec![random_peer()], None);
    let r1 = proto::DtnRoute::decode(&with_expiry.dtn_route[..]).unwrap();
    let r2 = proto::DtnRoute::decode(&no_expiry.dtn_route[..]).unwrap();
    assert_eq!(r1.expires_at, Some(1234));
    assert_eq!(r2.expires_at, None);
}

// ── Sled storage ──

#[test]
fn v2_sled_store_and_retrieve() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let tree = db.open_tree("v2-messages").unwrap();

    let receiver = random_peer();
    let (container_v2, keys) = build_container_v2(&receiver, vec![random_peer()], None);
    let route = proto::DtnRoute::decode(&container_v2.dtn_route[..]).unwrap();
    let sig = route.original_signature.clone();
    let bytes = container_v2.encode_to_vec();

    let entry = DtnRoutedV2Entry {
        container_v2_bytes: bytes.clone(),
        sender_public_key: keys.public().encode_protobuf(),
        size: container_v2.envelope.len() as u32,
        accepted_at: 12345,
        receiver_id: receiver.to_bytes(),
        tier: TIER_GRANT,
    };
    tree.insert(&sig, bincode::serialize(&entry).unwrap()).unwrap();
    tree.flush().unwrap();

    let stored = tree.get(&sig).unwrap().unwrap();
    let decoded: DtnRoutedV2Entry = bincode::deserialize(&stored).unwrap();
    assert_eq!(decoded.accepted_at, 12345);
    assert_eq!(decoded.tier, TIER_GRANT);
    // the stored container still decodes
    assert!(proto::DtnV2Container::decode(&decoded.container_v2_bytes[..]).is_ok());
}

#[test]
fn v2_sled_duplicate_detection() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let tree = db.open_tree("v2-dedup").unwrap();
    let sig = vec![0xDE, 0xAD, 0xBE, 0xEF];
    tree.insert(&sig, b"entry").unwrap();
    assert!(tree.contains_key(&sig).unwrap(), "duplicate must be detected");
}

#[test]
fn v2_sled_quota_tracking_lifecycle() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let quotas = db.open_tree("v2-quotas").unwrap();
    let sender_key = vec![0xAA, 0xBB];

    let quota = SenderQuotaEntry {
        used_bytes: 100,
        message_count: 1,
    };
    quotas
        .insert(&sender_key, bincode::serialize(&quota).unwrap())
        .unwrap();

    // add a second message
    let stored = quotas.get(&sender_key).unwrap().unwrap();
    let mut q: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
    q.used_bytes += 200;
    q.message_count += 1;
    quotas
        .insert(&sender_key, bincode::serialize(&q).unwrap())
        .unwrap();
    let q: SenderQuotaEntry =
        bincode::deserialize(&quotas.get(&sender_key).unwrap().unwrap()).unwrap();
    assert_eq!(q.used_bytes, 300);
    assert_eq!(q.message_count, 2);

    // free the first message
    let mut q = q;
    q.used_bytes -= 100;
    q.message_count -= 1;
    quotas
        .insert(&sender_key, bincode::serialize(&q).unwrap())
        .unwrap();
    let q: SenderQuotaEntry =
        bincode::deserialize(&quotas.get(&sender_key).unwrap().unwrap()).unwrap();
    assert_eq!(q.used_bytes, 200);
    assert_eq!(q.message_count, 1);
}
