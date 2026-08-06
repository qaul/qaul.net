// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! BLE GATT messaging protocol integration tests (Rust-only).
//!
//! These tests follow the crate-level integration test approach used in
//! `libqaul/tests` on `main`, but target the Linux `ble_module` framing logic.
//! They avoid real BLE hardware by mocking GATT writes/reads around the same
//! production encode/reassembly helpers used by `ble_service`.

use async_std::task;
use ble_module::gatt_protocol::{
    encode_direct_message_chunks, GattMessageReassembler, GATT_CHUNK_BYTES,
};
use ble_module::rpc::{proto_sys, utils::BleResultSender};
use prost::Message as _;

#[derive(Default)]
struct MockGattDevice {
    writes: Vec<Vec<u8>>,
}

impl MockGattDevice {
    fn send_direct_message(&mut self, qaul_id: &[u8], payload: &[u8]) -> Vec<Vec<u8>> {
        let chunks = encode_direct_message_chunks(qaul_id, payload).expect("encode GATT chunks");
        self.writes.extend(chunks.clone());
        chunks
    }
}

#[test]
fn test_gatt_direct_message_roundtrip_with_mock_device() {
    let mut mock_device = MockGattDevice::default();
    let qaul_id = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let payload: Vec<u8> = (0..150).map(|v| v as u8).collect();

    let chunks = mock_device.send_direct_message(&qaul_id, &payload);

    assert!(
        chunks.len() > 1,
        "payload should be split into multiple GATT writes"
    );
    assert!(
        chunks.iter().all(|chunk| chunk.len() <= GATT_CHUNK_BYTES),
        "all chunks must respect 20-byte GATT write size"
    );

    let mut decoder = GattMessageReassembler::default();
    let mut decoded = None;
    for chunk in &chunks {
        if let Some(message) = decoder.push_chunk(chunk).expect("decode chunk") {
            decoded = Some(message);
        }
    }

    let decoded = decoded.expect("decoded direct message");
    assert_eq!(decoded.qaul_id, qaul_id);
    assert_eq!(decoded.message, payload);
}

#[test]
fn test_gatt_reassembly_handles_delimiter_split_across_writes() {
    let qaul_id = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let payload = b"split-delimiter-boundary".to_vec();
    let chunks = encode_direct_message_chunks(&qaul_id, &payload).expect("encode GATT chunks");
    assert!(!chunks.is_empty());
    assert!(
        chunks[0].len() >= 2,
        "first chunk should include delimiter bytes"
    );

    // Simulate a device writing one delimiter byte first, then the remainder.
    let first = &chunks[0];
    let mut fragmented = Vec::new();
    fragmented.push(vec![first[0]]);
    fragmented.push(first[1..].to_vec());
    fragmented.extend(chunks.iter().skip(1).cloned());

    let mut decoder = GattMessageReassembler::default();
    let mut decoded = None;
    for chunk in fragmented {
        if let Some(message) = decoder.push_chunk(&chunk).expect("decode chunk") {
            decoded = Some(message);
        }
    }

    let decoded = decoded.expect("decoded direct message");
    assert_eq!(decoded.qaul_id, qaul_id);
    assert_eq!(decoded.message, payload);
}

#[test]
fn test_gatt_roundtrip_preserves_ciphertext_like_payload_in_ble_sys_message() {
    let qaul_id = vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x01, 0x02];
    // High-entropy-ish deterministic bytes, including values that previously
    // interacted badly with ad-hoc trimming (0x00) and frame delimiter bytes (0x24).
    let ciphertext_like_payload: Vec<u8> = (0..96)
        .map(|i| ((i * 37) as u8) ^ [0x00, 0x24, 0xff, 0x80][i % 4])
        .collect();

    let chunks = encode_direct_message_chunks(&qaul_id, &ciphertext_like_payload)
        .expect("encode GATT chunks");

    let mut decoder = GattMessageReassembler::default();
    let direct = chunks
        .iter()
        .filter_map(|chunk| decoder.push_chunk(chunk).expect("decode chunk"))
        .next()
        .expect("decoded direct message");

    let (tx, rx) = async_std::channel::unbounded::<Vec<u8>>();
    let mut result_sender = BleResultSender::new(tx);
    result_sender.send_direct_received(direct.qaul_id.clone(), direct.message.clone());

    let encoded_sys_msg = task::block_on(async { rx.recv().await.expect("BLE sys message") });
    let decoded_sys_msg = proto_sys::ble::Message::decode(&encoded_sys_msg[..])
        .expect("decode BLE sys protobuf message");

    match decoded_sys_msg {
        proto_sys::ble::Message::DirectReceived(received) => {
            assert_eq!(received.from, qaul_id);
            assert_eq!(received.data, ciphertext_like_payload);
        }
        _ => panic!("unexpected BLE sys message variant"),
    }
}
