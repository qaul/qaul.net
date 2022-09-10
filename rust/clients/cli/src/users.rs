// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Users functions
//!
//! Request and get

use prost::Message;
use uuid::Uuid;

use super::rpc::Rpc;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.users.rs");
}

/// users function handling
pub struct Users {}

impl Users {
    /// CLI command interpretation
    ///
    /// The CLI commands of users are processed here
    pub fn cli(command: &str) {
        match command {
            // request list of all users
            cmd if cmd.starts_with("list") => {
                Self::request_user_list();
            }
            cmd if cmd.starts_with("online") => {
                Self::request_online_user_list();
            }
            // verify a user
            cmd if cmd.starts_with("verify ") => {
                let user_id = cmd.strip_prefix("verify ").unwrap();

                Self::send_user_update(user_id, true, false);
            }
            // block a user
            cmd if cmd.starts_with("block ") => {
                let user_id = cmd.strip_prefix("block ").unwrap();

                Self::send_user_update(user_id, false, true);
            }
            // unknown command
            _ => log::error!("unknown users command"),
        }
    }

    /// create rpc request for user list
    fn request_user_list() {
        // create request message
        let proto_message = proto::Users {
            message: Some(proto::users::Message::UserRequest(proto::UserRequest {})),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    fn request_online_user_list() {
        // create request message
        let proto_message = proto::Users {
            message: Some(proto::users::Message::UserOnlineRequest(
                proto::UserOnlineRequest {},
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    /// create rpc user update message
    fn send_user_update(user_id_base58: &str, verified: bool, blocked: bool) {
        let user_id = bs58::decode(user_id_base58).into_vec().unwrap();

        // create request message
        let proto_message = proto::Users {
            message: Some(proto::users::Message::UserUpdate(proto::UserEntry {
                name: String::from(""),
                id: user_id,
                key_base58: String::from(""),
                group_id: Vec::new(),
                connectivity: 0,
                verified,
                blocked,
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the users module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Users::decode(&data[..]) {
            Ok(users) => match users.message {
                Some(proto::users::Message::UserList(proto_userlist)) => {
                    let mut line = 1;
                    println!("");
                    println!("All known Users");
                    println!("No. | User Name | User Id | Veryfied | Blocked");
                    println!("    | Group ID | Public Key");

                    for user in proto_userlist.user {
                        let mut verified = "N";
                        let mut blocked = "N";

                        if user.verified {
                            verified = "Y";
                        }
                        if user.blocked {
                            blocked = "Y";
                        }
                        println!(
                            "{} | {} | {:?} | {} | {}",
                            line,
                            user.name,
                            bs58::encode(user.id).into_string(),
                            verified,
                            blocked
                        );
                        let group_uuid;
                        match Uuid::from_slice(&user.group_id) {
                            Ok(uuid) => {
                                group_uuid = uuid;
                                println!(
                                    "   | {} | {}",
                                    group_uuid.hyphenated().to_string(),
                                    user.key_base58
                                );
                            }
                            Err(e) => log::error!("{}", e),
                        }
                        line += 1;
                    }

                    println!("");
                }
                _ => {
                    log::error!("unprocessable RPC users message");
                }
            },
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
