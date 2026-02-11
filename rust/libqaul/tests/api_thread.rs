// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # API Thread Tests
//!
//! Tests for `start_instance_in_thread()` which starts the full system
//! with the event loop running in a background thread.

use crossbeam_channel::TryRecvError;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;

/// Shared instance for all tests in this file.
/// Uses Lazy to ensure the instance is created only once per process.
/// The event loop runs in a background thread.
static INSTANCE: Lazy<(Arc<libqaul::Libqaul>, TempDir)> = Lazy::new(|| {
    let dir = TempDir::new().expect("Failed to create temp directory");
    let path = dir.path().to_str().unwrap().to_string();
    let instance = libqaul::api::start_instance_in_thread(path, None);

    // Wait for initialization to complete
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(30);
    while !instance.is_initialized() {
        if start.elapsed() > timeout {
            panic!("Timeout waiting for libqaul initialization");
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    (instance, dir)
});

#[test]
fn test_start_instance_in_thread() {
    let (instance, _) = &*INSTANCE;
    // If we get here, the instance was created successfully
    assert!(Arc::strong_count(instance) >= 1);
}

#[test]
fn test_is_initialized_becomes_true() {
    let (instance, _) = &*INSTANCE;

    // The INSTANCE Lazy already waits for initialization,
    // so this should be true
    assert!(instance.is_initialized(), "is_initialized() should be true after event loop starts");
}

#[test]
fn test_node_id_accessible() {
    let (instance, _) = &*INSTANCE;

    let node_id = instance.node_id();
    let id_string = node_id.to_string();

    assert!(!id_string.is_empty(), "Node ID should be accessible from main thread");
    assert!(id_string.len() > 10, "Node ID should have reasonable length");
}

#[test]
fn test_storage_accessible() {
    let (instance, dir) = &*INSTANCE;

    let storage = instance.storage();
    let path = storage.get_path();

    assert_eq!(path, dir.path().to_str().unwrap(), "Storage path should be accessible");
}

#[test]
fn test_rpc_channel_functional() {
    // Ensure the instance is started so RPC channels are initialized
    let (_instance, _) = &*INSTANCE;

    // send_rpc should not panic (even with empty message)
    // We send an empty message which won't be valid, but the channel should accept it
    libqaul::api::send_rpc(vec![]);

    // receive_rpc should return Empty error (no response pending)
    let result = libqaul::api::receive_rpc();
    assert!(
        matches!(result, Err(TryRecvError::Empty)),
        "receive_rpc() should return Empty when no messages are queued"
    );
}

#[test]
fn test_node_keypair_accessible() {
    let (instance, _) = &*INSTANCE;

    let keypair = instance.node().keys();
    let public_key = keypair.public();

    // Verify we can derive PeerId from the public key
    let derived_id = libp2p::PeerId::from(public_key);
    assert_eq!(derived_id, instance.node_id(), "Derived PeerId should match node_id()");
}

#[test]
fn test_router_accessible() {
    let (instance, _) = &*INSTANCE;

    let router = instance.router();

    // The router module should have valid routing options
    assert!(router.sending_table_period > 0, "Routing table period should be positive");

    let config = router.get_configuration();
    assert!(config.ping_neighbour_period > 0, "Ping neighbour period should be positive");
}

#[test]
fn test_services_module_accessible() {
    let (instance, _) = &*INSTANCE;

    // Should be able to acquire lock on services module
    let services = instance.services();
    let _guard = services.read().unwrap();
    // If we get here, the lock was acquired successfully
}

#[test]
fn test_connections_module_accessible() {
    let (instance, _) = &*INSTANCE;

    // Should be able to acquire lock on connections module
    let connections = instance.connections();
    let _guard = connections.read().unwrap();
    // If we get here, the lock was acquired successfully
}
