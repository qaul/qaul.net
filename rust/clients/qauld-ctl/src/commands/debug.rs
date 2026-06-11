// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Debug RPC commands (libqaul-side diagnostics).
//!
//! Ported from `rust/clients/cli/src/debug.rs` with the transport swapped
//! from the in-process `Rpc::send_message` call to the `qauld-ctl`
//! `RpcCommand` trait, which routes the encoded bytes through the qauld
//! unix-socket / TCP RPC channel. The protobuf encode / decode bodies
//! are preserved verbatim from the qaul-cli source.

use prost::Message;
use serde_json::json;

use crate::{
    cli::{DebugSubcmd, LogSubcmd},
    commands::RpcCommand,
    proto::Modules,
};

/// protobuf RPC definition
use qaul_proto::qaul_rpc_debug as proto;

impl RpcCommand for DebugSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let proto_message = match self {
            DebugSubcmd::Path => proto::Debug {
                message: Some(proto::debug::Message::StoragePathRequest(
                    proto::StoragePathRequest {},
                )),
            },
            DebugSubcmd::Heartbeat => proto::Debug {
                message: Some(proto::debug::Message::HeartbeatRequest(
                    proto::HeartbeatRequest {},
                )),
            },
            DebugSubcmd::Panic => proto::Debug {
                message: Some(proto::debug::Message::Panic(proto::Panic {})),
            },
            DebugSubcmd::Log(args) => proto::Debug {
                message: Some(proto::debug::Message::LogToFile(proto::LogToFile {
                    enable: matches!(args.command, LogSubcmd::Enable),
                })),
            },
        };

        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");
        Ok((buf, Modules::Debug))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        match proto::Debug::decode(data) {
            Ok(debug) => match debug.message {
                Some(proto::debug::Message::StoragePathResponse(resp)) => {
                    if json {
                        let obj = json!({ "storage_path": resp.storage_path });
                        println!("{}", serde_json::to_string_pretty(&obj)?);
                    } else {
                        println!("Storage Path: {}", resp.storage_path);
                    }
                }
                Some(proto::debug::Message::HeartbeatResponse(_)) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&json!({ "heartbeat": true }))?);
                    } else {
                        println!("Heartbeat response received");
                    }
                }
                other => {
                    return Err(format!("debug: unexpected response variant: {other:?}").into());
                }
            },
            Err(error) => {
                return Err(format!("debug: failed to decode response: {error:?}").into());
            }
        }
        Ok(())
    }

    fn expects_response(&self) -> bool {
        // Path and Heartbeat round-trip; Panic and Log are fire-and-forget
        // (libqaul does not emit a response for them).
        matches!(self, DebugSubcmd::Path | DebugSubcmd::Heartbeat)
    }
}
