// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Module Interactions
//!
//! Request, display and send chat messages from CLI

use crate::{
    configuration::MatrixConfiguration,
    relay_bot::{MATRIX_CLIENT, MATRIX_CONFIG},
    users::QAUL_USERS,
};

use super::rpc::Rpc;
use matrix_sdk::{
    room::Room,
    ruma::{
        events::{room::message::MessageEventContent, AnyMessageEventContent},
        RoomId,
    },
};
use prost::Message;
use std::{collections::HashMap, fmt};
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
pub struct QaulMenu {}

impl Chat {
    /// CLI command interpretation
    ///
    /// The CLI commands of feed module are processed here
    pub fn cli(command: &str) {
        match command {
            // send chat message
            cmd if cmd.starts_with("send ") => {
                // get group id
                let command_string = cmd.strip_prefix("send ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    // convert group id from string to binary version
                    let mut group_id = vec![];
                    match Self::id_string_to_bin(group_id_str.to_string()) {
                        Ok(v) => {
                            group_id = v.clone();
                        }
                        _ => match Self::uuid_string_to_bin(group_id_str.to_string()) {
                            Ok(v) => {
                                group_id = v.clone();
                            }
                            _ => {
                                log::error!("invalid group id format");
                            }
                        },
                    }
                    // get message string
                    if let Some(message) = command_string.strip_prefix(group_id_str) {
                        // send message
                        Self::send_chat_message(group_id, message.to_string().trim().to_string());
                        println!("chat message sent [{}] {}", group_id_str, message);
                        return;
                    } else {
                        log::error!("prefix '{}' not found", group_id_str);
                        return;
                    }
                } else {
                    log::error!("chat send command incorrectly formatted");
                }
            }
            // request chat conversation
            cmd if cmd.starts_with("conversation") => {
                match cmd.strip_prefix("conversation ") {
                    Some(command_str) => {
                        let command_string = command_str.to_string();
                        let mut iter = command_string.split_whitespace();
                        let mut group_id = Vec::new();
                        let mut last_index = 0;

                        // convert group id from string to binary version
                        if let Some(group_id_str) = iter.next() {
                            match Self::id_string_to_bin(group_id_str.to_string()) {
                                Ok(id) => {
                                    group_id = id;
                                }
                                Err(_e) => {
                                    match Self::uuid_string_to_bin(group_id_str.to_string()) {
                                        Ok(id) => {
                                            group_id = id;
                                        }
                                        _ => {
                                            log::error!("invalid converstion id");
                                            return;
                                        }
                                    }
                                }
                            }
                        }

                        // convert last_received index string to number
                        if let Some(index_str) = iter.next() {
                            // option: get last_received
                            if let Ok(index) = index_str.parse::<u64>() {
                                last_index = index;
                            } else {
                                log::error!("chat conversation index is not a valid number");
                                return;
                            }
                        }

                        // request chat conversation
                        Self::request_chat_conversation(group_id, last_index);
                    }
                    None => {
                        // request all messages
                        log::error!("chat conversation command not correctly formatted");
                    }
                }
            }
            // unknown command
            _ => log::error!("unknown chat command"),
        }
    }

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

    /// Create and send feed message via rpc
    fn send_chat_message(group_id: Vec<u8>, message_text: String) {
        // create feed send message
        let proto_message = proto::Chat {
            message: Some(proto::chat::Message::Send(proto::ChatMessageSend {
                group_id,
                content: message_text,
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Chat.into(), "".to_string());
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

    fn analyze_content(content: &Vec<u8>) -> Result<Vec<String>, String> {
        let mut res: Vec<String> = vec![];

        if let Ok(content_message) = proto::ChatContentMessage::decode(&content[..]) {
            match content_message.message {
                Some(proto::chat_content_message::Message::ChatContent(chat_content)) => {
                    res.push(chat_content.text);
                    return Ok(res);
                }
                Some(proto::chat_content_message::Message::FileContent(file_content)) => {
                    res.push(
                        "file transfer id: ".to_string()
                            + file_content.file_id.to_string().as_str(),
                    );
                    res.push(
                        " name: ".to_string()
                            + file_content.file_name.as_str()
                            + " size: "
                            + file_content.file_size.to_string().as_str(),
                    );
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
                        let group_id_byte = proto_conversation.clone().group_id;
                        let group_id =
                            uuid::Uuid::from_bytes(proto_conversation.group_id.try_into().unwrap());

                        // println!("Conversation [ {} ]", group_id.to_string());
                        // println!("");
                        // println!("No. | Status | Sent At | Sender ID");
                        // println!("  [Message ID] Received At");
                        // println!("  Message Content");
                        // println!("");
                        let mut config = MATRIX_CONFIG.get().write().unwrap();
                        if !config.room_map.contains_key(&group_id) {
                            // print all messages in the group list
                            for message in proto_conversation.message_list {
                                if let Ok(ss) = Self::analyze_content(&message.content) {
                                    print! {"{} | ", message.index};
                                    match proto::MessageStatus::from_i32(message.status).unwrap() {
                                        proto::MessageStatus::Sending => print!(".. | "),
                                        proto::MessageStatus::Sent => print!("✓. | "),
                                        proto::MessageStatus::Confirmed => print!("✓✓ | "),
                                        proto::MessageStatus::ConfirmedByAll => print!("✓✓✓| "),
                                        proto::MessageStatus::Receiving => print!("🚚 | "),
                                        proto::MessageStatus::Received => print!("📨 | "),
                                    }

                                    print!("{} | ", message.sent_at);
                                    println!("{}", bs58::encode(message.sender_id).into_string());
                                    println!(
                                        " [{}] {}",
                                        bs58::encode(message.message_id).into_string(),
                                        message.received_at
                                    );

                                    for s in ss {
                                        // This part does not have any mapping with matrix room.
                                        // Show and navigate with help command.
                                        QaulMenu::help(group_id_byte.clone());
                                        println!("\t{}", s);
                                    }
                                    println!("");
                                }
                            }
                        } else {
                            let matrix_room = config.room_map.get_mut(&group_id).unwrap();
                            let last_index_grp = matrix_room.last_index;
                            let room_id = matrix_room.clone().matrix_room_id;
                            for message in proto_conversation.message_list {
                                if let Ok(ss) = Self::analyze_content(&message.content) {
                                    if message.index > last_index_grp {
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
                                        let users = QAUL_USERS.get();
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

impl QaulMenu {
    pub fn help(group_id: Vec<u8>) {
        let msg = format!("/users : To know all the existing users in matrix room.\n/invite : To invite users into the existing room.");
        Chat::send_chat_message(group_id, msg);
        //
    }
}
