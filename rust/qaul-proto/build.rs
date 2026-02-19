// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Build Rust Protobuf Files
//!
//! Compiles all .proto files and copies the generated Rust code
//! to the shared protobuf_generated/rust/ folder.

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let mut prost_build = prost_build::Config::new();

    // make chat messages serializable
    // in order to save them in the data base
    prost_build.type_attribute(
        "ChatMessage",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );
    prost_build.type_attribute(
        "MessageReceptionConfirmed",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );

    // make network messaging serializable
    // in order to save them in the data base
    prost_build.type_attribute(
        "Container",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );
    prost_build.type_attribute(
        "Envelope",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );
    prost_build.type_attribute(
        "Confirmation",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );

    // make group messages serializable
    // in order to save them in the data base
    prost_build.type_attribute(
        "GroupInfo",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );
    prost_build.type_attribute(
        "GroupMember",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );

    // make crypto message serializable
    // in order to save them in the data base
    prost_build.type_attribute(
        "Encrypted",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );
    prost_build.type_attribute("Data", "#[derive(serde::Serialize, serde::Deserialize)]");

    // compile these protobuf files
    match prost_build.compile_protos(
        &[
            "rpc/qaul_rpc.proto",
            "rpc/authentication.proto",
            "rpc/debug.proto",
            "connections/connections.proto",
            "node/node.proto",
            "node/user_accounts.proto",
            "router/users.proto",
            "router/router.proto",
            "router/router_net_info.proto",
            "services/feed/feed.proto",
            "services/feed/feed_net.proto",
            "services/group/group_net.proto",
            "services/group/group_rpc.proto",
            "services/rtc/rtc_net.proto",
            "services/rtc/rtc_rpc.proto",
            "services/chat/chat.proto",
            "services/chat/chatfile_net.proto",
            "services/chat/chatfile_rpc.proto",
            "connections/ble/ble.proto",
            "connections/ble/ble_net.proto",
            "connections/ble/ble_rpc.proto",
            "services/messaging/messaging.proto",
            "services/dtn/dtn_rpc.proto",
            "services/crypto/crypto_net.proto",
        ],
        &["proto"],
    ) {
        Ok(_) => {
            // copy generated protobuf files to the shared folder
            let out_dir = env::var_os("OUT_DIR").unwrap();
            let to = Path::new("../../protobuf/protobuf_generated/rust");

            let files = [
                "qaul.rpc.rs",
                "qaul.rpc.authentication.rs",
                "qaul.rpc.debug.rs",
                "qaul.rpc.connections.rs",
                "qaul.rpc.node.rs",
                "qaul.rpc.user_accounts.rs",
                "qaul.rpc.users.rs",
                "qaul.rpc.router.rs",
                "qaul.rpc.feed.rs",
                "qaul.rpc.chat.rs",
                "qaul.rpc.chatfile.rs",
                "qaul.rpc.group.rs",
                "qaul.rpc.dtn.rs",
                "qaul.rpc.rtc.rs",
                "qaul.rpc.ble.rs",
                "qaul.sys.ble.rs",
                "qaul.net.router_net_info.rs",
                "qaul.net.messaging.rs",
                "qaul.net.feed.rs",
                "qaul.net.chatfile.rs",
                "qaul.net.group.rs",
                "qaul.net.rtc.rs",
                "qaul.net.ble.rs",
                "qaul.net.crypto.rs",
            ];

            for file in &files {
                fs::copy(Path::new(&out_dir).join(file), to.join(file)).unwrap();
            }
        }
        Err(err) => {
            println!("cargo::warning=The qaul protobuf files were not compiled. This is not a problem, as long as you didn't change them. Reason: {}", err);
        }
    }
}
