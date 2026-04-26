// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Retransmit Qaul Messages
//!
//! Messages that couldn't be sent to a user are scheduled for retransmission.

use libp2p::PeerId;
use prost::Message;

use super::UnConfirmedMessage;
use crate::services::dtn;
use crate::utilities::qaul_id::QaulId;
use crate::utilities::timestamp::Timestamp;

/// Qaul Messaging Structure
pub struct MessagingRetransmit {}

impl MessagingRetransmit {
    /// process retransmission
    pub fn process(state: &crate::QaulState) {
        // get unconfirmed table
        let unconfirmed = match state.services.messaging.unconfirmed.write() {
            Ok(u) => u,
            Err(e) => {
                log::error!("Failed to acquire unconfirmed write lock: {}", e);
                return;
            }
        };
        if unconfirmed.unconfirmed.len() == 0 {
            // there are no message to retransmit
            return;
        }

        // get online users from route table
        let rs = state.get_router();
        let online_users = rs.routing_table.get_online_users();

        let mut updated = false;
        let cur_time = Timestamp::get_timestamp();
        for entry in unconfirmed.unconfirmed.iter() {
            if let Ok((signature, unconfirmed_message_bytes)) = entry {
                let mut unconfirmed_message: UnConfirmedMessage =
                    match bincode::deserialize(&unconfirmed_message_bytes) {
                        Ok(u) => u,
                        Err(e) => {
                            log::error!("Failed to deserialize unconfirmed message: {}", e);
                            continue;
                        }
                    };

                // let's assume message transmit in 3 seconds
                if cur_time < (unconfirmed_message.last_sent + 3000) {
                    continue;
                }

                // message scheduled via DTN, ignore retrans
                if unconfirmed_message.scheduled_dtn {
                    continue;
                }

                // expire messages older than 1 hour to prevent indefinite accumulation
                if cur_time.saturating_sub(unconfirmed_message.last_sent) > 3_600_000 {
                    log::warn!(
                        "retransmit: message expired (>1h), removing: {}",
                        bs58::encode(&signature).into_string()
                    );
                    if let Err(_e) = unconfirmed.unconfirmed.remove(&signature) {
                        log::error!("removing expired unconfirmed message error!");
                    }
                    updated = true;
                    continue;
                }

                let qaul_id = QaulId::bytes_as_q8id(&unconfirmed_message.receiver_id);
                //1. check receiver is online
                if let Some(_hc) = online_users.get(qaul_id) {
                    let mut timeout: u64 = 0;
                    if unconfirmed_message.scheduled {
                        timeout = 20 * 1000;
                    }

                    //check if expired timeout
                    if cur_time > (timeout + unconfirmed_message.last_sent) {
                        // queue into messaging queue
                        if let Ok(container) =
                            super::proto::Container::decode(&unconfirmed_message.container[..])
                        {
                            let receiver = match PeerId::from_bytes(&unconfirmed_message.receiver_id) {
                                Ok(r) => r,
                                Err(e) => {
                                    log::error!("Failed to parse receiver PeerId: {}", e);
                                    continue;
                                }
                            };

                            log::trace!(
                                "retrans message, signature: {}",
                                bs58::encode(&container.signature).into_string()
                            );
                            state.services.messaging.schedule_message(
                                receiver,
                                container,
                                true,
                                false,
                                unconfirmed_message.scheduled_dtn,
                                unconfirmed_message.is_dtn,
                            );

                            // update entry
                            let new_retry = unconfirmed_message.retry + 1;
                            if new_retry > 10 {
                                // max retries reached, remove from unconfirmed
                                log::warn!(
                                    "retransmit: max retries reached for message, removing: {}",
                                    bs58::encode(&signature).into_string()
                                );
                                if let Err(_e) = unconfirmed.unconfirmed.remove(&signature) {
                                    log::error!("removing expired unconfirmed message error!");
                                }
                                updated = true;
                            } else {
                                unconfirmed_message.retry = new_retry;
                                unconfirmed_message.last_sent = cur_time;
                                let unconfirmed_message_todb =
                                    match bincode::serialize(&unconfirmed_message) {
                                        Ok(b) => b,
                                        Err(e) => {
                                            log::error!(
                                                "retransmit: serialize unconfirmed entry error: {}",
                                                e
                                            );
                                            continue;
                                        }
                                    };
                                if let Err(_e) = unconfirmed
                                    .unconfirmed
                                    .insert(signature, unconfirmed_message_todb)
                                {
                                    log::error!("updating unconfirmed table error!");
                                } else {
                                    updated = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if updated {
            if let Err(_e) = unconfirmed.unconfirmed.flush() {
                log::error!("updating unconfirmed table error!");
            }
        }

        // Process V2 DTN routed messages
        dtn::Dtn::process_retransmit_v2(state);
    }
}
