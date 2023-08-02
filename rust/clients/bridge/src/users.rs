// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Users functions
//!
//! Request and get

use std::collections::HashMap;

use prost::Message;
use tokio::runtime::Runtime;
use uuid::Uuid;

use super::rpc::Rpc;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.users.rs");
}

use crate::relay_bot::MATRIX_CLIENT;

use matrix_sdk::{
    room::Room,
    ruma::{
        events::{room::message::MessageEventContent, AnyMessageEventContent},
        RoomId,
    },
};

pub static QAUL_USERS: state::Storage<HashMap<String, String>> = state::Storage::new();

/// users function handling
pub struct Users {}

impl Users {
    /// CLI command interpretation
    ///
    /// The CLI commands of users are processed here
    pub fn cli(command: &str) {
        match command {
            // request list of all users
            cmd if cmd.starts_with("matrix") => {
                Self::request_matrix_user_list();
            }

            cmd if cmd.starts_with("list") => {
                Self::request_user_list("".to_owned());
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
            // security number for a user
            cmd if cmd.starts_with("secure ") => {
                let user_id = cmd.strip_prefix("secure ").unwrap();
                Self::send_user_secure_number(user_id);
            }
            // unknown command
            _ => log::error!("unknown users command"),
        }
    }

    /// returns the list of users connected to matrix room
    fn request_matrix_user_list() {
        let matrix_client = MATRIX_CLIENT.get();
        let room_id = RoomId::try_from("!nGnOGFPgRafNcUAJJA:matrix.org").unwrap();
        let room = matrix_client.get_room(&room_id).unwrap();
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let members = room.joined_members().await.unwrap();
            for member in members {
                println!("{:#?}", member.display_name().unwrap());
            }
        });
    }

    /// create rpc request for user list
    pub fn request_user_list(request_id: String) {
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
        Rpc::send_message(buf, super::rpc::proto::Modules::Users.into(), request_id);
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

    /// create rpc user security number message
    fn send_user_secure_number(user_id_base58: &str) {
        let user_id = bs58::decode(user_id_base58).into_vec().unwrap();

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
            buf,
            super::rpc::proto::Modules::Users.into(),
            "".to_string(),
        );
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the users module.
    pub fn rpc(data: Vec<u8>, request_id: String) {
        match proto::Users::decode(&data[..]) {
            Ok(users) => match users.message {
                Some(proto::users::Message::UserList(proto_userlist)) => {
                    let mut line = 1;
                    // println!("");
                    // println!("All known Users");
                    // println!("No. | User Name | User Id | Veryfied | Blocked | Connectivity");
                    // println!("    | Group ID | Public Key");

                    if request_id != "" {
                        if let Ok(room_id) = RoomId::try_from(request_id) {
                            Self::matrix_rpc(proto_userlist.user.clone(), room_id);
                        } else {
                            // Send the Response to master matrix room
                            Self::matrix_rpc(
                                proto_userlist.user.clone(),
                                RoomId::try_from("!nGnOGFPgRafNcUAJJA:matrix.org").unwrap(),
                            );
                        }
                    }

                    let mut userMap: HashMap<String, String> = HashMap::new();

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
                        let users_list = format!(
                            "{} | {} | {:?} | {} | {} | {}",
                            line,
                            user.name,
                            bs58::encode(user.id.clone()).into_string(),
                            verified,
                            blocked,
                            onlined
                        );
                        // println!("{}", users_list);
                        userMap.insert(user.name, bs58::encode(user.id).into_string());
                        // Get the mapping for user.id and user.name
                        // Store this mapping globally and try to access it in chat.rs
                        let group_uuid;
                        match Uuid::from_slice(&user.group_id) {
                            Ok(uuid) => {
                                group_uuid = uuid;
                                // println!(
                                //     "   | {} | {}",
                                //     group_uuid.hyphenated().to_string(),
                                //     user.key_base58
                                // );
                            }
                            Err(e) => log::error!("{}", e),
                        }
                        if user.connections.len() > 0 {
                            // println!("  Connections: module | hc | rtt | via");
                            for cnn in user.connections {
                                let module = proto::ConnectionModule::from_i32(cnn.module)
                                    .unwrap()
                                    .as_str_name();
                                // println!(
                                //     "      {} | {} | {} | {}",
                                //     module,
                                //     cnn.hop_count,
                                //     cnn.rtt,
                                //     bs58::encode(cnn.via.clone()).into_string()
                                // );
                            }
                        }
                        line += 1;
                    }
                    QAUL_USERS.set(userMap);
                    // println!("");
                }
                Some(proto::users::Message::SecurityNumberResponse(resp)) => {
                    // println!("Security Number:");
                    let mut counter = 0;
                    for number in resp.security_number_blocks {
                        // print!("{:#05} ", number);
                        if counter == 3 {
                            println!("");
                        }
                        counter = counter + 1;
                    }
                    // println!("");
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

    /// Connect RPC function call with the Matrix Room and send message
    fn matrix_rpc(users_list: Vec<self::proto::UserEntry>, room_id: RoomId) {
        // Get the Room based on RoomID from the client information
        let matrix_client = MATRIX_CLIENT.get();
        let room = matrix_client.get_room(&room_id).unwrap();
        let mut result = String::new();
        if users_list.len() == 0 {
            result.push_str(&format!("No users Found"));
        } else {
            let mut number = 1;
            for user in users_list {
                // User is online
                if user.connectivity == 1 {
                    result.push_str(&format!(
                        "{} : {} | {}\n",
                        number,
                        user.name,
                        bs58::encode(user.id).into_string()
                    ));
                    number += 1;
                }
            }
        }
        // Check if the room is already joined or not
        if let Room::Joined(room) = room {
            // Build the message content to send to matrix
            let content =
                AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(result));

            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                // Sends messages into the matrix room
                room.send(content, None).await.unwrap();
            });
        };
    }

    pub fn peer_id_to_user_name() -> String {
        // Receive List of users from RPC
        return String::new();
    }
}
