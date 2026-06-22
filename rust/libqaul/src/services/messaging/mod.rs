// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Messaging Service
//!
//! The messaging service is used for sending, receiving and
//! relay chat messages.

use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled;
use std::collections::VecDeque;
use std::sync::RwLock;

#[cfg(emulate)]
mod network_emul;

pub mod process;
pub mod retransmit;

use super::chat::{ChatFile, ChatStorage};
use super::crypto::Crypto;
use crate::connections::ConnectionModule;
use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::storage::database::DataBase;
use crate::utilities::timestamp::Timestamp;
use process::MessagingProcess;
use qaul_messaging::QaulMessagingReceived;

/// Import protobuf message definition
pub use qaul_proto::qaul_net_messaging as proto;


/// Messaging Scheduling Structure
pub struct ScheduledMessage {
    receiver: PeerId,
    container: proto::Container,
    is_common: bool,
    is_forward: bool,
    scheduled_dtn: bool,
    is_dtn: bool,
}


// TODO: check if it wouldn't be easier to store
// the message
/// unconfirmed message
#[derive(Serialize, Deserialize, Clone)]
pub struct UnConfirmedMessage {
    // receiver id
    pub receiver_id: Vec<u8>,
    // message type
    pub message_type: MessagingServiceType,
    // message id
    pub message_id: Vec<u8>,
    // encoded container
    pub container: Vec<u8>,
    // last sent time
    pub last_sent: u64,
    // retry time
    pub retry: u32,
    // flag that transferred on the network
    pub scheduled: bool,
    // flag that transferred as DTN service
    pub scheduled_dtn: bool,
    // flag that indicate DTN message
    pub is_dtn: bool,
}

/// Unconfirmed Message Type
#[derive(Serialize, Deserialize, Clone)]
pub enum MessagingServiceType {
    /// Unconfirmed Message
    /// (this message does expect a confirmation)
    Unconfirmed,
    /// DTN message, originated from this host
    DtnOrigin,
    /// DTN message, stored on this host
    DtnStored,
    /// Crypto Handshake Message
    Crypto,
    /// Group Management Message
    Group,
    /// Chat Text Message
    Chat,
    /// Chat File Message
    ChatFile,
    /// RTC Message
    Rtc,
}

/// An outbound message whose plaintext is held in memory because it could
/// not be encrypted yet: the peer session was still completing its KK
/// handshake (`HalfOutgoing`), so `Crypto::encrypt` refuses to produce a
/// transport frame. The retransmit tick re-attempts it once the session
/// reaches `Transport`.
///
/// Held in RAM only — deliberately never persisted — so plaintext is not
/// written at rest; a restart simply drops anything still pending (no worse
/// than today, where such messages were dropped outright).
#[derive(Clone)]
pub struct PendingPlaintext {
    /// local sending user id (PeerId bytes)
    pub user_id: Vec<u8>,
    /// remote receiver id (PeerId bytes)
    pub receiver_id: Vec<u8>,
    /// plaintext message bytes to encrypt and send
    pub data: Vec<u8>,
    /// service type for the eventual unconfirmed entry
    pub message_type: MessagingServiceType,
    /// message id for the eventual unconfirmed entry
    pub message_id: Vec<u8>,
    /// whether the message expects a delivery confirmation
    pub needs_confirmation: bool,
    /// timestamp (ms) when first queued, used for bounded expiry
    pub queued_at: u64,
}

/// Plaintext of a sent, not-yet-confirmed message, kept so a retransmit can
/// RE-ENCRYPT it under the peer's current session when the originally-sent
/// ciphertext can no longer be decrypted by the receiver (the peer session
/// rotated and the old one was retired).
///
/// Persisted (unlike [`PendingPlaintext`], which is in-memory) so that
/// re-encryption survives a restart-triggered cold re-key — the dominant
/// rotation trigger. Stored on the sender's own device only; the same message
/// content already lives in the local chat database, so this is not a new
/// plaintext-at-rest exposure. Removed once the message is confirmed.
#[derive(Serialize, Deserialize, Clone)]
pub struct OutboundPlaintext {
    /// local sending user id (PeerId bytes)
    pub user_id: Vec<u8>,
    /// remote receiver id (PeerId bytes)
    pub receiver_id: Vec<u8>,
    /// plaintext message bytes to (re-)encrypt and send
    pub data: Vec<u8>,
    /// service type for the unconfirmed entry
    pub message_type: MessagingServiceType,
    /// message id for the unconfirmed entry
    pub message_id: Vec<u8>,
}

/// Unconfirmed Messages Structure
pub struct UnConfirmedMessages {
    /// signature => UnConfirmedMessage
    ///
    /// value: bincode of `UnConfirmedMessage`
    pub unconfirmed: sled::Tree,
    /// signature => bincode of [`OutboundPlaintext`]
    ///
    /// Sender-side plaintext for sent-but-unconfirmed messages, so retransmit
    /// can re-encrypt across a session rotation. Kept in lock-step with
    /// `unconfirmed` (same signature key, same write lock).
    pub outbound_plaintext: sled::Tree,
}

/// Qaul Messaging Structure
pub struct Messaging {
    /// ring buffer of messages scheduled for sending
    pub to_send: VecDeque<ScheduledMessage>,
}

/// Instance-based messaging state owning the scheduled message queue
/// and unconfirmed message tracking.
/// Replaces the global MESSAGING and UNCONFIRMED statics for multi-instance use.
pub struct MessagingState {
    /// Scheduled messages queue.
    pub messaging: RwLock<Messaging>,
    /// Unconfirmed messages tracking.
    pub unconfirmed: RwLock<UnConfirmedMessages>,
    /// In-memory queue of outbound plaintext awaiting a usable peer session
    /// (messages queued while a KK handshake is still in progress). Not
    /// persisted; see [`PendingPlaintext`].
    pub pending_plaintext: RwLock<Vec<PendingPlaintext>>,
    /// Sled database backing (kept alive for tree references).
    /// Wrapped in RwLock so `init_production` can swap it after construction.
    _db: RwLock<sled::Db>,
    /// Network emulator state (only compiled with the `emulate` cfg flag).
    #[cfg(emulate)]
    pub network_emul: RwLock<network_emul::NetworkEmulatorStat>,
}

impl MessagingState {
    /// Create a new empty MessagingState with a temporary in-memory database.
    pub fn new() -> Self {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let unconfirmed_tree = db.open_tree("unconfirmed").unwrap();
        let outbound_plaintext_tree = db.open_tree("outbound_plaintext").unwrap();
        Self {
            messaging: RwLock::new(Messaging {
                to_send: VecDeque::new(),
            }),
            unconfirmed: RwLock::new(UnConfirmedMessages {
                unconfirmed: unconfirmed_tree,
                outbound_plaintext: outbound_plaintext_tree,
            }),
            pending_plaintext: RwLock::new(Vec::new()),
            _db: RwLock::new(db),
            #[cfg(emulate)]
            network_emul: RwLock::new(network_emul::NetworkEmulatorStat {
                loss_rate: 5,
                total_message: 0,
                total_drop: 0,
            }),
        }
    }

    /// Create a MessagingState backed by a production sled database.
    pub fn from_production(db: sled::Db) -> Self {
        let unconfirmed_tree = db.open_tree("unconfirmed").unwrap();
        let outbound_plaintext_tree = db.open_tree("outbound_plaintext").unwrap();
        Self {
            messaging: RwLock::new(Messaging {
                to_send: VecDeque::new(),
            }),
            unconfirmed: RwLock::new(UnConfirmedMessages {
                unconfirmed: unconfirmed_tree,
                outbound_plaintext: outbound_plaintext_tree,
            }),
            pending_plaintext: RwLock::new(Vec::new()),
            _db: RwLock::new(db),
            #[cfg(emulate)]
            network_emul: RwLock::new(network_emul::NetworkEmulatorStat {
                loss_rate: 5,
                total_message: 0,
                total_drop: 0,
            }),
        }
    }

    /// Re-initialize this MessagingState with a production sled database.
    /// Replaces the temporary in-memory DB and unconfirmed tree with
    /// production-backed ones. Called from `Messaging::init()`.
    pub fn init_production(&self, db: sled::Db) {
        let unconfirmed_tree = db.open_tree("unconfirmed").unwrap();
        let outbound_plaintext_tree = db.open_tree("outbound_plaintext").unwrap();
        {
            let mut unconfirmed = self.unconfirmed.write().unwrap();
            unconfirmed.unconfirmed = unconfirmed_tree;
            unconfirmed.outbound_plaintext = outbound_plaintext_tree;
        }
        {
            let mut db_lock = self._db.write().unwrap();
            *db_lock = db;
        }
    }

    /// Schedule a message for sending (instance method).
    pub fn schedule_message(
        &self,
        receiver: PeerId,
        container: proto::Container,
        is_common: bool,
        is_forward: bool,
        scheduled_dtn: bool,
        is_dtn: bool,
    ) {
        let msg = ScheduledMessage {
            receiver,
            container,
            is_common,
            is_forward,
            scheduled_dtn,
            is_dtn,
        };

        let mut messaging = self.messaging.write().unwrap();
        messaging.to_send.push_back(msg);
    }

    /// Check scheduler for next message to send (instance method).
    /// Takes routing table state as an explicit parameter.
    pub fn check_scheduler(
        &self,
        routing_table: &crate::router::table::RoutingTableState,
    ) -> Option<(PeerId, ConnectionModule, Vec<u8>)> {
        let message_item: Option<ScheduledMessage>;
        {
            let mut messaging = self.messaging.write().unwrap();
            message_item = messaging.to_send.pop_front();
        }

        if let Some(message) = message_item {
            if let Some(route) = routing_table.get_route_to_user(message.receiver) {
                self.on_scheduled_message(&message.container.signature);
                let data = message.container.encode_to_vec();
                return Some((route.node, route.module, data));
            }
        }

        None
    }

    /// Save a message to the unconfirmed table (instance method).
    pub fn save_unconfirmed_message(
        &self,
        message_type: MessagingServiceType,
        message_id: &[u8],
        receiver: &PeerId,
        container: &proto::Container,
        is_dtn: bool,
    ) {
        let new_entry = UnConfirmedMessage {
            receiver_id: receiver.to_bytes(),
            container: container.encode_to_vec(),
            last_sent: Timestamp::get_timestamp(),
            message_type,
            message_id: message_id.to_vec(),
            retry: 1,
            scheduled: false,
            scheduled_dtn: false,
            is_dtn,
        };
        let entry_bytes = match bincode::serialize(&new_entry) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!("Failed to serialize unconfirmed entry: {}", e);
                return;
            }
        };
        let unconfirmed = self.unconfirmed.write().unwrap();
        if let Err(e) = unconfirmed.unconfirmed.insert(container.signature.clone(), entry_bytes) {
            log::error!("{}", e);
        }
        if let Err(e) = unconfirmed.unconfirmed.flush() {
            log::error!("Error unconfirmed table flush: {}", e);
        }
    }

    /// Process confirmation message (instance method).
    pub fn on_confirmed_message(&self, signature: &[u8]) {
        let unconfirmed = self.unconfirmed.write().unwrap();
        match unconfirmed.unconfirmed.remove(signature) {
            Ok(_v) => {
                if let Err(e) = unconfirmed.unconfirmed.flush() {
                    log::error!("Error unconfirmed table flush: {}", e);
                }
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
        // the message is confirmed: drop its retransmit plaintext too
        let _ = unconfirmed.outbound_plaintext.remove(signature);
    }

    pub(crate) fn on_scheduled_message(&self, signature: &[u8]) {
        self.mark_unconfirmed(signature, |msg| msg.scheduled = true);
    }

    pub fn on_scheduled_as_dtn_message(&self, signature: &[u8]) {
        self.mark_unconfirmed(signature, |msg| msg.scheduled_dtn = true);
    }

    /// Queue an outbound message whose peer session is still mid-handshake,
    /// to be re-attempted by the retransmit tick once the session is ready.
    pub fn enqueue_pending_plaintext(&self, item: PendingPlaintext) {
        match self.pending_plaintext.write() {
            Ok(mut q) => q.push(item),
            Err(e) => log::error!("pending_plaintext lock poisoned: {}", e),
        }
    }

    /// Drop pending items older than `max_age_ms`. Returns the number dropped.
    /// Bounds the queue so a peer that never completes its handshake cannot
    /// make it grow without limit.
    pub fn prune_expired_pending(&self, now: u64, max_age_ms: u64) -> usize {
        match self.pending_plaintext.write() {
            Ok(mut q) => {
                let before = q.len();
                q.retain(|p| now.saturating_sub(p.queued_at) <= max_age_ms);
                before - q.len()
            }
            Err(e) => {
                log::error!("pending_plaintext lock poisoned: {}", e);
                0
            }
        }
    }

    /// Take (drain) all pending plaintext items.
    pub fn take_pending_plaintext(&self) -> Vec<PendingPlaintext> {
        match self.pending_plaintext.write() {
            Ok(mut q) => std::mem::take(&mut *q),
            Err(e) => {
                log::error!("pending_plaintext lock poisoned: {}", e);
                Vec::new()
            }
        }
    }

    /// Persist the plaintext of a sent-but-unconfirmed message, keyed by its
    /// signature, so a retransmit can re-encrypt it across a session rotation
    /// (see [`OutboundPlaintext`]). Stored in the same tree-group as the
    /// unconfirmed entry; removed when the message is confirmed or expires.
    ///
    /// Callers must NOT already hold the `unconfirmed` lock (this acquires it).
    pub fn save_outbound_plaintext(&self, signature: &[u8], item: OutboundPlaintext) {
        let bytes = match bincode::serialize(&item) {
            Ok(b) => b,
            Err(e) => {
                log::error!("failed to serialize outbound plaintext: {}", e);
                return;
            }
        };
        let unconfirmed = match self.unconfirmed.write() {
            Ok(u) => u,
            Err(e) => {
                log::error!("unconfirmed lock poisoned: {}", e);
                return;
            }
        };
        if let Err(e) = unconfirmed.outbound_plaintext.insert(signature, bytes) {
            log::error!("failed to store outbound plaintext: {}", e);
        } else {
            let _ = unconfirmed.outbound_plaintext.flush();
        }
    }

    /// Load the stored retransmit plaintext for a message signature, if any.
    pub fn get_outbound_plaintext(&self, signature: &[u8]) -> Option<OutboundPlaintext> {
        let unconfirmed = self.unconfirmed.read().ok()?;
        match unconfirmed.outbound_plaintext.get(signature) {
            Ok(Some(bytes)) => bincode::deserialize(&bytes).ok(),
            _ => None,
        }
    }

    /// Shared helper: load an unconfirmed message, apply a mutation, persist it back.
    fn mark_unconfirmed(&self, signature: &[u8], mutate: impl FnOnce(&mut UnConfirmedMessage)) {
        let unconfirmed = self.unconfirmed.write().unwrap();
        let Some(bytes) = unconfirmed.unconfirmed.get(signature).unwrap() else {
            return;
        };
        let mut msg: UnConfirmedMessage = bincode::deserialize(&bytes).unwrap();
        if msg.scheduled {
            return;
        }
        mutate(&mut msg);
        let serialized = bincode::serialize(&msg).unwrap();
        if let Err(_e) = unconfirmed.unconfirmed.insert(signature.to_vec(), serialized) {
            log::error!("error updating unconfirmed table");
        } else if let Err(_e) = unconfirmed.unconfirmed.flush() {
            log::error!("error updating unconfirmed table");
        }
    }
}

impl Messaging {
    /// Initialize messaging and create the ring buffer.
    pub fn init(state: &crate::QaulState) {
        #[cfg(emulate)]
        /// init emulator
        network_emul::NetworkEmulator::init();

        let db = DataBase::get_node_db(state);
        state.services.messaging.init_production(db);
    }

    /// Process confirmation message
    ///
    /// Removes the message from the unconfirmed table and notifies
    /// the related service (if needed) that the message was received.
    pub fn on_confirmed_message(
        state: &crate::QaulState,
        signature: &[u8],
        sender_id: PeerId,
        user_account: UserAccount,
        confirmation: proto::Confirmation,
    ) {
        log::trace!(
            "incoming confirmation message for signature: {}",
            bs58::encode(signature).into_string()
        );

        // Captured for the post-confirmation crypto-rotation hook below
        // (the arms may consume `sender_id` / `user_account`).
        let rotation_local_id = user_account.id.clone();
        let rotation_remote_id = sender_id.clone();

        let unconfirmed = state.services.messaging.unconfirmed.write().unwrap();

        // check and remove unconfirmed from DB
        match unconfirmed.unconfirmed.remove(signature) {
            Ok(v) => {
                if let Err(e) = unconfirmed.unconfirmed.flush() {
                    log::error!("Error unconfirmed table flush: {}", e);
                }

                match v {
                    Some(unconfirmed_bytes) => {
                        let unconfirmed: UnConfirmedMessage = match bincode::deserialize(&unconfirmed_bytes) {
                            Ok(u) => u,
                            Err(e) => {
                                log::error!("Failed to deserialize unconfirmed message: {}", e);
                                return;
                            }
                        };

                        // check message and decide what to do
                        match unconfirmed.message_type {
                            MessagingServiceType::Unconfirmed => {
                                log::trace!("Confirmation: Unconfirmed");
                            }
                            MessagingServiceType::DtnOrigin => {
                                log::trace!("Confirmation: DtnOrigin");
                                // what kind of message do we have here?
                                // TODO: check chat storage as sent ...
                            }
                            MessagingServiceType::DtnStored => {
                                log::trace!("Confirmation: DtnStored");
                            }
                            MessagingServiceType::Crypto => {
                                log::trace!("Confirmation: Crypto");
                            }
                            MessagingServiceType::Group => {
                                log::trace!("Confirmation: Group");
                                // don't do anything for group messages
                            }
                            MessagingServiceType::Chat => {
                                log::trace!("Confirmation: Chat");
                                // set received info in chat data base
                                ChatStorage::update_confirmation(
                                    state,
                                    user_account.id,
                                    sender_id,
                                    &unconfirmed.message_id,
                                    confirmation.received_at,
                                );
                            }
                            MessagingServiceType::ChatFile => {
                                log::trace!("Confirmation: ChatFile");
                                match unconfirmed.message_id.try_into() {
                                    Ok(arr) => {
                                        let file_id = u64::from_be_bytes(arr);

                                        // confirm message reception in data base
                                        ChatFile::update_confirmation(
                                            state,
                                            user_account.id,
                                            sender_id,
                                            file_id,
                                            confirmation.received_at,
                                        );
                                    }
                                    Err(e) => {
                                        log::error!("couldn't convert file_id to u64: {:?}", e);
                                    }
                                }
                            }
                            MessagingServiceType::Rtc => {
                                log::trace!("Confirmation: Rtc");
                                // TODO CONFIRM RTC MESSAGE
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }

        // the message is confirmed: drop its retransmit plaintext too
        let _ = unconfirmed.outbound_plaintext.remove(signature);

        // Release the unconfirmed-table lock before the rotation hook,
        // which takes its own read lock on the same table.
        drop(unconfirmed);

        // A confirmation may have cleared the last outbound message on
        // a draining crypto session; let rotation retire it now if it
        // is also fully drained inbound.
        crate::services::crypto::Crypto::on_outbound_confirmed(
            state,
            rotation_local_id,
            rotation_remote_id,
        );
    }

    /// pack, sign and schedule a message for sending
    ///
    /// The function returns the message signature on success,
    /// otherwise an error message string.

    pub fn pack_and_send_message(
        state: &crate::QaulState,
        user_account: &UserAccount,
        receiver: &PeerId,
        data: Vec<u8>,
        message_type: MessagingServiceType,
        message_id: &[u8],
        message_needs_confirmation: bool,
    ) -> Result<Vec<u8>, String> {
        log::trace!("pack_and_send_message to {}", receiver.to_base58());

        // If the peer session is still completing its KK handshake, encryption
        // would fail and the message would be dropped. Instead, queue the
        // plaintext in memory; the retransmit tick re-sends it once the session
        // reaches Transport. Probing before encrypt avoids cloning `data` on
        // the common (already-established) path — file chunks can be tens of KB.
        if Crypto::session_pending_handshake(state, user_account, receiver.clone()) {
            state.services.messaging.enqueue_pending_plaintext(PendingPlaintext {
                user_id: user_account.id.to_bytes(),
                receiver_id: receiver.to_bytes(),
                data,
                message_type,
                message_id: message_id.to_vec(),
                needs_confirmation: message_needs_confirmation,
                queued_at: Timestamp::get_timestamp(),
            });
            log::debug!(
                "queued outbound message for {} (peer handshake in progress)",
                receiver.to_base58()
            );
            // Accepted (queued for retry); there is no signature yet. Callers
            // branch only on Err vs Ok, so an empty signature is harmless.
            return Ok(Vec::new());
        }

        // Keep a copy of the plaintext for confirmable messages so a
        // retransmit can re-encrypt it if the peer session rotates (and the old
        // one is retired) before the message is confirmed — see
        // `OutboundPlaintext`. Cloned only for confirmable messages, so
        // confirmations and one-shot messages don't pay the cost.
        let retry_plaintext = if message_needs_confirmation {
            Some(data.clone())
        } else {
            None
        };

        // encrypt data
        let encrypted_message: proto::Encrypted;
        let encryption_result = Crypto::encrypt(state, data, user_account.to_owned(), receiver.clone());

        match encryption_result {
            Some(encrypted) => {
                encrypted_message = encrypted;
            }
            None => return Err("Encryption error occurred".to_string()),
        }

        let result = Self::pack_and_send_encrypted_data(
            state,
            user_account,
            receiver,
            encrypted_message,
            message_type.clone(),
            message_id,
            message_needs_confirmation,
        );

        // On success, persist the plaintext keyed by the message signature, so a
        // retransmit can re-encrypt it under the current session if the original
        // one rotates away before confirmation. Only this path stores it; crypto
        // handshake frames call pack_and_send_encrypted_data directly and are
        // intentionally excluded. Removed when the message is confirmed.
        if let (Ok(signature), Some(plaintext)) = (&result, retry_plaintext) {
            state.services.messaging.save_outbound_plaintext(
                signature,
                OutboundPlaintext {
                    user_id: user_account.id.to_bytes(),
                    receiver_id: receiver.to_bytes(),
                    data: plaintext,
                    message_type,
                    message_id: message_id.to_vec(),
                },
            );
        }
        result
    }

    /// pack, sign and schedule encrypted message data
    ///
    /// The function returns the message signature on success,
    /// otherwise an error message string.
    pub fn pack_and_send_encrypted_data(
        state: &crate::QaulState,
        user_account: &UserAccount,
        receiver: &PeerId,
        encrypted_message: proto::Encrypted,
        message_type: MessagingServiceType,
        message_id: &[u8],
        message_needs_confirmation: bool,
    ) -> Result<Vec<u8>, String> {
        log::trace!(
            "pack_and_send_encrypted_data\n\tsender_id: {},\n\treceiver_id: {},\n\tneeds confirmation: {:?}",
            user_account.id.to_base58(),
            receiver.to_base58(),
            message_needs_confirmation
        );

        let envelop_payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::Encrypted(
                encrypted_message,
            )),
        };

        // create envelope
        let envelope = proto::Envelope {
            sender_id: user_account.id.to_bytes(),
            receiver_id: receiver.to_bytes(),
            payload: envelop_payload.encode_to_vec(),
        };

        // encode envelope
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        envelope
            .encode(&mut envelope_buf)
            .expect("Vec<u8> provides capacity as needed");

        // sign message
        if let Ok(signature) = user_account.keys.sign(&envelope_buf) {
            // create container
            let container = proto::Container {
                signature: signature.clone(),
                envelope: Some(envelope),
            };

            // in common message case, save into unconfirmed table
            if message_needs_confirmation {
                state.services.messaging.save_unconfirmed_message(
                    message_type,
                    message_id,
                    receiver,
                    &container,
                    false,
                );
            }

            // schedule message for sending
            state.services.messaging.schedule_message(
                receiver.clone(),
                container,
                message_needs_confirmation,
                false,
                false,
                false,
            );

            // return signature
            Ok(signature)
        } else {
            return Err("messaging signing error".to_string());
        }
    }

    /// pack, sign and schedule a message for sending
    pub fn send_dtn_message(
        state: &crate::QaulState,
        user_account: &UserAccount,
        storage_node_id: &PeerId,
        org_container: &proto::Container,
    ) -> Result<Vec<u8>, String> {
        // create Dtn message
        let dtn_payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::Dtn(
                org_container.encode_to_vec(),
            )),
        };
        let envelope_dtn = proto::Envelope {
            sender_id: user_account.id.to_bytes(),
            receiver_id: storage_node_id.to_bytes(),
            payload: dtn_payload.encode_to_vec(),
        };

        if let Ok(signature_dtn) = user_account.keys.sign(&envelope_dtn.encode_to_vec()) {
            // create dtn container
            let container_dtn = proto::Container {
                signature: signature_dtn.clone(),
                envelope: Some(envelope_dtn),
            };

            // in common message case, save into unconfirmed table
            state.services.messaging.save_unconfirmed_message(
                MessagingServiceType::Chat,
                &[],
                &storage_node_id,
                &container_dtn,
                true,
            );

            // schedule message for sending
            state.services.messaging.schedule_message(
                storage_node_id.clone(),
                container_dtn,
                true,
                false,
                true,
                true,
            );

            // return signature
            Ok(signature_dtn)
        } else {
            return Err("dtn messaging signing error".to_string());
        }
    }

    /// Pack, sign and schedule a DtnRoutedV2 message for sending
    pub fn send_dtn_routed_v2_message(
        state: &crate::QaulState,
        user_account: &UserAccount,
        target_id: &PeerId,
        routed_v2: proto::DtnRoutedV2,
    ) -> Result<Vec<u8>, String> {
        // Create envelope payload with DtnRoutedV2
        let dtn_payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::DtnRoutedV2(routed_v2)),
        };
        let envelope = proto::Envelope {
            sender_id: user_account.id.to_bytes(),
            receiver_id: target_id.to_bytes(),
            payload: dtn_payload.encode_to_vec(),
        };

        if let Ok(signature) = user_account.keys.sign(&envelope.encode_to_vec()) {
            let container = proto::Container {
                signature: signature.clone(),
                envelope: Some(envelope),
            };

            // NOTE: V2 routed messages are deliberately NOT tracked in the
            // messaging unconfirmed table. Reliable delivery and cleanup are
            // owned by the V2 custody store (`db_ref_routed_v2`, keyed by the
            // stable end-to-end `original_signature`): retried by
            // `process_retransmit_v2` and cleared by `on_dtn_response_v2`.
            // A messaging unconfirmed entry here would be keyed by this hop's
            // wrapper-envelope `signature`, which the end-to-end `DtnResponse`
            // (carrying `original_signature`) can never match — so it would
            // never be confirmed, never expire (retransmit skips DTN entries),
            // and leak permanently, one entry per V2 message sent.

            state.services.messaging.schedule_message(
                target_id.clone(),
                container,
                true,
                false,
                true,
                true,
            );

            Ok(signature)
        } else {
            Err("dtn v2 messaging signing error".to_string())
        }
    }

    /// schedule a message
    ///
    /// schedule a message for sending.
    /// This function adds the message to the ring buffer for sending.
    /// This buffer is checked regularly by libqaul for sending.
    ///
    pub fn schedule_message(
        state: &crate::QaulState,
        receiver: PeerId,
        container: proto::Container,
        is_common: bool,
        is_forward: bool,
        scheduled_dtn: bool,
        is_dtn: bool,
    ) {
        #[cfg(emulate)]
        if network_emul::NetworkEmulator::is_lost() {
            log::error!(
                "drop message, signature: {}",
                bs58::encode(&container.signature).into_string()
            );
            return;
        }

        let msg = ScheduledMessage {
            receiver,
            container,
            is_common,
            is_forward,
            scheduled_dtn,
            is_dtn,
        };

        // add it to sending queue
        let mut messaging = state.services.messaging.messaging.write().unwrap();
        const MAX_QUEUE_SIZE: usize = 10_000;
        if messaging.to_send.len() >= MAX_QUEUE_SIZE {
            log::warn!(
                "messaging send queue full ({} messages), dropping oldest message",
                messaging.to_send.len()
            );
            messaging.to_send.pop_front();
        }
        messaging.to_send.push_back(msg);
    }

    /// Check Scheduler
    ///
    /// Check if there is a message scheduled for sending.
    ///
    pub fn check_scheduler(state: &crate::QaulState) -> Option<(PeerId, ConnectionModule, Vec<u8>)> {
        let message_item: Option<ScheduledMessage>;

        // get scheduled messaging buffer
        {
            let mut messaging = state.services.messaging.messaging.write().unwrap();
            message_item = messaging.to_send.pop_front();
        }

        if let Some(message) = message_item {
            // check for route
            let rs = state.get_router();
            if let Some(route) = rs.routing_table.get_route_to_user(message.receiver) {
                // update unconfirmed table set scheduled flag.
                state.services.messaging.on_scheduled_message(&message.container.signature);

                // create binary message
                let data = message.container.encode_to_vec();

                // return information
                return Some((route.node, route.module, data));
            } else {
                // user is offline we schedule through DTN service
                if !message.is_forward
                    && !message.is_dtn
                    && !message.scheduled_dtn
                    && message.is_common
                {
                    // get storage node id
                    if let Some(envelope) = message.container.envelope.as_ref() {
                        if let Ok(my_user_id) = PeerId::from_bytes(&envelope.sender_id) {
                            if let Some(storage_node_id) =
                                super::dtn::Dtn::get_storage_user(state, &my_user_id)
                            {
                                if let Some(user_account) =
                                    UserAccounts::get_by_id(state, my_user_id)
                                {
                                    if let Err(_e) = Self::send_dtn_message(
                                        state,
                                        &user_account,
                                        &storage_node_id,
                                        &message.container,
                                    ) {
                                        log::error!("DTN scheduling error!");
                                    } else {
                                        log::error!("DTN scheduled...");
                                        // update unconfirmed table
                                        state.services.messaging.on_scheduled_as_dtn_message(
                                            &message.container.signature,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Send a confirmation message for a received message
    pub fn send_confirmation(
        state: &crate::QaulState,
        user_id: &PeerId,
        receiver_id: &PeerId,
        signature: &[u8],
    ) -> Result<Vec<u8>, String> {
        log::trace!(
            "send confirmation message to\n\tuser_id: {}\n\tfor signature: {}",
            user_id,
            bs58::encode(signature).into_string()
        );

        if let Some(user) = UserAccounts::get_by_id(state,user_id.clone()) {
            // create timestamp
            let timestamp = Timestamp::get_timestamp();

            // pack message
            let send_message = proto::Messaging {
                message: Some(proto::messaging::Message::ConfirmationMessage(
                    proto::Confirmation {
                        signature: signature.to_vec(),
                        received_at: timestamp,
                    },
                )),
            };

            // encode chat message
            let mut message_buf = Vec::with_capacity(send_message.encoded_len());
            send_message
                .encode(&mut message_buf)
                .expect("Vec<u8> provides capacity as needed");

            // send message via messaging
            Self::pack_and_send_message(
                state,
                &user,
                receiver_id,
                message_buf,
                MessagingServiceType::Unconfirmed,
                &[],
                false,
            )
        } else {
            return Err("invalid user_id".to_string());
        }
    }

    /// received message from qaul_messaging behaviour
    pub fn received(state: &crate::QaulState, received: QaulMessagingReceived) {
        // decode message container
        match proto::Container::decode(&received.data[..]) {
            Ok(container) => {
                let receiver_id = match container.envelope.as_ref() {
                    Some(envelope) => match PeerId::from_bytes(&envelope.receiver_id) {
                        Ok(receiver_id) => receiver_id,
                        Err(e) => {
                            log::error!(
                                "invalid peer ID of message {}: {}",
                                bs58::encode(&container.signature).into_string(),
                                e
                            );
                            return;
                        }
                    },
                    None => return,
                };

                // check if message is local user account
                match UserAccounts::get_by_id(state,receiver_id) {
                    // we are the receiving node,
                    // process and save the message
                    Some(user_account) => {
                        MessagingProcess::process_received_message(state, user_account, container)
                    }

                    // schedule it for further sending otherwise
                    None => {
                        state.services.messaging.schedule_message(receiver_id, container, true, true, false, false)
                    }
                }
            }
            Err(e) => log::error!("Messaging container decoding error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pending(queued_at: u64) -> PendingPlaintext {
        PendingPlaintext {
            user_id: vec![1],
            receiver_id: vec![2],
            data: vec![3, 4, 5],
            message_type: MessagingServiceType::Chat,
            message_id: vec![6],
            needs_confirmation: true,
            queued_at,
        }
    }

    /// A message that cannot be encrypted yet (peer mid-handshake) is held in
    /// the in-memory queue rather than dropped, and survives until taken.
    /// Regression: such messages were lost (encrypt returned None and the send
    /// path returned Err before queuing anything).
    #[test]
    fn pending_plaintext_is_queued_and_drained() {
        let ms = MessagingState::new();
        ms.enqueue_pending_plaintext(pending(1000));
        ms.enqueue_pending_plaintext(pending(2000));

        let drained = ms.take_pending_plaintext();
        assert_eq!(drained.len(), 2);
        // draining empties the queue
        assert!(ms.take_pending_plaintext().is_empty());
    }

    /// The queue is bounded: items stuck longer than the max age are dropped,
    /// so a peer that never completes its handshake can't grow it without
    /// limit. Fresher items are retained.
    #[test]
    fn pending_plaintext_expires_when_too_old() {
        let ms = MessagingState::new();
        ms.enqueue_pending_plaintext(pending(1000)); // old
        ms.enqueue_pending_plaintext(pending(5_000_000)); // fresh

        // now = 5_001_000, max age = 1h: the 1000-stamped item is ~5_000_000ms
        // old (> 1h) and dropped; the 5_000_000-stamped item is ~1s old, kept.
        let dropped = ms.prune_expired_pending(5_001_000, 3_600_000);
        assert_eq!(dropped, 1);

        let remaining = ms.take_pending_plaintext();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].queued_at, 5_000_000);
    }

    /// A confirmable message's plaintext is persisted (so a retransmit can
    /// re-encrypt it across a session rotation) and removed once the message
    /// is confirmed — otherwise the store would grow without bound.
    #[test]
    fn outbound_plaintext_stored_and_cleared_on_confirm() {
        let ms = MessagingState::new();
        let sig = b"sig-1".to_vec();
        ms.save_outbound_plaintext(
            &sig,
            OutboundPlaintext {
                user_id: vec![1],
                receiver_id: vec![2],
                data: vec![3, 4, 5],
                message_type: MessagingServiceType::Chat,
                message_id: vec![6],
            },
        );
        let got = ms.get_outbound_plaintext(&sig).expect("stored");
        assert_eq!(got.data, vec![3, 4, 5]);

        // confirming the message must drop its retransmit plaintext
        ms.on_confirmed_message(&sig);
        assert!(
            ms.get_outbound_plaintext(&sig).is_none(),
            "plaintext must be cleared once the message is confirmed"
        );
    }
}
