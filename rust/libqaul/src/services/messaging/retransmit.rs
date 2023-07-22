// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Retransmit Qaul Messages
//!
//! Messages that couldn't be sent to a user are scheduled for retransmission.

use libp2p::PeerId;
use prost::Message;

use crate::router;
use crate::utilities::qaul_id::QaulId;
use crate::utilities::timestamp::Timestamp;

/// Qaul Messaging Structure
pub struct MessagingRetransmit {}

impl MessagingRetransmit {
    /// process retransmission
    pub fn process() {
        // get unconfirmed table
        let unconfirmed = super::UNCONFIRMED.get().write().unwrap();
        if unconfirmed.unconfirmed.len() == 0 {
            // there are no message to retrans
            return;
        }

        // get online users from route table
        let online_users = router::table::RoutingTable::get_online_users();

        let mut updated = false;
        let cur_time = Timestamp::get_timestamp();
        for entry in unconfirmed.unconfirmed.iter() {
            if let Ok((signature, mut unconfirmed_message)) = entry {
                // let's assume message transmit in 3 seconds
                if cur_time < (unconfirmed_message.last_sent + 3000) {
                    continue;
                }

                // message scheduled via DTN, ignore retrans
                if unconfirmed_message.scheduled_dtn {
                    continue;
                }

                let qaul_id = QaulId::bytes_to_q8id(unconfirmed_message.receiver_id.clone());
                //1. check receiver is online
                if let Some(_hc) = online_users.get(&qaul_id) {
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
                            let receiver =
                                PeerId::from_bytes(&unconfirmed_message.receiver_id).unwrap();

                            log::trace!(
                                "retrans message, signature: {}",
                                bs58::encode(container.signature.clone()).into_string()
                            );
                            super::Messaging::schedule_message(
                                receiver.clone(),
                                container.clone(),
                                true,
                                false,
                                unconfirmed_message.scheduled_dtn,
                                unconfirmed_message.is_dtn,
                            );

                            // update entry
                            let mut new_retry = unconfirmed_message.retry;
                            if new_retry > 10 {
                                new_retry = 1;
                            }

                            unconfirmed_message.retry = new_retry;
                            unconfirmed_message.last_sent = cur_time;
                            if let Err(_e) = unconfirmed
                                .unconfirmed
                                .insert(signature, unconfirmed_message.clone())
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

        if updated {
            if let Err(_e) = unconfirmed.unconfirmed.flush() {
                log::error!("updating unconfirmed table error!");
            }
        }
    }
}
