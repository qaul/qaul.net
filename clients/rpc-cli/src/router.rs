//! # Node module functions

use prost::Message;
use super::rpc::Rpc;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.router.rs"); }

/// router module function handling
pub struct Router {}

impl Router {
    /// CLI command interpretation
    /// 
    /// The CLI commands of router module are processed here
    pub fn cli(command: &str) {
        match command {
            // request list of all users
            cmd if cmd.starts_with("users list") => {
                Self::request_user_list();
            },
            // unknown command
            _ => log::error!("unknown router command"),
        }
    }

    /// create rpc request for user list
    fn request_user_list() {
        // create request message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::UserRequest(
                proto::UserRequest {}
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Router.into(), "".to_string());
    }

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the router module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Router::decode(&data[..]) {
            Ok(router) => {
                match router.message {
                    Some(proto::router::Message::UserList(proto_userlist)) => {
                        let mut line = 1;
                        println!("All known Users");
                        println!("No. | User Name | User Id | Public Key");

                        for user in proto_userlist.user {
                            println!("{} | {} | {:?} | {:?}", line, user.name, user.id_base58, user.key_base58);
                            line += 1;
                        }                            
                    }
                    _ => {
                        log::error!("unprocessable RPC router message");
                    },
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }
}