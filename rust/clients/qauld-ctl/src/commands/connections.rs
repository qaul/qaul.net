// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Internet peer-node management.
//!
//! Ported from `rust/clients/cli/src/connections.rs` with the transport
//! swapped to qauld-ctl's `RpcCommand` trait. The protobuf payload
//! construction and the InternetNodesList decoder are preserved verbatim
//! from the qaul-cli source.

use prost::Message;
use serde_json::json;

use crate::{
    cli::{ConnectionsSubcmd, NodesSubcmd},
    commands::RpcCommand,
    proto::Modules,
};

/// protobuf RPC definition
use qaul_proto::qaul_rpc_connections as proto;

impl RpcCommand for ConnectionsSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let ConnectionsSubcmd::Nodes(args) = self;
        let proto_message = match &args.command {
            NodesSubcmd::List => proto::Connections {
                message: Some(proto::connections::Message::InternetNodesRequest(
                    proto::InternetNodesRequest {},
                )),
            },
            NodesSubcmd::Add { address, name } => proto::Connections {
                message: Some(proto::connections::Message::InternetNodesAdd(
                    proto::InternetNodesEntry {
                        address: address.clone(),
                        name: name.clone(),
                        enabled: true,
                    },
                )),
            },
            NodesSubcmd::Remove { address } => proto::Connections {
                message: Some(proto::connections::Message::InternetNodesRemove(
                    proto::InternetNodesEntry {
                        address: address.clone(),
                        name: String::new(),
                        enabled: false,
                    },
                )),
            },
            NodesSubcmd::Rename { address, name } => proto::Connections {
                message: Some(proto::connections::Message::InternetNodesRename(
                    proto::InternetNodesEntry {
                        address: address.clone(),
                        name: name.clone(),
                        enabled: true,
                    },
                )),
            },
            NodesSubcmd::Activate { address } => proto::Connections {
                message: Some(proto::connections::Message::InternetNodesState(
                    proto::InternetNodesEntry {
                        address: address.clone(),
                        name: String::new(),
                        enabled: true,
                    },
                )),
            },
            NodesSubcmd::Deactivate { address } => proto::Connections {
                message: Some(proto::connections::Message::InternetNodesState(
                    proto::InternetNodesEntry {
                        address: address.clone(),
                        name: String::new(),
                        enabled: false,
                    },
                )),
            },
        };

        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");
        Ok((buf, Modules::Connections))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        match proto::Connections::decode(data) {
            Ok(connections) => match connections.message {
                Some(proto::connections::Message::InternetNodesList(list)) => {
                    let info_label = match proto::Info::try_from(list.info) {
                        Ok(proto::Info::Request) => "",
                        Ok(proto::Info::AddSuccess) => {
                            "Address successfully added to 'Internet Peer Nodes List'"
                        }
                        Ok(proto::Info::AddErrorInvalid) => {
                            "ERROR: Invalid address, couldn't be added to 'Internet Peer Nodes List'"
                        }
                        Ok(proto::Info::RemoveSuccess) => {
                            "Address successfully removed from 'Internet Peer Nodes List'"
                        }
                        Ok(proto::Info::StateSuccess) => {
                            "Address successfully state changed in 'Internet Peer Nodes List'"
                        }
                        Ok(proto::Info::RemoveErrorNotFound) => {
                            "ERROR: Address not found in 'Internet Peer Nodes List'"
                        }
                        Err(_) => "Unknown Reason for 'Internet Peer Nodes List' response",
                    };

                    if json {
                        let nodes: Vec<_> = list
                            .nodes
                            .iter()
                            .map(|n| {
                                json!({
                                    "address": n.address,
                                    "name": n.name,
                                    "enabled": n.enabled,
                                })
                            })
                            .collect();
                        let obj = json!({
                            "info": info_label,
                            "nodes": nodes,
                        });
                        println!("{}", serde_json::to_string_pretty(&obj)?);
                    } else {
                        println!();
                        if !info_label.is_empty() {
                            println!("{info_label}");
                            println!();
                        }
                        println!("Internet Peer Nodes List");
                        println!("No. | Address | Name | Enabled");
                        for (i, node) in list.nodes.iter().enumerate() {
                            println!(
                                "{} | {} | {} | {}",
                                i + 1,
                                node.address,
                                node.name,
                                node.enabled
                            );
                        }
                        println!();
                    }
                }
                other => {
                    log::warn!("connections: unexpected response variant: {other:?}");
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
