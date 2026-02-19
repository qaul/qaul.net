// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul-proto
//!
//! Centralized protobuf type definitions for the qaul.net project.
//! All proto files are compiled here and exposed as public modules.

#[allow(clippy::all)]
pub mod qaul_rpc {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_authentication {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.authentication.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_debug {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.debug.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_node {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.node.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_user_accounts {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.user_accounts.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_users {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.users.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_router {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.router.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_connections {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.connections.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_feed {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.feed.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_chat {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.chat.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_chatfile {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.chatfile.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_group {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.group.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_dtn {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.dtn.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_rtc {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.rtc.rs"));
}
#[allow(clippy::all)]
pub mod qaul_rpc_ble {
    include!(concat!(env!("OUT_DIR"), "/qaul.rpc.ble.rs"));
}
#[allow(clippy::all)]
pub mod qaul_sys_ble {
    include!(concat!(env!("OUT_DIR"), "/qaul.sys.ble.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_router_net_info {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.router_net_info.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_messaging {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.messaging.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_feed {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.feed.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_chatfile {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.chatfile.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_group {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.group.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_rtc {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.rtc.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_ble {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.ble.rs"));
}
#[allow(clippy::all)]
pub mod qaul_net_crypto {
    include!(concat!(env!("OUT_DIR"), "/qaul.net.crypto.rs"));
}
