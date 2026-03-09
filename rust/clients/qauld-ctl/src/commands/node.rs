use prost::Message;
use serde_json::json;

use crate::{cli::NodeSubcmd, commands::RpcCommand, proto::Modules};

/// protobuf RPC definition
use qaul_proto::qaul_rpc_node as proto;

use proto::{node, Node};

impl RpcCommand for NodeSubcmd {
    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        let proto_message = Node {
            message: Some(node::Message::GetNodeInfo(true)),
        };
        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");
        Ok((buf, Modules::Node))
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        match Node::decode(data) {
            Ok(node) => {
                match node.message {
                    Some(node::Message::Info(proto_nodeinformation)) => {
                        // print information
                        if json {
                            let obj = json!({ "node_id": proto_nodeinformation.id_base58, "addresses": proto_nodeinformation.addresses });
                            println!("{}", serde_json::to_string_pretty(&obj)?);
                        } else {
                            println!("Node ID is: {}", proto_nodeinformation.id_base58);
                            println!("Node Addresses are:");
                            for address in proto_nodeinformation.addresses {
                                println!("    {}", address);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(error) => {
                eprintln!("{:?}", error);
                log::error!("{:?}", error);
            }
        };
        Ok(())
    }
}
