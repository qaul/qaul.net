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
        state = match Self::create_crypto_state::<D>(true, user_account.clone(), remote_key) {
            Some(s) => s,
            None => {
                log::error!("Failed to create crypto state for handshake 1");
                return (None, 0, 0);
            }
        };
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
            Some(U8Array::from_slice(match state.re.clone() {
                Some(re) => re,
                None => {
                    log::error!("Missing remote ephemeral key in crypto state");
                    return (None, 0);
                }
            }.as_slice())),
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
        let cipher_out = match state.clone().cipher_out {
            Some(key) => key,
            None => {
                log::error!("Missing cipher_out key in transport state");
                return (None, 0);
            }
        };
        let mut cipher: CipherState<C> =
            CipherState::new(cipher_out.as_slice(), nonce);

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
        state = match Self::create_crypto_state::<D>(false, user_account.clone(), remote_key) {
            Some(s) => s,
            None => {
                log::error!("Failed to create crypto state for decryption handshake 1");
                return None;
            }
        };
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
        let cipher_in = match state.cipher_in.clone() {
            Some(key) => key,
            None => {
                log::error!("Missing cipher_in key in transport state");
                return None;
            }
        };
        let mut cipher: CipherState<C> =
            CipherState::new(cipher_in.as_slice(), nonce);

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
    ) -> Option<CryptoState>
    where
        D: DH,
    {
        // create a new random session id
        let mut rng = rand::rng();
        let session_id: u32 = rng.random();

        // create private key
        let private_key = match Crypto25519::private_key_to_montgomery(user_account.keys) {
            Some(key) => key,
            None => {
                log::error!("Failed to convert private key to montgomery form");
                return None;
            }
        };

        // create public key
        let remote_public_key = match Crypto25519::public_key_to_montgomery(remote_key) {
            Some(key) => key,
            None => {
                log::error!("Failed to convert remote public key to montgomery form");
                return None;
            }
        };

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

        Some(state)
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
        //
        // Snapshot the current inbound high-water on the (still-primary)
        // session as the drain base now, before any in-flight rotation
        // traffic arrives. From here until finalisation,
        // `after_decrypt_rotation` records every inbound nonce on this
        // session into the drain bitmap, so when we learn the peer's
        // final nonce at finalise time the bitmap already covers the
        // tail and the drain completes the moment the last in-flight
        // message arrives — never prematurely.
        let drain_base = primary.highest_index_nonce_in;
        let meta = match storage.get_rotation_meta(remote_id) {
            Some(mut m) => {
                m.pending_initiated_session_id = Some(new_session_id);
                m.draining_recv_base = drain_base;
                m.draining_recv_seen = Vec::new();
                m
            }
            None => RotationMeta {
                primary_session_id: primary_id,
                pending_initiated_session_id: Some(new_session_id),
                draining_recv_base: drain_base,
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

        // Simultaneous-rotation collision resolution: **lower PeerId
        // wins**. A fixed, symmetric tie-break that neither side's
        // random session-id generation can influence — both ends
        // compute the same winner from the two PeerIds alone.
        let existing_meta = storage.get_rotation_meta(remote_id);
        if let Some(meta) = &existing_meta {
            if let Some(mine) = meta.pending_initiated_session_id {
                let i_win = user_account.id.to_bytes() < remote_id.to_bytes();
                if i_win {
                    // Our rotation wins; ignore the incoming one and wait
                    // for the peer to drop theirs and answer ours.
                    log::trace!(
                        "rotation collision: our PeerId is lower, ours ({}) wins over theirs ({})",
                        mine,
                        incoming.new_session_id
                    );
                    return None;
                } else {
                    // Peer's PeerId is lower; their rotation wins. Abandon
                    // our pending state and run the responder path.
                    log::trace!(
                        "rotation collision: peer PeerId is lower, theirs ({}) wins, abandoning ours ({})",
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
        //
        // The old session now drains the A->B direction (inbound, from
        // the perspective of this responder). The drain base is this
        // node's current inbound high-water on the old session — the
        // `RotateHandshakeFirst` we just processed arrived on it, so
        // everything up to here is already received. The drain *target*
        // (the initiator's final A->B nonce) is not yet known; it
        // arrives in `RotateHandshakeFinal`, so `draining_recv_target`
        // stays `None` and the session keeps draining until then.
        let drain_base = primary_state.highest_index_nonce_in;

        // Preserve any prior last_retired so the decrypt path can still
        // detect late arrivals on the most recent retirement.
        let prior = storage.get_rotation_meta(remote_id);
        let new_meta = RotationMeta {
            primary_session_id: incoming.new_session_id,
            pending_initiated_session_id: None,
            draining_session_id: Some(primary_id),
            draining_recv_target: None,
            draining_recv_base: drain_base,
            draining_recv_seen: Vec::new(),
            last_retired_session_id: prior.as_ref().and_then(|m| m.last_retired_session_id),
        };
        storage.save_rotation_meta(remote_id, &new_meta);

        // `final_nonce_out` (this responder's last B->A nonce on the old
        // session) is filled in by the caller at send time, when it is
        // the nonce this very `RotateHandshakeSecond` is encrypted with.
        Some(proto_net::RotateHandshakeSecond {
            new_session_id: incoming.new_session_id,
            noise_e: noise_e_out,
            nonce: incoming.nonce,
            final_nonce_out: 0,
            received_at: Timestamp::get_timestamp(),
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
    /// Returns the `RotateHandshakeFinal` (cut-over ACK) to send back
    /// on success, or `None` on any mismatch (unknown session_id, bad
    /// nonce, pending state absent/wrong kind). The caller fills in
    /// the ACK's `final_nonce_out` at send time and sends it under the
    /// old (draining) session.
    pub fn rotate_finalize_initiator<D, C, H, P>(
        storage: CryptoAccount,
        remote_id: PeerId,
        incoming: proto_net::RotateHandshakeSecond,
    ) -> Option<proto_net::RotateHandshakeFinal>
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
                return None;
            }
        };
        if !matches!(pending_state.state, CryptoProcessState::HalfOutgoing) {
            log::warn!(
                "rotate_finalize_initiator: pending state for session {} is not HalfOutgoing",
                incoming.new_session_id
            );
            return None;
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
                return None;
            }
        };
        if decrypted != incoming.nonce {
            log::warn!("rotate_finalize_initiator: nonce mismatch");
            return None;
        }

        // Flip meta: pending -> primary, old primary -> draining.
        let old_meta = match storage.get_rotation_meta(remote_id) {
            Some(m) => m,
            None => {
                log::warn!("rotate_finalize_initiator: no rotation meta");
                return None;
            }
        };
        let old_primary = old_meta.primary_session_id;
        let now_ms = Timestamp::get_timestamp();

        // The old session drains the B->A direction. We learned the
        // peer's final B->A nonce from this `RotateHandshakeSecond`, so
        // set it as the drain target. The base and the in-flight bitmap
        // were seeded at `rotate_initiate` and accumulated by
        // `after_decrypt_rotation` since, so the drain completes once
        // every nonce up to the target has arrived — clock-free.
        let new_meta = RotationMeta {
            primary_session_id: incoming.new_session_id,
            pending_initiated_session_id: None,
            draining_session_id: Some(old_primary),
            draining_recv_target: Some(incoming.final_nonce_out),
            draining_recv_base: old_meta.draining_recv_base,
            draining_recv_seen: old_meta.draining_recv_seen,
            last_retired_session_id: old_meta.last_retired_session_id,
        };
        storage.save_rotation_meta(remote_id, &new_meta);

        // Emit a `Rotated` event so clients can surface the state
        // transition to the UI.
        events::record(
            &storage,
            RotationEvent {
                kind: RotationEventKind::Rotated,
                remote_id,
                primary_session_id: incoming.new_session_id,
                draining_session_id: old_primary,
                timestamp_ms: now_ms,
            },
        );

        // `final_nonce_out` (our last A->B nonce on the old session) is
        // filled in by the caller at send time.
        Some(proto_net::RotateHandshakeFinal {
            new_session_id: incoming.new_session_id,
            nonce: incoming.nonce,
            final_nonce_out: 0,
        })
    }

    /// Record an inbound nonce on a draining session into the drain
    /// bitmap and persist the meta. This only tracks reception; the
    /// decision to retire the old session is made by
    /// `Crypto::try_retire_drain`, which also requires that this
    /// node's outbound traffic on the old session has been confirmed
    /// (so an unconfirmed message is never stranded on a retired
    /// session). There is no timer anywhere.
    pub fn record_drain_received(storage: &CryptoAccount, remote_id: PeerId, nonce: u64) {
        let mut meta = match storage.get_rotation_meta(remote_id) {
            Some(m) => m,
            None => return,
        };
        meta.mark_drain_received(nonce);
        storage.save_rotation_meta(remote_id, &meta);
    }

    /// Retire the draining session for `remote_id`: delete its
    /// `CryptoState` row (dropping its cipher material), clear the
    /// draining fields on the meta, and emit `DrainCompleted`. The
    /// caller is responsible for having checked both the inbound drain
    /// (`RotationMeta::drain_complete`) and outbound confirmation.
    pub fn retire_drain(storage: &CryptoAccount, remote_id: PeerId, mut meta: RotationMeta) {
        let drain_id = match meta.draining_session_id {
            Some(id) => id,
            None => return,
        };
        storage.delete_state(remote_id, drain_id);
        meta.clear_drain(drain_id);
        storage.save_rotation_meta(remote_id, &meta);
        events::record(
            storage,
            RotationEvent {
                kind: RotationEventKind::DrainCompleted,
                remote_id,
                primary_session_id: 0,
                draining_session_id: drain_id,
                timestamp_ms: 0,
            },
        );
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
    /// that the nonce-drain retires.
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

    // record_drain_received marks the nonce bitmap and persists it,
    // without retiring (retirement is decided in mod.rs, gated on
    // outbound confirmation). Inbound-completeness is reported by
    // RotationMeta::drain_complete.
    #[test]
    fn record_marks_bitmap_and_reports_completion() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));
        // Base 10, target 13 → must receive nonces 11, 12, 13.
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 42,
                draining_session_id: Some(7),
                draining_recv_target: Some(13),
                draining_recv_base: 10,
                ..Default::default()
            },
        );

        // Top nonce first (out of order) — not complete, 11/12 missing.
        CryptoNoise::record_drain_received(&acct, remote, 13);
        let meta = acct.get_rotation_meta(remote).unwrap();
        assert!(meta.drain_nonce_seen(13));
        assert!(!meta.drain_complete(), "gap below the top nonce");
        // record_received never deletes the session.
        assert!(acct.get_state_by_id(remote, 7).is_some());

        CryptoNoise::record_drain_received(&acct, remote, 12);
        assert!(!acct.get_rotation_meta(remote).unwrap().drain_complete());
        CryptoNoise::record_drain_received(&acct, remote, 11);
        assert!(
            acct.get_rotation_meta(remote).unwrap().drain_complete(),
            "window (10,13] complete"
        );
    }

    // While the drain target is unknown (responder before
    // RotateHandshakeFinal), drain_complete is always false no matter
    // how many nonces are recorded.
    #[test]
    fn drain_incomplete_while_target_unknown() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_rotation_meta(
            remote,
            &RotationMeta {
                primary_session_id: 42,
                draining_session_id: Some(7),
                draining_recv_target: None,
                draining_recv_base: 0,
                ..Default::default()
            },
        );
        for nonce in 0..50 {
            CryptoNoise::record_drain_received(&acct, remote, nonce);
        }
        assert!(!acct.get_rotation_meta(remote).unwrap().drain_complete());
    }

    // retire_drain deletes the draining state, clears the meta, and
    // records the retirement.
    #[test]
    fn retire_drain_deletes_and_stamps() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 7, dummy_state(7));
        let meta = RotationMeta {
            primary_session_id: 42,
            draining_session_id: Some(7),
            draining_recv_target: Some(0),
            draining_recv_base: 0,
            ..Default::default()
        };
        acct.save_rotation_meta(remote, &meta);

        CryptoNoise::retire_drain(&acct, remote, meta);

        assert!(acct.get_state_by_id(remote, 7).is_none());
        let after = acct.get_rotation_meta(remote).unwrap();
        assert_eq!(after.draining_session_id, None);
        assert_eq!(after.last_retired_session_id, Some(7));
    }

    // record_drain_received is a no-op when there is no meta row.
    #[test]
    fn record_noop_without_meta() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        acct.save_state(remote, 1, dummy_state(1));
        CryptoNoise::record_drain_received(&acct, remote, 5);
        assert!(acct.get_rotation_meta(remote).is_none());
        assert!(acct.get_state_by_id(remote, 1).is_some());
    }
}
