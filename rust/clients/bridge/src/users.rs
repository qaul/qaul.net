// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Users functions
//!
//! Request and get

use prost::Message;
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::runtime::Runtime;

use super::rpc::Rpc;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.users.rs");
}

use crate::relay_bot::{MATRIX_CLIENT, MATRIX_CONFIG};

use matrix_sdk::{
    room::Room,
    ruma::{
        events::{room::message::MessageEventContent, AnyMessageEventContent},
        RoomId,
    },
};

pub static QAUL_USERS: state::Storage<RwLock<HashMap<String, String>>> = state::Storage::new();

/// users function handling
pub struct Users {}

impl Users {
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

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the users module.
    pub fn rpc(data: Vec<u8>, request_id: String) {
        match proto::Users::decode(&data[..]) {
            Ok(users) => match users.message {
                Some(proto::users::Message::UserList(proto_userlist)) => {
                    if request_id != "" {
                        if let Ok(room_id) = RoomId::try_from(request_id) {
                            Self::matrix_rpc(proto_userlist.user.clone(), room_id);
                        } else {
                            // Send the Response to master matrix room
                            let config = MATRIX_CONFIG.get().read().unwrap();
                            Self::matrix_rpc(
                                proto_userlist.user.clone(),
                                RoomId::try_from(config.feed.feed_room.clone()).expect("Invalid Feed Room. Please update correct room into configuration"),
                            );
                        }
                    }
                    QAUL_USERS.set(RwLock::new(HashMap::new()));
                    let mut qaul_users = QAUL_USERS.get().write().unwrap();
                    for user in proto_userlist.user {
                        if user.connectivity == 1 {
                            qaul_users.insert(user.name, bs58::encode(user.id).into_string());
                        }
                    }
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
}
