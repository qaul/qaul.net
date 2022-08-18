// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Messaging process
//!
//! The messaging service is used for sending, receiving and

use libp2p::PeerId;
use prost::Message;

use crate::router;

use crate::services::chat;
use crate::services::crypto::Crypto;
use crate::services::filesharing;
use crate::services::group;
use crate::services::rtc;

use super::ConversationId;

/// Qaul Messaging Structure
pub struct MessagingProcess {}

impl MessagingProcess {
    /// process direct message from network
    pub fn on_direct_message(
        sender_id: &PeerId,
        receiver_id: &PeerId,
        data: &Vec<u8>,
        signature: &Vec<u8>,
    ) {
        // decode mesaging
        let messaging;
        match super::proto::Messaging::decode(&data[..]) {
            Ok(v) => {
                messaging = v;
            }
            _ => {
                log::error!(
                    "Error decoding Messaging Message {} from {} to {}",
                    bs58::encode(signature).into_string(),
                    sender_id.to_base58(),
                    receiver_id.to_base58()
                );
                return;
            }
        }

        match messaging.message {
            Some(super::proto::messaging::Message::ConfirmationMessage(confirm)) => {
                //update unconfirmed_table
                if let Some(msg_id) = super::Messaging::on_confirmed_message(&confirm.signature) {
                    //update chat table received_at and state
                    chat::Chat::update_confirmation(receiver_id, &msg_id, confirm.received_at);
                }
            }
            Some(super::proto::messaging::Message::CryptoService(_crypto)) => {}
            Some(super::proto::messaging::Message::RtcStreamMessage(_rtc_stream)) => {}
            Some(super::proto::messaging::Message::GroupNotifyMessage(group_notify)) => {
                group::Group::on_notify(sender_id, receiver_id, &group_notify.content);
            }
            Some(super::proto::messaging::Message::CommonMessage(common)) => {
                // check converssation id
                let conversation_id;
                match ConversationId::from_bytes(&common.conversation_id) {
                    Ok(v) => {
                        conversation_id = v;
                    }
                    _ => {
                        log::error!("invalid conversation id");
                        return;
                    }
                }

                log::error!(
                    "common msg_id={}, conversation_id={}",
                    bs58::encode(common.message_id.clone()).into_string(),
                    conversation_id.to_base58()
                );

                match common.payload {
                    Some(super::proto::common_message::Payload::ChatMessage(ref chat_message)) => {
                        chat::Chat::save_incoming_message(
                            receiver_id,
                            sender_id,
                            chat::rpc_proto::ContentType::Chat.try_into().unwrap(),
                            &chat_message.content.encode_to_vec(),
                            common.sent_at,
                            &conversation_id,
                            &common.message_id,
                            chat::rpc_proto::MessageStatus::Received,
                        );
                    }
                    Some(super::proto::common_message::Payload::FileMessage(ref file_message)) => {
                        chat::Chat::save_incoming_message(
                            receiver_id,
                            sender_id,
                            chat::rpc_proto::ContentType::File.try_into().unwrap(),
                            &file_message.content,
                            common.sent_at,
                            &conversation_id,
                            &common.message_id,
                            chat::rpc_proto::MessageStatus::Received,
                        );
                        filesharing::FileShare::net(
                            &sender_id,
                            &receiver_id,
                            &common.conversation_id,
                            &file_message.content,
                        );
                    }
                    Some(super::proto::common_message::Payload::GroupMessage(
                        ref group_message,
                    )) => {
                        group::Group::net(&sender_id, &receiver_id, &group_message.content);

                        chat::Chat::save_incoming_message(
                            receiver_id,
                            sender_id,
                            chat::rpc_proto::ContentType::Group.try_into().unwrap(),
                            &group_message.content,
                            common.sent_at,
                            &conversation_id,
                            &common.message_id,
                            chat::rpc_proto::MessageStatus::Received,
                        );
                    }
                    Some(super::proto::common_message::Payload::RtcMessage(ref rtc_message)) => {
                        chat::Chat::save_incoming_message(
                            receiver_id,
                            sender_id,
                            chat::rpc_proto::ContentType::Rtc.try_into().unwrap(),
                            &rtc_message.content,
                            common.sent_at,
                            &conversation_id,
                            &common.message_id,
                            chat::rpc_proto::MessageStatus::Received,
                        );
                    }
                    _ => {
                        log::error!("process_direct_message: unknown common message type");
                        return;
                    }
                }

                log::error!(
                    "sender={}, receiver={}",
                    sender_id.to_base58(),
                    receiver_id.to_base58()
                );

                //update group status
                if let Err(e) = group::GroupMessage::on_message(
                    sender_id,
                    receiver_id,
                    &conversation_id.to_bytes(),
                    &common.message_id,
                ) {
                    log::error!("group stattus processing error {}", e);
                }

                // send confirm message
                if let Err(e) =
                    super::Messaging::send_confirmation(receiver_id, sender_id, signature)
                {
                    log::error!("send confirmation failed {}", e);
                }
            }
            _ => {
                log::error!("process_direct_message: unknown message type");
                return;
            }
        }
    }

    /// process received message
    pub fn process_received_message(container: super::proto::Container) {
        // check envelop
        let envelope;
        match container.envelope {
            Some(v) => {
                envelope = v;
            }
            _ => {
                log::error!("No Envelope in Message Container");
                return;
            }
        }

        // check sender_id
        let sender_id;
        match PeerId::from_bytes(&envelope.sender_id) {
            Ok(v) => {
                sender_id = v;
            }
            _ => {
                log::error!("Error retrieving PeerId");
                return;
            }
        }

        // check key
        let key;
        match router::users::Users::get_pub_key(&sender_id) {
            Some(v) => {
                key = v;
            }
            _ => {
                log::error!("No key found for user {}", sender_id.to_base58());
                return;
            }
        }

        // verify sign
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        envelope
            .encode(&mut envelope_buf)
            .expect("Vec<u8> provides capacity as needed");
        if !key.verify(&envelope_buf, &container.signature) {
            log::error!("verification failed");
            return;
        }

        // check receiver_id
        let receiver_id;
        match PeerId::from_bytes(&envelope.receiver_id) {
            Ok(v) => {
                receiver_id = v;
            }
            _ => {
                log::error!("Error retrieving PeerId");
                return;
            }
        }

        match super::proto::EnvelopPayload::decode(&envelope.payload[..]) {
            Ok(payload) => {
                match payload.payload {
                    Some(super::proto::envelop_payload::Payload::Encrypted(encrypted)) => {
                        // decrypt data
                        // let mut decrypted: Vec<u8> = Vec::new();
                        // for data_message in encrypted.data {
                        //     if let Some(mut decrypted_chunk) = Crypto::decrypt(data_message.data, data_message.nonce, receiver_id, sender_id.clone()) {
                        //         decrypted.append(&mut decrypted_chunk);
                        //     }
                        //     else {
                        //         log::error!("decryption error");
                        //         return;
                        //     }
                        // }
                        // Self::on_direct_message(&sender_id, &receiver_id, &decrypted, &container.signature);
                        Self::on_direct_message(
                            &sender_id,
                            &receiver_id,
                            &encrypted.data,
                            &container.signature,
                        );
                    }
                    Some(super::proto::envelop_payload::Payload::Dtn(_dtn)) => {}
                    _ => {
                        log::error!("unknown envelop payload");
                        return;
                    }
                }
            }
            _ => {
                log::error!("envelop payload decode error");
                return;
            }
        }
    }
}
