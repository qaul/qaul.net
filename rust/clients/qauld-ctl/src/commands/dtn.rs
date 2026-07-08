// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! DTN (Delay-Tolerant Networking) RPC commands.
//!
//! Ported from `rust/clients/cli/src/dtn.rs` with the transport swapped
//! to qauld-ctl's `RpcCommand` trait. The protobuf request construction
//! and the per-response printers are preserved from the qaul-cli source.

use prost::Message;
use serde_json::json;

use crate::{
    cli::{DtnCustodySubcmd, DtnSubcmd},
    commands::RpcCommand,
    proto::Modules,
};

use super::id_string_to_bin;

/// protobuf RPC definition
use qaul_proto::qaul_rpc_dtn as proto;

/// Parse a hop-numbered custody route from `hop:id` tokens.
///
/// Each token is `<hop>:<base58 id>`. Tokens sharing a hop number are
/// grouped into one `DtnRouteHop` (interchangeable alternatives at that
/// hop), preserving first-seen order.
pub(crate) fn parse_hop_route(
    tokens: &[String],
) -> Result<Vec<proto::DtnRouteHop>, Box<dyn std::error::Error>> {
    let mut order: Vec<u32> = Vec::new();
    let mut hops: std::collections::HashMap<u32, Vec<Vec<u8>>> = std::collections::HashMap::new();
    for token in tokens {
        let (hop_str, id_str) = token
            .split_once(':')
            .ok_or_else(|| format!("route entry '{}' must be in hop:id form", token))?;
        let hop: u32 = hop_str
            .trim()
            .parse()
            .map_err(|_| format!("invalid hop number '{}'", hop_str))?;
        let id = id_string_to_bin(id_str.trim().to_string())?;
        if !hops.contains_key(&hop) {
            order.push(hop);
        }
        hops.entry(hop).or_default().push(id);
    }
    Ok(order
        .into_iter()
        .map(|hop| proto::DtnRouteHop {
            hop,
            ids: hops.remove(&hop).unwrap_or_default(),
        })
        .collect())
}

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
            DtnSubcmd::Custody { command } => proto::Dtn {
                message: Some(proto::dtn::Message::DtnSetCustodyEnabledRequest(
                    proto::DtnSetCustodyEnabledRequest {
                        enabled: matches!(command, DtnCustodySubcmd::Enable),
                    },
                )),
            },
            DtnSubcmd::SendRouted {
                receiver_id,
                data_file,
                custody_route,
                expiry_seconds,
                max_handoffs,
            } => {
                let receiver = id_string_to_bin(receiver_id.clone())?;
                let custody = parse_hop_route(custody_route)?;
                let data = std::fs::read(data_file)?;
                proto::Dtn {
                    message: Some(proto::dtn::Message::DtnSendRoutedRequest(
                        proto::DtnSendRoutedRequest {
                            receiver_id: receiver,
                            data,
                            custody_route: custody,
                            expiry_seconds: *expiry_seconds,
                            max_handoffs: *max_handoffs,
                        },
                    )),
                }
            }
            DtnSubcmd::Route { command } => match command {
                crate::cli::DtnRouteSubcmd::Set {
                    receiver_id,
                    custody_route,
                } => proto::Dtn {
                    message: Some(proto::dtn::Message::DtnSetSenderRouteRequest(
                        proto::DtnSetSenderRouteRequest {
                            receiver_id: id_string_to_bin(receiver_id.clone())?,
                            custody_route: parse_hop_route(custody_route)?,
                        },
                    )),
                },
                crate::cli::DtnRouteSubcmd::List => proto::Dtn {
                    message: Some(proto::dtn::Message::DtnSenderRoutesRequest(
                        proto::DtnSenderRoutesRequest {},
                    )),
                },
                crate::cli::DtnRouteSubcmd::Remove { receiver_id } => proto::Dtn {
                    message: Some(proto::dtn::Message::DtnRemoveSenderRouteRequest(
                        proto::DtnRemoveSenderRouteRequest {
                            receiver_id: id_string_to_bin(receiver_id.clone())?,
                        },
                    )),
                },
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
                            "used_size_v2_bytes": s.used_size_v2,
                            "dtn_message_count_v2": s.dtn_message_count_v2,
                        });
                        println!("{}", serde_json::to_string_pretty(&obj)?);
                    } else {
                        println!("====================================");
                        println!("DTN State");
                        println!("\tUsed Storage Size: {} MB", s.used_size);
                        println!("\tDTN Messages: {}", s.dtn_message_count);
                        println!("\tUnconfirmed Messages: {}", s.unconfirmed_count);
                        println!("\tV2 Custody Storage Used: {} bytes", s.used_size_v2);
                        println!("\tV2 Custody Messages: {}", s.dtn_message_count_v2);
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
                Some(proto::dtn::Message::DtnSetCustodyEnabledResponse(r)) => {
                    print_status(json, "DTN Set Custody Enabled", r.status, &r.message)?;
                }
                Some(proto::dtn::Message::DtnSendRoutedResponse(r)) => {
                    print_status(json, "DTN Send Routed", r.status, &r.message)?;
                }
                Some(proto::dtn::Message::DtnSetSenderRouteResponse(r)) => {
                    print_status(json, "DTN Set Sender Route", r.status, &r.message)?;
                }
                Some(proto::dtn::Message::DtnRemoveSenderRouteResponse(r)) => {
                    print_status(json, "DTN Remove Sender Route", r.status, &r.message)?;
                }
                Some(proto::dtn::Message::DtnSenderRoutesResponse(r)) => {
                    if json {
                        let routes: Vec<serde_json::Value> = r
                            .routes
                            .iter()
                            .map(|route| {
                                let hops: Vec<serde_json::Value> = route
                                    .custody_route
                                    .iter()
                                    .map(|h| {
                                        json!({
                                            "hop": h.hop,
                                            "ids": h.ids.iter()
                                                .map(|c| bs58::encode(c).into_string())
                                                .collect::<Vec<_>>(),
                                        })
                                    })
                                    .collect();
                                json!({
                                    "receiver_id": bs58::encode(&route.receiver_id).into_string(),
                                    "custody_route": hops,
                                    "created_at": route.created_at,
                                })
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&routes)?);
                    } else {
                        println!("====================================");
                        println!("DTN Sender Routes ({})", r.routes.len());
                        for route in &r.routes {
                            println!(
                                "  -> {}",
                                bs58::encode(&route.receiver_id).into_string()
                            );
                            for h in &route.custody_route {
                                let ids: Vec<String> = h
                                    .ids
                                    .iter()
                                    .map(|c| bs58::encode(c).into_string())
                                    .collect();
                                println!("      hop {}: {}", h.hop, ids.join(", "));
                            }
                        }
                    }
                }
                other => {
                    return Err(format!("dtn: unexpected response variant: {other:?}").into());
                }
            },
            Err(error) => {
                return Err(format!("dtn: failed to decode response: {error:?}").into());
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
    } else if status {
        println!("====================================");
        println!("{label}");
        println!("\tSuccess");
    } else {
        eprintln!("====================================");
        eprintln!("{label}");
        eprintln!("\tFailed");
        eprintln!("\t{message}");
    }
    if !status {
        return Err(format!("{label}: {message}").into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Valid base58 PeerId strings (>= 52 chars, as id_string_to_bin requires).
    const ID_A: &str = "12D3KooWHqFzG5fKSCGYTe6Mg3XEFgw8T9GSTsww79ptvaVfGp1r";
    const ID_B: &str = "12D3KooWAgSafsscJgLgPfkgqd297QdMQ6jJxLueE6DDTygpGYRK";
    const ID_C: &str = "12D3KooWCSFj52oARtgnyzVLbGsfTDBe3WB3pjy15Zrch9PPpoKc";

    fn id() -> String {
        ID_A.to_string()
    }

    #[test]
    fn parse_hop_route_groups_same_hop_alternatives() {
        let tokens = vec![
            format!("1:{ID_A}"),
            format!("2:{ID_B}"),
            format!("2:{ID_C}"),
        ];
        let route = parse_hop_route(&tokens).expect("valid route");
        assert_eq!(route.len(), 2);
        assert_eq!(route[0].hop, 1);
        assert_eq!(route[0].ids.len(), 1);
        assert_eq!(route[1].hop, 2);
        assert_eq!(route[1].ids.len(), 2); // b and c share hop 2
    }

    #[test]
    fn parse_hop_route_rejects_missing_colon() {
        assert!(parse_hop_route(&[id()]).is_err());
    }

    #[test]
    fn parse_hop_route_rejects_bad_hop_number() {
        assert!(parse_hop_route(&[format!("x:{}", id())]).is_err());
    }
}
