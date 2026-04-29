// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Noise related Crypto Functions
//!
//! This handles the encryption and decryption via the noise related functions.

use libp2p::identity::PublicKey;
use libp2p::PeerId;
use noise_protocol::{Cipher, CipherState, HandshakeState, Hash, U8Array, DH};
use rand::Rng;

use super::{Crypto25519, CryptoAccount, CryptoProcessState, CryptoState};
use crate::node::user_accounts::UserAccount;
use crate::router::users::Users;

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

        // Capture the post-msg-1 partial cipher for handshake extras.
        //
        // `HandshakeState::get_ciphers` calls `SymmetricState::split`,
        // which is a pure HKDF derivation off the current chaining
        // key. After `write_message_vec` for KK msg 1 the initiator
        // has applied `MixKey(es)` and `MixKey(ss)`; the responder
        // arrives at the same `ck` once it processes msg 1, so both
        // sides derive the same `(c1, c2)` here. We use `c1` (the
        // initiator-to-responder direction by Noise convention) for
        // pre-completion frames sent under this session.
        //
        // The post-msg-2 split (in `encrypt_noise_kk_handshake_2`)
        // operates on a different `ck` and produces a different key
        // pair, so transport messages and extras never share keys.
        let (c1, _c2) = handshake.get_ciphers();
        let (pre_key, _pre_nonce) = c1.extract();
        state.pre_cipher_out = Some(pre_key.as_slice().to_vec());

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

        // Capture the post-msg-1 partial cipher for handshake extras.
        //
        // Mirrors the initiator side in `encrypt_noise_kk_handshake_1`:
        // both parties call `split` on the same `ck` after msg 1 is
        // processed, so the responder lands on the same `(c1, c2)` and
        // can decrypt extras the initiator emitted under `c1`. Stored
        // here so a daemon restart between msg 1 and msg 2 does not
        // lose the ability to decrypt queued extras.
        let (c1, _c2) = handshake.get_ciphers();
        let (pre_key, _pre_nonce) = c1.extract();
        state.pre_cipher_in = Some(pre_key.as_slice().to_vec());

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

    /// Encrypt a pre-completion (handshake-extras) payload on the
    /// initiator side.
    ///
    /// The session must be in `HalfOutgoing`; this is the branch that
    /// would otherwise fail with "Can't send further messages after
    /// handshake". We instead reuse the partial CipherState captured
    /// at KK msg 1 and produce a `HandshakeExtraPayload`-shaped
    /// ciphertext indexed by `pre_index_out`.
    ///
    /// Returns `(ciphertext, pre_index)` on success. On failure
    /// (missing pre-cipher key, no extras feature, limit hit) returns
    /// `None`; the caller is responsible for surfacing the failure
    /// (UI "queued limit reached" / "fall back to fresh session").
    pub fn encrypt_noise_kk_handshake_extra<D, C, H, P>(
        _qaul_state: &crate::QaulState,
        data: Vec<u8>,
        storage: CryptoAccount,
        mut crypto_state: CryptoState,
        remote_id: PeerId,
    ) -> Option<(Vec<u8>, u64)>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        // Pre-cipher must have been captured at KK msg 1 time.
        let key = match crypto_state.pre_cipher_out.as_ref() {
            Some(k) => k.clone(),
            None => {
                log::error!(
                    "encrypt_noise_kk_handshake_extra: missing pre_cipher_out for session {}",
                    crypto_state.session_id
                );
                return None;
            }
        };

        let pre_index = crypto_state.pre_index_out;

        // Catastrophic but theoretical: 2^64 - 1 extras on a single
        // stuck handshake. Refuse to roll over rather than reuse a
        // nonce.
        if pre_index >= u64::MAX - 1 {
            log::error!(
                "pre_index overflow for session {}: refusing to encrypt further extras",
                crypto_state.session_id
            );
            return None;
        }

        let mut cipher: CipherState<C> = CipherState::new(key.as_slice(), pre_index);
        let ciphertext = cipher.encrypt_vec(data.as_slice());

        crypto_state.pre_index_out = pre_index + 1;
        // pre_bytes_accounted is symmetric on both sides for limit
        // enforcement; the responder uses it for `max_pre_bytes`.
        // The initiator tracks its own outbound aggregate too so the
        // send path can abandon the session once the limit is hit.
        crypto_state.pre_bytes_accounted = crypto_state
            .pre_bytes_accounted
            .saturating_add(ciphertext.len() as u64);
        storage.save_state(remote_id, crypto_state.session_id, crypto_state);

        Some((ciphertext, pre_index))
    }

    /// Decrypt a pre-completion (handshake-extras) payload on the
    /// responder side.
    ///
    /// `pre_index` is the value carried by the inbound
    /// `HandshakeExtraPayload`; the responder uses it as the AEAD
    /// nonce. Returns `None` on:
    ///
    /// - missing `pre_cipher_in` (msg 1 was never processed for this
    ///   session — caller buffers in the orphan store),
    /// - `pre_index >= max_pre_messages` (out-of-range),
    /// - duplicate `pre_index` already in `pre_index_in_seen`,
    /// - AEAD authentication failure.
    ///
    /// On success, the bitmap and accounting fields on
    /// `CryptoState` are updated and persisted before returning.
    pub fn decrypt_noise_kk_handshake_extra<D, C, H, P>(
        qaul_state: &crate::QaulState,
        ciphertext: Vec<u8>,
        pre_index: u64,
        storage: CryptoAccount,
        mut crypto_state: CryptoState,
        remote_id: PeerId,
    ) -> Option<Vec<u8>>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let key = match crypto_state.pre_cipher_in.as_ref() {
            Some(k) => k.clone(),
            None => {
                log::trace!(
                    "decrypt_noise_kk_handshake_extra: pre_cipher_in not set for session {} — orphan",
                    crypto_state.session_id
                );
                return None;
            }
        };

        // Range bound — keeps `pre_index_in_seen` from growing
        // without bound when an attacker stamps a pathological index.
        let max_pre_messages = {
            let cfg = crate::storage::configuration::Configuration::get(qaul_state);
            cfg.handshake_extras.max_pre_messages
        };
        if pre_index >= max_pre_messages as u64 {
            log::warn!(
                "decrypt_noise_kk_handshake_extra: pre_index {} >= max_pre_messages {} for session {}",
                pre_index,
                max_pre_messages,
                crypto_state.session_id
            );
            return None;
        }

        // Duplicate check via the seen-bitmap. A duplicate pre_index
        // would otherwise reuse an AEAD nonce on decrypt — the
        // CipherState is fresh per call so the actual decryption
        // would still authenticate, but we drop deliberately to
        // satisfy the "ordering rules: duplicates dropped" spec.
        if bitmap_test(&crypto_state.pre_index_in_seen, pre_index) {
            log::trace!(
                "decrypt_noise_kk_handshake_extra: dropping duplicate pre_index {} for session {}",
                pre_index,
                crypto_state.session_id
            );
            return None;
        }

        let mut cipher: CipherState<C> = CipherState::new(key.as_slice(), pre_index);
        let plaintext = match cipher.decrypt_vec(ciphertext.as_slice()) {
            Ok(p) => p,
            Err(_) => {
                log::error!(
                    "decrypt_noise_kk_handshake_extra: AEAD authentication failed for session {} pre_index {}",
                    crypto_state.session_id,
                    pre_index
                );
                return None;
            }
        };

        // Bookkeeping. Bitmap set, highest tracking, byte accounting.
        bitmap_set(&mut crypto_state.pre_index_in_seen, pre_index);
        if pre_index > crypto_state.pre_index_in_highest {
            crypto_state.pre_index_in_highest = pre_index;
        }
        crypto_state.pre_bytes_accounted = crypto_state
            .pre_bytes_accounted
            .saturating_add(ciphertext.len() as u64);
        storage.save_state(remote_id, crypto_state.session_id, crypto_state);

        Some(plaintext)
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
            // Pre-completion (handshake-extras) fields default to
            // empty / zero. They get populated only after KK msg 1
            // is written / read; see persist-cipher-snapshot work in
            // encrypt_noise_kk_handshake_1 / decrypt_noise_kk_handshake_1.
            pre_cipher_out: None,
            pre_index_out: 0,
            pre_cipher_in: None,
            pre_index_in_highest: 0,
            pre_index_in_seen: Vec::new(),
            pre_bytes_accounted: 0,
        };

        state
    }
}

// ---------------------------------------------------------------
// Pre-completion (handshake-extras) seen-bitmap helpers.
//
// `CryptoState::pre_index_in_seen` is a packed bitmap of accepted
// `pre_index` values (bit `i` set means we already decrypted the
// extra at index `i`). Length grows on demand and is bounded at the
// caller by `HandshakeExtras::max_pre_messages`, so the byte index
// arithmetic is checked range there rather than here.
// ---------------------------------------------------------------

/// Return whether bit `idx` is set in the bitmap.
fn bitmap_test(bitmap: &[u8], idx: u64) -> bool {
    let byte = (idx / 8) as usize;
    if byte >= bitmap.len() {
        return false;
    }
    let mask = 1u8 << (idx % 8) as u8;
    bitmap[byte] & mask != 0
}

/// Set bit `idx` in the bitmap, growing it with zero bytes as needed.
fn bitmap_set(bitmap: &mut Vec<u8>, idx: u64) {
    let byte = (idx / 8) as usize;
    while bitmap.len() <= byte {
        bitmap.push(0);
    }
    bitmap[byte] |= 1u8 << (idx % 8) as u8;
}
