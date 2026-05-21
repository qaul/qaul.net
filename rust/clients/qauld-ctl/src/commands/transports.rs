// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Transports management RPC commands.
//!
//! Ported from `rust/clients/cli/src/transports.rs` with the transport
//! swapped to qauld-ctl's `RpcCommand` trait. The protobuf request
//! construction and the per-response printers are preserved from the
//! qaul-cli source.

use prost::Message;
use serde_json::json;

use crate::{cli::TransportsSubcmd, commands::RpcCommand, proto::Modules};

/// protobuf RPC definition
use qaul_proto::qaul_rpc_transports as proto;

impl RpcCommand for TransportsSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let proto_message = match self {
            TransportsSubcmd::List => proto::Transports {
                message: Some(proto::transports::Message::ListRequest(
                    proto::TransportsListRequest {},
                )),
            },
            TransportsSubcmd::Enable { id } => proto::Transports {
                message: Some(proto::transports::Message::SetEnabled(
                    proto::TransportSetEnabled {
                        id: id.clone(),
                        enabled: true,
                    },
                )),
            },
            TransportsSubcmd::Disable { id } => proto::Transports {
                message: Some(proto::transports::Message::SetEnabled(
                    proto::TransportSetEnabled {
                        id: id.clone(),
                        enabled: false,
                    },
                )),
            },
        };

        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");
        Ok((buf, Modules::Transports))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        match proto::Transports::decode(data) {
            Ok(msg) => match msg.message {
                Some(proto::transports::Message::List(list)) => {
                    if json {
                        let arr: Vec<_> = list
                            .transports
                            .iter()
                            .map(|t| {
                                json!({
                                    "id": t.id,
                                    "label": t.label,
                                    "status": t.status,
                                    "enabled": t.enabled,
                                    "supports_runtime_toggle": t.supports_runtime_toggle,
                                    "is_local_only": t.is_local_only,
                                })
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&arr)?);
                    } else if list.transports.is_empty() {
                        println!("(no transports registered)");
                    } else {
                        println!(
                            "{:<10} | {:<20} | {:<10} | {:>7} | {:>13} | {:>11}",
                            "id", "label", "status", "enabled", "runtime_toggle", "local_only"
                        );
                        for t in &list.transports {
                            println!(
                                "{:<10} | {:<20} | {:<10} | {:>7} | {:>13} | {:>11}",
                                t.id,
                                t.label,
                                t.status,
                                t.enabled,
                                t.supports_runtime_toggle,
                                t.is_local_only,
                            );
                        }
                    }
                }
                Some(proto::transports::Message::SetEnabledResult(r)) => {
                    if json {
                        let obj = json!({
                            "id": r.id,
                            "success": r.success,
                            "error": r.error,
                        });
                        println!("{}", serde_json::to_string_pretty(&obj)?);
                    } else if r.success {
                        println!("transport '{}' updated", r.id);
                    } else {
                        eprintln!("transport '{}' update FAILED: {}", r.id, r.error);
                        return Err(format!("transport '{}' update failed: {}", r.id, r.error).into());
                    }
                }
                Some(proto::transports::Message::ListRequest(_))
                | Some(proto::transports::Message::SetEnabled(_)) => {
                    // requests echoed back; nothing to render
                }
                None => return Err("empty transports RPC response".into()),
            },
            Err(e) => {
                return Err(format!("transports: failed to decode response: {e}").into());
            }
        }
        Ok(())
    }
}
