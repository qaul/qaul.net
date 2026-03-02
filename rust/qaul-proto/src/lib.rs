// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul-proto
//!
//! Centralized protobuf type definitions for the qaul.net project.
//! All proto files are compiled here and exposed as public modules.

#[allow(clippy::all)]
pub mod qaul_rpc {
    include!("../../../protobuf/generated/rust/qaul.rpc.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_authentication {
    include!("../../../protobuf/generated/rust/qaul.rpc.authentication.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_debug {
    include!("../../../protobuf/generated/rust/qaul.rpc.debug.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_node {
    include!("../../../protobuf/generated/rust/qaul.rpc.node.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_user_accounts {
    include!("../../../protobuf/generated/rust/qaul.rpc.user_accounts.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_users {
    include!("../../../protobuf/generated/rust/qaul.rpc.users.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_router {
    include!("../../../protobuf/generated/rust/qaul.rpc.router.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_connections {
    include!("../../../protobuf/generated/rust/qaul.rpc.connections.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_feed {
    include!("../../../protobuf/generated/rust/qaul.rpc.feed.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_chat {
    include!("../../../protobuf/generated/rust/qaul.rpc.chat.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_chatfile {
    include!("../../../protobuf/generated/rust/qaul.rpc.chatfile.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_group {
    include!("../../../protobuf/generated/rust/qaul.rpc.group.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_dtn {
    include!("../../../protobuf/generated/rust/qaul.rpc.dtn.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_rtc {
    include!("../../../protobuf/generated/rust/qaul.rpc.rtc.rs");
}
#[allow(clippy::all)]
pub mod qaul_rpc_ble {
    include!("../../../protobuf/generated/rust/qaul.rpc.ble.rs");
}
#[allow(clippy::all)]
pub mod qaul_sys_ble {
    include!("../../../protobuf/generated/rust/qaul.sys.ble.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_router_net_info {
    include!("../../../protobuf/generated/rust/qaul.net.router_net_info.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_messaging {
    include!("../../../protobuf/generated/rust/qaul.net.messaging.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_feed {
    include!("../../../protobuf/generated/rust/qaul.net.feed.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_chatfile {
    include!("../../../protobuf/generated/rust/qaul.net.chatfile.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_group {
    include!("../../../protobuf/generated/rust/qaul.net.group.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_rtc {
    include!("../../../protobuf/generated/rust/qaul.net.rtc.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_ble {
    include!("../../../protobuf/generated/rust/qaul.net.ble.rs");
}
#[allow(clippy::all)]
pub mod qaul_net_crypto {
    include!("../../../protobuf/generated/rust/qaul.net.crypto.rs");
}
