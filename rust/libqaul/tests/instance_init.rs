// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Instance Initialization Tests
//!
//! Tests for `Libqaul::new()` without running the event loop.
//! These tests verify that instance creation works correctly
//! and all modules are properly initialized.

use once_cell::sync::Lazy;
use std::sync::Arc;
use tempfile::TempDir;

/// Shared instance for all tests in this file.
/// Uses Lazy to ensure the instance is created only once per process.
static INSTANCE: Lazy<(Arc<libqaul::Libqaul>, TempDir)> = Lazy::new(|| {
    let dir = TempDir::new().expect("Failed to create temp directory");
    let path = dir.path().to_str().unwrap().to_string();
    let instance = async_std::task::block_on(libqaul::start_instance(path, None));
    (instance, dir)
});

#[test]
fn test_instance_creates_successfully() {
    let (instance, _) = &*INSTANCE;
    // If we get here, the instance was created successfully
    assert!(Arc::strong_count(instance) >= 1);
}

#[test]
fn test_node_id_is_valid() {
    let (instance, _) = &*INSTANCE;
    let node_id = instance.node_id();

    // PeerId should not be the default/empty value
    // A valid PeerId has a non-empty string representation
    let id_string = node_id.to_string();
    assert!(!id_string.is_empty(), "Node ID should not be empty");
    assert!(
        id_string.len() > 10,
        "Node ID should have reasonable length"
    );
}

#[test]
fn test_node_id_derived_from_keypair() {
    let (instance, _) = &*INSTANCE;

    let keypair = instance.node().keys();
    let derived_id = libp2p::PeerId::from(keypair.public());
    let node_id = instance.node_id();

    assert_eq!(
        derived_id, node_id,
        "PeerId derived from keypair should match node_id()"
    );
}

#[test]
fn test_storage_path_matches() {
    let (instance, dir) = &*INSTANCE;

    let expected_path = dir.path().to_str().unwrap();
    let actual_path = instance.storage_path();

    assert_eq!(
        actual_path, expected_path,
        "Storage path should match the provided path"
    );
}

#[test]
fn test_config_file_created() {
    let (_, dir) = &*INSTANCE;

    let config_path = dir.path().join("config.yaml");
    assert!(
        config_path.exists(),
        "config.yaml should be created in the storage directory"
    );
}

#[test]
fn test_database_directory_created() {
    let (_, dir) = &*INSTANCE;

    // The sled database creates a directory named "node.db" in the storage path
    let db_path = dir.path().join("node.db");
    assert!(
        db_path.exists(),
        "Database directory (node.db) should be created"
    );
    assert!(db_path.is_dir(), "Database path should be a directory");
}

#[test]
fn test_is_initialized_false_without_run() {
    let (instance, _) = &*INSTANCE;

    // Without calling run(), is_initialized should be false
    // because the INITIALIZED flag is only set after the event loop starts
    assert!(
        !instance.is_initialized(),
        "is_initialized() should be false before run() is called"
    );
}

#[test]
fn test_node_has_ed25519_keys() {
    let (instance, _) = &*INSTANCE;

    let keypair = instance.node().keys();
    // Try to convert to ed25519, which will succeed if it's an ed25519 keypair
    let ed25519_result = keypair.clone().try_into_ed25519();
    assert!(ed25519_result.is_ok(), "Node keypair should be ED25519");
}

#[test]
fn test_node_has_topic() {
    let (instance, _) = &*INSTANCE;

    let topic = instance.node().topic();
    // The topic should be "pages" as defined in NodeIdentity::generate()
    let topic_id = topic.id();
    assert!(!topic_id.is_empty(), "Topic should not be empty");
}

#[test]
fn test_config_node_marked_initialized() {
    let (_, dir) = &*INSTANCE;

    // The config file on disk should have the node marked as initialized
    // Note: We read from the file directly because the in-memory config
    // in StorageModule was captured before Node::init() saved the updated config.
    let config_path = dir.path().to_str().unwrap();
    let config = libqaul::storage::configuration::Configuration::load_or_create(config_path);

    // After instance creation, the node should be marked as initialized in the saved file
    assert_eq!(
        config.node.initialized, 1,
        "Node should be marked as initialized in config file"
    );
    assert!(
        !config.node.id.is_empty(),
        "Node ID in config file should not be empty"
    );
    assert!(
        !config.node.keys.is_empty(),
        "Node keys in config file should not be empty"
    );
}
