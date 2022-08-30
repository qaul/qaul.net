// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul DTN Service
//!
//! The DTN service sends and receives DTN messages into the network.
//! They should reach everyone in the network.

//use bs58::decode;
use libp2p::PeerId;

use prost::Message;

use sled_extensions::{bincode::Tree, DbExt};
use state::Storage;

use std::{convert::TryInto, sync::RwLock};

use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::storage::configuration::Configuration;

use crate::storage::database::DataBase;

use super::messaging::Messaging;
use super::messaging::{proto, MessagingServiceType};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DtnMessageEntry {
    pub org_sig: Vec<u8>,
    pub size: u32,
}

/// dtn storage state
#[derive(Clone)]
pub struct DtnStorageState {
    pub message_counts: u32,
    pub used_size: u64,
    pub db_ref: Tree<DtnMessageEntry>,
    pub db_ref_id: Tree<Vec<u8>>,
}
/// mutable state of storge
pub static STORAGESTATE: Storage<RwLock<DtnStorageState>> = Storage::new();

/// qaul Delayed
///
pub struct Dtn {}

impl Dtn {
    pub fn init() {
        let db = DataBase::get_node_db();

        // open trees
        let dtn_messages: Tree<DtnMessageEntry> = db.open_bincode_tree("dtn-messages").unwrap();
        let db_ref_id: Tree<Vec<u8>> = db.open_bincode_tree("dtn-messages-ids").unwrap();

        //calc current used size
        let mut used_size: u64 = 0;
        for entry in dtn_messages.iter() {
            if let Ok((_, ent)) = entry {
                used_size = used_size + (ent.size as u64);
            }
        }
        let storage_state = DtnStorageState {
            message_counts: dtn_messages.len() as u32,
            used_size,
            db_ref: dtn_messages,
            db_ref_id: db_ref_id,
        };

        STORAGESTATE.set(RwLock::new(storage_state));
    }

    fn process_storage_node_message(
        user_account: &UserAccount,
        receiver_id: &PeerId,
        org_sig: &Vec<u8>,
        dtn_payload: &Vec<u8>,
    ) -> (i32, i32) {
        let mut storage_state = STORAGESTATE.get().write().unwrap();

        // check already received
        if storage_state.db_ref_id.contains_key(org_sig).unwrap() {
            return (
                super::messaging::proto::dtn_response::ResponseType::Accepted
                    .try_into()
                    .unwrap(),
                super::messaging::proto::dtn_response::Reason::None
                    .try_into()
                    .unwrap(),
            );
        }

        let config = Configuration::get();
        // check storage
        let new_size = storage_state.used_size + (dtn_payload.len() as u64);
        let total_limit = (config.storage.size_total as u64) * 1024 * 1024;
        if new_size > total_limit {
            return (
                super::messaging::proto::dtn_response::ResponseType::Rejected
                    .try_into()
                    .unwrap(),
                super::messaging::proto::dtn_response::Reason::OverallQuota
                    .try_into()
                    .unwrap(),
            );
        }

        // repack message and send
        let payload = super::messaging::proto::EnvelopPayload {
            payload: Some(super::messaging::proto::envelop_payload::Payload::Dtn(
                dtn_payload.clone(),
            )),
        };

        let envelop = super::messaging::proto::Envelope {
            sender_id: user_account.id.to_bytes(),
            receiver_id: receiver_id.to_bytes(),
            payload: payload.encode_to_vec(),
        };

        if let Ok(signature) = user_account.keys.sign(&envelop.encode_to_vec()) {
            // save dtn message entry
            storage_state.message_counts = storage_state.message_counts + 1;
            storage_state.used_size = new_size;

            if let Err(_e) = storage_state.db_ref.insert(
                signature.clone(),
                DtnMessageEntry {
                    org_sig: org_sig.clone(),
                    size: dtn_payload.len() as u32,
                },
            ) {
                log::error!("dnt entry storing error!");
            } else {
                if let Err(_e) = storage_state.db_ref.flush() {
                    log::error!("dnt entry flushing error!");
                }
            }

            // save message id
            if let Err(_e) = storage_state
                .db_ref_id
                .insert(org_sig.clone(), signature.clone())
            {
                log::error!("dtn id db storing error!");
            } else {
                if let Err(_e) = storage_state.db_ref_id.flush() {
                    log::error!("dtn id db flushing error!");
                }
            }

            let container = super::messaging::proto::Container {
                signature: signature.clone(),
                envelope: Some(envelop),
            };
            super::messaging::Messaging::save_unconfirmed_message(
                MessagingServiceType::DtnStored,
                &vec![],
                receiver_id,
                &container,
                true,
            );
        }

        (
            super::messaging::proto::dtn_response::ResponseType::Accepted
                .try_into()
                .unwrap(),
            super::messaging::proto::dtn_response::Reason::None
                .try_into()
                .unwrap(),
        )
        // update storage state
    }

    pub fn net(user_id: &PeerId, sender_id: &PeerId, signature: &Vec<u8>, dtn_payload: &Vec<u8>) {
        if let Some(user_account) = UserAccounts::get_by_id(*user_id) {
            match proto::Container::decode(&dtn_payload[..]) {
                Ok(container) => {
                    let envelope = container.envelope.as_ref().unwrap();

                    let mut res: (i32, i32) = (
                        super::messaging::proto::dtn_response::ResponseType::Accepted
                            .try_into()
                            .unwrap(),
                        super::messaging::proto::dtn_response::Reason::None
                            .try_into()
                            .unwrap(),
                    );

                    //if container.envelope.receiver_id
                    if let Ok(receiver_id) = PeerId::from_bytes(&envelope.receiver_id) {
                        if receiver_id == *user_id {
                            // by process geneal message, the confirm message is transfered to the original sender.
                            super::messaging::process::MessagingProcess::process_received_message(
                                user_account.clone(),
                                container,
                            );
                        } else {
                            res = Self::process_storage_node_message(
                                &user_account,
                                &receiver_id,
                                signature,
                                dtn_payload,
                            );
                        }

                        // we send DTN response message
                        let dnt_response = super::messaging::proto::DtnResponse {
                            response_type: res.0,
                            reason: res.1,
                            signature: signature.clone(),
                        };
                        let send_message = proto::Messaging {
                            message: Some(proto::messaging::Message::DtnResponse(dnt_response)),
                        };
                        if let Err(_) = super::messaging::Messaging::pack_and_send_message(
                            &user_account,
                            sender_id,
                            send_message.encode_to_vec(),
                            MessagingServiceType::DtnStored,
                            &Vec::new(),
                            false,
                        ) {
                            log::error!("send dtn message error!");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
