// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Connections Management Functions
//!
//! * get list of statically defined internet peer nodes
//! * add an internet peer node
//! * remove an internet peer node

use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.connections.rs");
}

/// connections function handling
pub struct Connections {}

impl Connections {
   
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
                        println!("No. | Address | Name | Enabled");

                        for node in proto_list.nodes {
                            println!(
                                "{} | {} | {} | {}",
                                line, node.address, node.name, node.enabled
                            );
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
