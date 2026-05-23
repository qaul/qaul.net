// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Shared RPC plumbing for qauld clients.
//!
//! Provides the protocol-level pieces every client needs: the
//! `RpcCommand` trait, the `RpcTransport` abstraction with socket and
//! (optional) embedded impls, the `QaulRpc` envelope, helpers for
//! decoding base58 / UUID identifiers, and re-exports of the
//! protobuf modules.
//!
//! Per-command implementations of `RpcCommand` (which are CLI-shape
//! specific) live in the consuming crate (e.g. qauld-ctl). The TUI
//! and any future client can reuse this crate without inheriting
//! qauld-ctl's clap grammar.

use std::fmt;

/// Re-export of qaul-proto::qaul_rpc — this is the QaulRpc envelope.
pub use qaul_proto::qaul_rpc as proto;

pub mod transport;

pub use transport::RpcTransport;
pub use transport::SocketTransport;
#[cfg(feature = "embedded")]
pub use transport::EmbeddedTransport;

/// A single RPC command that can be sent to a running qauld daemon.
///
/// Implementors are typically CLI subcommand enums; the trait lets
/// the dispatch loop encode a request, decode the response, and know
/// whether a response is expected at all.
pub trait RpcCommand {
    /// Encode the command into a raw protobuf request payload and
    /// indicate which RPC module it targets.
    fn encode_request(&self) -> Result<(Vec<u8>, proto::Modules), Box<dyn std::error::Error>>;

    /// Decode the daemon's response payload. `json` controls whether
    /// output should be machine-readable.
    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>>;

    /// Whether the command expects a response from the daemon. Most
    /// commands do; toggles like dtn-set are fire-and-forget.
    fn expects_response(&self) -> bool {
        true
    }
}

/// Convert a base58-encoded id (typically a libp2p `PeerId`) into
/// the raw bytes the wire format expects. Length is validated.
pub fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
    if id.len() < 52 {
        return Err("id not long enough".to_string());
    }
    bs58::decode(id)
        .into_vec()
        .map_err(|e| fmt::format(format_args!("{}", e)))
}

/// Convert a UUID string (group ids, message ids, etc.) to the raw
/// 16-byte representation used on the wire.
pub fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
    match uuid::Uuid::parse_str(id_str.as_str()) {
        Ok(id) => Ok(id.as_bytes().to_vec()),
        _ => Err("invalid uuid".to_string()),
    }
}
