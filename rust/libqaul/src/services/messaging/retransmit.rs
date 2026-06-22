// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Retransmit Qaul Messages
//!
//! Messages that couldn't be sent to a user are scheduled for retransmission.

use libp2p::PeerId;
use prost::Message;

use super::UnConfirmedMessage;
use crate::node::user_accounts::UserAccounts;
use crate::services::dtn;
use crate::utilities::qaul_id::QaulId;
use crate::utilities::timestamp::Timestamp;

/// Qaul Messaging Structure
pub struct MessagingRetransmit {}

impl MessagingRetransmit {
    /// process retransmission
    pub fn process(state: &crate::QaulState) {
        // These run on every tick regardless of the unconfirmed table, because
        // they use separate stores and the unconfirmed-table early-return below
        // must not skip them:
        //  - messages queued while a peer session was completing its handshake
        //    (pending_plaintext); after the handshake completes the unconfirmed
        //    table may well be empty, so this must not be gated on it.
        //  - V2 routed DTN messages, tracked in the DTN custody store.
        Self::flush_pending_plaintext(state);
        dtn::Dtn::process_retransmit_v2(state);

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
                                    bincode::serialize(&unconfirmed_message).unwrap();
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
    }

    /// Re-attempt sending messages that were queued because their peer
    /// session was mid-handshake (see [`super::PendingPlaintext`]).
    ///
    /// Items whose session is still not ready are kept (preserving their
    /// original queue time so they can still expire); items older than 1h are
    /// dropped so the queue stays bounded even if a peer never completes its
    /// handshake.
    fn flush_pending_plaintext(state: &crate::QaulState) {
        // Bound the queue first (drops anything stuck for over an hour).
        let now = Timestamp::get_timestamp();
        let dropped = state
            .services
            .messaging
            .prune_expired_pending(now, 3_600_000);
        if dropped > 0 {
            log::warn!("retransmit: dropped {} expired pending message(s)", dropped);
        }

        let items = state.services.messaging.take_pending_plaintext();
        if items.is_empty() {
            return;
        }

        let mut keep = Vec::new();
        for item in items {
            let receiver = match PeerId::from_bytes(&item.receiver_id) {
                Ok(r) => r,
                Err(e) => {
                    log::error!("pending flush: invalid receiver id: {}", e);
                    continue;
                }
            };
            let user_id = match PeerId::from_bytes(&item.user_id) {
                Ok(u) => u,
                Err(e) => {
                    log::error!("pending flush: invalid user id: {}", e);
                    continue;
                }
            };
            let user_account = match UserAccounts::get_by_id(state, user_id) {
                Some(ua) => ua,
                None => {
                    log::error!("pending flush: sending user account no longer exists");
                    continue;
                }
            };

            // Still mid-handshake? Keep it and try again next tick.
            if crate::services::crypto::Crypto::session_pending_handshake(
                state,
                &user_account,
                receiver,
            ) {
                keep.push(item);
                continue;
            }

            // Session is ready (or absent, which starts a fresh handshake):
            // send for real. On success the message enters the normal
            // unconfirmed-table flow.
            if let Err(e) = super::Messaging::pack_and_send_message(
                state,
                &user_account,
                &receiver,
                item.data,
                item.message_type,
                &item.message_id,
                item.needs_confirmation,
            ) {
                log::error!("pending flush send failed: {}", e);
            }
        }

        // Re-queue items still waiting on a handshake.
        if !keep.is_empty() {
            if let Ok(mut q) = state.services.messaging.pending_plaintext.write() {
                q.extend(keep);
            }
        }
    }
}
