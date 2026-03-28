// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # DTN Routed V2 Integration Tests
//!
//! Tests the directed custody routing message types end-to-end:
//! - Protobuf encode/decode through the full envelope chain
//! - V2 storage entry serialization via bincode (as used in sled)
//! - Expiry and handoff validation logic
//! - Multi-route message construction and round-trip
//! - Duplicate detection via sled (temporary DB)
//! - Quota tracking via sled (temporary DB)

use libp2p::identity::Keypair;
use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Protobuf types from qaul-proto (public crate)
use qaul_proto::qaul_net_messaging as proto;

/// Mirror of DtnRoutedV2Entry from libqaul (not publicly exported).
/// We redefine it here to test the sled storage layer independently.
#[derive(Serialize, Deserialize, Clone)]
struct DtnRoutedV2Entry {
    routed_v2_bytes: Vec<u8>,
    sender_public_key: Vec<u8>,
    size: u32,
    accepted_at: u64,
    receiver_id: Vec<u8>,
}

/// Mirror of SenderQuotaEntry.
#[derive(Default, Serialize, Deserialize, Clone)]
struct SenderQuotaEntry {
    used_bytes: u64,
    message_count: u32,
}

/// Per-sender quota limit (same as in libqaul).
const V2_PER_SENDER_QUOTA: u64 = 10 * 1024 * 1024;

fn random_peer() -> PeerId {
    let keys = Keypair::generate_ed25519();
    PeerId::from(keys.public())
}

/// Build a DtnRoutedV2 with a real inner Container.
fn build_test_v2(
    receiver: &PeerId,
    custodian_routes: Vec<Vec<PeerId>>,
    expires_at: u64,
    remaining_handoffs: u32,
) -> proto::DtnRoutedV2 {
    let sender = random_peer();
    let envelope = proto::Envelope {
        sender_id: sender.to_bytes(),
        receiver_id: receiver.to_bytes(),
        payload: vec![],
    };
    let container = proto::Container {
        signature: vec![0x01, 0x02],
        envelope: Some(envelope),
    };

    let routes: Vec<proto::CustodyRoute> = custodian_routes
        .into_iter()
        .map(|custodians| proto::CustodyRoute {
            custody_users: custodians.iter().map(|c| c.to_bytes()).collect(),
            next_index: 0,
        })
        .collect();

    proto::DtnRoutedV2 {
        container: container.encode_to_vec(),
        routes,
        original_signature: (0..16).map(|_| rand::random::<u8>()).collect(),
        sender_public_key: sender.to_bytes(),
        expires_at,
        remaining_handoffs,
    }
}

// ── Full envelope chain tests ──

#[test]
fn v2_message_survives_full_envelope_chain() {
    let receiver = random_peer();
    let sender = random_peer();
    let custodian = random_peer();

    let v2 = build_test_v2(&receiver, vec![vec![custodian]], 0, 5);

    // Wrap in EnvelopPayload
    let payload = proto::EnvelopPayload {
        payload: Some(proto::envelop_payload::Payload::DtnRoutedV2(v2.clone())),
    };
    let payload_bytes = payload.encode_to_vec();

    // Wrap in Envelope
    let envelope = proto::Envelope {
        sender_id: sender.to_bytes(),
        receiver_id: receiver.to_bytes(),
        payload: payload_bytes,
    };

    // Wrap in Container
    let container = proto::Container {
        signature: vec![0xDE, 0xAD],
        envelope: Some(envelope),
    };
    let container_bytes = container.encode_to_vec();

    // Now decode the whole chain
    let decoded_container = proto::Container::decode(&container_bytes[..]).unwrap();
    let decoded_envelope = decoded_container.envelope.unwrap();
    assert_eq!(
        PeerId::from_bytes(&decoded_envelope.receiver_id).unwrap(),
        receiver
    );

    let decoded_payload = proto::EnvelopPayload::decode(&decoded_envelope.payload[..]).unwrap();
    match decoded_payload.payload {
        Some(proto::envelop_payload::Payload::DtnRoutedV2(decoded_v2)) => {
            assert_eq!(decoded_v2.remaining_handoffs, 5);
            assert_eq!(decoded_v2.routes.len(), 1);
            assert_eq!(decoded_v2.routes[0].custody_users[0], custodian.to_bytes());

            // Decode the inner container to get the ultimate receiver
            let inner = proto::Container::decode(&decoded_v2.container[..]).unwrap();
            let inner_recv =
                PeerId::from_bytes(&inner.envelope.unwrap().receiver_id).unwrap();
            assert_eq!(inner_recv, receiver);
        }
        _ => panic!("Expected DtnRoutedV2 payload"),
    }
}

#[test]
fn v2_message_in_dtn_oneof() {
    let receiver = random_peer();
    let v2 = build_test_v2(&receiver, vec![vec![random_peer()]], 0, 3);

    // Wrap in Dtn message (the other transport path)
    let dtn = proto::Dtn {
        message: Some(proto::dtn::Message::RoutedV2(v2.clone())),
    };
    let encoded = dtn.encode_to_vec();
    let decoded = proto::Dtn::decode(&encoded[..]).unwrap();

    match decoded.message {
        Some(proto::dtn::Message::RoutedV2(decoded_v2)) => {
            assert_eq!(decoded_v2.remaining_handoffs, 3);
        }
        _ => panic!("Expected RoutedV2 variant"),
    }
}

// ── Sled storage tests ──

#[test]
fn v2_sled_store_and_retrieve() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let tree = db.open_tree("v2-messages").unwrap();

    let receiver = random_peer();
    let v2 = build_test_v2(&receiver, vec![vec![random_peer()]], 0, 5);
    let sig = v2.original_signature.clone();
    let v2_bytes = v2.encode_to_vec();

    let entry = DtnRoutedV2Entry {
        routed_v2_bytes: v2_bytes.clone(),
        sender_public_key: v2.sender_public_key.clone(),
        size: v2_bytes.len() as u32,
        accepted_at: 12345,
        receiver_id: receiver.to_bytes(),
    };

    // Store
    tree.insert(&sig, bincode::serialize(&entry).unwrap())
        .unwrap();
    tree.flush().unwrap();

    // Retrieve and decode
    let stored = tree.get(&sig).unwrap().unwrap();
    let decoded: DtnRoutedV2Entry = bincode::deserialize(&stored).unwrap();
    assert_eq!(decoded.size, v2_bytes.len() as u32);
    assert_eq!(decoded.accepted_at, 12345);

    let inner_v2 = proto::DtnRoutedV2::decode(&decoded.routed_v2_bytes[..]).unwrap();
    assert_eq!(inner_v2.remaining_handoffs, 5);
}

#[test]
fn v2_sled_duplicate_detection() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let tree = db.open_tree("v2-dedup").unwrap();

    let sig = vec![0xDE, 0xAD, 0xBE, 0xEF];

    // First message accepted
    tree.insert(&sig, b"message-data").unwrap();
    assert!(tree.contains_key(&sig).unwrap());

    // Second message with same sig should be detected
    let is_dup = tree.contains_key(&sig).unwrap();
    assert!(is_dup, "duplicate should be detected");
}

#[test]
fn v2_sled_quota_tracking_lifecycle() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let quotas = db.open_tree("v2-quotas").unwrap();
    let messages = db.open_tree("v2-msgs").unwrap();

    let sender_key = vec![0xAA, 0xBB];
    let sig1 = vec![0x01];
    let sig2 = vec![0x02];

    // Accept message 1 (size: 100)
    let entry1 = DtnRoutedV2Entry {
        routed_v2_bytes: vec![0; 100],
        sender_public_key: sender_key.clone(),
        size: 100,
        accepted_at: 1000,
        receiver_id: vec![],
    };
    messages
        .insert(&sig1, bincode::serialize(&entry1).unwrap())
        .unwrap();
    let quota = SenderQuotaEntry {
        used_bytes: 100,
        message_count: 1,
    };
    quotas
        .insert(&sender_key, bincode::serialize(&quota).unwrap())
        .unwrap();

    // Accept message 2 (size: 200)
    let entry2 = DtnRoutedV2Entry {
        routed_v2_bytes: vec![0; 200],
        sender_public_key: sender_key.clone(),
        size: 200,
        accepted_at: 2000,
        receiver_id: vec![],
    };
    messages
        .insert(&sig2, bincode::serialize(&entry2).unwrap())
        .unwrap();
    let stored = quotas.get(&sender_key).unwrap().unwrap();
    let mut quota: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
    quota.used_bytes += 200;
    quota.message_count += 1;
    quotas
        .insert(&sender_key, bincode::serialize(&quota).unwrap())
        .unwrap();

    // Verify quota
    let stored = quotas.get(&sender_key).unwrap().unwrap();
    let quota: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
    assert_eq!(quota.used_bytes, 300);
    assert_eq!(quota.message_count, 2);

    // Forward message 1 (simulate acceptance) — remove and update quota
    let removed = messages.remove(&sig1).unwrap().unwrap();
    let removed_entry: DtnRoutedV2Entry = bincode::deserialize(&removed).unwrap();
    let stored = quotas.get(&sender_key).unwrap().unwrap();
    let mut quota: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
    quota.used_bytes -= removed_entry.size as u64;
    quota.message_count -= 1;
    quotas
        .insert(&sender_key, bincode::serialize(&quota).unwrap())
        .unwrap();

    // Verify updated quota
    let stored = quotas.get(&sender_key).unwrap().unwrap();
    let quota: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
    assert_eq!(quota.used_bytes, 200);
    assert_eq!(quota.message_count, 1);
}

#[test]
fn v2_per_sender_quota_enforcement() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let quotas = db.open_tree("v2-quota-enforce").unwrap();

    let sender_key = vec![0xCC];

    // Sender near quota limit
    let quota = SenderQuotaEntry {
        used_bytes: V2_PER_SENDER_QUOTA - 100,
        message_count: 50,
    };
    quotas
        .insert(&sender_key, bincode::serialize(&quota).unwrap())
        .unwrap();

    // Small message should fit
    let stored = quotas.get(&sender_key).unwrap().unwrap();
    let q: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
    assert!(
        q.used_bytes + 50 <= V2_PER_SENDER_QUOTA,
        "50-byte message should fit"
    );

    // Large message should not fit
    assert!(
        q.used_bytes + 200 > V2_PER_SENDER_QUOTA,
        "200-byte message should be rejected"
    );
}

// ── Expiry and handoff validation tests ──

#[test]
fn v2_expired_messages_detected_in_storage_scan() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let tree = db.open_tree("v2-expiry").unwrap();

    let receiver = random_peer();

    // Expired message
    let expired_v2 = build_test_v2(&receiver, vec![vec![random_peer()]], 1, 5);
    let expired_entry = DtnRoutedV2Entry {
        routed_v2_bytes: expired_v2.encode_to_vec(),
        sender_public_key: expired_v2.sender_public_key.clone(),
        size: 10,
        accepted_at: 0,
        receiver_id: receiver.to_bytes(),
    };
    tree.insert(
        &expired_v2.original_signature,
        bincode::serialize(&expired_entry).unwrap(),
    )
    .unwrap();

    // Non-expired message
    let valid_v2 = build_test_v2(&receiver, vec![vec![random_peer()]], u64::MAX, 5);
    let valid_entry = DtnRoutedV2Entry {
        routed_v2_bytes: valid_v2.encode_to_vec(),
        sender_public_key: valid_v2.sender_public_key.clone(),
        size: 10,
        accepted_at: 0,
        receiver_id: receiver.to_bytes(),
    };
    tree.insert(
        &valid_v2.original_signature,
        bincode::serialize(&valid_entry).unwrap(),
    )
    .unwrap();

    // Scan and classify
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let mut expired_sigs = Vec::new();
    let mut valid_count = 0;

    for entry in tree.iter() {
        let (sig, bytes) = entry.unwrap();
        let stored: DtnRoutedV2Entry = bincode::deserialize(&bytes).unwrap();
        let v2 = proto::DtnRoutedV2::decode(&stored.routed_v2_bytes[..]).unwrap();
        if v2.expires_at > 0 && now > v2.expires_at {
            expired_sigs.push(sig.to_vec());
        } else {
            valid_count += 1;
        }
    }

    assert_eq!(expired_sigs.len(), 1);
    assert_eq!(valid_count, 1);

    // Remove expired
    for sig in &expired_sigs {
        tree.remove(sig).unwrap();
    }
    assert_eq!(tree.len(), 1);
}

#[test]
fn v2_multi_route_construction_and_advancement() {
    let c1 = random_peer();
    let c2 = random_peer();
    let c3 = random_peer();
    let _receiver = random_peer();

    let mut v2 = proto::DtnRoutedV2 {
        container: vec![],
        routes: vec![
            proto::CustodyRoute {
                custody_users: vec![c1.to_bytes(), c2.to_bytes()],
                next_index: 0,
            },
            proto::CustodyRoute {
                custody_users: vec![c3.to_bytes()],
                next_index: 0,
            },
        ],
        original_signature: vec![0xAA],
        sender_public_key: vec![],
        expires_at: 0,
        remaining_handoffs: 6,
    };

    // Simulate forwarding to c2 (index 1 in route 0)
    let target = c2;
    v2.remaining_handoffs = v2.remaining_handoffs.saturating_sub(1);
    for route in &mut v2.routes {
        for (i, user_bytes) in route.custody_users.iter().enumerate() {
            if let Ok(uid) = PeerId::from_bytes(user_bytes) {
                if uid == target && i as u32 >= route.next_index {
                    route.next_index = (i as u32) + 1;
                    break;
                }
            }
        }
    }

    assert_eq!(v2.remaining_handoffs, 5);
    assert_eq!(v2.routes[0].next_index, 2); // past c2
    assert_eq!(v2.routes[1].next_index, 0); // route 2 unchanged

    // Encode and decode — verify state persists
    let encoded = v2.encode_to_vec();
    let decoded = proto::DtnRoutedV2::decode(&encoded[..]).unwrap();
    assert_eq!(decoded.routes[0].next_index, 2);
    assert_eq!(decoded.remaining_handoffs, 5);
}
