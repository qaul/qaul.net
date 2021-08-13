//! # RPC client functions

use prost::Message;
use libqaul;

/// include generated protobuf RPC rust definition file
pub mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs"); }


/// RPC message communication between client
/// and libqaul.
pub struct Rpc {}

impl Rpc {
    /// encode and send an rpc message to libqaul
    pub fn send_message(data: Vec<u8>, module: i32, request_id: String, user_id: Vec<u8>) {
        // Create RPC message container
        let proto_message = proto::QaulRpc {
            module,
            request_id,
            user_id,
            data,
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send the message
        libqaul::threaded::send_rpc_to_libqaul(buf);
    }

    /// receive an rpc message from libqaul
    pub fn received_message(data: Vec<u8>) {
        match proto::QaulRpc::decode(&data[..]) {
            Ok(message) => {
                log::info!("qaul rpc message received");

                match proto::Modules::from_i32(message.module) {
                    Some(proto::Modules::Node) => {
                        super::node::Node::rpc(message.data);
                    },
                    Some(proto::Modules::Rpc) => {
                        // TODO: authorisation
                    },
                    Some(proto::Modules::Useraccounts) => {
                        // TODO
                    },
                    Some(proto::Modules::Router) => {
                        // TODO
                    },
                    Some(proto::Modules::Feed) => {
                        // TODO
                    },
                    Some(proto::Modules::None) => {},
                    None => {},
                }
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }
}
