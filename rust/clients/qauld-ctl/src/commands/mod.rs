// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Trait definitions for commands to be passed to qauld

use crate::proto;

mod authentication;
mod node;
mod user_accounts;
mod users;

pub use user_accounts::default_user_proto_message;

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
