// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Trait definitions for commands to be passed to qauld

use crate::proto;
use std::fmt;

mod authentication;
mod chat;
mod chatfile;
mod feed;
mod group;
mod node;
mod user_accounts;
mod users;

pub use user_accounts::decode_default_user;
pub use user_accounts::default_user_proto_message;
use uuid::Uuid;
/// Convert Group ID from String to Binary
fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
    match Uuid::parse_str(id_str.as_str()) {
        Ok(id) => Ok(id.as_bytes().to_vec()),
        _ => Err("invalid group id".to_string()),
    }
}

/// Convert Group ID from String to Binary
fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
    // check length
    if id.len() < 52 {
        return Err("Group ID not long enough".to_string());
    }

    // convert input
    match bs58::decode(id).into_vec() {
        Ok(id_bin) => Ok(id_bin),
        Err(e) => {
            let err = fmt::format(format_args!("{}", e));
            Err(err)
        }
    }
}

/// Represents a single RPC command that can be sent to a running qauld daemon over the Unix socket.
pub trait RpcCommand {
    /// Encodes a CLI subcommand into a raw protobuf request payload and the target RPC module
    fn encode_request(&self) -> Result<(Vec<u8>, proto::Modules), Box<dyn std::error::Error>>;
    /// Decodes a raw protobuf response payload and prints the result to stdout.
    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
    /// Returns true if this command expects a response from the daemon, false for fire-and-forget commands.
    fn expects_response(&self) -> bool {
        true
    }
}
