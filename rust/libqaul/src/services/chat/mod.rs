// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Module
//!
//! Contains the chat conversations

use libp2p::PeerId;
use prost::Message;

pub mod file;
pub mod message;
pub mod storage;

use crate::connections::{internet::Internet, lan::Lan};
use crate::node::user_accounts::UserAccounts;
use crate::rpc::Rpc;
pub use file::ChatFile;
pub use message::ChatMessage;
pub use storage::ChatStorage;

/// Import protobuf message definition generated by
/// the rust module prost-build.
pub mod rpc_proto {
    include!("qaul.rpc.chat.rs");
}
use super::group;

/// qaul Chat module general logic
pub struct Chat {}

impl Chat {
    /// initialize chat module
    pub fn init() {
        // initialize the chat storage
        ChatStorage::init();

        // initialize the chat file management
        ChatFile::init();
    }

    /// Generate a Chat Message ID
    ///
    /// Every chat message has a unique message id.
    ///
    /// This ID is created from the following things
    ///
    /// * the conversation_id (group_id)
    /// * the sender id
    /// * the message index of the sender
    pub fn generate_message_id(group_id: &Vec<u8>, sender_id: &PeerId, index: u32) -> Vec<u8> {
        let group_crc = crc::crc64::checksum_iso(group_id);
        let sender_crc = crc::crc64::checksum_iso(&sender_id.to_bytes());
        let mut buff0 = group_crc.to_be_bytes().to_vec();
        let mut buff = sender_crc.to_be_bytes().to_vec();
        let mut index_bytes = index.to_be_bytes().to_vec();

        buff0.append(&mut buff);
        buff0.append(&mut index_bytes);
        buff0
    }

    /// Process incoming RPC request messages for chat module
    pub fn rpc(
        data: Vec<u8>,
        user_id: Vec<u8>,
        _lan: Option<&mut Lan>,
        _internet: Option<&mut Internet>,
    ) {
        let account_id = PeerId::from_bytes(&user_id).unwrap();

        match rpc_proto::Chat::decode(&data[..]) {
            Ok(chat) => {
                match chat.message {
                    Some(rpc_proto::chat::Message::ConversationRequest(conversation_request)) => {
                        // get messages of a conversation from data base
                        let conversation_list = ChatStorage::get_messages(
                            account_id,
                            conversation_request.conversation_id,
                        );

                        // pack message
                        let proto_message = rpc_proto::Chat {
                            message: Some(rpc_proto::chat::Message::ConversationList(
                                conversation_list,
                            )),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");
                        Rpc::send_message(
                            buf,
                            crate::rpc::proto::Modules::Chat.into(),
                            "".to_string(),
                            Vec::new(),
                        );
                        // send messageproto::Container, "".to_string(), Vec::new() );
                    }
                    Some(rpc_proto::chat::Message::Send(message)) => {
                        // print message
                        log::info!("sending chat message: {}", message.content.clone());

                        // get user account from user_id
                        let user_account;
                        match PeerId::from_bytes(&user_id) {
                            Ok(user_id_decoded) => match UserAccounts::get_by_id(user_id_decoded) {
                                Some(account) => {
                                    user_account = account;
                                }
                                None => {
                                    log::error!(
                                        "user account id not found: {:?}",
                                        user_id_decoded.to_base58()
                                    );
                                    return;
                                }
                            },
                            Err(e) => {
                                log::error!("user account id could'nt be encoded: {:?}", e);
                                return;
                            }
                        }

                        // send the message
                        if let Err(error) = ChatMessage::send_chat_message(
                            &user_account.id,
                            &message.conversation_id,
                            message.content,
                        ) {
                            log::error!("Outgoing chat message error: {}", error)
                        }
                    }
                    _ => {
                        log::error!("Unhandled Protobuf Chat Message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
