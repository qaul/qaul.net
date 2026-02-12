use prost::Message;

use crate::{
    cli::NodeSubcmd,
    commands::RpcCommand,
    proto::{node, Modules, Node},
};

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
        Ok((buf, crate::proto::Modules::Node))
    }

    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        match Node::decode(data) {
            Ok(node) => {
                match node.message {
                    Some(node::Message::Info(proto_nodeinformation)) => {
                        // print information
                        println!("Node ID is: {}", proto_nodeinformation.id_base58);
                        println!("Node Addresses are:");
                        for address in proto_nodeinformation.addresses {
                            println!("    {}", address);
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
