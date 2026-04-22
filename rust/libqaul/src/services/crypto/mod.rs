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
use serde::{Deserialize, Serialize};

mod crypto25519;
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
                    pending_initiated_session_id: None,
                    draining_session_id: None,
                    draining_until: None,
                    draining_remaining_volume: None,
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
}
