// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Connections Management Functions
//!
//! * get list of statically defined internet peer nodes
//! * add an internet peer node
//! * remove an internet peer node

use super::rpc::Rpc;
use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.connections.rs");
}

/// connections function handling
pub struct Connections {}

impl Connections {
    /// CLI command interpretation
    ///
    /// The CLI commands of users are processed here
    pub fn cli(command: &str) {
        match command {
            // request list of all internet peer nodes
            cmd if cmd.starts_with("nodes list") => {
                Self::internet_nodes_list();
            }
            // add internet peer node
            cmd if cmd.starts_with("nodes add ") => {
                let address = cmd.strip_prefix("nodes add ").unwrap();

                Self::internet_node_add(String::from(address));
            }
            // remove an internet peer node
            cmd if cmd.starts_with("nodes remove ") => {
                let address = cmd.strip_prefix("nodes remove ").unwrap();

                Self::internet_node_remove(String::from(address));
            }
            // active an internet peer node
            cmd if cmd.starts_with("nodes active ") => {
                let address = cmd.strip_prefix("nodes active ").unwrap();

                Self::internet_node_active(String::from(address));
            }
            // active an internet peer node
            cmd if cmd.starts_with("nodes deactive ") => {
                let address = cmd.strip_prefix("nodes deactive ").unwrap();

                Self::internet_node_deactive(String::from(address));
            }
            // unknown command
            _ => log::error!("unknown connections command"),
        }
    }

    /// send an rpc request for internet peering nodes list
    fn internet_nodes_list() {
        // create request message
        let proto_message = proto::Connections {
            message: Some(proto::connections::Message::InternetNodesRequest(
                proto::InternetNodesRequest {},
            )),
        };

        // send message
        Self::send_message(proto_message);
    }

    /// send an RPC message to add a new internet peer node connection
    fn internet_node_add(address: String) {
        // create message
        let proto_message = proto::Connections {
            message: Some(proto::connections::Message::InternetNodesAdd(
                proto::InternetNodesEntry {
                    address,
                    enabled: true,
                },
            )),
        };

        // send message
        Self::send_message(proto_message);
    }

    /// Send an rpc message to remove a specific internet peer node connection
    ///
    /// The nodes are specified by their libp2p multiaddress
    fn internet_node_remove(address: String) {
        // create message
        let proto_message = proto::Connections {
            message: Some(proto::connections::Message::InternetNodesRemove(
                proto::InternetNodesEntry {
                    address,
                    enabled: false,
                },
            )),
        };

        // send message
        Self::send_message(proto_message);
    }

    /// Send an rpc message to active a specific internet peer node connection
    ///
    /// The nodes are specified by their libp2p multiaddress
    fn internet_node_active(address: String) {
        // create message
        let proto_message = proto::Connections {
            message: Some(proto::connections::Message::InternetNodesState(
                proto::InternetNodesEntry {
                    address,
                    enabled: true,
                },
            )),
        };
        // send message
        Self::send_message(proto_message);
    }

    /// Send an rpc message to deactive a specific internet peer node connection
    ///
    /// The nodes are specified by their libp2p multiaddress
    fn internet_node_deactive(address: String) {
        // create message
        let proto_message = proto::Connections {
            message: Some(proto::connections::Message::InternetNodesState(
                proto::InternetNodesEntry {
                    address,
                    enabled: false,
                },
            )),
        };
        // send message
        Self::send_message(proto_message);
    }

    /// Encode and send a protobuf connections message to RPC
    fn send_message(message: proto::Connections) {
        // encode message
        let mut buf = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Connections.into(),
            "".to_string(),
        );
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary connections RPC messages
    /// and display their content
    pub fn rpc(data: Vec<u8>) {
        match proto::Connections::decode(&data[..]) {
            Ok(connections) => {
                match connections.message {
                    Some(proto::connections::Message::InternetNodesList(proto_list)) => {
                        let mut line = 1;
                        println!("");

                        match proto::Info::from_i32(proto_list.info) {
                            Some(proto::Info::Request) => {
                                // all fine no further info
                            }
                            Some(proto::Info::AddSuccess) => {
                                println!(
                                    "Address successfully added to 'Internet Peer Nodes List'"
                                );
                                println!("");
                            }
                            Some(proto::Info::AddErrorInvalid) => {
                                println!("ERROR: Invalid address, couldn't be added to 'Internet Peer Nodes List'");
                                println!("");
                            }
                            Some(proto::Info::RemoveSuccess) => {
                                println!(
                                    "Address successfully removed from 'Internet Peer Nodes List'"
                                );
                                println!("");
                            }
                            Some(proto::Info::StateSuccess) => {
                                println!(
                                    "Address successfully state changed in 'Internet Peer Nodes List'"
                                );
                                println!("");
                            }
                            Some(proto::Info::RemoveErrorNotFound) => {
                                println!("ERROR: Address not found in 'Internet Peer Nodes List'");
                                println!("");
                            }
                            None => {
                                println!("Unknown Reason for 'Internet Peer Nodes List' response");
                                println!("");
                            }
                        };

                        println!("Internet Peer Nodes List");
                        println!("No. | Address | Enabled");

                        for node in proto_list.nodes {
                            println!("{} | {} | {}", line, node.address, node.enabled);
                            line += 1;
                        }

                        println!("");
                    }
                    _ => {
                        log::error!("unprocessable connections RPC message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
