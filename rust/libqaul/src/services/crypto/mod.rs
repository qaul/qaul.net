// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # End to End Encryption from user to user
//!
//! This module provides the end to end encryption functionality
//! for the messaging service.
//!
//! The cryptography is based on the Noise protocol.
//! qaul uses the `Noise_KK_25519_ChaChaPoly_SHA256` pattern.
//!
//! This file manages the encryption session.

use libp2p::PeerId;
use noise_rust_crypto::{ChaCha20Poly1305, Sha256, X25519};
use prost::Message;
use serde::{Deserialize, Serialize};

mod crypto25519;
pub mod events;
mod noise;
pub mod sessionmanager;
mod storage;

use rand::Rng;

use super::messaging;
use crate::node::user_accounts::UserAccount;
use crate::services::crypto::sessionmanager::CryptoSessionManager;
use crate::storage::configuration::Configuration;
use crate::utilities::timestamp::Timestamp;
pub use crypto25519::Crypto25519;
pub use noise::CryptoNoise;
pub use storage::CryptoAccount;
pub use storage::CryptoStorage;
pub use storage::CryptoStorageState;
use storage::RotationMeta;

/// The State Data of the Noise Protocol
#[derive(Clone, Serialize, Deserialize)]
pub struct CryptoState {
    /// Session ID
    ///
    /// The session ID is a random 4 byte u32 number
    pub session_id: u32,
    /// the state of this Noise of this
    pub state: CryptoProcessState,
    /// are we the initiator?
    pub initiator: bool,
    /// local static key
    pub s: Vec<u8>,
    /// remote static key
    pub rs: Vec<u8>,
    /// local ephemeral
    pub e: Vec<u8>,
    /// remote ephemeral
    pub re: Option<Vec<u8>>,
    /// Cipher key to encrypt outgoing messages
    pub cipher_out: Option<Vec<u8>>,
    /// nonce index for outgoing messages
    pub index_nonce_out: u64,
    /// cipher key to decrypt incoming messages
    ///
    /// As messages can arrive out of order, libqaul has
    /// to deal with the message index (= nonces) itself.
    pub cipher_in: Option<Vec<u8>>,
    /// highest message index of incoming messages
    pub highest_index_nonce_in: u64,
    /// Missing out of order message indexes
    ///
    /// These are indexes of messages that are lower then
    /// the highest message but have not arrived yet.
    /// Due to the delay tolerance of the system, this
    /// can happen.
    /// They shall be stored in the data base.
    /// Once we have a direct connection to the user, we
    /// can synchronize all messages and actively query for
    /// all missing messages.
    pub out_of_order_indexes: bool,
    /// Wall-clock timestamp (ms since epoch) at which this session
    /// transitioned to `Transport`. Drives the time-based rotation
    /// trigger (`CryptoRotation::period_seconds`). Zero for sessions
    /// persisted before this field existed; those sessions will rotate
    /// on the first outbound message after rotation is enabled.
    #[serde(default)]
    pub established_at: u64,
}

/// The State of Noise Protocol Handshake
#[derive(Clone, Serialize, Deserialize)]
pub enum CryptoProcessState {
    /// We sent a first handshake message,
    /// and we are still missing a return message.
    HalfOutgoing,
    /// We received a first handshake message,
    /// and we haven't sent the handshake return message.
    HalfIncoming,
    /// We are in transport mode.
    /// We have the symmetric en and decryption keys.
    Transport,
}

/// Crypto Module Structure
pub struct Crypto {}

impl Crypto {
    /// Initialize the crypto module at startup
    pub fn init() {
        // initialize the storage module
        CryptoStorage::init();
    }

    /// Encrypt an Outgoing Message
    ///
    /// This uses the `Noise_KK_X25519_ChaChaPoly_Sha256`
    /// to encrypt messages.
    /// It also takes care of the handshake messages
    /// and saves the handshake state to the data base.
    ///
    /// * data: the message data to encrypt
    /// * user_account: sender id
    /// * remote_id: receiver id
    ///
    /// The function returns the packed message on success,
    /// or none on failure.
    pub fn encrypt(
        state: &crate::QaulState,
        data: Vec<u8>,
        user_account: UserAccount,
        remote_id: PeerId,
    ) -> Option<messaging::proto::Encrypted> {
        let nonce: u64;
        let encrypted_option;
        let session_id: u32;
        let process_state: messaging::proto::CryptoState;

        // get data base object
        let crypto_account = CryptoStorage::get_db_ref(state, user_account.id.clone());

        // check if there is a handshake state?
        // Prefer the rotation_meta-designated primary when available
        // (avoids ambiguity while two Transport rows coexist mid-rotation).
        match Self::resolve_primary_state(&crypto_account, remote_id) {
            Some(session) => {
                // Remember whether we're in Transport — we'll decide
                // post-encrypt whether to fire a rotation.
                let was_transport = matches!(session.state, CryptoProcessState::Transport);

                // encrypt with existing crypto state
                if let Some((my_encrypted_option, my_nonce, my_session_id, my_process_state)) =
                    Self::encrypt_with_state(data, remote_id, crypto_account.clone(), session)
                {
                    encrypted_option = my_encrypted_option;
                    nonce = my_nonce;
                    session_id = my_session_id;
                    process_state = my_process_state;

                    // After a successful Transport encrypt, see if the
                    // time or outbound-volume trigger wants to kick off
                    // a rotation for this peer. This is a side-effect —
                    // it does not affect the message we just produced.
                    if was_transport {
                        Self::fire_rotation_if_triggered(
                            state,
                            &user_account,
                            &crypto_account,
                            remote_id,
                        );
                    }
                } else {
                    return None;
                }
            }
            None => {
                // create new session and start handshake
                (encrypted_option, nonce, session_id) =
                    CryptoNoise::encrypt_noise_kk_handshake_1::<
                        X25519,
                        ChaCha20Poly1305,
                        Sha256,
                        &[u8],
                    >(state, data, user_account, crypto_account, remote_id);

                log::trace!("encrypt with new session_id: {}", session_id);

                process_state = messaging::proto::CryptoState::Handshake;
            }
        }

        // create and return encrypted message
        if let Some(encrypted_data) = encrypted_option {
            return Some(Self::create_encrypted_protobuf(
                nonce,
                session_id,
                encrypted_data,
                process_state,
            ));
        }

        None
    }

    /// Return the `CryptoState` row that the caller should use as the
    /// primary session for this peer.
    ///
    /// Priority:
    /// 1. `rotation_meta.primary_session_id` if that row exists in the
    ///    crypto_state tree.
    /// 2. Otherwise fall back to `CryptoAccount::get_state` (the legacy
    ///    pre-rotation lookup).
    ///
    /// Returning the meta-designated row is important during the brief
    /// window in which a responder has completed KK step-2 but the
    /// initiator has not yet finalised: both sides have two `Transport`
    /// rows for the same peer, and `get_state`'s session-id-order
    /// iteration would otherwise pick non-deterministically between them.
    pub(super) fn resolve_primary_state(
        crypto_account: &CryptoAccount,
        remote_id: PeerId,
    ) -> Option<CryptoState> {
        if let Some(meta) = crypto_account.get_rotation_meta(remote_id) {
            if let Some(s) = crypto_account.get_state_by_id(remote_id, meta.primary_session_id) {
                return Some(s);
            }
        }
        crypto_account.get_state(remote_id)
    }

    /// Encrypt a message with a specific crypto state
    pub(super) fn encrypt_with_state(
        data: Vec<u8>,
        remote_id: PeerId,
        crypto_account: CryptoAccount,
        crypto_state: CryptoState,
    ) -> Option<(Option<Vec<u8>>, u64, u32, messaging::proto::CryptoState)> {
        let nonce: u64;
        let encrypted_option: Option<Vec<u8>>;
        let session_id: u32;
        let process_state: messaging::proto::CryptoState;

        log::trace!(
            "encrypt with existing session_id {}",
            crypto_state.session_id
        );

        // get session id
        session_id = crypto_state.session_id;

        // encrypt in accordance to session state
        match crypto_state.state {
            CryptoProcessState::HalfOutgoing => {
                log::trace!("session state HalfOutgoing");
                // we cannot send more messages at the moment, before we haven't
                // received the handshake confirmation.
                // TODO: build functionality to send further asymmetrically encrypted messages
                log::error!("Can't send further messages after handshake");
                return None;
            }
            CryptoProcessState::HalfIncoming => {
                log::trace!("session state HalfIncoming");
                // encrypt handshake 2 message
                (encrypted_option, nonce) =
                    CryptoNoise::encrypt_noise_kk_handshake_2::<
                        X25519,
                        ChaCha20Poly1305,
                        Sha256,
                        &[u8],
                    >(data, crypto_account, crypto_state, remote_id);

                process_state = messaging::proto::CryptoState::Handshake;
            }
            CryptoProcessState::Transport => {
                log::trace!("session state Transport");

                // encrypt transport message
                (encrypted_option, nonce) =
                    CryptoNoise::encrypt_noise_kk_transport::<
                        X25519,
                        ChaCha20Poly1305,
                        Sha256,
                        &[u8],
                    >(data, crypto_account, crypto_state, remote_id);

                process_state = messaging::proto::CryptoState::Transport;
            }
        }

        return Some((encrypted_option, nonce, session_id, process_state));
    }

    /// Create encrypted protobuf message
    pub(super) fn create_encrypted_protobuf(
        nonce: u64,
        session_id: u32,
        encrypted_data: Vec<u8>,
        process_state: messaging::proto::CryptoState,
    ) -> messaging::proto::Encrypted {
        let mut data_messages: Vec<messaging::proto::Data> = Vec::new();
        data_messages.push(messaging::proto::Data {
            nonce,
            data: encrypted_data,
        });

        return messaging::proto::Encrypted {
            state: process_state.into(),
            session_id,
            data: data_messages,
        };
    }

    /// Post-encrypt hook: if rotation is enabled and the current
    /// session has exceeded either the time-based
    /// (`cfg.crypto_rotation.period_seconds`) or the outbound-volume
    /// (`cfg.crypto_rotation.volume_messages`) trigger and no rotation
    /// is already in flight for this peer, start a new rotation by
    /// calling `CryptoNoise::rotate_initiate` and emitting the
    /// `RotateHandshakeFirst` as a `CryptoserviceContainer` under the
    /// currently-primary session.
    ///
    /// This is a fire-and-forget side effect: it logs and returns on
    /// failure. It must be called from `encrypt` (not
    /// `encrypt_with_state`), because the latter is reused by the
    /// inbound `SecondHandshake` response path and must not re-enter.
    fn fire_rotation_if_triggered(
        state: &crate::QaulState,
        user_account: &UserAccount,
        crypto_account: &CryptoAccount,
        remote_id: PeerId,
    ) {
        // Snapshot the relevant config bits and drop the read lock
        // before doing any rotation work — rotate_initiate /
        // pack_and_send_encrypted_data may take other libqaul locks.
        let (enabled, period_ms, volume_messages) = {
            let cfg = Configuration::get(state);
            (
                cfg.crypto_rotation.enabled,
                cfg.crypto_rotation.period_seconds.saturating_mul(1000),
                cfg.crypto_rotation.volume_messages,
            )
        };
        if !enabled {
            return;
        }

        // Read the (primary) state back after the encrypt — its
        // index_nonce_out has already been incremented. `get_state`
        // priority ranks Transport above HalfOutgoing, so if a new
        // HalfOutgoing row from an earlier in-flight rotation exists
        // we still pick the live primary here.
        let session = match crypto_account.get_state(remote_id) {
            Some(s) if matches!(s.state, CryptoProcessState::Transport) => s,
            _ => return,
        };

        let now_ms = Timestamp::get_timestamp();
        let age_ms = now_ms.saturating_sub(session.established_at);
        let time_fired = session.established_at != 0 && age_ms >= period_ms;
        let volume_fired = session.index_nonce_out >= volume_messages;

        if !(time_fired || volume_fired) {
            return;
        }

        // Don't launch a second rotation while one is already in flight.
        if let Some(meta) = crypto_account.get_rotation_meta(remote_id) {
            if meta.pending_initiated_session_id.is_some() {
                return;
            }
        }

        log::info!(
            "rotation trigger for peer {}: time_fired={} volume_fired={} age_ms={} nonce_out={}",
            remote_id.to_base58(),
            time_fired,
            volume_fired,
            age_ms,
            session.index_nonce_out
        );

        let rotate_first = match CryptoNoise::rotate_initiate::<
            X25519,
            ChaCha20Poly1305,
            Sha256,
            &[u8],
        >(state, user_account.clone(), crypto_account.clone(), remote_id)
        {
            Some(rf) => rf,
            None => {
                log::warn!("rotate_initiate returned None for {}", remote_id.to_base58());
                return;
            }
        };

        // Build the Messaging::CryptoService payload carrying the
        // rotate_first frame and encrypt it under the (still) primary
        // session. `rotate_initiate` created a new HalfOutgoing row for
        // the new session, but `get_state` priority puts Transport
        // (primary) above HalfOutgoing, so we pick the primary here.
        let payload = CryptoSessionManager::create_rotate_first_message(rotate_first);
        let primary = match crypto_account.get_state(remote_id) {
            Some(s) if matches!(s.state, CryptoProcessState::Transport) => s,
            _ => {
                log::warn!(
                    "fire_rotation_if_triggered: lost primary Transport for {}",
                    remote_id.to_base58()
                );
                return;
            }
        };
        let (encrypted_option, msg_nonce, sess_id, proc_state) = match Self::encrypt_with_state(
            payload,
            remote_id,
            crypto_account.clone(),
            primary,
        ) {
            Some(v) => v,
            None => {
                log::warn!(
                    "failed to encrypt rotate_first for {}",
                    remote_id.to_base58()
                );
                return;
            }
        };
        let encrypted_bytes = match encrypted_option {
            Some(b) => b,
            None => return,
        };

        let encrypted_message =
            Self::create_encrypted_protobuf(msg_nonce, sess_id, encrypted_bytes, proc_state);

        // Fresh 16-byte message_id for this CryptoService frame.
        let mut rng = rand::rng();
        let mut message_id = vec![0u8; 16];
        rng.fill(&mut message_id[..]);

        match messaging::Messaging::pack_and_send_encrypted_data(
            state,
            user_account,
            &remote_id,
            encrypted_message,
            &message_id,
            true,
        ) {
            Ok(_) => log::trace!("sent RotateHandshakeFirst to {}", remote_id.to_base58()),
            Err(e) => log::error!("failed sending rotate_first: {}", e),
        }
    }

    /// Post-decrypt rotation bookkeeping.
    ///
    /// Called once for every successfully decrypted Transport message
    /// in either the primary or draining session:
    ///
    /// - If the message was received on the **draining** session, we
    ///   consume one unit of `draining_remaining_volume`. When the
    ///   budget drops to zero the meta is rewritten so the next
    ///   `drain_expired_rotations` tick retires the row.
    /// - If the message was received on the **primary** session and
    ///   `highest_index_nonce_in` has crossed the inbound-volume
    ///   rotation trigger, fire a rotation from this (receiver) side
    ///   exactly like the send path does.
    fn after_decrypt_rotation(
        state: &crate::QaulState,
        user_account: &UserAccount,
        crypto_account: &CryptoAccount,
        remote_id: PeerId,
        session_id: u32,
    ) {
        let (enabled, volume_messages) = {
            let cfg = Configuration::get(state);
            (
                cfg.crypto_rotation.enabled,
                cfg.crypto_rotation.volume_messages,
            )
        };
        if !enabled {
            return;
        }

        let meta = match crypto_account.get_rotation_meta(remote_id) {
            Some(m) => m,
            None => {
                // No rotation activity yet, but an inbound-volume
                // trigger can still fire — fall through.
                RotationMeta {
                    primary_session_id: session_id,
                    ..Default::default()
                }
            }
        };

        // Draining volume decrement path.
        if Some(session_id) == meta.draining_session_id {
            let new_remaining = meta
                .draining_remaining_volume
                .unwrap_or(0)
                .saturating_sub(1);
            let updated = RotationMeta {
                draining_remaining_volume: Some(new_remaining),
                ..meta
            };
            crypto_account.save_rotation_meta(remote_id, &updated);
            return;
        }

        // Inbound-volume trigger path (message on the primary).
        if session_id != meta.primary_session_id {
            return;
        }
        if meta.pending_initiated_session_id.is_some() {
            return; // rotation already in flight from this side
        }
        let session = match crypto_account.get_state_by_id(remote_id, session_id) {
            Some(s) if matches!(s.state, CryptoProcessState::Transport) => s,
            _ => return,
        };
        if session.highest_index_nonce_in < volume_messages {
            return;
        }
        log::info!(
            "inbound-volume rotation trigger for peer {} (highest_in={})",
            remote_id.to_base58(),
            session.highest_index_nonce_in
        );
        Self::fire_rotation_if_triggered(state, user_account, crypto_account, remote_id);
    }

    /// Decrypt an incoming message
    ///
    /// This uses the `Noise_KK_X25519_ChaChaPoly_Sha256`
    /// to decrypt messages.
    /// It also takes care of the first handshake messages
    /// and saves the handshake state to the data base.
    ///
    /// * data: the encrypted data
    /// * nonce: the nonce of this message
    /// * user_account: sender id
    /// * remote_id: receiver id
    ///
    /// The function returns the decrypted data on success or none otherwise.
    pub fn decrypt(
        state: &crate::QaulState,
        message: messaging::proto::Encrypted,
        user_account: UserAccount,
        remote_id: PeerId,
        message_id: &Vec<u8>,
    ) -> Option<Vec<u8>> {
        // get data base object
        let crypto_account = CryptoStorage::get_db_ref(state, user_account.id.clone());

        log::trace!(
            "decrypt message\n\tmessage_id: {}\n\tsession_id: {}",
            bs58::encode(message_id).into_string(),
            message.session_id
        );

        // check if there is a handshake state?
        match crypto_account.get_state_by_id(remote_id, message.session_id) {
            Some(session) => {
                log::trace!("decrypt session id found: {}", session.session_id);

                // decide how to go further
                match (
                    messaging::proto::CryptoState::try_from(message.state),
                    session.state.clone(),
                ) {
                    (
                        Ok(messaging::proto::CryptoState::Handshake),
                        CryptoProcessState::HalfOutgoing,
                    ) => {
                        log::trace!("decrypt {}: second handshake", session.session_id);

                        // decrypt second handshake message
                        for data in message.data {
                            let message = CryptoNoise::decrypt_noise_kk_handshake_2::<
                                X25519,
                                ChaCha20Poly1305,
                                Sha256,
                                &[u8],
                            >(
                                data.data, session, crypto_account, remote_id
                            );

                            // return second handshake confirmation message

                            return message;
                        }
                    }
                    (
                        Ok(messaging::proto::CryptoState::Transport),
                        CryptoProcessState::Transport,
                    ) => {
                        log::trace!(
                            "decrypt session {}: decrypt transport message",
                            session.session_id
                        );

                        let session_id = session.session_id;

                        // decrypt transport message
                        for data in message.data {
                            let decrypted = CryptoNoise::decrypt_noise_kk_transport::<
                                X25519,
                                ChaCha20Poly1305,
                                Sha256,
                                &[u8],
                            >(
                                data.data,
                                data.nonce,
                                session,
                                crypto_account.clone(),
                                remote_id,
                            );

                            if decrypted.is_some() {
                                // Post-decrypt rotation bookkeeping: draining
                                // volume decrement, primary inbound-volume
                                // trigger check.
                                Self::after_decrypt_rotation(
                                    state,
                                    &user_account,
                                    &crypto_account,
                                    remote_id,
                                    session_id,
                                );
                            }
                            return decrypted;
                        }
                    }
                    (
                        Ok(messaging::proto::CryptoState::Transport),
                        CryptoProcessState::HalfOutgoing,
                    ) => {
                        log::trace!(
                            "decrypt session {}: saving incoming transport, as handshake is not completed",
                            session.session_id
                        );

                        // get nonce from message
                        let nonce;
                        if message.data.len() > 0 {
                            nonce = message.data[0].nonce;
                        } else {
                            return None;
                        }

                        // store in cache and wait until we got the handshake message
                        crypto_account.save_cache_message(
                            remote_id,
                            message.session_id,
                            nonce,
                            message,
                        );

                        return None;
                    }
                    _ => {
                        // Any other state is invalid
                        return None;
                    }
                }
            }
            None => {
                log::trace!("decrypt no session found");

                // Past-grace detection: if rotation_meta remembers
                // retiring *this* session_id, the sender used an
                // already-expired session and the UI should surface
                // "ask the sender to resend".
                if let Some(meta) = crypto_account.get_rotation_meta(remote_id) {
                    if Some(message.session_id) == meta.last_retired_session_id {
                        log::info!(
                            "dropping message on retired session {} from {}",
                            message.session_id,
                            remote_id.to_base58()
                        );
                        events::record(events::RotationEvent {
                            kind: events::RotationEventKind::MessageDroppedPastGrace,
                            remote_id,
                            primary_session_id: 0,
                            draining_session_id: message.session_id,
                            timestamp_ms: 0,
                        });
                        return None;
                    }
                }

                // check what kind of message we are getting
                match messaging::proto::CryptoState::try_from(message.state) {
                    Ok(messaging::proto::CryptoState::Handshake) => {
                        log::trace!("decrypt incoming first handshake");

                        // decrypt new handshake
                        for data in message.data {
                            if let Some((decrypted_data, crypto_state)) =
                                CryptoNoise::decrypt_noise_kk_handshake_1::<
                                    X25519,
                                    ChaCha20Poly1305,
                                    Sha256,
                                    &[u8],
                                >(
                                    state,
                                    data.data,
                                    crypto_account.clone(),
                                    remote_id,
                                    user_account.clone(),
                                    message.session_id,
                                )
                            {
                                // create confirmation messaging message
                                let messaging_message =
                                    CryptoSessionManager::create_second_handshake_message(
                                        message_id.to_owned(),
                                    );

                                // encrypt confirmation message
                                if let Some((Some(message), nonce, session_id, crypto_state)) =
                                    Self::encrypt_with_state(
                                        messaging_message,
                                        remote_id,
                                        crypto_account,
                                        crypto_state,
                                    )
                                {
                                    log::trace!(
                                        "create first handshake cryptoservice confirmation message"
                                    );
                                    let encrypted_message = Self::create_encrypted_protobuf(
                                        nonce,
                                        session_id,
                                        message,
                                        crypto_state,
                                    );

                                    // pack and send encrypted message
                                    match messaging::Messaging::pack_and_send_encrypted_data(
                                        state,
                                        &user_account,
                                        &remote_id,
                                        encrypted_message,
                                        message_id,
                                        true,
                                    ) {
                                        Ok(message_signature) => {
                                            log::trace!("sending cryptoservice secondhandshake message with\n\tsignature: {}", bs58::encode(message_signature).into_string());
                                        }
                                        Err(error_message) => log::error!(
                                            "failed sending 2nd handshake message {}",
                                            error_message
                                        ),
                                    }
                                } else {
                                    log::error!("failed encrypting cryptosession 2nd handshake confirmation");
                                }

                                // return decrypted first handshake message
                                return Some(decrypted_data);
                            } else {
                                // decryption of first handshake failed
                                return None;
                            }
                        }
                    }
                    _ => {
                        // Any other state is invalid
                        log::error!("decrypt: incoming transport state, with missing sesssion");
                        return None;
                    }
                }
            }
        }

        None
    }

    // ------------------------------------------------------------------
    //                         Crypto RPC handler
    // ------------------------------------------------------------------

    /// Dispatch a `Modules::Crypto` RPC request.
    ///
    /// Decodes the `Crypto` oneof and routes to the matching handler:
    /// `GetConfigRequest` reads the current `CryptoRotation` out of
    /// `Configuration`; `SetConfigRequest` applies a partial update
    /// (only present fields mutate), persists to `config.yaml`, and
    /// returns the updated state in `SetConfigResponse.applied`.
    pub fn rpc(data: Vec<u8>, _user_id: Vec<u8>, request_id: String) {
        use qaul_proto::qaul_rpc_crypto as proto_rpc;

        match proto_rpc::Crypto::decode(&data[..]) {
            Ok(msg) => match msg.message {
                Some(proto_rpc::crypto::Message::GetConfigRequest(_req)) => {
                    Self::handle_get_config(request_id);
                }
                Some(proto_rpc::crypto::Message::SetConfigRequest(req)) => {
                    Self::handle_set_config(req, request_id);
                }
                Some(proto_rpc::crypto::Message::GetEventsRequest(req)) => {
                    Self::handle_get_events(req, request_id);
                }
                Some(proto_rpc::crypto::Message::GetConfigResponse(_))
                | Some(proto_rpc::crypto::Message::SetConfigResponse(_))
                | Some(proto_rpc::crypto::Message::GetEventsResponse(_)) => {
                    // Responses are libqaul -> client only; clients
                    // that echo them back are ignored.
                    log::warn!("Crypto RPC received a response message from client; dropping");
                }
                None => log::error!("Crypto RPC message from client was empty"),
            },
            Err(e) => log::error!("Crypto RPC decode error: {}", e),
        }
    }

    fn handle_get_config(request_id: String) {
        use qaul_proto::qaul_rpc_crypto as proto_rpc;
        let snapshot = Self::snapshot_config();
        let out = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::GetConfigResponse(snapshot)),
        };
        crate::rpc::Rpc::send_message(
            out.encode_to_vec(),
            crate::rpc::proto::Modules::Crypto.into(),
            request_id,
            Vec::new(),
        );
    }

    fn handle_set_config(
        req: qaul_proto::qaul_rpc_crypto::SetConfigRequest,
        request_id: String,
    ) {
        use qaul_proto::qaul_rpc_crypto as proto_rpc;

        // Validate: every numeric field, when present, must be > 0.
        // Accepting zero would mean "rotate immediately on every
        // message" (period) or "retire draining on first message"
        // (grace) — almost certainly a client mistake.
        let validation_error = [
            ("period_seconds", req.period_seconds),
            ("volume_messages", req.volume_messages),
            ("grace_period_seconds", req.grace_period_seconds),
            ("grace_volume_messages", req.grace_volume_messages),
        ]
        .into_iter()
        .find_map(|(name, value)| match value {
            Some(0) => Some(format!("{} must be > 0", name)),
            _ => None,
        });

        if let Some(err) = validation_error {
            let applied = Self::snapshot_config();
            let resp = proto_rpc::Crypto {
                message: Some(proto_rpc::crypto::Message::SetConfigResponse(
                    proto_rpc::SetConfigResponse {
                        success: false,
                        error: err,
                        applied: Some(applied),
                    },
                )),
            };
            crate::rpc::Rpc::send_message(
                resp.encode_to_vec(),
                crate::rpc::proto::Modules::Crypto.into(),
                request_id,
                Vec::new(),
            );
            return;
        }

        // Apply the partial update.
        {
            let mut cfg = Configuration::get_mut();
            if let Some(v) = req.enabled {
                cfg.crypto_rotation.enabled = v;
            }
            if let Some(v) = req.period_seconds {
                cfg.crypto_rotation.period_seconds = v;
            }
            if let Some(v) = req.volume_messages {
                cfg.crypto_rotation.volume_messages = v;
            }
            if let Some(v) = req.grace_period_seconds {
                cfg.crypto_rotation.grace_period_seconds = v;
            }
            if let Some(v) = req.grace_volume_messages {
                cfg.crypto_rotation.grace_volume_messages = v;
            }
        }

        // Persist to disk (skipped under cfg(test): the test fixture
        // installs a config directly and never invokes the Storage
        // path).
        #[cfg(not(test))]
        Configuration::save();

        let applied = Self::snapshot_config();
        let resp = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::SetConfigResponse(
                proto_rpc::SetConfigResponse {
                    success: true,
                    error: String::new(),
                    applied: Some(applied),
                },
            )),
        };
        crate::rpc::Rpc::send_message(
            resp.encode_to_vec(),
            crate::rpc::proto::Modules::Crypto.into(),
            request_id,
            Vec::new(),
        );
    }

    fn handle_get_events(
        req: qaul_proto::qaul_rpc_crypto::GetRotationEventsRequest,
        request_id: String,
    ) {
        use qaul_proto::qaul_rpc_crypto as proto_rpc;
        let limit = req.limit as usize;
        let events = events::query(req.since_ms, limit);
        let proto_events: Vec<proto_rpc::RotationEvent> = events
            .into_iter()
            .map(|e| proto_rpc::RotationEvent {
                timestamp_ms: e.timestamp_ms,
                kind: match e.kind {
                    events::RotationEventKind::Rotated => {
                        proto_rpc::RotationEventKind::Rotated as i32
                    }
                    events::RotationEventKind::GraceExpired => {
                        proto_rpc::RotationEventKind::GraceExpired as i32
                    }
                    events::RotationEventKind::MessageDroppedPastGrace => {
                        proto_rpc::RotationEventKind::MessageDroppedPastGrace as i32
                    }
                },
                remote_id: e.remote_id.to_bytes(),
                primary_session_id: e.primary_session_id,
                draining_session_id: e.draining_session_id,
            })
            .collect();
        let resp = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::GetEventsResponse(
                proto_rpc::GetRotationEventsResponse {
                    events: proto_events,
                },
            )),
        };
        crate::rpc::Rpc::send_message(
            resp.encode_to_vec(),
            crate::rpc::proto::Modules::Crypto.into(),
            request_id,
            Vec::new(),
        );
    }

    /// Snapshot the current `CryptoRotation` into a proto response.
    fn snapshot_config() -> qaul_proto::qaul_rpc_crypto::GetConfigResponse {
        let cfg = Configuration::get();
        qaul_proto::qaul_rpc_crypto::GetConfigResponse {
            enabled: cfg.crypto_rotation.enabled,
            period_seconds: cfg.crypto_rotation.period_seconds,
            volume_messages: cfg.crypto_rotation.volume_messages,
            grace_period_seconds: cfg.crypto_rotation.grace_period_seconds,
            grace_volume_messages: cfg.crypto_rotation.grace_volume_messages,
        }
    }
}

#[cfg(test)]
mod phase2_tests {
    //! Phase 2 unit tests — exercise the new rotation-aware helpers
    //! (`resolve_primary_state`, `after_decrypt_rotation`) against
    //! in-memory sled storage and a test-only configuration.
    //!
    //! End-to-end rotation tests (full Noise handshake, cross-peer
    //! dispatch, replay / collision) require the global libqaul stack
    //! and live in plan.md Phase 4 (`tests/integration/local_mesh.py`).

    use super::*;
    use crate::services::crypto::storage::{CryptoStorage, RotationMeta};
    use crate::storage::configuration::{Configuration, CryptoRotation};
    use libp2p::identity::Keypair;
    use std::sync::Once;

    static CONFIG_INIT: Once = Once::new();

    /// Install a test `Configuration` with rotation enabled once per
    /// process. Idempotent.
    ///
    /// We can't use `Configuration::default()` here: `Internet::default`
    /// pulls values out of the `DEFCONFIGS` global which is only set
    /// by `Libqaul::new`. Build the struct literally from the
    /// sub-module `::default()`s that *are* self-contained.
    pub(super) fn install_test_config() {
        use crate::storage::configuration::{
            DebugOption, Internet, Lan, Node, RoutingOptions,
        };
        CONFIG_INIT.call_once(|| {
            let cfg = Configuration {
                node: Node {
                    initialized: 0,
                    id: String::new(),
                    keys: String::new(),
                },
                lan: Lan {
                    active: false,
                    listen: Vec::new(),
                },
                internet: Internet {
                    active: false,
                    peers: Vec::new(),
                    do_listen: false,
                    listen: Vec::new(),
                },
                user_accounts: Vec::new(),
                debug: DebugOption { log: false },
                routing: RoutingOptions::default(),
                crypto_rotation: CryptoRotation {
                    enabled: true,
                    period_seconds: 7 * 24 * 3600,
                    volume_messages: 1_000_000,
                    grace_period_seconds: 3600,
                    grace_volume_messages: 256,
                },
            };
            Configuration::init_for_tests(cfg);
        });
    }

    fn fresh_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    fn dummy_state(session_id: u32) -> CryptoState {
        CryptoState {
            session_id,
            state: CryptoProcessState::Transport,
            initiator: true,
            s: vec![],
            rs: vec![],
            e: vec![],
            re: None,
            cipher_out: Some(vec![0u8; 32]),
            index_nonce_out: 0,
            cipher_in: Some(vec![0u8; 32]),
            highest_index_nonce_in: 0,
            out_of_order_indexes: false,
            established_at: 0,
        }
    }

    // ------------------------------------------------------------------
    //                       resolve_primary_state
    // ------------------------------------------------------------------

    // When rotation_meta names a primary and that state row exists,
    // resolve_primary_state must return *that* row, even if the tree
    // has multiple Transport rows for the same peer (the post-
    // responder-step window).
    #[test]
    fn resolve_primary_prefers_meta_designated_row() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 10, dummy_state(10));
        acct.save_state(remote, 20, dummy_state(20));
        acct.save_rotation_meta(remote, &RotationMeta::primary_only(20));

        let resolved = Crypto::resolve_primary_state(&acct, remote).unwrap();
        assert_eq!(resolved.session_id, 20);
    }

    // With no rotation_meta row, fall back to the legacy get_state.
    #[test]
    fn resolve_primary_falls_back_without_meta() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));

        let resolved = Crypto::resolve_primary_state(&acct, remote).unwrap();
        assert_eq!(resolved.session_id, 7);
    }

    // If the meta-designated primary session has no matching state
    // row (stale meta, interrupted write, etc.), fall back rather
    // than surface `None`.
    #[test]
    fn resolve_primary_ignores_missing_state_for_meta_primary() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));
        acct.save_rotation_meta(remote, &RotationMeta::primary_only(42));

        let resolved = Crypto::resolve_primary_state(&acct, remote).unwrap();
        assert_eq!(resolved.session_id, 7, "should fall back to existing row");
    }

    // ------------------------------------------------------------------
    //                       after_decrypt_rotation
    // ------------------------------------------------------------------

    fn dummy_user_account() -> UserAccount {
        let keys = Keypair::generate_ed25519();
        let id = keys.public().to_peer_id();
        UserAccount {
            id,
            keys,
            name: "test".into(),
            password_hash: None,
            password_salt: None,
        }
    }

    // A message decrypted on the draining session decrements the
    // remaining-volume budget.
    #[test]
    fn after_decrypt_decrements_draining_volume() {
        install_test_config();
        let acct = CryptoStorage::test_account();
        let user_account = dummy_user_account();
        let remote = fresh_peer();
        acct.save_state(remote, 50, dummy_state(50));
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 100,
                draining_session_id: Some(50),
                draining_until: Some(u64::MAX),
                draining_remaining_volume: Some(10),
                ..Default::default()
            },
        );

        Crypto::after_decrypt_rotation(&user_account, &acct, remote, 50);

        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.draining_remaining_volume, Some(9));
        // Primary fields untouched.
        assert_eq!(meta.primary_session_id, 100);
        assert_eq!(meta.draining_session_id, Some(50));
    }

    // Decrementing a budget already at zero saturates (no underflow).
    #[test]
    fn after_decrypt_saturates_at_zero() {
        install_test_config();
        let acct = CryptoStorage::test_account();
        let user_account = dummy_user_account();
        let remote = fresh_peer();
        acct.save_state(remote, 50, dummy_state(50));
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 100,
                draining_session_id: Some(50),
                draining_until: Some(u64::MAX),
                draining_remaining_volume: Some(0),
                ..Default::default()
            },
        );

        Crypto::after_decrypt_rotation(&user_account, &acct, remote, 50);

        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.draining_remaining_volume, Some(0));
    }

    // When rotation is disabled in config, the draining branch never
    // runs — nothing mutates. We can only exercise this when the
    // shared `install_test_config` *hasn't* run; use a fresh account
    // without calling `install_test_config`. But because the InitCell
    // is process-wide, once a previous test installs the enabled
    // config it stays enabled. Instead, assert behaviour under the
    // normal enabled path when session_id doesn't match any role:
    // no mutation should happen.
    #[test]
    fn after_decrypt_noop_on_unrelated_session() {
        install_test_config();
        let acct = CryptoStorage::test_account();
        let user_account = dummy_user_account();
        let remote = fresh_peer();
        acct.save_state(remote, 99, dummy_state(99));
        let original = RotationMeta {
            primary_session_id: 100,
            draining_session_id: Some(50),
            draining_until: Some(u64::MAX),
            draining_remaining_volume: Some(10),
            ..Default::default()
        };
        acct.save_rotation_meta(remote, &original);

        // session_id 99 matches neither primary (100) nor draining
        // (50) → no mutation.
        Crypto::after_decrypt_rotation(&user_account, &acct, remote, 99);

        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta, original);
    }
}

#[cfg(test)]
mod phase3_tests {
    //! Phase 3 unit tests — exercise `Crypto::rpc` against installed
    //! test configuration and a live RPC channel.
    //!
    //! The RPC-config mutation tests share process-global state
    //! (`CONFIG`, `EXTERN_RECEIVE`) with Phase 2 tests, so all
    //! mutating tests take a module-scoped `CONFIG_LOCK` and revert
    //! their changes before releasing it.

    use super::phase2_tests::install_test_config;
    use super::*;
    use crate::rpc::Rpc;
    use qaul_proto::qaul_rpc_crypto as proto_rpc;
    use std::sync::{Mutex, Once};

    /// Serialise RPC-config tests so one test's SetConfigRequest
    /// doesn't race another test's GetConfigRequest assertion.
    pub(super) static CONFIG_LOCK: Mutex<()> = Mutex::new(());
    static RPC_INIT: Once = Once::new();

    pub(super) fn install_rpc_for_tests() {
        RPC_INIT.call_once(|| {
            let _ = Rpc::init();
        });
    }

    /// Drop every pending libqaul->extern message so the test's
    /// own response is the first thing we pick up.
    pub(super) fn drain_rpc_channel() {
        while Rpc::receive_from_libqaul().is_ok() {}
    }

    /// Invoke `Crypto::rpc` with an encoded Crypto RPC container and
    /// read back the one response it emits, decoded as
    /// `proto_rpc::Crypto`.
    pub(super) fn rpc_round_trip(req: proto_rpc::Crypto) -> proto_rpc::Crypto {
        drain_rpc_channel();
        Crypto::rpc(req.encode_to_vec(), Vec::new(), "test-req".into());
        let raw = Rpc::receive_from_libqaul().expect("no RPC response was produced");
        let envelope = crate::rpc::proto::QaulRpc::decode(&raw[..]).expect("QaulRpc decode");
        assert_eq!(
            envelope.module,
            crate::rpc::proto::Modules::Crypto as i32,
            "response module should be Crypto"
        );
        assert_eq!(envelope.request_id, "test-req");
        proto_rpc::Crypto::decode(&envelope.data[..]).expect("Crypto decode")
    }

    /// `GetConfigRequest` must round-trip back the currently-installed
    /// `CryptoRotation` values.
    #[test]
    fn rpc_get_config_returns_installed_config() {
        let _g = CONFIG_LOCK.lock().unwrap();
        install_test_config();
        install_rpc_for_tests();

        let req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::GetConfigRequest(
                proto_rpc::GetConfigRequest {},
            )),
        };
        let resp = rpc_round_trip(req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::GetConfigResponse(r)) => r,
            other => panic!("expected GetConfigResponse, got {:?}", other.is_some()),
        };

        // Fields should match whatever `install_test_config` baked in:
        // volume/period/grace values from the shared fixture.
        assert!(body.enabled);
        assert_eq!(body.period_seconds, 7 * 24 * 3600);
        assert_eq!(body.volume_messages, 1_000_000);
        assert_eq!(body.grace_period_seconds, 3600);
        assert_eq!(body.grace_volume_messages, 256);
    }

    /// A `SetConfigRequest` with only `period_seconds` set must leave
    /// every other field untouched and report the new value via
    /// `SetConfigResponse.applied`. The test restores the original
    /// period before releasing the lock so it doesn't pollute any
    /// subsequent test that also reads the config.
    #[test]
    fn rpc_set_config_partial_update_preserves_other_fields() {
        let _g = CONFIG_LOCK.lock().unwrap();
        install_test_config();
        install_rpc_for_tests();

        // snapshot prior state so we can revert after.
        let original_period = Configuration::get().crypto_rotation.period_seconds;

        let new_period = original_period.saturating_add(123);
        let set_req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::SetConfigRequest(
                proto_rpc::SetConfigRequest {
                    enabled: None,
                    period_seconds: Some(new_period),
                    volume_messages: None,
                    grace_period_seconds: None,
                    grace_volume_messages: None,
                },
            )),
        };
        let resp = rpc_round_trip(set_req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::SetConfigResponse(r)) => r,
            other => panic!("expected SetConfigResponse, got {:?}", other.is_some()),
        };
        assert!(body.success, "expected success, got error: {}", body.error);
        let applied = body.applied.expect("applied config should be populated");
        assert_eq!(applied.period_seconds, new_period);
        assert_eq!(applied.volume_messages, 1_000_000, "untouched");
        assert_eq!(applied.grace_period_seconds, 3600, "untouched");
        assert!(applied.enabled, "untouched");

        // revert for isolation.
        let revert = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::SetConfigRequest(
                proto_rpc::SetConfigRequest {
                    period_seconds: Some(original_period),
                    ..Default::default()
                },
            )),
        };
        let _ = rpc_round_trip(revert);
    }

    /// Zero-valued numeric fields are a near-certain client mistake
    /// (rotate immediately on every message, retire draining on
    /// first message). The handler rejects them and echoes the
    /// unchanged config back in `applied`.
    #[test]
    fn rpc_set_config_rejects_zero_fields() {
        let _g = CONFIG_LOCK.lock().unwrap();
        install_test_config();
        install_rpc_for_tests();

        let original = Configuration::get().crypto_rotation.clone();

        let req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::SetConfigRequest(
                proto_rpc::SetConfigRequest {
                    period_seconds: Some(0),
                    ..Default::default()
                },
            )),
        };
        let resp = rpc_round_trip(req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::SetConfigResponse(r)) => r,
            _ => panic!("expected SetConfigResponse"),
        };
        assert!(!body.success);
        assert!(
            body.error.contains("period_seconds"),
            "error should mention the offending field, got: {}",
            body.error
        );
        let applied = body.applied.unwrap();
        // Unchanged — the handler must not have mutated the config.
        assert_eq!(applied.period_seconds, original.period_seconds);
        assert_eq!(applied.volume_messages, original.volume_messages);
    }
}

#[cfg(test)]
mod phase3_events_tests {
    //! Phase 3 event-surface unit tests — ring buffer behaviour,
    //! drain emission, past-grace detection, and the
    //! `GetRotationEventsRequest` round trip.
    //!
    //! Every test here touches the process-global event-log
    //! `InitCell`, so they share an `EVENT_LOG_LOCK` to serialise
    //! mutations that are observed by subsequent assertions.

    use super::phase2_tests::install_test_config;
    use super::phase3_tests;
    use super::*;
    use crate::services::crypto::events;
    use libp2p::identity::Keypair;
    use qaul_proto::qaul_rpc_crypto as proto_rpc;
    use std::sync::Mutex;

    static EVENT_LOG_LOCK: Mutex<()> = Mutex::new(());

    fn fresh_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    // The ring buffer appends in order and caps at MAX_EVENTS,
    // dropping the oldest.
    #[test]
    fn event_log_caps_at_max_events() {
        let _g = EVENT_LOG_LOCK.lock().unwrap();
        events::clear_for_tests();
        let peer = fresh_peer();
        // Stream past the cap.
        for i in 0..(events::MAX_EVENTS + 50) {
            events::record(events::RotationEvent {
                kind: events::RotationEventKind::Rotated,
                remote_id: peer,
                primary_session_id: i as u32,
                draining_session_id: 0,
                timestamp_ms: 1_000_000 + i as u64,
            });
        }
        let all = events::query(0, 0);
        assert_eq!(all.len(), events::MAX_EVENTS, "log should cap at MAX_EVENTS");
        // Oldest events dropped: the smallest primary_session_id in
        // the survivors must equal 50 (since we pushed 0..256+50).
        assert_eq!(all.first().unwrap().primary_session_id, 50);
        assert_eq!(
            all.last().unwrap().primary_session_id,
            (events::MAX_EVENTS + 50 - 1) as u32
        );
    }

    // `since_ms` filters events strictly older, `limit` caps to the
    // newest `limit` entries.
    #[test]
    fn event_log_query_filters_and_limits() {
        let _g = EVENT_LOG_LOCK.lock().unwrap();
        events::clear_for_tests();
        let peer = fresh_peer();
        for i in 0..10 {
            events::record(events::RotationEvent {
                kind: events::RotationEventKind::Rotated,
                remote_id: peer,
                primary_session_id: i,
                draining_session_id: 0,
                timestamp_ms: 1_000 + i as u64,
            });
        }
        // since_ms filter
        let filtered = events::query(1_005, 0);
        assert_eq!(filtered.len(), 5, "events at ts 1005..1009");
        assert_eq!(filtered.first().unwrap().primary_session_id, 5);

        // limit filter keeps the newest 3.
        let limited = events::query(0, 3);
        assert_eq!(limited.len(), 3);
        assert_eq!(limited.first().unwrap().primary_session_id, 7);
        assert_eq!(limited.last().unwrap().primary_session_id, 9);
    }

    // `drain_expired_rotations` emits `GraceExpired` and stamps
    // `last_retired_*` so the decrypt path can detect past-grace
    // messages afterwards.
    #[test]
    fn drain_emits_grace_expired_and_stamps_meta() {
        let _g = EVENT_LOG_LOCK.lock().unwrap();
        install_test_config();
        events::clear_for_tests();
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 42,
                draining_session_id: Some(7),
                draining_until: Some(10_000),
                draining_remaining_volume: Some(100),
                ..Default::default()
            },
        );
        CryptoNoise::drain_expired_rotations(acct.clone(), 10_001);

        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.last_retired_session_id, Some(7));
        assert_eq!(meta.last_retired_at, Some(10_001));

        let log = events::query(0, 0);
        assert!(
            log.iter()
                .any(|e| e.kind == events::RotationEventKind::GraceExpired
                    && e.draining_session_id == 7),
            "GraceExpired event was not emitted; log={:?}",
            log
        );
    }

    // Local helper: minimal CryptoState fixture (same as phase2_tests).
    fn dummy_state(session_id: u32) -> CryptoState {
        CryptoState {
            session_id,
            state: CryptoProcessState::Transport,
            initiator: true,
            s: vec![],
            rs: vec![],
            e: vec![],
            re: None,
            cipher_out: Some(vec![0u8; 32]),
            index_nonce_out: 0,
            cipher_in: Some(vec![0u8; 32]),
            highest_index_nonce_in: 0,
            out_of_order_indexes: false,
            established_at: 0,
        }
    }

    // `GetRotationEventsRequest` returns whatever the log currently
    // holds.
    #[test]
    fn rpc_get_events_returns_recorded_events() {
        // Both CONFIG_LOCK (RPC channel) and EVENT_LOG_LOCK (the
        // event log) are held; this test needs exclusive access to
        // each. Always acquire CONFIG_LOCK first to avoid lock-
        // ordering inversions with tests that only take CONFIG_LOCK.
        let _g_cfg = phase3_tests::CONFIG_LOCK.lock().unwrap();
        let _g_log = EVENT_LOG_LOCK.lock().unwrap();
        install_test_config();
        phase3_tests::install_rpc_for_tests();
        events::clear_for_tests();
        phase3_tests::drain_rpc_channel();

        let peer = fresh_peer();
        events::record(events::RotationEvent {
            kind: events::RotationEventKind::Rotated,
            remote_id: peer,
            primary_session_id: 5,
            draining_session_id: 3,
            timestamp_ms: 42_000,
        });

        let req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::GetEventsRequest(
                proto_rpc::GetRotationEventsRequest {
                    since_ms: 0,
                    limit: 0,
                },
            )),
        };
        let resp = phase3_tests::rpc_round_trip(req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::GetEventsResponse(r)) => r,
            _ => panic!("expected GetEventsResponse"),
        };
        assert!(
            !body.events.is_empty(),
            "expected at least one event; got {:?}",
            body.events
        );
        let e = body.events.last().unwrap();
        assert_eq!(e.timestamp_ms, 42_000);
        assert_eq!(e.primary_session_id, 5);
        assert_eq!(e.draining_session_id, 3);
        assert_eq!(e.kind, proto_rpc::RotationEventKind::Rotated as i32);
        assert_eq!(e.remote_id, peer.to_bytes());
    }
}
