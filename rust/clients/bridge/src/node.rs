// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Node module functions

use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.node.rs"); }

/// node module function handling
pub struct Node {}

impl Node {

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the node module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Node::decode(&data[..]) {
            Ok(node) => {
                match node.message {
                    Some(proto::node::Message::Info(proto_nodeinformation)) => {
                        // print information
                        println!("Node ID is: {}", proto_nodeinformation.id_base58 );
                        println!("Node Addresses are:");
                        for address in proto_nodeinformation.addresses {
                            println!("    {}", address);
                        }
                    }
                    _ => {},
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }
}