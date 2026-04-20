// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Users functions
//!
//! Request and get

use prost::Message;
use uuid::Uuid;

use super::rpc::Rpc;

/// protobuf RPC definition
use qaul_proto::qaul_rpc_users as proto;

/// users function handling
pub struct Users {}

impl Users {
    /// CLI command interpretation
    ///
    /// The CLI commands of users are processed here
    pub fn cli(state: &super::CliState, command: &str) {
        match command {
            // request list of all users
            cmd if cmd.starts_with("list") => {
                Self::request_user_list(state);
            }
            cmd if cmd.starts_with("online") => {
                Self::request_online_user_list(state);
            }
            // verify a user
            cmd if cmd.starts_with("verify ") => {
                if let Some(user_id) = cmd.strip_prefix("verify ") {
                    Self::send_user_update(state, user_id, true, false);
                } else {
                    log::error!("verify command incorrectly formatted");
                }
            }
            // block a user
            cmd if cmd.starts_with("block ") => {
                if let Some(user_id) = cmd.strip_prefix("block ") {
                    Self::send_user_update(state, user_id, false, true);
                } else {
                    log::error!("block command incorrectly formatted");
                }
            }
            // security number for a user
            cmd if cmd.starts_with("secure ") => {
                if let Some(user_id) = cmd.strip_prefix("secure ") {
                    Self::send_user_secure_number(state, user_id);
                } else {
                    log::error!("secure command incorrectly formatted");
                }
            }
            // get a single user by id
            cmd if cmd.starts_with("get ") => {
                if let Some(user_id) = cmd.strip_prefix("get ") {
                    Self::request_user_by_id(state, user_id);
                } else {
                    log::error!("get command incorrectly formatted");
                }
            }
            // unknown command
            _ => log::error!("unknown users command"),
        }
    }

    /// create rpc request for user list
    fn request_user_list(state: &super::CliState) {
        // create request message
        let proto_message = proto::Users {
            message: Some(proto::users::Message::UserRequest(proto::UserRequest {
                offset: 0,
                limit: 0,
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            state,
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    fn request_online_user_list(state: &super::CliState) {
        // create request message
        let proto_message = proto::Users {
            message: Some(proto::users::Message::UserOnlineRequest(
                proto::UserOnlineRequest {
                    offset: 0,
                    limit: 0,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            state,
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    /// create rpc user security number message
    fn send_user_secure_number(state: &super::CliState, user_id_base58: &str) {
        let user_id = match bs58::decode(user_id_base58).into_vec() {
            Ok(v) => v,
            Err(e) => {
                log::error!("invalid base58 user id '{}': {}", user_id_base58, e);
                return;
            }
        };

        // create request message
        let proto_message = proto::Users {
            message: Some(proto::users::Message::SecurityNumberRequest(
                proto::SecurityNumberRequest { user_id },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            state,
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    /// create rpc request to get a single user by id
    fn request_user_by_id(state: &super::CliState, user_id_base58: &str) {
        let user_id = match bs58::decode(user_id_base58).into_vec() {
            Ok(v) => v,
            Err(e) => {
                log::error!("invalid base58 user id '{}': {}", user_id_base58, e);
                return;
            }
        };

        // create request message
        let proto_message = proto::Users {
            message: Some(proto::users::Message::GetUserByIdRequest(
                proto::GetUserByIdRequest { user_id },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            state,
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    /// create rpc user update message
    fn send_user_update(state: &super::CliState, user_id_base58: &str, verified: bool, blocked: bool) {
        let user_id = match bs58::decode(user_id_base58).into_vec() {
            Ok(v) => v,
            Err(e) => {
                log::error!("invalid base58 user id '{}': {}", user_id_base58, e);
                return;
            }
        };

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
                connections: vec![],
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            state,
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
                    println!("No. | User Name | User Id | Verified | Blocked | Connectivity");
                    println!("    | Group ID | Public Key");

                    for user in proto_userlist.user {
                        let mut verified = "N";
                        let mut blocked = "N";
                        let mut onlined = "Offline";

                        if user.verified {
                            verified = "Y";
                        }
                        if user.blocked {
                            blocked = "Y";
                        }
                        if user.connectivity == 1 {
                            onlined = "Online";
                        }
                        println!(
                            "{} | {} | {:?} | {} | {} | {}",
                            line,
                            user.name,
                            bs58::encode(user.id).into_string(),
                            verified,
                            blocked,
                            onlined
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
                        if user.connections.len() > 0 {
                            println!("  Connections: module | hc | rtt | via");
                            for cnn in user.connections {
                                let module = proto::ConnectionModule::try_from(cnn.module)
                                    .unwrap_or(proto::ConnectionModule::None)
                                    .as_str_name();
                                println!(
                                    "      {} | {} | {} | {}",
                                    module,
                                    cnn.hop_count,
                                    cnn.rtt,
                                    bs58::encode(cnn.via.clone()).into_string()
                                );
                            }
                        }
                        line += 1;
                    }

                    println!("");
                }
                Some(proto::users::Message::GetUserByIdResponse(resp)) => match resp.user {
                    Some(user) => {
                        let mut verified = "N";
                        let mut blocked = "N";
                        let mut onlined = "Offline";

                        if user.verified {
                            verified = "Y";
                        }
                        if user.blocked {
                            blocked = "Y";
                        }
                        if user.connectivity == 1 {
                            onlined = "Online";
                        }

                        println!("");
                        println!("User Info");
                        println!("Name: {}", user.name);
                        println!("ID: {}", bs58::encode(&user.id).into_string());
                        println!(
                            "Verified: {} | Blocked: {} | Connectivity: {}",
                            verified, blocked, onlined
                        );

                        match Uuid::from_slice(&user.group_id) {
                            Ok(uuid) => {
                                println!("Group ID: {}", uuid.hyphenated().to_string());
                            }
                            Err(e) => log::error!("{}", e),
                        }

                        println!("Public Key: {}", user.key_base58);

                        if user.connections.len() > 0 {
                            println!("Connections: module | hc | rtt | via");
                            for cnn in user.connections {
                                let module = proto::ConnectionModule::try_from(cnn.module)
                                    .unwrap_or(proto::ConnectionModule::None)
                                    .as_str_name();
                                println!(
                                    "  {} | {} | {} | {}",
                                    module,
                                    cnn.hop_count,
                                    cnn.rtt,
                                    bs58::encode(cnn.via.clone()).into_string()
                                );
                            }
                        }
                        println!("");
                    }
                    None => {
                        println!("User not found.");
                    }
                },
                Some(proto::users::Message::SecurityNumberResponse(resp)) => {
                    println!("Security Number:");
                    let mut counter = 0;
                    for number in resp.security_number_blocks {
                        print!("{:#05} ", number);
                        if counter == 3 {
                            println!("");
                        }
                        counter = counter + 1;
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
