// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Module Interactions
//!
//! Request, display and send chat messages from CLI

use super::rpc::Rpc;
use prost::Message;
use std::fmt;

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
    /// CLI command interpretation
    ///
    /// The CLI commands of feed module are processed here
    pub fn cli(command: &str) {
        match command {
            // send chat message
            cmd if cmd.starts_with("send ") => {
                // get conversation id
                let command_string = cmd.strip_prefix("send ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(conversation_id_str) = iter.next() {
                    // convert conversation id from string to binary version
                    let mut conversation_id = vec![];
                    match Self::id_string_to_bin(conversation_id_str.to_string()) {
                        Ok(v) => {
                            conversation_id = v.clone();
                        }
                        _ => match Self::uuid_string_to_bin(conversation_id_str.to_string()) {
                            Ok(v) => {
                                conversation_id = v.clone();
                            }
                            _ => {
                                log::error!("invalid conversation id format");
                            }
                        },
                    }
                    // get message string
                    if let Some(message) = command_string.strip_prefix(conversation_id_str) {
                        // send message
                        Self::send_chat_message(
                            conversation_id,
                            message.to_string().trim().to_string(),
                        );
                        println!("chat message sent [{}] {}", conversation_id_str, message);
                        return;
                    } else {
                        log::error!("prefix '{}' not found", conversation_id_str);
                        return;
                    }
                } else {
                    log::error!("chat send command incorrectly formatted");
                }
            }
            // request chat overview
            cmd if cmd.starts_with("overview") => {
                match cmd.strip_prefix("overview ") {
                    Some(index_str) => {
                        if let Ok(index) = index_str.parse::<u64>() {
                            // request chat overview
                            Self::request_chat_overview(index);
                        } else {
                            log::error!("chat overview index is not a valid number");
                        }
                    }
                    None => {
                        // request all messages
                        Self::request_chat_overview(0);
                    }
                }
            }
            // request chat conversation
            cmd if cmd.starts_with("conversation") => {
                match cmd.strip_prefix("conversation ") {
                    Some(command_str) => {
                        let command_string = command_str.to_string();
                        let mut iter = command_string.split_whitespace();
                        let mut conversation_id = Vec::new();
                        let mut last_index = 0;

                        // convert conversation id from string to binary version
                        if let Some(conversation_id_str) = iter.next() {
                            match Self::id_string_to_bin(conversation_id_str.to_string()) {
                                Ok(id) => {
                                    conversation_id = id;
                                }
                                Err(_e) => {
                                    match Self::uuid_string_to_bin(conversation_id_str.to_string())
                                    {
                                        Ok(id) => {
                                            conversation_id = id;
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
                        Self::request_chat_conversation(conversation_id, last_index);
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

    /// Convert Conversation ID from String to Binary
    fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
        // check length
        if id.len() < 52 {
            return Err("Conversation ID not long enough".to_string());
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

    /// Convert Conversation ID from String to Binary
    fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
        match uuid::Uuid::parse_str(id_str.as_str()) {
            Ok(id) => Ok(id.as_bytes().to_vec()),
            _ => Err("invalid group id".to_string()),
        }
    }

    /// Create and send feed message via rpc
    fn send_chat_message(conversation_id: Vec<u8>, message_text: String) {
        // create feed send message
        let proto_message = proto::Chat {
            message: Some(proto::chat::Message::Send(proto::ChatMessageSend {
                conversation_id,
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

    /// Request chat overview via rpc
    ///
    /// Provides an overview over all conversations with the amount
    /// of unread messages, and the last message.
    fn request_chat_overview(_last_index: u64) {
        // create feed list request message
        let proto_message = proto::Chat {
            message: Some(proto::chat::Message::OverviewRequest(
                proto::ChatOverviewRequest {},
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

    /// Request chat conversation via rpc
    ///
    /// This provides all chat messages of a specific conversation.
    /// The conversation is addressed via it's conversation id
    fn request_chat_conversation(conversation_id: Vec<u8>, last_index: u64) {
        // create feed list request message
        let proto_message = proto::Chat {
            message: Some(proto::chat::Message::ConversationRequest(
                proto::ChatConversationRequest {
                    conversation_id,
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
        status: proto::MessageStatus,
        content_type: i32,
        content: &Vec<u8>,
    ) -> Result<Vec<String>, String> {
        let mut res: Vec<String> = vec![];
        let tp;
        match proto::ChatContentType::from_i32(content_type) {
            Some(v) => tp = v,
            _ => {
                return Err("".to_string());
            }
        }

        match tp {
            proto::ChatContentType::None => {}
            proto::ChatContentType::Chat => {
                if let Ok(v) = String::decode(&content[..]) {
                    res.push(v.clone());
                    return Ok(res);
                }
            }
            proto::ChatContentType::File => {
                if let Ok(v) = proto_file::ChatFileContainer::decode(&content[..]) {
                    match v.message {
                        Some(proto_file::chat_file_container::Message::FileInfo(file_info)) => {
                            res.push(
                                "file transfer id: ".to_string()
                                    + file_info.file_id.to_string().as_str(),
                            );
                            res.push(
                                " name: ".to_string()
                                    + file_info.file_name.as_str()
                                    + " size: "
                                    + file_info.file_size.to_string().as_str(),
                            );
                            return Ok(res);
                        }
                        _ => {}
                    }
                }
            }
            // REMOVE
            /*proto::ChatContentType::Group => {
                if let Ok(v) = proto_group::GroupContainer::decode(&content[..]) {
                    match v.message {
                        Some(proto_group::group_container::Message::InviteMember(invite)) => {
                            let group_id =
                                uuid::Uuid::from_bytes(invite.group_id.try_into().unwrap());
                            if status == proto::MessageStatus::Sending
                                || status == proto::MessageStatus::Sent
                            {
                                res.push(
                                    "Sent group invite group id: ".to_string()
                                        + group_id.to_string().as_str(),
                                );
                            } else {
                                res.push(
                                    "Received group invite group id: ".to_string()
                                        + group_id.to_string().as_str(),
                                );
                            }
                            res.push(
                                "name: ".to_string()
                                    + invite.group_name.as_str()
                                    + " members: "
                                    + invite.members_count.to_string().as_str(),
                            );
                            return Ok(res);
                        }
                        Some(proto_group::group_container::Message::ReplyInvite(reply_invite)) => {
                            let group_id =
                                uuid::Uuid::from_bytes(reply_invite.group_id.try_into().unwrap());
                            if status == proto::MessageStatus::Sending
                                || status == proto::MessageStatus::Sent
                            {
                                res.push(
                                    "Sent group accept group id: ".to_string()
                                        + group_id.to_string().as_str(),
                                );
                            } else {
                                res.push(
                                    "Received group accept group id: ".to_string()
                                        + group_id.to_string().as_str(),
                                );
                            }
                            return Ok(res);
                        }
                        Some(proto_group::group_container::Message::Removed(_removed)) => {}
                        _ => {}
                    }
                }
            }*/
            proto::ChatContentType::Group => {
                if let Ok(v) = proto::GroupEvent::decode(&content[..]) {
                    match proto::GroupEventType::from_i32(v.event_type).unwrap() {
                        proto::GroupEventType::Joined => {
                            res.push(
                                "New user joined group, user id: ".to_string()
                                    + bs58::encode(v.user_id).into_string().as_str(),
                            );
                            return Ok(res);
                        }
                        proto::GroupEventType::Left => {
                            res.push(
                                "User left group, user id: ".to_string()
                                    + bs58::encode(v.user_id).into_string().as_str(),
                            );
                            return Ok(res);
                        }
                        _ => {}
                    }
                }
            }
            proto::ChatContentType::Rtc => {}
        }
        Err("".to_string())
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the feed module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Chat::decode(&data[..]) {
            Ok(chat) => {
                match chat.message {
                    Some(proto::chat::Message::OverviewList(proto_overview)) => {
                        // Overview table
                        println!("");
                        println!("Conversations Overview");
                        println!("");
                        println!("[ CONVERSATION ID ] CONVERSATION NAME");
                        println!("  Unread Messages | Last Message Index | Last Timestamp | Last Message Content");
                        println!("");

                        // print all messages in the feed list
                        for message in proto_overview.overview_list {
                            println! {"[ {} ] {}", bs58::encode(message.conversation_id).into_string(), message.name};
                            print!("  {} | ", message.unread);
                            print!("{} | ", message.last_message_index);
                            print!("{} | ", message.last_message_at);
                            if let Ok(ss) = Self::analyze_content(
                                proto::MessageStatus::Sent,
                                message.content_type,
                                &message.content,
                            ) {
                                for s in ss {
                                    println!("\t{}", s);
                                }
                            }
                            println!("");
                        }
                    }
                    Some(proto::chat::Message::ConversationList(proto_conversation)) => {
                        // Conversation table
                        println!("");
                        let conversation_id = uuid::Uuid::from_bytes(
                            proto_conversation.conversation_id.try_into().unwrap(),
                        );

                        println!("Conversation [ {} ]", conversation_id.to_string());
                        println!("");
                        println!("No. | Status | Sent At | Sender ID");
                        println!("  [Message ID] Received At");
                        println!("  Message Content");
                        println!("");

                        // print all messages in the feed list
                        for message in proto_conversation.message_list {
                            if let Ok(ss) = Self::analyze_content(
                                proto::MessageStatus::from_i32(message.status).unwrap(),
                                message.content_type,
                                &message.content,
                            ) {
                                print! {"{} | ", message.index};
                                match message.status {
                                    0 => print!(".. | "),
                                    1 => print!("✓. | "),
                                    2 => print!("✓✓ | "),
                                    _ => print!("   | "),
                                }

                                print!("{} | ", message.sent_at);
                                println!("{}", bs58::encode(message.sender_id).into_string());
                                println!(
                                    " [{}] {}",
                                    bs58::encode(message.message_id).into_string(),
                                    message.received_at
                                );

                                for s in ss {
                                    println!("\t{}", s);
                                }
                                println!("");
                            }
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
}
