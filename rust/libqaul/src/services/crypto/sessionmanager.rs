// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Crypto Session Manager
//!
//! Handling of the crypto sessions, such as session confirmation.
//!
//! The crypto session manager uses the crypto_net protobuf file
//! containing the Cryptoservice messages.
//! All cryptoservice protobuf messages need to be confirmed by
//! the receiver.

use libp2p::PeerId;
use noise_rust_crypto::{ChaCha20Poly1305, Sha256, X25519};
use prost::Message;
use rand::Rng;

use crate::node::user_accounts::UserAccount;
use crate::services::crypto::{Crypto, CryptoNoise, CryptoProcessState, CryptoStorage};
use crate::services::messaging;
use crate::utilities::timestamp::Timestamp;

/// Import protobuf crypto service definition
pub use qaul_proto::qaul_net_crypto as proto_net;

/// Crypto Session Management
#[derive(Clone)]
pub struct CryptoSessionManager {}

impl CryptoSessionManager {
    /// decode and process crypto session protobuf messages
    pub fn process_cryptoservice_container(
        state: &crate::QaulState,
        sender_id: &PeerId,
        user_account: UserAccount,
        data: Vec<u8>,
    ) {
        log::trace!("process_cryptoservice_container");
        // decode protobuf cryptoservice message container
        match proto_net::CryptoserviceContainer::decode(&data[..]) {
            Ok(cryptoservice_container) => match cryptoservice_container.message {
                Some(proto_net::cryptoservice_container::Message::SecondHandshake(
                    second_handshake,
                )) => {
                    Self::process_second_handshake(state, &user_account, sender_id, second_handshake);
                }
                Some(proto_net::cryptoservice_container::Message::RotateFirst(rotate_first)) => {
                    Self::process_rotate_first(state, &user_account, sender_id, rotate_first);
                }
                Some(proto_net::cryptoservice_container::Message::RotateSecond(rotate_second)) => {
                    Self::process_rotate_second(state, &user_account, sender_id, rotate_second);
                }
                None => {
                    log::error!(
                        "Cryptoservice message from {} was empty",
                        sender_id.to_base58()
                    )
                }
            },
            Err(e) => {
                log::error!(
                    "Error decoding Cryptoservice Message from {} to {}: {}",
                    sender_id.to_base58(),
                    user_account.id.to_base58(),
                    e
                );
            }
        }
    }

    /// process second hand shake
    fn process_second_handshake(
        state: &crate::QaulState,
        user_account: &UserAccount,
        sender_id: &PeerId,
        second_handshake: proto_net::SecondHandshake,
    ) {
        log::trace!("process_second_handshake");

        // confirm reception of the message
        messaging::Messaging::on_confirmed_message(
            state,
            &second_handshake.signature,
            sender_id.to_owned(),
            user_account.to_owned(),
            messaging::proto::Confirmation {
                signature: second_handshake.signature.clone(),
                received_at: second_handshake.received_at,
            },
        );
    }

    /// Handle an incoming `RotateHandshakeFirst`.
    ///
    /// Runs `CryptoNoise::rotate_complete_responder`, which:
    /// - applies the collision-resolution rule (lower session_id wins);
    /// - runs KK step 2 under the new session_id, transitioning the
    ///   new `CryptoState` to `Transport`;
    /// - updates `rotation_meta` so the new session is primary and
    ///   the old session is the draining row.
    ///
    /// On success we encrypt the resulting `RotateHandshakeSecond`
    /// **under the old (now-draining) session** — the initiator has
    /// not yet promoted, so that is still its primary. Using the
    /// draining row here is a one-shot rotation-completion operation;
    /// no regular user payload is ever sent under draining.
    fn process_rotate_first(
        state: &crate::QaulState,
        user_account: &UserAccount,
        sender_id: &PeerId,
        rotate_first: proto_net::RotateHandshakeFirst,
    ) {
        let crypto_account = CryptoStorage::get_db_ref(state, user_account.id.clone());

        let rotate_second = match CryptoNoise::rotate_complete_responder::<
            X25519,
            ChaCha20Poly1305,
            Sha256,
            &[u8],
        >(
            state,
            user_account.clone(),
            crypto_account.clone(),
            *sender_id,
            rotate_first,
        ) {
            Some(rs) => rs,
            None => {
                log::warn!(
                    "rotate_complete_responder produced no response for {} \
                     (collision loss or nonce mismatch)",
                    sender_id.to_base58()
                );
                return;
            }
        };

        // `rotate_complete_responder` has already flipped the meta:
        // draining = old primary, primary = new. Pull the draining row
        // out — it holds the session key the initiator still expects.
        let meta = match crypto_account.get_rotation_meta(*sender_id) {
            Some(m) => m,
            None => {
                log::warn!(
                    "process_rotate_first: rotation_meta missing after responder step for {}",
                    sender_id.to_base58()
                );
                return;
            }
        };
        let draining_id = match meta.draining_session_id {
            Some(id) => id,
            None => {
                log::warn!(
                    "process_rotate_first: no draining_session_id for {}",
                    sender_id.to_base58()
                );
                return;
            }
        };
        let draining_state = match crypto_account.get_state_by_id(*sender_id, draining_id) {
            Some(s) if matches!(s.state, CryptoProcessState::Transport) => s,
            _ => {
                log::warn!(
                    "process_rotate_first: draining session {} not in Transport for {}",
                    draining_id,
                    sender_id.to_base58()
                );
                return;
            }
        };

        let payload = Self::create_rotate_second_message(rotate_second);
        let (encrypted_option, msg_nonce, sess_id, proc_state) = match Crypto::encrypt_with_state(
            payload,
            *sender_id,
            crypto_account.clone(),
            draining_state,
        ) {
            Some(v) => v,
            None => {
                log::warn!(
                    "process_rotate_first: failed to encrypt rotate_second for {}",
                    sender_id.to_base58()
                );
                return;
            }
        };
        let encrypted_bytes = match encrypted_option {
            Some(b) => b,
            None => return,
        };
        let encrypted_message =
            Crypto::create_encrypted_protobuf(msg_nonce, sess_id, encrypted_bytes, proc_state);

        let mut rng = rand::rng();
        let mut message_id = vec![0u8; 16];
        rng.fill(&mut message_id[..]);

        match messaging::Messaging::pack_and_send_encrypted_data(
            state,
            user_account,
            sender_id,
            encrypted_message,
            &message_id,
            true,
        ) {
            Ok(_) => log::trace!(
                "sent RotateHandshakeSecond to {}",
                sender_id.to_base58()
            ),
            Err(e) => log::error!("failed sending rotate_second: {}", e),
        }
    }

    /// Handle an incoming `RotateHandshakeSecond` by finalising our
    /// own pending rotation via `CryptoNoise::rotate_finalize_initiator`.
    fn process_rotate_second(
        state: &crate::QaulState,
        user_account: &UserAccount,
        sender_id: &PeerId,
        rotate_second: proto_net::RotateHandshakeSecond,
    ) {
        let crypto_account = CryptoStorage::get_db_ref(state, user_account.id.clone());
        let ok = CryptoNoise::rotate_finalize_initiator::<X25519, ChaCha20Poly1305, Sha256, &[u8]>(
            state,
            crypto_account,
            *sender_id,
            rotate_second,
        );
        if !ok {
            log::warn!(
                "rotate_finalize_initiator failed for {}",
                sender_id.to_base58()
            );
        } else {
            log::trace!(
                "rotation finalised: new primary session for {}",
                sender_id.to_base58()
            );
        }
    }

    /// create second handshake protobuf message
    ///
    /// return binary messaging message
    pub fn create_second_handshake_message(signature: Vec<u8>) -> Vec<u8> {
        // create timestamp
        let received_at = Timestamp::get_timestamp();

        // pack message
        let proto_cryptoservice_message = proto_net::CryptoserviceContainer {
            message: Some(
                proto_net::cryptoservice_container::Message::SecondHandshake(
                    proto_net::SecondHandshake {
                        signature,
                        received_at,
                    },
                ),
            ),
        };

        // encode binary message
        let mut cryptoservice_buf = Vec::with_capacity(proto_cryptoservice_message.encoded_len());
        proto_cryptoservice_message
            .encode(&mut cryptoservice_buf)
            .expect("Vec<u8> provides capacity as needed");

        // create messaging message
        let proto_messaging_message = messaging::proto::Messaging {
            message: Some(messaging::proto::messaging::Message::CryptoService(
                messaging::proto::CryptoService {
                    content: cryptoservice_buf,
                },
            )),
        };

        // encode messaging message
        let mut messaging_buf = Vec::with_capacity(proto_messaging_message.encoded_len());
        proto_messaging_message
            .encode(&mut messaging_buf)
            .expect("Vec<u8> provides capacity as needed");

        messaging_buf
    }

    /// Build a `Messaging { CryptoService { content } }` byte payload
    /// carrying a `RotateHandshakeFirst`. Mirror of
    /// `create_second_handshake_message` for the rotation path.
    pub fn create_rotate_first_message(rotate_first: proto_net::RotateHandshakeFirst) -> Vec<u8> {
        let container = proto_net::CryptoserviceContainer {
            message: Some(proto_net::cryptoservice_container::Message::RotateFirst(
                rotate_first,
            )),
        };
        Self::wrap_cryptoservice(container)
    }

    /// Build a `Messaging { CryptoService { content } }` byte payload
    /// carrying a `RotateHandshakeSecond`.
    pub fn create_rotate_second_message(
        rotate_second: proto_net::RotateHandshakeSecond,
    ) -> Vec<u8> {
        let container = proto_net::CryptoserviceContainer {
            message: Some(proto_net::cryptoservice_container::Message::RotateSecond(
                rotate_second,
            )),
        };
        Self::wrap_cryptoservice(container)
    }

    /// Encode a `CryptoserviceContainer` and wrap it in a
    /// `Messaging::CryptoService` frame ready to be encrypted and
    /// handed to `Messaging::pack_and_send_encrypted_data`.
    fn wrap_cryptoservice(container: proto_net::CryptoserviceContainer) -> Vec<u8> {
        let mut cryptoservice_buf = Vec::with_capacity(container.encoded_len());
        container
            .encode(&mut cryptoservice_buf)
            .expect("Vec<u8> provides capacity as needed");

        let messaging_message = messaging::proto::Messaging {
            message: Some(messaging::proto::messaging::Message::CryptoService(
                messaging::proto::CryptoService {
                    content: cryptoservice_buf,
                },
            )),
        };

        let mut out = Vec::with_capacity(messaging_message.encoded_len());
        messaging_message
            .encode(&mut out)
            .expect("Vec<u8> provides capacity as needed");
        out
    }
}
