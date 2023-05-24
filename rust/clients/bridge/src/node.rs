// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Node module functions

use prost::Message;
use super::rpc::Rpc;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.node.rs"); }

/// node module function handling
pub struct Node {}

impl Node {
    /// CLI command interpretation
    /// 
    /// The CLI commands of node module are processed here
    pub fn cli(command: &str) {
        match command {
            // node functions
            cmd if cmd.starts_with("info") => {
                Self::info();
            },
            // unknown command
            _ => log::error!("unknown node command"),
        }
    }

    /// create rpc info request
    fn info() {
        // create info request message
        let proto_message = proto::Node {
            message: Some(proto::node::Message::GetNodeInfo(true)),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Node.into(), "".to_string());
    }

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