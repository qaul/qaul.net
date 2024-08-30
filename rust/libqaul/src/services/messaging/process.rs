// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Process Received Messages
//!
//! This processes the received messages from the messaging module
//! which have reached it's destination

use libp2p::PeerId;
use prost::Message;

use crate::node::user_accounts::UserAccount;
use crate::router;
use crate::services::chat::{self, rpc_proto, ChatFile, ChatStorage};
use crate::services::crypto::sessionmanager::CryptoSessionManager;
use crate::services::crypto::Crypto;
use crate::services::dtn;
use crate::services::group::{self, Group, GroupId};
use crate::services::rtc;
use crate::utilities::timestamp::Timestamp;

/// Qaul Messaging Structure
pub struct MessagingProcess {}

impl MessagingProcess {
    /// process decrypted message
    pub fn on_decrypted_message(
        sender_id: &PeerId,
        user_account: UserAccount,
        data: &Vec<u8>,
        signature: &Vec<u8>,
    ) {
        log::trace!("on_decrypted_message arrived");

        // decode messaging
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
                    user_account.id.to_base58()
                );
                return;
            }
        }

        match messaging.message {
            Some(super::proto::messaging::Message::ConfirmationMessage(confirmation)) => {
                // process confirmation message
                super::Messaging::on_confirmed_message(
                    &confirmation.signature,
                    sender_id.to_owned(),
                    user_account,
                    confirmation.clone(),
                );
            }
            Some(super::proto::messaging::Message::CryptoService(cryptoservice)) => {
                log::trace!(
                    "received cryptoservice message from {}",
                    sender_id.clone().to_string()
                );
                // process crypto service message
                CryptoSessionManager::process_cryptoservice_container(
                    sender_id,
                    user_account.clone(),
                    cryptoservice.content,
                );

                // send confirm message
                if let Err(e) =
                    super::Messaging::send_confirmation(&user_account.id, sender_id, signature)
                {
                    log::error!("send confirmation failed {}", e);
                }
            }
            Some(super::proto::messaging::Message::DtnResponse(dtn_response)) => {
                // update DTN state
                dtn::Dtn::on_dtn_response(&dtn_response);

                // update unconfirmed table
                super::Messaging::on_confirmed_message(
                    &dtn_response.signature,
                    sender_id.to_owned(),
                    user_account,
                    super::proto::Confirmation {
                        signature: dtn_response.signature.clone(),
                        received_at: Timestamp::get_timestamp(),
                    },
                );
            }
            Some(super::proto::messaging::Message::RtcStreamMessage(_rtc_stream)) => {}
            Some(super::proto::messaging::Message::GroupInviteMessage(group_invite)) => {
                // TODO: pass on user_account
                group::Group::net(sender_id, &user_account.id, &group_invite.content);
                //group::Group::on_notify(sender_id, receiver_id, &group_notify.content);

                // send confirm message
                // TODO: pass on user_account
                if let Err(e) =
                    super::Messaging::send_confirmation(&user_account.id, sender_id, signature)
                {
                    log::error!("send confirmation failed {}", e);
                }
            }
            Some(super::proto::messaging::Message::CommonMessage(common)) => {
                // create group id
                let group_id;
                match GroupId::from_bytes(&common.group_id) {
                    Ok(v) => group_id = v,
                    _ => {
                        log::warn!("received message from {} with invalid group id", sender_id);
                        return;
                    }
                }

                // Check if group exists.
                // Reject message if one of the conditions are not true.
                let group: Group;
                match group::GroupManage::get_group_create_direct(
                    user_account.id,
                    group_id.clone(),
                    sender_id,
                ) {
                    Some(my_group) => {
                        group = my_group;

                        // Check if we & sender are members of the group.
                        if !group.are_members(&user_account.id.to_bytes(), &sender_id.to_bytes()) {
                            return;
                        }
                    }
                    None => {
                        log::warn!("received message from {} for unexisting group", sender_id);
                        return;
                    }
                }

                match common.payload {
                    Some(super::proto::common_message::Payload::ChatMessage(ref chat_message)) => {
                        // create ChatContentMessage
                        let content_message = rpc_proto::ChatContentMessage {
                            message: Some(rpc_proto::chat_content_message::Message::ChatContent(
                                rpc_proto::ChatContent {
                                    text: chat_message.content.clone(),
                                },
                            )),
                        };

                        ChatStorage::save_message(
                            &user_account.id,
                            &group_id,
                            sender_id,
                            &common.message_id,
                            common.sent_at,
                            content_message,
                            chat::rpc_proto::MessageStatus::Received,
                        );
                    }
                    Some(super::proto::common_message::Payload::FileMessage(ref file_message)) => {
                        ChatFile::process_net_chatfilecontainer(
                            sender_id.to_owned(),
                            user_account.clone(),
                            common.group_id,
                            common.message_id.clone(),
                            common.sent_at,
                            &file_message.content,
                        );
                    }
                    Some(super::proto::common_message::Payload::GroupMessage(
                        ref group_message,
                    )) => {
                        // TODO: pass on user_account
                        // process group message
                        group::Group::net(&sender_id, &user_account.id, &group_message.content);
                    }
                    Some(super::proto::common_message::Payload::RtcMessage(ref rtc_message)) => {
                        // process message in RTC module
                        rtc::Rtc::net(sender_id, &user_account.id, &rtc_message.content);
                    }
                    _ => {
                        log::error!("process_direct_message: unknown common message type");
                        return;
                    }
                }

                log::trace!(
                    "sender={}, receiver={}",
                    sender_id.to_base58(),
                    user_account.id.to_base58()
                );

                // TODO: hand over user_id
                // update group status
                if let Err(e) = group::GroupMessage::on_message(
                    sender_id,
                    &user_account.id,
                    &group_id.to_bytes(),
                    &common.message_id,
                ) {
                    log::error!("group status processing error {}", e);
                }

                // TODO: hand over user_account
                // send confirm message
                if let Err(e) =
                    super::Messaging::send_confirmation(&user_account.id, sender_id, signature)
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
    pub fn process_received_message(user_account: UserAccount, container: super::proto::Container) {
        // check envelop
        let envelope;
        match container.envelope {
            Some(v) => envelope = v,
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
                        let decrypted: Vec<u8>;
                        match Crypto::decrypt(
                            encrypted,
                            user_account.clone(),
                            sender_id.clone(),
                            &container.signature,
                        ) {
                            Some(decryption_result) => decrypted = decryption_result,
                            None => {
                                log::error!("decryption error");
                                return;
                            }
                        }

                        Self::on_decrypted_message(
                            &sender_id,
                            user_account,
                            &decrypted,
                            &container.signature,
                        );
                    }
                    Some(super::proto::envelop_payload::Payload::Dtn(dtn)) => {
                        dtn::Dtn::net(&receiver_id, &sender_id, &container.signature, &dtn);
                    }
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
