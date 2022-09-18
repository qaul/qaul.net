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
mod storage;

use super::messaging::proto;
use crate::node::user_accounts::UserAccount;
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
    ) -> Option<proto::Encrypted> {
        let nonce: u64;
        let encrypted_option: Option<Vec<u8>>;
        let session_id: u32;
        let process_state: proto::CryptoState;

        // get data base object
        let crypto_account = CryptoStorage::get_db_ref(user_account.id.clone());

        // check if there is a handshake state?
        match crypto_account.get_state(remote_id) {
            Some(session) => {
                log::trace!("encrypt with existing session_id {}", session.session_id);

                // get session id
                session_id = session.session_id;

                // encrypt in accordance to session state
                match session.state {
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
                            >(data, crypto_account, session, remote_id);

                        process_state = proto::CryptoState::Handshake;
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
                            >(data, crypto_account, session, remote_id);

                        process_state = proto::CryptoState::Transport;
                    }
                }
            }
            None => {
                log::trace!("encrypt with new session");

                // create new session and start handshake
                (encrypted_option, nonce, session_id) =
                    CryptoNoise::encrypt_noise_kk_handshake_1::<
                        X25519,
                        ChaCha20Poly1305,
                        Sha256,
                        &[u8],
                    >(data, user_account, crypto_account, remote_id);

                process_state = proto::CryptoState::Handshake;
            }
        }
        // create and return encrypted message
        if let Some(encrypted_data) = encrypted_option {
            let mut data_messages: Vec<proto::Data> = Vec::new();
            data_messages.push(proto::Data {
                nonce,
                data: encrypted_data,
            });

            return Some(proto::Encrypted {
                state: process_state.into(),
                session_id,
                data: data_messages,
            });
        }

        None
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
        message: proto::Encrypted,
        user_account: UserAccount,
        remote_id: PeerId,
    ) -> Option<Vec<u8>> {
        // get data base object
        let crypto_account = CryptoStorage::get_db_ref(user_account.id.clone());

        log::trace!("decrypt session_id: {}", message.session_id);

        // check if there is a handshake state?
        match crypto_account.get_state_by_id(remote_id, message.session_id) {
            Some(session) => {
                log::trace!("decrypt session id found: {}", session.session_id);

                // decide how to go further
                match (
                    proto::CryptoState::from_i32(message.state),
                    session.state.clone(),
                ) {
                    (Some(proto::CryptoState::Handshake), CryptoProcessState::HalfOutgoing) => {
                        log::debug!("decrypt {}: second handshake", session.session_id);

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

                            // TODO: check if there are unprocessed messages in cache

                            return message;
                        }
                    }
                    (Some(proto::CryptoState::Transport), CryptoProcessState::Transport) => {
                        log::debug!(
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
                    (Some(proto::CryptoState::Transport), CryptoProcessState::HalfOutgoing) => {
                        log::debug!(
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
                match proto::CryptoState::from_i32(message.state) {
                    Some(proto::CryptoState::Handshake) => {
                        log::debug!("decrypt incoming first handshake");

                        // decrypt new handshake
                        for data in message.data {
                            return CryptoNoise::decrypt_noise_kk_handshake_1::<
                                X25519,
                                ChaCha20Poly1305,
                                Sha256,
                                &[u8],
                            >(
                                data.data,
                                crypto_account,
                                remote_id,
                                user_account,
                                message.session_id,
                            );
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
