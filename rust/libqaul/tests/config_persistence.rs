// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Configuration Persistence Tests
//!
//! Tests for Configuration save/load functionality.
//! These tests use the Configuration struct directly without
//! creating a full Libqaul instance.
//!
//! Note: Since `Configuration::default()` depends on global state (DEFCONFIGS),
//! we construct Configuration instances manually in these tests.

use libqaul::storage::configuration::{
    Configuration, DebugOption, Internet, InternetPeer, Lan, Node, RoutingOptions, StorageOptions,
    UserAccount,
};
use tempfile::TempDir;

/// Create a minimal Configuration for testing without relying on Default trait
fn create_test_config() -> Configuration {
    Configuration {
        node: Node {
            initialized: 0,
            id: String::new(),
            keys: String::new(),
        },
        lan: Lan {
            active: true,
            listen: vec![
                String::from("/ip4/0.0.0.0/udp/0/quic-v1"),
                String::from("/ip4/0.0.0.0/tcp/0"),
            ],
        },
        internet: Internet {
            active: true,
            peers: vec![InternetPeer {
                address: String::from("/ip4/144.91.74.192/udp/9229/quic-v1"),
                name: String::from("qaul Community Node [IPv4]"),
                enabled: false,
            }],
            do_listen: false,
            listen: vec![
                String::from("/ip4/0.0.0.0/udp/0/quic-v1"),
                String::from("/ip4/0.0.0.0/tcp/0"),
            ],
        },
        user_accounts: Vec::new(),
        debug: DebugOption { log: false },
        routing: RoutingOptions {
            sending_table_period: 10,
            ping_neighbour_period: 5,
            hop_count_penalty: 10,
            maintain_period_limit: 300,
        },
    }
}

#[test]
fn test_default_config_values() {
    let config = create_test_config();

    // Check node defaults
    assert_eq!(
        config.node.initialized, 0,
        "Node should not be initialized by default"
    );
    assert!(
        config.node.id.is_empty(),
        "Node ID should be empty by default"
    );
    assert!(
        config.node.keys.is_empty(),
        "Node keys should be empty by default"
    );

    // Check LAN defaults
    assert!(config.lan.active, "LAN should be active by default");
    assert!(
        !config.lan.listen.is_empty(),
        "LAN listen addresses should not be empty"
    );

    // Check Internet defaults
    assert!(
        config.internet.active,
        "Internet should be active by default"
    );
    assert!(
        !config.internet.peers.is_empty(),
        "Internet peers should have default entries"
    );

    // Check routing defaults
    assert_eq!(
        config.routing.sending_table_period, 10,
        "Default sending_table_period should be 10"
    );
    assert_eq!(
        config.routing.ping_neighbour_period, 5,
        "Default ping_neighbour_period should be 5"
    );
    assert_eq!(
        config.routing.hop_count_penalty, 10,
        "Default hop_count_penalty should be 10"
    );
    assert_eq!(
        config.routing.maintain_period_limit, 300,
        "Default maintain_period_limit should be 300"
    );

    // Check debug defaults
    assert!(!config.debug.log, "Debug log should be disabled by default");
}

#[test]
fn test_config_save_and_reload() {
    let dir = TempDir::new().expect("Failed to create temp directory");
    let path = dir.path().to_str().unwrap();

    // Create a config with some custom values
    let mut config = create_test_config();
    config.node.initialized = 1;
    config.node.id = "test-node-id-12345".to_string();
    config.node.keys = "test-keys-base64".to_string();
    config.lan.active = false;
    config.debug.log = true;

    // Save the config
    config.save_to_path(path);

    // Verify the file was created
    let config_file = dir.path().join("config.yaml");
    assert!(config_file.exists(), "config.yaml should be created");

    // Reload the config
    let loaded_config = Configuration::load_or_create(path);

    // Verify the values match
    assert_eq!(
        loaded_config.node.initialized, 1,
        "Loaded node.initialized should match"
    );
    assert_eq!(
        loaded_config.node.id, "test-node-id-12345",
        "Loaded node.id should match"
    );
    assert_eq!(
        loaded_config.node.keys, "test-keys-base64",
        "Loaded node.keys should match"
    );
    assert!(!loaded_config.lan.active, "Loaded lan.active should match");
    assert!(loaded_config.debug.log, "Loaded debug.log should match");
}

#[test]
fn test_config_custom_routing_values() {
    let dir = TempDir::new().expect("Failed to create temp directory");
    let path = dir.path().to_str().unwrap();

    // Create a config with custom routing values
    let mut config = create_test_config();
    config.routing.sending_table_period = 20;
    config.routing.ping_neighbour_period = 10;
    config.routing.hop_count_penalty = 15;
    config.routing.maintain_period_limit = 600;

    // Save the config
    config.save_to_path(path);

    // Reload the config
    let loaded_config = Configuration::load_or_create(path);

    // Verify the routing values match
    assert_eq!(
        loaded_config.routing.sending_table_period, 20,
        "Loaded sending_table_period should match"
    );
    assert_eq!(
        loaded_config.routing.ping_neighbour_period, 10,
        "Loaded ping_neighbour_period should match"
    );
    assert_eq!(
        loaded_config.routing.hop_count_penalty, 15,
        "Loaded hop_count_penalty should match"
    );
    assert_eq!(
        loaded_config.routing.maintain_period_limit, 600,
        "Loaded maintain_period_limit should match"
    );
}

#[test]
fn test_config_user_accounts_persistence() {
    let dir = TempDir::new().expect("Failed to create temp directory");
    let path = dir.path().to_str().unwrap();

    // Create a config with a user account
    let mut config = create_test_config();

    let user = UserAccount {
        name: "TestUser".to_string(),
        id: "user-id-12345".to_string(),
        keys: "user-keys-base64".to_string(),
        storage: StorageOptions {
            users: vec!["friend1".to_string(), "friend2".to_string()],
            size_total: 2048,
        },
    };
    config.user_accounts.push(user);

    // Save the config
    config.save_to_path(path);

    // Reload the config
    let loaded_config = Configuration::load_or_create(path);

    // Verify the user account was saved
    assert_eq!(
        loaded_config.user_accounts.len(),
        1,
        "Should have one user account"
    );

    let loaded_user = &loaded_config.user_accounts[0];
    assert_eq!(loaded_user.name, "TestUser", "User name should match");
    assert_eq!(loaded_user.id, "user-id-12345", "User ID should match");
    assert_eq!(
        loaded_user.keys, "user-keys-base64",
        "User keys should match"
    );
    assert_eq!(
        loaded_user.storage.size_total, 2048,
        "User storage size should match"
    );
    assert_eq!(
        loaded_user.storage.users.len(),
        2,
        "User friends list should have 2 entries"
    );
}

#[test]
fn test_config_internet_peers_persistence() {
    let dir = TempDir::new().expect("Failed to create temp directory");
    let path = dir.path().to_str().unwrap();

    // Create a config with custom internet peers
    let mut config = create_test_config();
    config.internet.peers.clear();
    config.internet.peers.push(InternetPeer {
        address: "/ip4/192.168.1.1/tcp/9000".to_string(),
        name: "Custom Peer".to_string(),
        enabled: true,
    });
    config.internet.do_listen = true;

    // Save the config
    config.save_to_path(path);

    // Reload the config
    let loaded_config = Configuration::load_or_create(path);

    // Verify the internet peers were saved
    assert_eq!(
        loaded_config.internet.peers.len(),
        1,
        "Should have one internet peer"
    );
    assert!(loaded_config.internet.do_listen, "do_listen should be true");

    let loaded_peer = &loaded_config.internet.peers[0];
    assert_eq!(
        loaded_peer.address, "/ip4/192.168.1.1/tcp/9000",
        "Peer address should match"
    );
    assert_eq!(loaded_peer.name, "Custom Peer", "Peer name should match");
    assert!(loaded_peer.enabled, "Peer should be enabled");
}

#[test]
fn test_config_file_format_is_yaml() {
    let dir = TempDir::new().expect("Failed to create temp directory");
    let path = dir.path().to_str().unwrap();

    // Create and save a config
    let config = create_test_config();
    config.save_to_path(path);

    // Read the raw file content
    let config_file = dir.path().join("config.yaml");
    let content = std::fs::read_to_string(&config_file).expect("Failed to read config file");

    // Verify it looks like YAML (has YAML-style key-value pairs)
    assert!(
        content.contains("node:"),
        "Config should contain 'node:' section"
    );
    assert!(
        content.contains("lan:"),
        "Config should contain 'lan:' section"
    );
    assert!(
        content.contains("internet:"),
        "Config should contain 'internet:' section"
    );
    assert!(
        content.contains("routing:"),
        "Config should contain 'routing:' section"
    );
}
