// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Noise related Crypto Functions
//!
//! This handles the encryption and decryption via the noise related functions.

use libp2p::identity::PublicKey;
use libp2p::PeerId;
use noise_protocol::{Cipher, CipherState, HandshakeState, Hash, U8Array, DH};
use rand::Rng;

use super::events::{self, RotationEvent, RotationEventKind};
use super::storage::RotationMeta;
use super::{Crypto25519, CryptoAccount, CryptoProcessState, CryptoState};
use crate::node::user_accounts::UserAccount;
use crate::router::users::Users;
use crate::storage::configuration::Configuration;
use crate::utilities::timestamp::Timestamp;

/// Protobuf message definitions for the CryptoService.
pub use qaul_proto::qaul_net_crypto as proto_net;

pub struct CryptoNoise {}

impl CryptoNoise {
    /// Encrypt a Message with first handshake
    pub fn encrypt_noise_kk_handshake_1<D, C, H, P>(
        qaul_state: &crate::QaulState,
        data: Vec<u8>,
        user_account: UserAccount,
        storage: CryptoAccount,
        remote_id: PeerId,
    ) -> (Option<Vec<u8>>, u64, u32)
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let mut state: CryptoState;
        let message: Option<Vec<u8>>;
        let nonce: u64 = 0;

        // no saved state available: generate crypto state
        log::trace!("Initiate Crypto Handshake");

        // get receivers public key
        let remote_key: PublicKey;
        {
            let rs = qaul_state.get_router();
            match Users::get_pub_key(&rs, &remote_id) {
                Some(key) => remote_key = key,
                None => {
                    log::error!("No key found for user {:?}", remote_id);
                    return (None, 0, 0);
                }
            }
        }

        // create crypto state
        state = Self::create_crypto_state::<D>(true, user_account.clone(), remote_key);
        let session_id = state.session_id;

        log::trace!("new session generated with session_id: {}", session_id);

        // create handshake pattern
        let pattern = noise_protocol::patterns::noise_kk();

        // create handshake values
        let prologue: Vec<u8> = Vec::new();
        let e = D::genkey();

        // build noise state
        let mut handshake: HandshakeState<D, C, H> = noise_protocol::HandshakeState::new(
            pattern,
            true,
            prologue.as_slice(),
            Some(U8Array::from_slice(state.s.as_slice())),
            Some(e.clone()),
            Some(U8Array::from_slice(state.rs.as_slice())),
            None,
        );

        // create handshake message with encrypted data
        match handshake.write_message_vec(data.as_slice()) {
            Ok(output) => message = Some(output),
            Err(e) => {
                // TODO: delete old handshake
                log::error!("{}", e);
                return (None, 0, 0);
            }
        }

        // save state to data base
        state.e = e.as_slice().to_vec();
        //Self::save_handshake_state(user_account, remote_id, state);
        storage.save_state(remote_id, session_id, state);

        (message, nonce, session_id)
    }

    /// Encrypt a Message with first handshake
    pub fn encrypt_noise_kk_handshake_2<D, C, H, P>(
        data: Vec<u8>,
        storage: CryptoAccount,
        mut state: CryptoState,
        remote_id: PeerId,
    ) -> (Option<Vec<u8>>, u64)
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let mut message: Option<Vec<u8>> = None;
        let nonce: u64 = 0;

        // we need to send the second handshake message
        log::trace!("Send Handshake Response");

        // create handshake values
        let pattern = noise_protocol::patterns::noise_kk();
        let prologue: Vec<u8> = Vec::new();
        let e = D::Key::from_slice(state.e.as_slice());

        // build noise handshake state
        let mut handshake: HandshakeState<D, C, H> = noise_protocol::HandshakeState::new(
            pattern,
            false,
            prologue.as_slice(),
            Some(U8Array::from_slice(state.s.clone().as_slice())),
            Some(e),
            Some(U8Array::from_slice(state.rs.clone().as_slice())),
            Some(U8Array::from_slice(state.re.clone().unwrap().as_slice())),
        );

        // set message index
        handshake.set_index(1);

        // create handshake message
        match handshake.write_message_vec(data.as_slice()) {
            Ok(output) => {
                message = Some(output);
            }
            Err(e) => {
                // TODO: delete old handshake
                log::error!("{}", e);
                return (message, nonce);
            }
        }

        // get ciphers & put them to state
        let (cipher_key_in, cipher_key_out) = handshake.get_ciphers();
        let (key_out, _nonce_out) = cipher_key_out.extract();
        let (key_in, _nonce_in) = cipher_key_in.extract();

        state.state = CryptoProcessState::Transport;
        state.cipher_in = Some(key_in.as_slice().to_vec());
        state.highest_index_nonce_in = 0;
        state.cipher_out = Some(key_out.as_slice().to_vec());
        state.index_nonce_out = 0;
        state.established_at = Timestamp::get_timestamp();

        // save crypto state to data base
        storage.save_state(remote_id, state.session_id, state);

        (message, nonce)
    }

    /// Encrypt a Message in transport state
    pub fn encrypt_noise_kk_transport<D, C, H, P>(
        data: Vec<u8>,
        storage: CryptoAccount,
        mut state: CryptoState,
        remote_id: PeerId,
    ) -> (Option<Vec<u8>>, u64)
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let message: Option<Vec<u8>>;
        let nonce: u64;

        log::trace!("Encrypt with full encryption");

        // the handshake has been done, we can encrypt messages
        nonce = state.index_nonce_out;

        // create cipher
        let mut cipher: CipherState<C> =
            CipherState::new(state.clone().cipher_out.unwrap().as_slice(), nonce);

        // encrypt message
        message = Some(cipher.encrypt_vec(data.as_slice()));

        // save new nonce to state
        if nonce >= u64::MAX - 1 {
            log::error!(
                "nonce overflow imminent for session {}: session must be renegotiated",
                state.session_id
            );
        } else {
            state.index_nonce_out = nonce + 1;
        }

        // save state
        storage.save_state(remote_id, state.session_id, state);

        (message, nonce)
    }

    /// Decrypt handshake message 1
    ///
    /// Decrypt an incoming message.
    ///
    /// Returns the decrypted message and the current crypto state
    pub fn decrypt_noise_kk_handshake_1<D, C, H, P>(
        qaul_state: &crate::QaulState,
        data: Vec<u8>,
        _storage: CryptoAccount,
        remote_id: PeerId,
        user_account: UserAccount,
        session_id: u32,
    ) -> Option<(Vec<u8>, CryptoState)>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let mut state: CryptoState;
        let message: Vec<u8>;

        log::trace!("Decrypt Incoming handshake");

        // get receivers public key
        let remote_key: PublicKey;
        {
            let rs = qaul_state.get_router();
            match Users::get_pub_key(&rs, &remote_id) {
                Some(key) => remote_key = key,
                None => {
                    log::error!("No key found for user {:?}", remote_id);
                    return None;
                }
            }
        }

        // create initial crypto state
        state = Self::create_crypto_state::<D>(false, user_account.clone(), remote_key);
        // save session_id to state
        state.session_id = session_id;

        // create handshake values
        let pattern = noise_protocol::patterns::noise_kk();
        let prologue: Vec<u8> = Vec::new();
        let e = D::Key::from_slice(state.e.as_slice());

        // build noise handshake state
        let mut handshake: HandshakeState<D, C, H> = noise_protocol::HandshakeState::new(
            pattern,
            false,
            prologue.as_slice(),
            Some(U8Array::from_slice(state.s.as_slice())),
            Some(e),
            Some(U8Array::from_slice(state.rs.as_slice())),
            None,
        );

        // read and decrypt incoming handshake message
        match handshake.read_message_vec(data.as_slice()) {
            Ok(encrypted) => message = encrypted,
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        }

        // get remote ephemeral
        match handshake.get_re() {
            Some(re) => {
                // put it to state
                state.re = Some(Vec::from(re.as_slice()));
            }
            None => {
                // TODO: remove state
                return None;
            }
        }

        Some((message, state))
    }

    /// Decrypt handshake message 2
    ///
    /// Decrypt an incoming message.
    pub fn decrypt_noise_kk_handshake_2<D, C, H, P>(
        data: Vec<u8>,
        mut state: CryptoState,
        storage: CryptoAccount,
        remote_id: PeerId,
    ) -> Option<Vec<u8>>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let message: Option<Vec<u8>>;

        // this should be the responders hand shake message.
        log::trace!("Receiving handshake confirmation");

        // create handshake values
        let pattern = noise_protocol::patterns::noise_kk();
        let prologue: Vec<u8> = Vec::new();
        let e = D::Key::from_slice(state.e.as_slice());

        // build noise handshake state
        let mut handshake: HandshakeState<D, C, H> = noise_protocol::HandshakeState::new(
            pattern,
            true,
            prologue.as_slice(),
            Some(U8Array::from_slice(state.s.as_slice())),
            Some(e),
            Some(U8Array::from_slice(state.rs.as_slice())),
            None,
        );

        // set message index
        handshake.set_index(1);

        // read and decrypt incoming handshake message
        match handshake.read_message_vec(data.as_slice()) {
            Ok(encrypted) => message = Some(encrypted),
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        }

        // get remote ephemeral
        match handshake.get_re() {
            Some(re) => {
                // put it to state
                state.re = Some(Vec::from(re.as_slice()));
            }
            None => {
                // TODO: remove state
                return None;
            }
        }

        // get ciphers & put them to state
        let (cipher_key_out, cipher_key_in) = handshake.get_ciphers();
        let (key_out, nonce_out) = cipher_key_out.extract();
        let (key_in, nonce_in) = cipher_key_in.extract();

        log::trace!(
            "handshake initiation finished: nonce out: {}, in: {}",
            nonce_out,
            nonce_in
        );

        state.state = CryptoProcessState::Transport;
        state.cipher_in = Some(key_in.as_slice().to_vec());
        state.highest_index_nonce_in = 0;
        state.cipher_out = Some(key_out.as_slice().to_vec());
        state.index_nonce_out = 0;
        state.established_at = Timestamp::get_timestamp();

        // save state to data base
        storage.save_state(remote_id, state.session_id, state);

        message
    }

    /// Decrypt transport message
    ///
    /// Decrypt an incoming message.
    pub fn decrypt_noise_kk_transport<D, C, H, P>(
        data: Vec<u8>,
        nonce: u64,
        mut state: CryptoState,
        storage: CryptoAccount,
        remote_id: PeerId,
    ) -> Option<Vec<u8>>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let message: Option<Vec<u8>>;

        // we had a successful handshake and are in transport state
        log::trace!("Decrypting with full encryption");

        // create cipher
        let mut cipher: CipherState<C> =
            CipherState::new(state.cipher_in.clone().unwrap().as_slice(), nonce);

        // decrypt message
        match cipher.decrypt_vec(data.as_slice()) {
            Ok(decrypted) => {
                message = Some(decrypted);

                state.highest_index_nonce_in = nonce;
                storage.save_state(remote_id, state.session_id, state);
            }
            Err(_) => {
                log::error!("decryption error");
                return None;
            }
        }

        message
    }

    /// Create CryptoState during handshake phase
    /// for outgoing or incoming
    fn create_crypto_state<D>(
        initiator: bool,
        user_account: UserAccount,
        remote_key: PublicKey,
    ) -> CryptoState
    where
        D: DH,
    {
        // create a new random session id
        let mut rng = rand::rng();
        let session_id: u32 = rng.random();

        // create private key
        let private_key = Crypto25519::private_key_to_montgomery(user_account.keys).unwrap();

        // create public key
        let remote_public_key = Crypto25519::public_key_to_montgomery(remote_key).unwrap();

        // create new ephemeral key
        let e = D::genkey();

        // create CryptoState structure
        let rs_vec: Vec<u8> = Vec::from(remote_public_key.to_bytes());
        let e_vec: Vec<u8> = Vec::from(e.as_slice());
        let process_state: CryptoProcessState;
        if initiator {
            process_state = CryptoProcessState::HalfOutgoing;
        } else {
            process_state = CryptoProcessState::HalfIncoming;
        }

        let state = CryptoState {
            session_id,
            state: process_state,
            initiator,
            s: private_key,
            rs: rs_vec,
            e: e_vec,
            re: None,
            cipher_out: None,
            index_nonce_out: 0,
            cipher_in: None,
            highest_index_nonce_in: 0,
            out_of_order_indexes: false,
            established_at: 0,
        };

        state
    }

    // ------------------------------------------------------------------
    //                    Phase-1 session rotation primitives
    // ------------------------------------------------------------------
    //
    // These functions re-run the Noise KK handshake under a fresh
    // `session_id` while the previous session remains usable for
    // *receiving* messages until its grace window expires.
    //
    // The rotation-handshake bytes produced here are meant to be carried
    // in a `CryptoserviceContainer { rotate_first | rotate_second }` that
    // the messaging layer will send encrypted under the *currently
    // primary* session. Phase 1 only exposes the primitives; trigger
    // wiring (periodic task, volume counter) is Phase 2.

    /// Anti-replay nonce size for rotation handshakes.
    const ROTATION_NONCE_LEN: usize = 16;

    /// Start a session rotation as the initiator.
    ///
    /// Generates a fresh `new_session_id`, runs KK step 1 with new
    /// ephemerals (a brand-new `CryptoState` saved in `HalfOutgoing`),
    /// and records a `pending_initiated_session_id` on this peer's
    /// `RotationMeta`. The returned `RotateHandshakeFirst` is intended
    /// to be sent to the remote inside a `CryptoserviceContainer` on
    /// the currently-primary session.
    ///
    /// Returns `None` when no primary session exists for `remote_id`,
    /// or the underlying KK step-1 fails.
    pub fn rotate_initiate<D, C, H, P>(
        state: &crate::QaulState,
        user_account: UserAccount,
        storage: CryptoAccount,
        remote_id: PeerId,
    ) -> Option<proto_net::RotateHandshakeFirst>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        // Precondition: must have an existing primary session to rotate from.
        let primary = storage.get_state(remote_id)?;
        let primary_id = primary.session_id;

        // Anti-replay nonce embedded in the KK payload and echoed by the
        // responder; binds a specific rotation to a specific challenge so
        // a replayed handshake cannot resurrect a retired session.
        let mut rng = rand::rng();
        let mut nonce = vec![0u8; Self::ROTATION_NONCE_LEN];
        rng.fill(&mut nonce[..]);

        // Reuse the existing KK step-1 writer. This creates a new
        // CryptoState with a fresh random session_id in `HalfOutgoing`,
        // saves it under {remote_id, new_session_id}, and returns the
        // Noise write_message output (ephemeral + encrypted payload).
        let (noise_e, _msg_nonce, new_session_id) =
            Self::encrypt_noise_kk_handshake_1::<D, C, H, P>(
                state,
                nonce.clone(),
                user_account,
                storage.clone(),
                remote_id,
            );
        let noise_e = noise_e?;

        // Record the in-flight initiation on the rotation meta so the
        // collision-resolution rule can detect simultaneous rotations.
        let meta = match storage.get_rotation_meta(remote_id) {
            Some(mut m) => {
                m.pending_initiated_session_id = Some(new_session_id);
                m
            }
            None => RotationMeta {
                primary_session_id: primary_id,
                pending_initiated_session_id: Some(new_session_id),
                ..Default::default()
            },
        };
        storage.save_rotation_meta(remote_id, &meta);

        Some(proto_net::RotateHandshakeFirst {
            new_session_id,
            noise_e,
            nonce,
            initiated_at: Timestamp::get_timestamp(),
        })
    }

    /// Respond to an incoming `RotateHandshakeFirst`.
    ///
    /// Runs the KK step-2 under the incoming `new_session_id`, persists
    /// the new `CryptoState` in `Transport`, moves the previous
    /// primary into the grace window (draining), and returns the
    /// `RotateHandshakeSecond` to send back under the current session.
    ///
    /// Collision rule: if this node already has a pending initiation
    /// with `mine` and receives an incoming rotate_first with
    /// `incoming.new_session_id`, the numerically smaller session_id
    /// wins. If `incoming` wins, `mine`'s `HalfOutgoing` state is
    /// deleted and this node processes `incoming`. If `mine` wins,
    /// `None` is returned (incoming is ignored; this node's own
    /// rotation will continue).
    pub fn rotate_complete_responder<D, C, H, P>(
        state: &crate::QaulState,
        user_account: UserAccount,
        storage: CryptoAccount,
        remote_id: PeerId,
        incoming: proto_net::RotateHandshakeFirst,
    ) -> Option<proto_net::RotateHandshakeSecond>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        // Need an existing primary to move into draining.
        let primary_state = storage.get_state(remote_id)?;
        let primary_id = primary_state.session_id;

        // Simultaneous-rotation collision resolution.
        let existing_meta = storage.get_rotation_meta(remote_id);
        if let Some(meta) = &existing_meta {
            if let Some(mine) = meta.pending_initiated_session_id {
                if mine <= incoming.new_session_id {
                    // mine wins (or tie — defensively prefer ours). Ignore.
                    log::trace!(
                        "rotation collision: ours ({}) wins over theirs ({})",
                        mine,
                        incoming.new_session_id
                    );
                    return None;
                } else {
                    // incoming wins. Abandon our pending state.
                    log::trace!(
                        "rotation collision: theirs ({}) wins, abandoning ours ({})",
                        incoming.new_session_id,
                        mine
                    );
                    storage.delete_state(remote_id, mine);
                    // Fall through to run responder path.
                }
            }
        }

        // Decode the KK step-1 input — produces a CryptoState in
        // HalfIncoming with `re` populated, carrying the decrypted
        // payload (which must equal the nonce for anti-replay).
        let (payload, crypto_state) = Self::decrypt_noise_kk_handshake_1::<D, C, H, P>(
            state,
            incoming.noise_e.clone(),
            storage.clone(),
            remote_id,
            user_account,
            incoming.new_session_id,
        )?;

        // Verify nonce echoed back in the KK payload.
        if payload != incoming.nonce {
            log::warn!(
                "rotate_complete_responder: nonce mismatch from {}",
                remote_id.to_base58()
            );
            storage.delete_state(remote_id, incoming.new_session_id);
            return None;
        }

        // Run KK step-2 to produce the rotate_second bytes. Also
        // transitions the new state to Transport and saves it.
        let (noise_e_out, _) = Self::encrypt_noise_kk_handshake_2::<D, C, H, P>(
            incoming.nonce.clone(),
            storage.clone(),
            crypto_state,
            remote_id,
        );
        let noise_e_out = noise_e_out?;

        // Flip meta: old primary -> draining, incoming -> primary.
        let now_ms = Timestamp::get_timestamp();
        let cfg = Configuration::get(state);
        let grace_ms = cfg.crypto_rotation.grace_period_seconds * 1000;
        let grace_vol = cfg.crypto_rotation.grace_volume_messages;

        // Preserve any prior last_retired fields so the decrypt path
        // can still detect messages for the most recent retirement.
        let prior = storage.get_rotation_meta(remote_id);
        let new_meta = RotationMeta {
            primary_session_id: incoming.new_session_id,
            pending_initiated_session_id: None,
            draining_session_id: Some(primary_id),
            draining_until: Some(now_ms + grace_ms),
            draining_remaining_volume: Some(grace_vol),
            last_retired_session_id: prior.as_ref().and_then(|m| m.last_retired_session_id),
            last_retired_at: prior.as_ref().and_then(|m| m.last_retired_at),
        };
        storage.save_rotation_meta(remote_id, &new_meta);

        Some(proto_net::RotateHandshakeSecond {
            new_session_id: incoming.new_session_id,
            noise_e: noise_e_out,
            nonce: incoming.nonce,
            received_at: now_ms,
        })
    }

    /// Finalise a rotation on the initiator side.
    ///
    /// Called when a `RotateHandshakeSecond` arrives whose
    /// `new_session_id` matches `pending_initiated_session_id` on this
    /// peer's rotation meta. Completes KK step-2 on the pending
    /// `HalfOutgoing` `CryptoState`, flips primary to the new session
    /// id, and moves the old primary into the grace window.
    ///
    /// Returns `true` on success, `false` on any mismatch (unknown
    /// session_id, bad nonce, pending state absent/wrong kind).
    pub fn rotate_finalize_initiator<D, C, H, P>(
        state: &crate::QaulState,
        storage: CryptoAccount,
        remote_id: PeerId,
        incoming: proto_net::RotateHandshakeSecond,
    ) -> bool
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let pending_state = match storage.get_state_by_id(remote_id, incoming.new_session_id) {
            Some(s) => s,
            None => {
                log::warn!(
                    "rotate_finalize_initiator: no pending state for session {}",
                    incoming.new_session_id
                );
                return false;
            }
        };
        if !matches!(pending_state.state, CryptoProcessState::HalfOutgoing) {
            log::warn!(
                "rotate_finalize_initiator: pending state for session {} is not HalfOutgoing",
                incoming.new_session_id
            );
            return false;
        }

        let decrypted = match Self::decrypt_noise_kk_handshake_2::<D, C, H, P>(
            incoming.noise_e,
            pending_state,
            storage.clone(),
            remote_id,
        ) {
            Some(p) => p,
            None => {
                log::warn!("rotate_finalize_initiator: KK step-2 decrypt failed");
                return false;
            }
        };
        if decrypted != incoming.nonce {
            log::warn!("rotate_finalize_initiator: nonce mismatch");
            return false;
        }

        // Flip meta: pending -> primary, old primary -> draining.
        let old_meta = match storage.get_rotation_meta(remote_id) {
            Some(m) => m,
            None => {
                log::warn!("rotate_finalize_initiator: no rotation meta");
                return false;
            }
        };
        let old_primary = old_meta.primary_session_id;
        let now_ms = Timestamp::get_timestamp();
        let cfg = Configuration::get(state);
        let grace_ms = cfg.crypto_rotation.grace_period_seconds * 1000;
        let grace_vol = cfg.crypto_rotation.grace_volume_messages;

        let new_meta = RotationMeta {
            primary_session_id: incoming.new_session_id,
            pending_initiated_session_id: None,
            draining_session_id: Some(old_primary),
            draining_until: Some(now_ms + grace_ms),
            draining_remaining_volume: Some(grace_vol),
            last_retired_session_id: old_meta.last_retired_session_id,
            last_retired_at: old_meta.last_retired_at,
        };
        storage.save_rotation_meta(remote_id, &new_meta);

        // Emit a `Rotated` event so clients can surface the state
        // transition to the UI.
        events::record(RotationEvent {
            kind: RotationEventKind::Rotated,
            remote_id,
            primary_session_id: incoming.new_session_id,
            draining_session_id: old_primary,
            timestamp_ms: now_ms,
        });
        true
    }

    /// Retire any draining sessions whose grace window has elapsed.
    ///
    /// Scans the `rotation_meta` tree and, for every entry whose
    /// `draining_until <= now_ms` or whose `draining_remaining_volume`
    /// has hit zero, deletes the draining `CryptoState` row (sled
    /// drop zeroizes its bincode bytes — the in-memory ciphers from a
    /// `Some(Vec<u8>)` are also dropped) and clears the draining
    /// fields on the meta row.
    ///
    /// Intended to be called from a periodic task in Phase 2; exposed
    /// here so Phase 1 unit tests can exercise it directly.
    pub fn drain_expired_rotations(storage: CryptoAccount, now_ms: u64) {
        for result in storage.rotation_meta.iter() {
            let (key, value) = match result {
                Ok(kv) => kv,
                Err(e) => {
                    log::error!("drain_expired_rotations iter: {}", e);
                    continue;
                }
            };
            let meta: RotationMeta = match bincode::deserialize(&value) {
                Ok(m) => m,
                Err(e) => {
                    log::error!("drain_expired_rotations deserialize: {}", e);
                    continue;
                }
            };
            let expired = match (meta.draining_until, meta.draining_remaining_volume) {
                (Some(until), _) if now_ms >= until => true,
                (_, Some(0)) => true,
                _ => false,
            };
            if !expired {
                continue;
            }
            let drain_id = match meta.draining_session_id {
                Some(id) => id,
                None => continue,
            };
            let remote_id = match PeerId::from_bytes(&key) {
                Ok(p) => p,
                Err(e) => {
                    log::error!("drain_expired_rotations key decode: {}", e);
                    continue;
                }
            };
            storage.delete_state(remote_id, drain_id);
            let cleared = RotationMeta {
                draining_session_id: None,
                draining_until: None,
                draining_remaining_volume: None,
                last_retired_session_id: Some(drain_id),
                last_retired_at: Some(now_ms),
                ..meta
            };
            storage.save_rotation_meta(remote_id, &cleared);

            events::record(RotationEvent {
                kind: RotationEventKind::GraceExpired,
                remote_id,
                primary_session_id: 0,
                draining_session_id: drain_id,
                timestamp_ms: now_ms,
            });
        }
    }
}

#[cfg(test)]
mod rotation_tests {
    use super::*;
    use crate::services::crypto::storage::CryptoStorage;
    use libp2p::identity::Keypair;

    fn fresh_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    /// Build a dummy `CryptoState` for tests. Values are stand-ins
    /// — these tests do not exercise any Noise code paths, only the
    /// lifecycle of the `rotation_meta` <-> `crypto_state` tree pair
    /// that `drain_expired_rotations` scans.
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

    // Meta with a non-expired draining session must be left untouched.
    #[test]
    fn drain_leaves_unexpired() {
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

        // now < until, volume > 0 → not expired
        CryptoNoise::drain_expired_rotations(acct.clone(), 5_000);

        assert!(acct.get_state_by_id(remote, 7).is_some());
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.draining_session_id, Some(7));
        assert_eq!(meta.draining_until, Some(10_000));
    }

    // Past `draining_until` → draining state deleted, meta cleared.
    #[test]
    fn drain_retires_time_expired() {
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

        CryptoNoise::drain_expired_rotations(acct.clone(), 10_000);

        assert!(
            acct.get_state_by_id(remote, 7).is_none(),
            "draining state row should be deleted"
        );
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.primary_session_id, 42);
        assert_eq!(meta.draining_session_id, None);
        assert_eq!(meta.draining_until, None);
        assert_eq!(meta.draining_remaining_volume, None);
    }

    // `draining_remaining_volume == 0` is equivalent to expiry.
    #[test]
    fn drain_retires_volume_exhausted() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 42,
                draining_session_id: Some(7),
                draining_until: Some(u64::MAX),
                draining_remaining_volume: Some(0),
                ..Default::default()
            },
        );

        CryptoNoise::drain_expired_rotations(acct.clone(), 1);

        assert!(acct.get_state_by_id(remote, 7).is_none());
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.draining_session_id, None);
    }

    // A meta row with no draining fields is a no-op.
    #[test]
    fn drain_noop_on_primary_only_meta() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 1, dummy_state(1));
        acct.save_rotation_meta(remote, &RotationMeta::primary_only(1));

        CryptoNoise::drain_expired_rotations(acct.clone(), u64::MAX);

        assert!(acct.get_state_by_id(remote, 1).is_some());
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(meta.primary_session_id, 1);
    }
}
