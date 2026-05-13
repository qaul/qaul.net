// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! DTN (Delay-Tolerant Networking) RPC commands.
//!
//! Ported from `rust/clients/cli/src/dtn.rs` with the transport swapped
//! to qauld-ctl's `RpcCommand` trait. The protobuf request construction
//! and the per-response printers are preserved from the qaul-cli source.

use prost::Message;
use serde_json::json;

use crate::{cli::DtnSubcmd, commands::RpcCommand, proto::Modules};

use super::id_string_to_bin;

/// protobuf RPC definition
use qaul_proto::qaul_rpc_dtn as proto;

impl RpcCommand for DtnSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let proto_message = match self {
            DtnSubcmd::State => proto::Dtn {
                message: Some(proto::dtn::Message::DtnStateRequest(
                    proto::DtnStateRequest {},
                )),
            },
            DtnSubcmd::Config => proto::Dtn {
                message: Some(proto::dtn::Message::DtnConfigRequest(
                    proto::DtnConfigRequest {},
                )),
            },
            DtnSubcmd::Add { user_id } => proto::Dtn {
                message: Some(proto::dtn::Message::DtnAddUserRequest(
                    proto::DtnAddUserRequest {
                        user_id: id_string_to_bin(user_id.clone())?,
                    },
                )),
            },
            DtnSubcmd::Remove { user_id } => proto::Dtn {
                message: Some(proto::dtn::Message::DtnRemoveUserRequest(
                    proto::DtnRemoveUserRequest {
                        user_id: id_string_to_bin(user_id.clone())?,
                    },
                )),
            },
            DtnSubcmd::Size { size } => proto::Dtn {
                message: Some(proto::dtn::Message::DtnSetTotalSizeRequest(
                    proto::DtnSetTotalSizeRequest { total_size: *size },
                )),
            },
        };

        Ok((proto_message.encode_to_vec(), Modules::Dtn))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        match proto::Dtn::decode(data) {
            Ok(dtn) => match dtn.message {
                Some(proto::dtn::Message::DtnStateResponse(s)) => {
                    if json {
                        let obj = json!({
                            "used_size_mb": s.used_size,
                            "dtn_message_count": s.dtn_message_count,
                            "unconfirmed_count": s.unconfirmed_count,
                        });
                        println!("{}", serde_json::to_string_pretty(&obj)?);
                    } else {
                        println!("====================================");
                        println!("DTN State");
                        println!("\tUsed Storage Size: {} MB", s.used_size);
                        println!("\tDTN Messages: {}", s.dtn_message_count);
                        println!("\tUnconfirmed Messages: {}", s.unconfirmed_count);
                    }
                }
                Some(proto::dtn::Message::DtnConfigResponse(c)) => {
                    if json {
                        let users: Vec<String> = c
                            .users
                            .iter()
                            .map(|u| bs58::encode(u).into_string())
                            .collect();
                        let obj = json!({
                            "max_size_mb": c.total_size,
                            "users": users,
                        });
                        println!("{}", serde_json::to_string_pretty(&obj)?);
                    } else {
                        println!("====================================");
                        println!("DTN Options");
                        println!("\tMaximum Storage Size: {} MB", c.total_size);
                        println!("\tUsers");
                        for user in c.users {
                            println!("\t\t{}", bs58::encode(user).into_string());
                        }
                    }
                }
                Some(proto::dtn::Message::DtnAddUserResponse(r)) => {
                    print_status(json, "DTN Add User", r.status, &r.message)?;
                }
                Some(proto::dtn::Message::DtnRemoveUserResponse(r)) => {
                    print_status(json, "DTN Remove User", r.status, &r.message)?;
                }
                Some(proto::dtn::Message::DtnSetTotalSizeResponse(r)) => {
                    print_status(json, "DTN Set Total Size", r.status, &r.message)?;
                }
                other => {
                    log::warn!("dtn: unexpected response variant: {other:?}");
                }
            },
            Err(error) => {
                eprintln!("{:?}", error);
                log::error!("{:?}", error);
            }
        }
        Ok(())
    }
}

fn print_status(
    json: bool,
    label: &str,
    status: bool,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if json {
        let obj = json!({
            "operation": label,
            "success": status,
            "message": message,
        });
        println!("{}", serde_json::to_string_pretty(&obj)?);
    } else {
        println!("====================================");
        println!("{label}");
        if status {
            println!("\tSuccess");
        } else {
            println!("\tFailed");
            println!("\t{message}");
        }
    }
    Ok(())
}
