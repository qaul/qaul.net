// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Per-subcommand `RpcCommand` implementations.
//!
//! The trait itself and the helpers (`id_string_to_bin`,
//! `uuid_string_to_bin`) now live in the shared `qauld-rpc` crate so
//! the TUI and any future client can reuse them. We re-export them
//! here to keep existing intra-crate import paths working.

pub use qauld_rpc::{id_string_to_bin, uuid_string_to_bin, RpcCommand};

mod authentication;
mod ble;
mod chat;
mod chatfile;
mod connections;
mod debug;
mod dtn;
mod feed;
mod group;
mod node;
mod router;
#[cfg(feature = "rtc")]
mod rtc;
mod transports;
mod user_accounts;
mod users;

pub use user_accounts::decode_default_user;
pub use user_accounts::default_user_proto_message;
