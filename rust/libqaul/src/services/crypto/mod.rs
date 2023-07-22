// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
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

use super::messaging;
use crate::node::user_accounts::UserAccount;
use crate::services::crypto::sessionmanager::CryptoSessionManager;
pub use crypto25519::Crypto25519;
pub use noise::CryptoNoise;
pub use storage::CryptoAccount;
pub use storage::CryptoStorage;

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
        data: Vec<u8>,
        user_account: UserAccount,
        remote_id: PeerId,
    ) -> Option<messaging::proto::Encrypted> {
        let nonce: u64;
        let encrypted_option;
        let session_id: u32;
        let process_state: messaging::proto::CryptoState;

        // get data base object
        let crypto_account = CryptoStorage::get_db_ref(user_account.id.clone());

        // check if there is a handshake state?
        match crypto_account.get_state(remote_id) {
            Some(session) => {
                // encrypt with existing crypto state
                if let Some((my_encrypted_option, my_nonce, my_session_id, my_process_state)) =
                    Self::encrypt_with_state(data, remote_id, crypto_account, session)
                {
                    encrypted_option = my_encrypted_option;
                    nonce = my_nonce;
                    session_id = my_session_id;
                    process_state = my_process_state;
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
                    >(data, user_account, crypto_account, remote_id);

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

    /// Encrypt a message with a specific crypto state
    fn encrypt_with_state(
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
    fn create_encrypted_protobuf(
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
        message: messaging::proto::Encrypted,
        user_account: UserAccount,
        remote_id: PeerId,
        message_id: &Vec<u8>,
    ) -> Option<Vec<u8>> {
        // get data base object
        let crypto_account = CryptoStorage::get_db_ref(user_account.id.clone());

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
                    messaging::proto::CryptoState::from_i32(message.state),
                    session.state.clone(),
                ) {
                    (
                        Some(messaging::proto::CryptoState::Handshake),
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
                        Some(messaging::proto::CryptoState::Transport),
                        CryptoProcessState::Transport,
                    ) => {
                        log::trace!(
                            "decrypt session {}: decrypt transport message",
                            session.session_id
                        );

                        // decrypt transport message
                        for data in message.data {
                            return CryptoNoise::decrypt_noise_kk_transport::<
                                X25519,
                                ChaCha20Poly1305,
                                Sha256,
                                &[u8],
                            >(
                                data.data, data.nonce, session, crypto_account, remote_id
                            );
                        }
                    }
                    (
                        Some(messaging::proto::CryptoState::Transport),
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
                match messaging::proto::CryptoState::from_i32(message.state) {
                    Some(messaging::proto::CryptoState::Handshake) => {
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
