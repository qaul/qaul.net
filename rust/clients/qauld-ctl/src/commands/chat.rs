use crate::{
    cli::ChatSubcmd,
    commands::{id_string_to_bin, uuid_string_to_bin, RpcCommand},
    proto::Modules,
};
use prost::Message;
use uuid::Uuid;

/// protobuf RPC definition
use qaul_proto::qaul_rpc_chat as proto;

use proto::{
    chat, chat_content_message, Chat, ChatContentMessage, ChatConversationRequest, ChatMessageSend,
    GroupEventType, MessageStatus,
};

impl ChatSubcmd {
    fn analyze_content(&self, content: &Vec<u8>) -> Result<Vec<String>, String> {
        let mut res: Vec<String> = vec![];

        if let Ok(content_message) = ChatContentMessage::decode(&content[..]) {
            match content_message.message {
                Some(chat_content_message::Message::ChatContent(chat_content)) => {
                    res.push(chat_content.text);
                    return Ok(res);
                }
                Some(chat_content_message::Message::FileContent(file_content)) => {
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
                Some(chat_content_message::Message::GroupEvent(group_event)) => {
                    match GroupEventType::try_from(group_event.event_type) {
                        Ok(GroupEventType::Joined) => {
                            res.push(
                                "New user joined group, user id: ".to_string()
                                    + bs58::encode(group_event.user_id).into_string().as_str(),
                            );
                            return Ok(res);
                        }
                        Ok(GroupEventType::Left) => {
                            res.push(
                                "User left group, user id: ".to_string()
                                    + bs58::encode(group_event.user_id).into_string().as_str(),
                            );
                            return Ok(res);
                        }
                        Ok(GroupEventType::Removed) => {
                            res.push("You have been removed from the group".to_string());
                            return Ok(res);
                        }
                        Ok(GroupEventType::Created) => {
                            res.push("You created this group".to_string());
                            return Ok(res);
                        }
                        Ok(GroupEventType::InviteAccepted) => {
                            res.push("You accepted the invitation".to_string());
                            return Ok(res);
                        }
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                None => {}
            }
        }

        Err("content decoding error".to_string())
    }
}

impl RpcCommand for ChatSubcmd {
    fn expects_response(&self) -> bool {
        !matches!(&self, ChatSubcmd::Send { .. })
    }

    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match &self {
            ChatSubcmd::Send { message, group_id } => {
                let group_id_bytes = id_string_to_bin(group_id.to_string())
                    .or(uuid_string_to_bin(group_id.to_string()))?;

                let proto_message = Chat {
                    message: Some(chat::Message::Send(ChatMessageSend {
                        group_id: group_id_bytes,
                        content: message.to_string(),
                    })),
                };
                Ok((proto_message.encode_to_vec(), Modules::Chat))
            }
            ChatSubcmd::Conversation { group_id, index } => {
                let group_id = id_string_to_bin(group_id.to_string())
                    .or(uuid_string_to_bin(group_id.to_string()))?;

                let proto_message = Chat {
                    message: Some(chat::Message::ConversationRequest(
                        ChatConversationRequest {
                            group_id,
                            last_index: *index,
                        },
                    )),
                };

                Ok((proto_message.encode_to_vec(), Modules::Chat))
            }
        }
    }

    fn decode_response(&self, data: &[u8], json: bool) -> Result<(), Box<dyn std::error::Error>> {
        let chat = Chat::decode(data)?;
        match chat.message {
            Some(chat::Message::ConversationList(proto_conversation)) => {
                let group_id = Uuid::from_bytes(proto_conversation.group_id.try_into().unwrap());

                if json {
                    let messages: Vec<serde_json::Value> = proto_conversation
                        .message_list
                        .iter()
                        .filter_map(|message| {
                            self.analyze_content(&message.content).ok().map(|content| {
                                let status = MessageStatus::try_from(message.status)
                                    .map(|s| format!("{:?}", s))
                                    .unwrap_or_default();
                                serde_json::json!({
                                    "index": message.index,
                                    "message_id": bs58::encode(&message.message_id).into_string(),
                                    "sender_id": bs58::encode(&message.sender_id).into_string(),
                                    "status": status,
                                    "sent_at": message.sent_at,
                                    "received_at": message.received_at,
                                    "content": content,
                                })
                            })
                        })
                        .collect();
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "group_id": group_id.to_string(),
                            "messages": messages,
                        }))?
                    );
                } else {
                    println!("");
                    println!("Conversation [ {} ]", group_id.to_string());
                    println!("");
                    println!("No. | Status | Sent At | Sender ID");
                    println!("  [Message ID] Received At");
                    println!("  Message Content");
                    println!("");

                    for message in proto_conversation.message_list {
                        if let Ok(ss) = self.analyze_content(&message.content) {
                            print! {"{} | ", message.index};
                            match MessageStatus::try_from(message.status) {
                                Ok(MessageStatus::Sending) => print!(".. | "),
                                Ok(MessageStatus::Sent) => print!("✓. | "),
                                Ok(MessageStatus::Confirmed) => print!("✓✓ | "),
                                Ok(MessageStatus::ConfirmedByAll) => print!("✓✓✓| "),
                                Ok(MessageStatus::Receiving) => print!("🚚 | "),
                                Ok(MessageStatus::Received) => print!("📨 | "),
                                Err(_) => {}
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
            }
            _ => {
                log::error!("unprocessable RPC chat message");
            }
        };
        Ok(())
    }
}
