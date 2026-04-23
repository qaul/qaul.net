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
pub mod storage;

use rand::Rng;

use rand::Rng;

use super::messaging;
use crate::node::user_accounts::UserAccount;
use crate::services::crypto::sessionmanager::CryptoSessionManager;
use crate::storage::configuration::Configuration;
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
    /// Initiator-side cipher key derived from `HandshakeState::split()`
    /// applied to the chaining key after KK msg 1. Used to encrypt
    /// `HandshakeExtraPayload` frames while the session is in
    /// `HalfOutgoing`. Cleared on transition to `Transport` once every
    /// queued extra has been drained from the messaging layer.
    ///
    /// NOTE: rows are stored as bincode, which is not self-describing,
    /// so `#[serde(default)]` does NOT make rows written before these
    /// fields existed decode. Backward compatibility is handled by the
    /// explicit legacy fallback in `CryptoAccount::decode_state`
    /// (`storage.rs`); if you append fields here, extend
    /// `LegacyCryptoState` accordingly.
    #[serde(default)]
    pub pre_cipher_out: Option<Vec<u8>>,
    /// Strictly-increasing per-session counter used as the AEAD nonce
    /// for outbound extras and stamped onto each
    /// `HandshakeExtraPayload.pre_index`. Defaults to 0.
    #[serde(default)]
    pub pre_index_out: u64,
    /// Responder-side cipher key matching `pre_cipher_out` on the
    /// initiator. Stashed when KK msg 1 is processed so queued extras
    /// arriving across a daemon restart still decrypt.
    #[serde(default)]
    pub pre_cipher_in: Option<Vec<u8>>,
    /// Highest `pre_index` ever accepted on this session. Used in
    /// combination with `pre_index_in_seen` to detect duplicates.
    #[serde(default)]
    pub pre_index_in_highest: u64,
    /// Bitmap of seen `pre_index` values, length bounded by
    /// `HandshakeExtras::max_pre_messages`. Bit `i` set means we
    /// have already accepted (and forwarded) the extra with
    /// `pre_index = i`.
    #[serde(default)]
    pub pre_index_in_seen: Vec<u8>,
    /// Aggregate ciphertext bytes accepted on this session under
    /// extras. Used to enforce `HandshakeExtras::max_pre_bytes`.
    #[serde(default)]
    pub pre_bytes_accounted: u64,
    /// Wall-clock timestamp (ms since epoch) at which this session
    /// transitioned to `Transport`. Diagnostic / display only — no
    /// protocol decision depends on it, since rotation is clock-free.
    /// Zero for sessions persisted before this field existed.
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
    /// session has exceeded the outbound-volume
    /// (`cfg.crypto_rotation.volume_messages`) trigger, start a new
    /// rotation via `perform_rotation`. Rotation is clock-free — there
    /// is no time-based trigger.
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

        // Read the (primary) state back after the encrypt — its
        // index_nonce_out has already been incremented. `get_state`
        // priority ranks Transport above HalfOutgoing, so if a new
        // HalfOutgoing row from an earlier in-flight rotation exists
        // we still pick the live primary here.
        let session = match crypto_account.get_state(remote_id) {
            Some(s) if matches!(s.state, CryptoProcessState::Transport) => s,
            _ => return,
        };

        // Volume trigger only — rotation is clock-free, so there is no
        // time-based trigger.
        if session.index_nonce_out < volume_messages {
            return;
        }

        log::info!(
            "rotation trigger for peer {}: nonce_out={} >= volume_messages={}",
            remote_id.to_base58(),
            session.index_nonce_out,
            volume_messages
        );

        if let Err(e) = Self::perform_rotation(state, user_account, crypto_account, remote_id) {
            log::warn!(
                "triggered rotation failed for {}: {}",
                remote_id.to_base58(),
                e
            );
        }
    }

    /// Initiate a rotation with `remote_id`: call `rotate_initiate`,
    /// build the `RotateHandshakeFirst` frame, encrypt it under the
    /// still-primary session, and hand it to the messaging layer.
    ///
    /// On success returns `(previous_session_id, new_session_id)`.
    /// This is the shared back-end for both the automatic trigger
    /// path (`fire_rotation_if_triggered`) and the manual RPC path
    /// (`handle_trigger_rotation`). Both paths pre-validate that
    /// rotation is enabled and that no rotation is already in flight.
    ///
    /// Also enforces the Phase 5 capability gate: if the remote peer
    /// has not advertised the `ROTATION` bit in its most recent
    /// `UserInfo`, we refuse to rotate — an older binary on the
    /// other end would silently drop the `RotateHandshakeFirst`
    /// frame, leaving the initiator with a dangling `HalfOutgoing`
    /// state and the responder stuck on the old session. Returning
    /// `Err` here lets the caller keep using the legacy session.
    pub(super) fn perform_rotation(
        state: &crate::QaulState,
        user_account: &UserAccount,
        crypto_account: &CryptoAccount,
        remote_id: PeerId,
    ) -> Result<(u32, u32), String> {
        // Capability gate — refuse to rotate with peers that don't
        // advertise support (see `router::users::Capabilities`). The
        // router-state snapshot is taken once per call: a concurrent
        // `UserInfo` arrival that flips a peer's caps after the gate
        // is fine, because the gate is conservative — we only
        // proceed when caps were already known when we read.
        let caps = {
            let rs = state.get_router();
            crate::router::users::Users::get_capabilities(&rs, &remote_id)
        };
        if !crate::router::users::Capabilities::supports(
            caps,
            crate::router::users::Capabilities::ROTATION,
        ) {
            return Err("peer does not advertise rotation capability".to_string());
        }

        // Don't launch a second rotation while one is already in flight.
        if let Some(meta) = crypto_account.get_rotation_meta(remote_id) {
            if meta.pending_initiated_session_id.is_some() {
                return Err("rotation already in flight".to_string());
            }
        }

        let prev_session_id = match crypto_account.get_state(remote_id) {
            Some(s) if matches!(s.state, CryptoProcessState::Transport) => s.session_id,
            Some(_) => {
                return Err("no Transport session with peer (handshake pending)".to_string());
            }
            None => return Err("no session with peer".to_string()),
        };

        let rotate_first =
            match CryptoNoise::rotate_initiate::<X25519, ChaCha20Poly1305, Sha256, &[u8]>(
                state,
                user_account.clone(),
                crypto_account.clone(),
                remote_id,
            ) {
                Some(rf) => rf,
                None => return Err("rotate_initiate returned None".to_string()),
            };
        let new_session_id = rotate_first.new_session_id;

        // Build the Messaging::CryptoService payload carrying the
        // rotate_first frame and encrypt it under the (still) primary
        // session. `rotate_initiate` created a new HalfOutgoing row for
        // the new session, but `get_state` priority puts Transport
        // (primary) above HalfOutgoing, so we pick the primary here.
        let payload = CryptoSessionManager::create_rotate_first_message(rotate_first);
        let primary = match crypto_account.get_state(remote_id) {
            Some(s) if matches!(s.state, CryptoProcessState::Transport) => s,
            _ => return Err("lost primary Transport mid-rotation".to_string()),
        };
        let (encrypted_option, msg_nonce, sess_id, proc_state) =
            match Self::encrypt_with_state(payload, remote_id, crypto_account.clone(), primary) {
                Some(v) => v,
                None => return Err("failed to encrypt rotate_first".to_string()),
            };
        let encrypted_bytes = match encrypted_option {
            Some(b) => b,
            None => return Err("encrypt_with_state produced no ciphertext".to_string()),
        };

        let encrypted_message =
            Self::create_encrypted_protobuf(msg_nonce, sess_id, encrypted_bytes, proc_state);

        // Fresh 16-byte message_id for this CryptoService frame.
        let mut rng = rand::rng();
        let mut message_id = vec![0u8; 16];
        rng.fill(&mut message_id[..]);

        messaging::Messaging::pack_and_send_encrypted_data(
            state,
            user_account,
            &remote_id,
            encrypted_message,
            &message_id,
            true,
        )
        .map_err(|e| format!("pack_and_send_encrypted_data: {}", e))?;

        log::trace!("sent RotateHandshakeFirst to {}", remote_id.to_base58());
        Ok((prev_session_id, new_session_id))
    }

    /// Post-decrypt rotation bookkeeping.
    ///
    /// Called once for every successfully decrypted Transport message,
    /// with the message's `nonce`:
    ///
    /// - If the message was received on the **draining** session (or on
    ///   the soon-to-drain primary while a rotation we initiated is in
    ///   flight), record its nonce in the drain bitmap. When the drain
    ///   target is known and every nonce up to it has arrived, the
    ///   draining session is retired — no timer.
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
        nonce: u64,
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

        // Drain accounting. Track inbound nonces both on the active
        // draining session and on the soon-to-drain primary while a
        // rotation we initiated is still in flight (so the in-flight
        // tail is already covered when the drain target arrives).
        let is_draining = Some(session_id) == meta.draining_session_id;
        let is_predrain =
            meta.pending_initiated_session_id.is_some() && session_id == meta.primary_session_id;
        if is_draining || is_predrain {
            CryptoNoise::record_drain_received(crypto_account, remote_id, nonce);
            // Inbound progressed; the old session may now be fully
            // drained inbound — retire it if outbound is also clear.
            Self::try_retire_drain(state, crypto_account, remote_id);
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

    /// Retire the draining session for `remote_id` iff it is safe to:
    /// the inbound drain is complete (every nonce up to the peer's
    /// declared final has arrived) AND none of this node's outbound
    /// messages on that session are still awaiting confirmation.
    ///
    /// The outbound condition is what keeps clock-free draining
    /// correct: the messaging retransmit layer replays a message's
    /// original old-session ciphertext, so retiring the session while
    /// an unconfirmed message still references it would strand that
    /// message (the peer would drop it as `MessageDroppedPostDrain`
    /// and never confirm). We therefore keep the old session's keys
    /// alive until its outbound traffic is confirmed — bounded by the
    /// retransmit layer's own retry/expiry cap, so a dead peer's
    /// draining row is still reclaimed.
    pub(super) fn try_retire_drain(
        state: &crate::QaulState,
        crypto_account: &CryptoAccount,
        remote_id: PeerId,
    ) {
        let meta = match crypto_account.get_rotation_meta(remote_id) {
            Some(m) => m,
            None => return,
        };
        let drain_id = match meta.draining_session_id {
            Some(id) => id,
            None => return,
        };
        if !meta.drain_complete() {
            return; // inbound not finished draining
        }
        if !Self::outbound_confirmed_on_session(state, remote_id, drain_id) {
            return; // unconfirmed outbound still references the old session
        }
        CryptoNoise::retire_drain(Some(state), crypto_account, remote_id, meta);
    }

    /// Re-check drain retirement after one of our outbound messages to
    /// `remote_id` was confirmed. Called from the messaging
    /// confirmation path so a session waiting only on outbound
    /// confirmation is retired promptly once the last ack arrives.
    pub fn on_outbound_confirmed(state: &crate::QaulState, account_id: PeerId, remote_id: PeerId) {
        let crypto_account = CryptoStorage::get_db_ref(state, account_id);
        Self::try_retire_drain(state, &crypto_account, remote_id);
    }

    /// Whether every outbound message this node sent to `remote_id`
    /// under `session_id` has been confirmed — i.e. no entry in the
    /// messaging `unconfirmed` table still targets that (peer, session).
    fn outbound_confirmed_on_session(
        state: &crate::QaulState,
        remote_id: PeerId,
        session_id: u32,
    ) -> bool {
        let remote_bytes = remote_id.to_bytes();
        let unconfirmed = state.services.messaging.unconfirmed.read().unwrap();
        for entry in unconfirmed.unconfirmed.iter() {
            let (_sig, bytes) = match entry {
                Ok(kv) => kv,
                Err(_) => continue,
            };
            let um: messaging::UnConfirmedMessage = match bincode::deserialize(&bytes) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if um.receiver_id != remote_bytes {
                continue;
            }
            if Self::container_session_id(&um.container) == Some(session_id) {
                return false;
            }
        }
        true
    }

    /// Extract the `Encrypted.session_id` from an encoded messaging
    /// `Container`, if the container carries an encrypted payload.
    fn container_session_id(container_bytes: &[u8]) -> Option<u32> {
        let container = messaging::proto::Container::decode(container_bytes).ok()?;
        let envelope = container.envelope?;
        let payload = messaging::proto::EnvelopPayload::decode(&envelope.payload[..]).ok()?;
        match payload.payload? {
            messaging::proto::envelop_payload::Payload::Encrypted(enc) => Some(enc.session_id),
            _ => None,
        }
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
                                // Post-decrypt rotation bookkeeping: drain
                                // nonce accounting + retire, and primary
                                // inbound-volume trigger check.
                                Self::after_decrypt_rotation(
                                    state,
                                    &user_account,
                                    &crypto_account,
                                    remote_id,
                                    session_id,
                                    data.nonce,
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

                // Post-drain detection: if rotation_meta remembers
                // retiring *this* session_id, the sender used an
                // already-drained session and the UI should surface
                // "ask the sender to resend".
                if let Some(meta) = crypto_account.get_rotation_meta(remote_id) {
                    if Some(message.session_id) == meta.last_retired_session_id {
                        log::info!(
                            "dropping message on retired session {} from {}",
                            message.session_id,
                            remote_id.to_base58()
                        );
                        events::record_and_emit(
                            Some(state),
                            events::RotationEvent {
                                kind: events::RotationEventKind::MessageDroppedPostDrain,
                                remote_id,
                                primary_session_id: 0,
                                draining_session_id: message.session_id,
                                timestamp_ms: 0,
                            },
                        );
                        return None;
                    }
                }

                // check what kind of message we are getting
                match messaging::proto::CryptoState::try_from(message.state) {
                    Ok(messaging::proto::CryptoState::Handshake) => {
                        log::trace!("decrypt incoming first handshake");

                        // session id this incoming handshake establishes;
                        // used below to supersede any prior session with
                        // this peer (cold re-key).
                        let incoming_session_id = message.session_id;

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

                                // Cold re-key healing: if the peer already
                                // had a session with us, this fresh
                                // (identity-authenticated) handshake
                                // supersedes it so we stop using the old
                                // session for outbound. A no-op for genuine
                                // first contact.
                                let acct =
                                    CryptoStorage::get_db_ref(state, user_account.id.clone());
                                Self::supersede_with_cold_session(
                                    &acct,
                                    remote_id,
                                    incoming_session_id,
                                );

                                // return decrypted first handshake message
                                return Some(decrypted_data);
                            } else {
                                // decryption of first handshake failed
                                return None;
                            }
                        }
                    }
                    _ => {
                        // A Transport message arrived for a session we
                        // have no state for. Two cases:
                        //
                        // 1. We have NO session with this peer at all
                        //    (we lost our encryption state, or never had
                        //    one). This is the cold re-key / state-healing
                        //    case: re-establish identity-authenticated by
                        //    running a fresh KK handshake.
                        // 2. We DO have a session (the peer is just
                        //    sending stale traffic on an old session it
                        //    hasn't stopped retransmitting). Drop it — the
                        //    peer will converge on our current session.
                        //
                        // Gating cold re-key on "no session at all" is the
                        // storm guard: once we start a handshake we have a
                        // HalfOutgoing session, so further stale traffic
                        // hits case 2 and never re-triggers.
                        if crypto_account.get_state(remote_id).is_none() {
                            log::info!(
                                "cold re-key: traffic for unknown session {} from {} and no local session — re-handshaking",
                                message.session_id,
                                remote_id.to_base58()
                            );
                            Self::initiate_cold_rekey(
                                state,
                                &user_account,
                                &crypto_account,
                                remote_id,
                            );
                        } else {
                            log::trace!(
                                "decrypt: stale traffic on unknown session {} from {} (have a current session) — dropping",
                                message.session_id,
                                remote_id.to_base58()
                            );
                        }
                        return None;
                    }
                }
            }
        }

        None
    }

    /// Initiate a cold re-key: run a fresh KK first handshake with
    /// `remote_id` to re-establish a session after losing local state.
    ///
    /// Unlike rotation (which rides inside an existing session), this is
    /// the state-healing path — there is no session to encrypt under.
    /// The KK handshake is mutually authenticated by the peers' static
    /// identity keys, so re-establishing this way is itself
    /// identity-authenticated; no separate signed request is needed.
    ///
    /// The caller must have already checked that no session exists for
    /// `remote_id` (the storm guard). Requires the peer's static public
    /// key, which `encrypt_noise_kk_handshake_1` looks up from the
    /// router users table (separate storage that survives crypto-state
    /// loss); if the key is unknown the handshake is a no-op and the
    /// session is re-learned once the peer is rediscovered.
    fn initiate_cold_rekey(
        state: &crate::QaulState,
        user_account: &UserAccount,
        crypto_account: &CryptoAccount,
        remote_id: PeerId,
    ) {
        // Empty payload — this handshake carries no application data; it
        // exists only to re-establish the session.
        let (encrypted_option, nonce, session_id) =
            CryptoNoise::encrypt_noise_kk_handshake_1::<X25519, ChaCha20Poly1305, Sha256, &[u8]>(
                state,
                Vec::new(),
                user_account.clone(),
                crypto_account.clone(),
                remote_id,
            );
        let encrypted_data = match encrypted_option {
            Some(d) => d,
            None => {
                log::warn!(
                    "cold re-key: could not build handshake for {} (peer static key unknown?)",
                    remote_id.to_base58()
                );
                return;
            }
        };
        let encrypted_message = Self::create_encrypted_protobuf(
            nonce,
            session_id,
            encrypted_data,
            messaging::proto::CryptoState::Handshake,
        );
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
            Ok(_) => log::trace!(
                "cold re-key: sent first handshake to {}",
                remote_id.to_base58()
            ),
            Err(e) => log::error!("cold re-key: failed sending first handshake: {}", e),
        }
    }

    /// Supersede any prior session(s) with `remote_id` after a fresh
    /// incoming cold handshake established `new_session_id`.
    ///
    /// When a peer that already has a session with us re-handshakes from
    /// scratch (because it lost its state), completing the new handshake
    /// leaves us with two Transport sessions. Because the KK handshake
    /// is authenticated by the peer's static identity key, a completed
    /// new handshake provably came from the real peer, so it is safe to
    /// promote it and drop the old session — otherwise we might keep
    /// sending under the old session that the peer can no longer decrypt.
    fn supersede_with_cold_session(
        crypto_account: &CryptoAccount,
        remote_id: PeerId,
        new_session_id: u32,
    ) {
        // Delete any other Transport session for this peer.
        let stale: Vec<u32> = crypto_account
            .transport_session_ids(remote_id)
            .into_iter()
            .filter(|sid| *sid != new_session_id)
            .collect();
        if stale.is_empty() {
            // Brand-new first contact (no prior session) — nothing to
            // supersede. Still record the primary so resolve_primary_state
            // is unambiguous.
            crypto_account
                .save_rotation_meta(remote_id, &RotationMeta::primary_only(new_session_id));
            return;
        }
        for sid in stale {
            log::info!(
                "cold re-key: superseding old session {} with {} for {}",
                sid,
                new_session_id,
                remote_id.to_base58()
            );
            crypto_account.delete_state(remote_id, sid);
        }
        // Reset rotation state to just the new primary.
        crypto_account.save_rotation_meta(remote_id, &RotationMeta::primary_only(new_session_id));
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
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, _user_id: Vec<u8>, request_id: String) {
        use qaul_proto::qaul_rpc_crypto as proto_rpc;

        match proto_rpc::Crypto::decode(&data[..]) {
            Ok(msg) => match msg.message {
                Some(proto_rpc::crypto::Message::GetConfigRequest(_req)) => {
                    Self::handle_get_config(state, request_id);
                }
                Some(proto_rpc::crypto::Message::SetConfigRequest(req)) => {
                    Self::handle_set_config(state, req, request_id);
                }
                Some(proto_rpc::crypto::Message::GetEventsRequest(req)) => {
                    Self::handle_get_events(state, req, request_id);
                }
                Some(proto_rpc::crypto::Message::TriggerRotationRequest(req)) => {
                    Self::handle_trigger_rotation(state, req, request_id);
                }
                Some(proto_rpc::crypto::Message::GetEventsRequest(req)) => {
                    Self::handle_get_events(req, request_id);
                }
                Some(proto_rpc::crypto::Message::GetConfigResponse(_))
                | Some(proto_rpc::crypto::Message::SetConfigResponse(_))
                | Some(proto_rpc::crypto::Message::GetEventsResponse(_))
                | Some(proto_rpc::crypto::Message::TriggerRotationResponse(_)) => {
                    // Responses are libqaul -> client only; clients
                    // that echo them back are ignored.
                    log::warn!("Crypto RPC received a response message from client; dropping");
                }
                None => log::error!("Crypto RPC message from client was empty"),
            },
            Err(e) => log::error!("Crypto RPC decode error: {}", e),
        }
    }

    fn handle_get_config(state: &crate::QaulState, request_id: String) {
        use qaul_proto::qaul_rpc_crypto as proto_rpc;
        let snapshot = Self::snapshot_config(state);
        let out = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::GetConfigResponse(snapshot)),
        };
        crate::rpc::Rpc::send_message(
            state,
            out.encode_to_vec(),
            crate::rpc::proto::Modules::Crypto.into(),
            request_id,
            Vec::new(),
        );
    }

    fn handle_set_config(
        state: &crate::QaulState,
        req: qaul_proto::qaul_rpc_crypto::SetConfigRequest,
        request_id: String,
    ) {
        use qaul_proto::qaul_rpc_crypto as proto_rpc;

        // Validate: the volume trigger, when present, must be > 0.
        // Accepting zero would mean "rotate immediately on every
        // message" — almost certainly a client mistake.
        let validation_error = [("volume_messages", req.volume_messages)]
            .into_iter()
            .find_map(|(name, value)| match value {
                Some(0) => Some(format!("{} must be > 0", name)),
                _ => None,
            });

        if let Some(err) = validation_error {
            let applied = Self::snapshot_config(state);
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
                state,
                resp.encode_to_vec(),
                crate::rpc::proto::Modules::Crypto.into(),
                request_id,
                Vec::new(),
            );
            return;
        }

        // Apply the partial update.
        {
            let mut cfg = Configuration::get_mut(state);
            if let Some(v) = req.enabled {
                cfg.crypto_rotation.enabled = v;
            }
            if let Some(v) = req.volume_messages {
                cfg.crypto_rotation.volume_messages = v;
            }
        }

        // Persist to disk (skipped under cfg(test): the test fixture
        // installs a config directly and never invokes the Storage
        // path).
        #[cfg(not(test))]
        Configuration::save(state);

        let applied = Self::snapshot_config(state);
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
            state,
            resp.encode_to_vec(),
            crate::rpc::proto::Modules::Crypto.into(),
            request_id,
            Vec::new(),
        );
    }

    fn handle_get_events(
        state: &crate::QaulState,
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
                    events::RotationEventKind::DrainCompleted => {
                        proto_rpc::RotationEventKind::DrainCompleted as i32
                    }
                    events::RotationEventKind::MessageDroppedPostDrain => {
                        proto_rpc::RotationEventKind::MessageDroppedPostDrain as i32
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
            state,
            resp.encode_to_vec(),
            crate::rpc::proto::Modules::Crypto.into(),
            request_id,
            Vec::new(),
        );
    }

    /// Handle a `TriggerRotationRequest`.
    ///
    /// Resolves the caller's default user account, validates the
    /// `remote_id` bytes, and delegates to `perform_rotation`. When
    /// rotation is disabled in the current configuration the request
    /// is rejected even though `perform_rotation` itself does not
    /// check — operator tooling should surface the disabled state
    /// rather than silently forcing a rotation.
    fn handle_trigger_rotation(
        state: &crate::QaulState,
        req: qaul_proto::qaul_rpc_crypto::TriggerRotationRequest,
        request_id: String,
    ) {
        use crate::node::user_accounts::UserAccounts;
        use qaul_proto::qaul_rpc_crypto as proto_rpc;

        let mut out = proto_rpc::TriggerRotationResponse {
            success: false,
            error: String::new(),
            new_session_id: 0,
            previous_session_id: 0,
        };

        // Closure captures `state` and `request_id` by reference; the
        // outer function consumes `request_id` exactly once when this
        // is invoked. The borrow checker is satisfied because every
        // early-return path calls `send` exactly once.
        let send = |resp: proto_rpc::TriggerRotationResponse| {
            let envelope = proto_rpc::Crypto {
                message: Some(proto_rpc::crypto::Message::TriggerRotationResponse(resp)),
            };
            crate::rpc::Rpc::send_message(
                state,
                envelope.encode_to_vec(),
                crate::rpc::proto::Modules::Crypto.into(),
                request_id,
                Vec::new(),
            );
        };

        if !Configuration::get(state).crypto_rotation.enabled {
            out.error = "crypto rotation is disabled".into();
            return send(out);
        }

        let remote_id = match PeerId::from_bytes(&req.remote_id) {
            Ok(p) => p,
            Err(e) => {
                out.error = format!("invalid remote_id: {}", e);
                return send(out);
            }
        };

        let user_account = match UserAccounts::get_default_user(state) {
            Some(u) => u,
            None => {
                out.error = "no default user account".into();
                return send(out);
            }
        };

        let crypto_account = CryptoStorage::get_db_ref(state, user_account.id.clone());

        match Self::perform_rotation(state, &user_account, &crypto_account, remote_id) {
            Ok((prev, new)) => {
                out.success = true;
                out.previous_session_id = prev;
                out.new_session_id = new;
            }
            Err(e) => {
                out.error = e;
            }
        }
        send(out);
    }

    /// Snapshot the current `CryptoRotation` into a proto response.
    fn snapshot_config(state: &crate::QaulState) -> qaul_proto::qaul_rpc_crypto::GetConfigResponse {
        let cfg = Configuration::get(state);
        qaul_proto::qaul_rpc_crypto::GetConfigResponse {
            enabled: cfg.crypto_rotation.enabled,
            volume_messages: cfg.crypto_rotation.volume_messages,
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
    use crate::storage::configuration::CryptoRotation;
    use libp2p::identity::Keypair;
    use std::sync::Arc;

    /// Build a fresh `QaulState` for tests with crypto rotation
    /// enabled. Each call returns an independent state — tests own
    /// their config, so we don't need cross-test locking on the
    /// configuration like the pre-instance-based codebase did.
    pub(super) fn make_test_state() -> Arc<crate::QaulState> {
        let state = Arc::new(crate::QaulState::new_for_simulation());
        {
            let mut cfg = state.config.inner.write().unwrap();
            cfg.crypto_rotation = CryptoRotation {
                enabled: true,
                volume_messages: 1_000_000,
            };
        }
        state
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
            pre_cipher_out: None,
            pre_index_out: 0,
            pre_cipher_in: None,
            pre_index_in_highest: 0,
            pre_index_in_seen: Vec::new(),
            pre_bytes_accounted: 0,
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

    /// Insert an unconfirmed outbound message targeting `receiver`
    /// under `session_id` into the test state's messaging table, so
    /// `outbound_confirmed_on_session` sees it as still in flight.
    fn insert_unconfirmed(
        state: &crate::QaulState,
        receiver: PeerId,
        session_id: u32,
        signature: &[u8],
    ) {
        let encrypted = messaging::proto::Encrypted {
            state: messaging::proto::CryptoState::Transport as i32,
            session_id,
            data: vec![],
        };
        let payload = messaging::proto::EnvelopPayload {
            payload: Some(messaging::proto::envelop_payload::Payload::Encrypted(
                encrypted,
            )),
        };
        let envelope = messaging::proto::Envelope {
            sender_id: vec![],
            receiver_id: receiver.to_bytes(),
            payload: payload.encode_to_vec(),
        };
        let container = messaging::proto::Container {
            signature: signature.to_vec(),
            envelope: Some(envelope),
        };
        let um = messaging::UnConfirmedMessage {
            receiver_id: receiver.to_bytes(),
            message_type: messaging::MessagingServiceType::Chat,
            message_id: vec![],
            container: container.encode_to_vec(),
            last_sent: 0,
            retry: 0,
            scheduled: false,
            scheduled_dtn: false,
            is_dtn: false,
        };
        let tbl = state.services.messaging.unconfirmed.write().unwrap();
        tbl.unconfirmed
            .insert(signature.to_vec(), bincode::serialize(&um).unwrap())
            .unwrap();
    }

    // The core of option (a): a draining session that is fully drained
    // inbound must NOT be retired while one of our outbound messages on
    // it is still unconfirmed (else the retransmit layer would strand
    // that message on a retired session). It retires once the outbound
    // is confirmed (removed from the unconfirmed table).
    #[test]
    fn drain_waits_for_outbound_confirmation() {
        let state = make_test_state();
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 42,
                draining_session_id: Some(7),
                draining_recv_target: Some(0), // inbound already complete
                draining_recv_base: 0,
                ..Default::default()
            },
        );
        // An unconfirmed outbound message still references session 7.
        let sig = vec![9u8; 8];
        insert_unconfirmed(&state, remote, 7, &sig);

        // Inbound is complete, but outbound is not → must not retire.
        Crypto::try_retire_drain(&state, &acct, remote);
        assert!(
            acct.get_state_by_id(remote, 7).is_some(),
            "must not retire while outbound on the old session is unconfirmed"
        );

        // Confirm the outbound (remove from unconfirmed) → now retires.
        {
            let tbl = state.services.messaging.unconfirmed.write().unwrap();
            tbl.unconfirmed.remove(&sig).unwrap();
        }
        Crypto::try_retire_drain(&state, &acct, remote);
        assert!(
            acct.get_state_by_id(remote, 7).is_none(),
            "retires once inbound drained AND outbound confirmed"
        );
        assert_eq!(
            acct.get_rotation_meta(remote)
                .unwrap()
                .last_retired_session_id,
            Some(7)
        );
    }

    // A message decrypted on the draining session records its nonce
    // in the drain bitmap; the session is not retired until the whole
    // (base, target] window has arrived.
    #[test]
    fn after_decrypt_records_drain_nonce() {
        let state = make_test_state();
        let acct = CryptoStorage::test_account();
        let user_account = dummy_user_account();
        let remote = fresh_peer();
        acct.save_state(remote, 50, dummy_state(50));
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 100,
                draining_session_id: Some(50),
                draining_recv_target: Some(3), // need nonces 1,2,3
                draining_recv_base: 0,
                ..Default::default()
            },
        );

        // Receive nonce 1 only — not complete, session survives.
        Crypto::after_decrypt_rotation(&state, &user_account, &acct, remote, 50, 1);
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.draining_session_id, Some(50), "still draining");
        assert!(meta.drain_nonce_seen(1), "nonce 1 recorded");
        assert_eq!(meta.primary_session_id, 100);

        // Receive 2 then 3 → window (0,3] complete → retired.
        Crypto::after_decrypt_rotation(&state, &user_account, &acct, remote, 50, 2);
        Crypto::after_decrypt_rotation(&state, &user_account, &acct, remote, 50, 3);
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.draining_session_id, None, "drain complete → retired");
        assert!(acct.get_state_by_id(remote, 50).is_none());
    }

    // While a rotation we initiated is in flight, inbound nonces on the
    // soon-to-drain primary are pre-recorded so the tail is covered when
    // the target arrives at finalisation.
    #[test]
    fn after_decrypt_predrain_records_on_primary() {
        let state = make_test_state();
        let acct = CryptoStorage::test_account();
        let user_account = dummy_user_account();
        let remote = fresh_peer();
        acct.save_state(remote, 100, dummy_state(100));
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 100,
                pending_initiated_session_id: Some(200),
                draining_recv_base: 5,
                ..Default::default()
            },
        );

        // Inbound nonce 7 on the still-primary session during rotation.
        Crypto::after_decrypt_rotation(&state, &user_account, &acct, remote, 100, 7);

        let meta = acct.get_rotation_meta(remote).unwrap();
        assert!(meta.drain_nonce_seen(7), "pre-drain nonce recorded");
    }

    // When session_id matches neither primary nor draining, no
    // mutation should happen even with rotation enabled in config.
    #[test]
    fn after_decrypt_noop_on_unrelated_session() {
        let state = make_test_state();
        let acct = CryptoStorage::test_account();
        let user_account = dummy_user_account();
        let remote = fresh_peer();
        acct.save_state(remote, 99, dummy_state(99));
        let original = RotationMeta {
            primary_session_id: 100,
            draining_session_id: Some(50),
            draining_recv_target: Some(u64::MAX),
            draining_recv_base: 0,
            ..Default::default()
        };
        acct.save_rotation_meta(remote, &original);

        // session_id 99 matches neither primary (100) nor draining
        // (50) → no mutation.
        Crypto::after_decrypt_rotation(&state, &user_account, &acct, remote, 99, 1);

        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta, original);
    }

    // Cold re-key supersede: a fresh handshake session must replace any
    // prior Transport session(s) for the peer — the old session is
    // deleted and the new one becomes primary — so we never keep using
    // a session the peer (which lost its state) can no longer decrypt.
    #[test]
    fn cold_rekey_supersedes_prior_session() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        // Prior session X=10 (what the peer lost) and the new cold
        // handshake session Y=20.
        acct.save_state(remote, 10, dummy_state(10));
        acct.save_state(remote, 20, dummy_state(20));
        acct.save_rotation_meta(remote, &RotationMeta::primary_only(10));

        Crypto::supersede_with_cold_session(&acct, remote, 20);

        assert!(
            acct.get_state_by_id(remote, 10).is_none(),
            "old session deleted"
        );
        assert!(
            acct.get_state_by_id(remote, 20).is_some(),
            "new session kept"
        );
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.primary_session_id, 20, "new session is primary");
        assert_eq!(meta.draining_session_id, None);
    }

    // Supersede on genuine first contact (no prior session) is a no-op
    // beyond recording the new primary.
    #[test]
    fn cold_rekey_supersede_noop_on_first_contact() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 20, dummy_state(20));

        Crypto::supersede_with_cold_session(&acct, remote, 20);

        assert!(acct.get_state_by_id(remote, 20).is_some());
        assert_eq!(
            acct.get_rotation_meta(remote).unwrap().primary_session_id,
            20
        );
    }
}

#[cfg(test)]
mod phase3_tests {
    //! Phase 3 unit tests — exercise `Crypto::rpc` against a per-test
    //! `QaulState` plus its embedded RPC channel.
    //!
    //! Each test owns its own `QaulState` (built by
    //! `phase2_tests::make_test_state`), so config and RPC channels
    //! are isolated — no cross-test locking needed.

    use super::phase2_tests::make_test_state;
    use super::*;
    use crate::rpc::Rpc;
    use qaul_proto::qaul_rpc_crypto as proto_rpc;

    /// Drop every pending libqaul->extern message on `state` so the
    /// test's own response is the first thing we pick up.
    pub(super) fn drain_rpc_channel(state: &crate::QaulState) {
        while Rpc::receive_from_libqaul(state).is_ok() {}
    }

    /// Invoke `Crypto::rpc` with an encoded Crypto RPC container and
    /// read back the one response it emits, decoded as
    /// `proto_rpc::Crypto`.
    pub(super) fn rpc_round_trip(
        state: &crate::QaulState,
        req: proto_rpc::Crypto,
    ) -> proto_rpc::Crypto {
        drain_rpc_channel(state);
        Crypto::rpc(state, req.encode_to_vec(), Vec::new(), "test-req".into());
        let raw = Rpc::receive_from_libqaul(state).expect("no RPC response was produced");
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
        let state = make_test_state();

        let req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::GetConfigRequest(
                proto_rpc::GetConfigRequest {},
            )),
        };
        let resp = rpc_round_trip(&state, req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::GetConfigResponse(r)) => r,
            other => panic!("expected GetConfigResponse, got {:?}", other.is_some()),
        };

        // Fields should match whatever `make_test_state` baked in.
        assert!(body.enabled);
        assert_eq!(body.volume_messages, 1_000_000);
    }

    /// A `SetConfigRequest` with only `volume_messages` set must leave
    /// every other field untouched and report the new value via
    /// `SetConfigResponse.applied`.
    #[test]
    fn rpc_set_config_partial_update_preserves_other_fields() {
        let state = make_test_state();

        // snapshot prior state.
        let original_volume = Configuration::get(&state).crypto_rotation.volume_messages;

        let new_volume = original_volume.saturating_add(123);
        let set_req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::SetConfigRequest(
                proto_rpc::SetConfigRequest {
                    enabled: None,
                    volume_messages: Some(new_volume),
                },
            )),
        };
        let resp = rpc_round_trip(&state, set_req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::SetConfigResponse(r)) => r,
            other => panic!("expected SetConfigResponse, got {:?}", other.is_some()),
        };
        assert!(body.success, "expected success, got error: {}", body.error);
        let applied = body.applied.expect("applied config should be populated");
        assert_eq!(applied.volume_messages, new_volume);
        assert!(applied.enabled, "untouched");
    }

    /// A zero `volume_messages` is a near-certain client mistake
    /// (rotate immediately on every message). The handler rejects it
    /// and echoes the unchanged config back in `applied`.
    #[test]
    fn rpc_set_config_rejects_zero_fields() {
        let state = make_test_state();

        let original = Configuration::get(&state).crypto_rotation.clone();

        let req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::SetConfigRequest(
                proto_rpc::SetConfigRequest {
                    volume_messages: Some(0),
                    ..Default::default()
                },
            )),
        };
        let resp = rpc_round_trip(&state, req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::SetConfigResponse(r)) => r,
            _ => panic!("expected SetConfigResponse"),
        };
        assert!(!body.success);
        assert!(
            body.error.contains("volume_messages"),
            "error should mention the offending field, got: {}",
            body.error
        );
        let applied = body.applied.unwrap();
        // Unchanged — the handler must not have mutated the config.
        assert_eq!(applied.volume_messages, original.volume_messages);
    }

    /// `TriggerRotationRequest` must fail with a descriptive error
    /// when rotation is disabled.
    #[test]
    fn rpc_trigger_rotation_rejected_when_disabled() {
        let state = make_test_state();

        // flip off
        let disable = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::SetConfigRequest(
                proto_rpc::SetConfigRequest {
                    enabled: Some(false),
                    ..Default::default()
                },
            )),
        };
        let _ = rpc_round_trip(&state, disable);

        // any peer id will do — the handler must short-circuit on
        // the disabled check before touching the id.
        let peer_bytes = libp2p::identity::Keypair::generate_ed25519()
            .public()
            .to_peer_id()
            .to_bytes();
        let req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::TriggerRotationRequest(
                proto_rpc::TriggerRotationRequest {
                    remote_id: peer_bytes,
                },
            )),
        };
        let resp = rpc_round_trip(&state, req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::TriggerRotationResponse(r)) => r,
            _ => panic!("expected TriggerRotationResponse"),
        };
        assert!(!body.success);
        assert!(
            body.error.contains("disabled"),
            "error should mention disabled, got: {}",
            body.error
        );
    }

    /// Malformed `remote_id` bytes must be rejected before reaching
    /// the session lookup.
    #[test]
    fn rpc_trigger_rotation_rejects_invalid_peer_bytes() {
        let state = make_test_state();

        let req = proto_rpc::Crypto {
            message: Some(proto_rpc::crypto::Message::TriggerRotationRequest(
                proto_rpc::TriggerRotationRequest {
                    // not a libp2p multihash
                    remote_id: vec![0xFF, 0xAA, 0x00, 0x11],
                },
            )),
        };
        let resp = rpc_round_trip(&state, req);
        let body = match resp.message {
            Some(proto_rpc::crypto::Message::TriggerRotationResponse(r)) => r,
            _ => panic!("expected TriggerRotationResponse"),
        };
        assert!(!body.success);
        assert!(
            body.error.contains("invalid remote_id"),
            "error should mention remote_id, got: {}",
            body.error
        );
    }
}

#[cfg(test)]
mod phase5_tests {
    //! Phase 5 unit tests — capability-gate enforcement. A peer
    //! that has not advertised `Capabilities::ROTATION` must not
    //! be rotated with, even when rotation is enabled locally and
    //! a session exists. The check runs before any state mutation
    //! so a legacy peer keeps the existing session intact.

    use super::phase2_tests::make_test_state;
    use super::*;
    use crate::node::user_accounts::UserAccount;
    use crate::router::users::{Capabilities, Users};
    use libp2p::identity::Keypair;

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

    /// Unknown peer: `get_capabilities` returns 0 → gate must reject.
    #[test]
    fn perform_rotation_rejects_peer_without_caps() {
        let state = make_test_state();
        let acct = CryptoStorage::test_account();
        let user = dummy_user_account();
        let remote = Keypair::generate_ed25519().public().to_peer_id();

        // No set_capabilities_for_tests → caps stay at 0.
        let err = Crypto::perform_rotation(&state, &user, &acct, remote)
            .expect_err("gate should refuse peer without ROTATION cap");
        assert!(
            err.contains("capability"),
            "unexpected error from gate: {err}"
        );
    }

    /// A peer whose caps bitset has the `ROTATION` bit set should
    /// pass the gate and fail further down (no session exists).
    /// That latter failure proves the gate let us through.
    #[test]
    fn perform_rotation_allows_peer_with_caps() {
        let state = make_test_state();
        let acct = CryptoStorage::test_account();
        let user = dummy_user_account();
        let remote = Keypair::generate_ed25519().public().to_peer_id();

        let rs = state.get_router();
        Users::set_capabilities_for_tests(&rs, &remote, Capabilities::ROTATION);

        let err = Crypto::perform_rotation(&state, &user, &acct, remote)
            .expect_err("no session should fail after the gate");
        assert!(
            !err.contains("capability"),
            "gate should have accepted ROTATION-capable peer, got: {err}"
        );
        assert!(
            err.contains("no session") || err.contains("no Transport"),
            "expected downstream no-session error, got: {err}"
        );
    }

    /// Wrong bits set but ROTATION absent must still be rejected —
    /// tests the bitmask semantics rather than a simple non-zero
    /// check.
    #[test]
    fn perform_rotation_rejects_unrelated_caps() {
        let state = make_test_state();
        let acct = CryptoStorage::test_account();
        let user = dummy_user_account();
        let remote = Keypair::generate_ed25519().public().to_peer_id();

        // Pretend a future hypothetical capability bit is advertised
        // but rotation is not. The gate must still reject.
        let unrelated = !Capabilities::ROTATION;
        let rs = state.get_router();
        Users::set_capabilities_for_tests(&rs, &remote, unrelated);

        let err = Crypto::perform_rotation(&state, &user, &acct, remote)
            .expect_err("gate should ignore unrelated caps");
        assert!(err.contains("capability"), "got: {err}");
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

    use super::phase2_tests::make_test_state;
    use super::phase3_tests;
    use super::*;
    use crate::services::crypto::events;
    use libp2p::identity::Keypair;
    use qaul_proto::qaul_rpc_crypto as proto_rpc;
    use std::sync::Mutex;

    /// The event log itself is still process-global (`OnceLock` ring
    /// buffer), so tests that observe it across emission/query
    /// boundaries must serialise through this lock. Per-test
    /// `QaulState` does not isolate the event log.
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
        assert_eq!(
            all.len(),
            events::MAX_EVENTS,
            "log should cap at MAX_EVENTS"
        );
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

    // A completed nonce-drain emits `DrainCompleted` and stamps
    // `last_retired_session_id` so the decrypt path can detect
    // post-drain messages afterwards.
    #[test]
    fn drain_emits_drain_completed_and_stamps_meta() {
        let _g = EVENT_LOG_LOCK.lock().unwrap();
        let state = make_test_state();
        events::clear_for_tests();
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));
        // Base 0, target 2 → need nonces 1 and 2 to complete.
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 42,
                draining_session_id: Some(7),
                draining_recv_target: Some(2),
                draining_recv_base: 0,
                ..Default::default()
            },
        );
        // Record inbound nonces, then retire (no unconfirmed outbound in
        // the test state's messaging table → outbound is "clear").
        CryptoNoise::record_drain_received(&acct, remote, 1);
        CryptoNoise::record_drain_received(&acct, remote, 2);
        Crypto::try_retire_drain(&state, &acct, remote);

        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.last_retired_session_id, Some(7));
        assert_eq!(meta.draining_session_id, None);

        let log = events::query(0, 0);
        assert!(
            log.iter()
                .any(|e| e.kind == events::RotationEventKind::DrainCompleted
                    && e.draining_session_id == 7),
            "DrainCompleted event was not emitted; log={:?}",
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
            pre_cipher_out: None,
            pre_index_out: 0,
            pre_cipher_in: None,
            pre_index_in_highest: 0,
            pre_index_in_seen: Vec::new(),
            pre_bytes_accounted: 0,
            established_at: 0,
        }
    }

    // `GetRotationEventsRequest` returns whatever the log currently
    // holds.
    #[test]
    fn rpc_get_events_returns_recorded_events() {
        // The event log is process-global so this test serialises
        // against other event-log tests. Per-test `QaulState` gives
        // us an isolated RPC channel without an extra lock.
        let _g_log = EVENT_LOG_LOCK.lock().unwrap();
        let state = make_test_state();
        events::clear_for_tests();
        phase3_tests::drain_rpc_channel(&state);

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
        let resp = phase3_tests::rpc_round_trip(&state, req);
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
