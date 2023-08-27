// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Module Interactions
//!
//! Request, display and send chat messages from CLI

use super::rpc::Rpc;
use crate::{
    configuration::MatrixConfiguration,
    relay_bot::{MATRIX_CLIENT, MATRIX_CONFIG},
    user_accounts::BOT_USER_ACCOUNT_ID,
    users::QAUL_USERS,
};
use matrix_sdk::{
    room::Room,
    ruma::{
        events::{room::message::MessageEventContent, AnyMessageEventContent},
        RoomId,
    },
};
use mime::{self, Mime, STAR_STAR};
use prost::Message;
use std::{
    collections::HashMap,
    fmt,
    fs::{self, File},
    path::PathBuf,
};
use tokio::runtime::Runtime;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chat.rs");
}
mod proto_message {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.net.messaging.rs");
}
mod proto_group {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.net.group.rs");
}
mod proto_file {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.net.chatfile.rs");
}

/// chat module function handling
pub struct Chat {}

impl Chat {

    /// Convert Group ID from String to Binary
    pub fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
        // check length
        if id.len() < 52 {
            return Err("Group ID not long enough".to_string());
        }

        // convert input
        match bs58::decode(id).into_vec() {
            Ok(id_bin) => Ok(id_bin),
            Err(e) => {
                let err = fmt::format(format_args!("{}", e));
                Err(err)
            }
        }
    }

    /// Convert Group ID from String to Binary
    pub fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
        match uuid::Uuid::parse_str(id_str.as_str()) {
            Ok(id) => Ok(id.as_bytes().to_vec()),
            _ => Err("invalid group id".to_string()),
        }
    }

    /// Request chat conversation via rpc
    ///
    /// This provides all chat messages of a specific conversation.
    /// The conversation is addressed via it's group id
    pub fn request_chat_conversation(group_id: Vec<u8>, last_index: u64) {
        // create feed list request message
        let proto_message = proto::Chat {
            message: Some(proto::chat::Message::ConversationRequest(
                proto::ChatConversationRequest {
                    group_id,
                    last_index,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Chat.into(), "".to_string());
    }

    fn analyze_content(
        message: &proto::ChatMessage,
        room_id: &RoomId,
    ) -> Result<Vec<String>, String> {
        loop {
            match proto::MessageStatus::from_i32(message.status).unwrap() {
                proto::MessageStatus::Sending => print!("Sending Qaul Message"),
                proto::MessageStatus::Sent => print!("Sent Qaul Message"),
                proto::MessageStatus::Confirmed => print!("Confirmed"),
                proto::MessageStatus::ConfirmedByAll => print!("Confirming Qaul Message"),
                proto::MessageStatus::Receiving => print!("Receiving Qaul Message"),
                proto::MessageStatus::Received => break,
            }
        }
        println!("Message Received from Qaul");
        let content: &Vec<u8> = &message.content;
        let mut res: Vec<String> = vec![];

        if let Ok(content_message) = proto::ChatContentMessage::decode(&content[..]) {
            match content_message.message {
                Some(proto::chat_content_message::Message::ChatContent(chat_content)) => {
                    res.push(chat_content.text);
                    return Ok(res);
                }

                Some(proto::chat_content_message::Message::FileContent(file_content)) => {
                    let mut file_path = String::new();
                    file_path.push_str(&file_content.file_id.to_string());
                    file_path.push('.');
                    file_path.push_str(&file_content.file_extension.to_string());
                    println!("Qaul->Matrix FilePath : {}", file_path);
                    let extension = file_content.file_extension.to_string();
                    let file_name = file_content.file_name;
                    send_file_to_matrix(file_path, room_id, extension, file_name);
                    return Ok(res);
                }

                Some(proto::chat_content_message::Message::GroupEvent(group_event)) => {
                    match proto::GroupEventType::from_i32(group_event.event_type).unwrap() {
                        proto::GroupEventType::Joined => {
                            res.push(
                                "New user joined group, user id: ".to_string()
                                    + bs58::encode(group_event.user_id).into_string().as_str(),
                            );
                            return Ok(res);
                        }
                        proto::GroupEventType::Left => {
                            res.push(
                                "User left group, user id: ".to_string()
                                    + bs58::encode(group_event.user_id).into_string().as_str(),
                            );
                            return Ok(res);
                        }
                        proto::GroupEventType::Removed => {
                            res.push("You have been removed from the group".to_string());
                            return Ok(res);
                        }
                        proto::GroupEventType::Created => {
                            res.push("You created this group".to_string());
                            return Ok(res);
                        }
                        proto::GroupEventType::InviteAccepted => {
                            res.push("You accepted the invitation".to_string());
                            return Ok(res);
                        }
                        _ => {}
                    }
                }
                None => {}
            }
        }

        Err("content decoding error".to_string())
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the feed module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Chat::decode(&data[..]) {
            Ok(chat) => {
                match chat.message {
                    Some(proto::chat::Message::ConversationList(proto_conversation)) => {
                        // Conversation table
                        let group_id =
                            uuid::Uuid::from_bytes(proto_conversation.group_id.try_into().unwrap());
                        let mut config = MATRIX_CONFIG.get().write().unwrap();
                        if !config.room_map.contains_key(&group_id) {
                            println!("No Mapping found");
                        } else {
                            let matrix_room = config.room_map.get_mut(&group_id).unwrap();
                            let last_index_grp = matrix_room.last_index;
                            let room_id = matrix_room.clone().matrix_room_id;
                            for message in proto_conversation.message_list {
                                if message.index > last_index_grp {
                                    if let Ok(ss) = Self::analyze_content(&message, &room_id) {
                                        print! {"{} | ", message.index};
                                        // message.sender_id is same as user.id
                                        match proto::MessageStatus::from_i32(message.status)
                                            .unwrap()
                                        {
                                            proto::MessageStatus::Sending => print!(".. | "),
                                            proto::MessageStatus::Sent => print!("✓. | "),
                                            proto::MessageStatus::Confirmed => print!("✓✓ | "),
                                            proto::MessageStatus::ConfirmedByAll => print!("✓✓✓| "),
                                            proto::MessageStatus::Receiving => print!("🚚 | "),
                                            proto::MessageStatus::Received => print!("📨 | "),
                                        }

                                        print!("{} | ", message.sent_at);
                                        let users = QAUL_USERS.get().read().unwrap();
                                        println!("{:#?}", users);
                                        let sender_id =
                                            bs58::encode(message.sender_id).into_string();
                                        println!("{}", sender_id);
                                        let user_name =
                                            Self::find_user_for_given_id(users.clone(), sender_id)
                                                .unwrap();
                                        println!(
                                            " [{}] {}",
                                            bs58::encode(message.message_id).into_string(),
                                            message.received_at
                                        );

                                        for s in ss {
                                            // This part is mapped with the matrix room.
                                            // Allow inviting the users or removing them.
                                            Self::matrix_send(&s, &room_id, user_name.clone());
                                            println!("\t{}", s);
                                        }
                                        println!("");
                                        matrix_room.update_last_index(message.index);
                                    }
                                }
                            }
                            MatrixConfiguration::save(config.clone());
                        }
                    }

                    _ => {
                        log::error!("unprocessable RPC chat message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    fn matrix_send(message: &String, room_id: &RoomId, user: String) {
        // Get the Room based on RoomID from the client information
        let matrix_client = MATRIX_CLIENT.get();
        let room = matrix_client.get_room(&room_id).unwrap();
        // Check if the room is already joined or not
        if let Room::Joined(room) = room {
            // Build the message content to send to matrix
            let final_msg = format!("{} : {}", user, message);
            let content =
                AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(final_msg));

            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                // Sends messages into the matrix room
                room.send(content, None).await.unwrap();
            });
        }
    }

    pub fn find_user_for_given_id(map: HashMap<String, String>, value: String) -> Option<String> {
        for (key, val) in map {
            if val == value {
                return Some(key);
            }
        }
        None
    }
}

fn send_file_to_matrix(file_path: String, room_id: &RoomId, extension: String, file_name: String) {
    let path = std::env::current_dir().unwrap();
    let mut storage_path = path.as_path().to_str().unwrap().to_string();
    let user = BOT_USER_ACCOUNT_ID.get();
    storage_path.push_str(&format!("/{}", user));
    storage_path.push_str(&format!("/files/{}", file_path));

    let matrix_client = MATRIX_CLIENT.get();
    let room = matrix_client.get_room(&room_id).unwrap();
    if let Room::Joined(room) = room {
        // Build the message content to send to matrix
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // Sends messages into the matrix room
            println!("{}", storage_path);
            let file_buff = PathBuf::from(storage_path.clone());
            // TODO : Always check for the
            let mut buff = File::open(file_buff).unwrap();
            let mut content_type: &Mime = &STAR_STAR;
            println!("{}", extension);
            match extension.as_str() {
                "jpg" | "png" | "jpeg" | "gif" | "bmp" | "svg" => content_type = &mime::IMAGE_STAR,
                "pdf" => content_type = &mime::APPLICATION_PDF,
                _ => println!("Please raise a github ticket since we don't allow this file-type."),
            }
            room.send_attachment(&file_name, content_type, &mut buff, None)
                .await
                .unwrap();
        });
        // Delete the file from bot server.
        println!("Deleting file from : {}", storage_path);
        fs::remove_file(storage_path).expect("could not remove file");
    };
}
