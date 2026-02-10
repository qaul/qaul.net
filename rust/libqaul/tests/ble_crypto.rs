// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # BLE Crypto Integration Tests
//!
//! Tests for the BLE transport encryption module.
//! These tests verify that the BleCryptoModule works correctly in isolation.

use libqaul::connections::ble::crypto::{BleCryptoModule, BleSessionState};

// ============================================================================
// BleCryptoModule isolation tests (no global state needed)
// ============================================================================

#[test]
fn test_ble_crypto_module_new() {
    let module = BleCryptoModule::new();
    assert_eq!(module.session_count(), 0);
}

#[test]
fn test_ble_crypto_module_default() {
    let module = BleCryptoModule::default();
    assert_eq!(module.session_count(), 0);
}

#[test]
fn test_no_session_established_for_unknown_id() {
    let module = BleCryptoModule::new();
    let small_id = vec![1, 2, 3, 4];
    assert!(!module.is_session_established(&small_id));
}

#[test]
fn test_get_session_id_returns_none_for_unknown() {
    let module = BleCryptoModule::new();
    let small_id = vec![1, 2, 3, 4];
    assert!(module.get_session_id(&small_id).is_none());
}

#[test]
fn test_on_node_unavailable_does_not_panic_for_unknown() {
    let mut module = BleCryptoModule::new();
    let small_id = vec![1, 2, 3, 4];
    // Should not panic even if no session exists
    module.on_node_unavailable(&small_id);
    assert_eq!(module.session_count(), 0);
}

#[test]
fn test_session_state_clone() {
    let state = BleSessionState::HandshakePending;
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

#[test]
fn test_session_state_debug() {
    let state = BleSessionState::Established;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Established"));
}

#[test]
fn test_session_state_equality() {
    assert_eq!(BleSessionState::HandshakePending, BleSessionState::HandshakePending);
    assert_eq!(BleSessionState::Established, BleSessionState::Established);
    assert_ne!(BleSessionState::HandshakePending, BleSessionState::Established);
}

#[test]
fn test_multiple_modules_are_independent() {
    // Create two separate modules to verify they don't share state
    let mut module1 = BleCryptoModule::new();
    let module2 = BleCryptoModule::new();

    let small_id = vec![1, 2, 3, 4];

    // Clean up in module1
    module1.on_node_unavailable(&small_id);

    // Module2 should be unaffected
    assert_eq!(module1.session_count(), 0);
    assert_eq!(module2.session_count(), 0);

    // Both should report no session
    assert!(!module1.is_session_established(&small_id));
    assert!(!module2.is_session_established(&small_id));
}

#[test]
fn test_encrypt_fails_without_session() {
    let mut module = BleCryptoModule::new();
    let small_id = vec![1, 2, 3, 4];
    let plaintext = b"Hello, world!".to_vec();

    let result = module.encrypt(&small_id, plaintext);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "No session found for encryption");
}

#[test]
fn test_decrypt_fails_without_session() {
    use libqaul::connections::ble::proto_net::EncryptedBleTransport;

    let mut module = BleCryptoModule::new();
    let small_id = vec![1, 2, 3, 4];
    let encrypted = EncryptedBleTransport {
        session_id: 12345,
        nonce: 0,
        ciphertext: vec![0, 1, 2, 3],
    };

    let result = module.decrypt(&small_id, encrypted);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "No session found for decryption");
}

#[test]
fn test_process_handshake_2_fails_without_session() {
    use libqaul::connections::ble::proto_net::NoiseHandshake;

    let mut module = BleCryptoModule::new();
    let small_id = vec![1, 2, 3, 4];
    let handshake = NoiseHandshake {
        session_id: 12345,
        message_number: 2,
        payload: vec![],
    };

    let result = module.process_handshake_2(&small_id, handshake);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "No session found for handshake 2");
}
