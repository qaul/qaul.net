// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # BLE Transport Encryption Module
//!
//! This module provides Noise protocol transport encryption for BLE communication.
//! It uses `Noise_KK_X25519_ChaChaPoly_SHA256` pattern, similar to the existing
//! crypto module used for messaging.
//!
//! ## Instance-based vs Global State
//!
//! This module supports both instance-based and global state access:
//! - `BleCryptoModule` is the instance-based struct that can be stored in `Libqaul`
//! - `BleCrypto` provides static methods for backward compatibility with global state

use libp2p::identity::PublicKey;
use libp2p::PeerId;
use noise_protocol::{CipherState, HandshakeState, U8Array, DH};
use noise_rust_crypto::{ChaCha20Poly1305, Sha256, X25519};
use rand::Rng;
use state::InitCell;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::proto_net;
use crate::node::user_accounts::UserAccounts;
use crate::router::users::Users;
use crate::services::crypto::Crypto25519;

/// Global state for BLE crypto sessions (for backward compatibility)
static BLE_CRYPTO: InitCell<RwLock<BleCryptoModule>> = InitCell::new();

/// BLE Crypto Module - Instance-based session manager
///
/// This struct holds all BLE encryption session state for a single libqaul instance.
/// It can be stored in the `Libqaul` struct or accessed via the global `BleCrypto` wrapper.
pub struct BleCryptoModule {
    /// Map from small_id to crypto session state
    sessions: BTreeMap<Vec<u8>, BleCryptoState>,
}

/// State of a BLE crypto session
pub struct BleCryptoState {
    /// Session identifier (random 4 byte number)
    pub session_id: u32,
    /// Current state of the session
    pub state: BleSessionState,
    /// Are we the initiator of the handshake?
    #[allow(dead_code)]
    pub initiator: bool,
    /// Local static key (X25519 private key in montgomery form)
    pub s: Vec<u8>,
    /// Remote static key (X25519 public key in montgomery form)
    pub rs: Vec<u8>,
    /// Local ephemeral key
    pub e: Vec<u8>,
    /// Remote ephemeral key (received during handshake)
    pub re: Option<Vec<u8>>,
    /// Cipher key for outgoing messages
    pub cipher_out: Option<Vec<u8>>,
    /// Cipher key for incoming messages
    pub cipher_in: Option<Vec<u8>>,
    /// Nonce counter for outgoing messages
    pub nonce_out: u64,
    /// Highest nonce seen for incoming messages (for out-of-order handling)
    pub nonce_in_highest: u64,
}

/// State of a BLE encryption session
#[derive(Clone, Debug, PartialEq)]
pub enum BleSessionState {
    /// Sent handshake message 1, awaiting message 2
    HandshakePending,
    /// Transport encryption is active
    Established,
}

impl BleCryptoModule {
    /// Create a new BleCryptoModule instance
    pub fn new() -> Self {
        Self {
            sessions: BTreeMap::new(),
        }
    }

    /// Check if a session is established for the given small_id
    pub fn is_session_established(&self, small_id: &[u8]) -> bool {
        if let Some(session) = self.sessions.get(small_id) {
            return session.state == BleSessionState::Established;
        }
        false
    }

    /// Get session ID for a given small_id (if session exists)
    #[allow(dead_code)]
    pub fn get_session_id(&self, small_id: &[u8]) -> Option<u32> {
        self.sessions.get(small_id).map(|s| s.session_id)
    }

    /// Initiate a handshake with a remote node
    ///
    /// This should be called after node identification is received.
    /// Returns the first handshake message to send to the peer.
    pub fn initiate_handshake(
        &mut self,
        small_id: &[u8],
        remote_id: PeerId,
    ) -> Option<proto_net::NoiseHandshake> {
        log::info!("BLE crypto: initiating handshake with {:?}", small_id);

        // Get the default user account
        let user_account = match UserAccounts::get_default_user() {
            Some(account) => account,
            None => {
                log::error!("BLE crypto: no user account available for handshake");
                return None;
            }
        };

        // Get the remote public key
        let remote_key = match Users::get_pub_key(&remote_id) {
            Some(key) => key,
            None => {
                log::error!("BLE crypto: no public key found for remote node");
                return None;
            }
        };

        // Create crypto state
        let state = Self::create_crypto_state::<X25519>(true, &user_account.keys, remote_key)?;
        let session_id = state.session_id;

        // Create handshake
        let pattern = noise_protocol::patterns::noise_kk();
        let prologue: Vec<u8> = Vec::new();
        let e = <X25519 as DH>::Key::from_slice(&state.e);

        let mut handshake: HandshakeState<X25519, ChaCha20Poly1305, Sha256> =
            HandshakeState::new(
                pattern,
                true,
                prologue.as_slice(),
                Some(U8Array::from_slice(&state.s)),
                Some(e),
                Some(U8Array::from_slice(&state.rs)),
                None,
            );

        // Create handshake message 1 (empty payload for BLE to save bandwidth)
        let payload = match handshake.write_message_vec(&[]) {
            Ok(output) => output,
            Err(e) => {
                log::error!("BLE crypto: failed to create handshake message 1: {}", e);
                return None;
            }
        };

        // Save session state
        self.sessions.insert(small_id.to_vec(), state);

        log::info!("BLE crypto: handshake 1 created, session_id: {}", session_id);

        Some(proto_net::NoiseHandshake {
            session_id,
            message_number: 1,
            payload,
        })
    }

    /// Process incoming handshake message 1 and generate response
    ///
    /// This is called when we receive the first handshake message from a peer.
    /// Returns the second handshake message to send back.
    pub fn process_handshake_1(
        &mut self,
        small_id: &[u8],
        handshake: proto_net::NoiseHandshake,
        remote_id: PeerId,
    ) -> Result<proto_net::NoiseHandshake, String> {
        log::info!("BLE crypto: processing handshake 1 from {:?}", small_id);

        // Get the default user account
        let user_account = match UserAccounts::get_default_user() {
            Some(account) => account,
            None => {
                return Err("No user account available".to_string());
            }
        };

        // Get the remote public key
        let remote_key = match Users::get_pub_key(&remote_id) {
            Some(key) => key,
            None => {
                return Err("No public key found for remote node".to_string());
            }
        };

        // Create crypto state (we are the responder)
        let mut state = Self::create_crypto_state::<X25519>(false, &user_account.keys, remote_key)
            .ok_or("Failed to create crypto state")?;

        // Override session_id with the one from the initiator
        state.session_id = handshake.session_id;

        // Create handshake state for responder
        let pattern = noise_protocol::patterns::noise_kk();
        let prologue: Vec<u8> = Vec::new();
        let e = <X25519 as DH>::Key::from_slice(&state.e);

        let mut hs: HandshakeState<X25519, ChaCha20Poly1305, Sha256> = HandshakeState::new(
            pattern,
            false,
            prologue.as_slice(),
            Some(U8Array::from_slice(&state.s)),
            Some(e),
            Some(U8Array::from_slice(&state.rs)),
            None,
        );

        // Read handshake message 1
        match hs.read_message_vec(&handshake.payload) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("Failed to read handshake message 1: {}", e));
            }
        }

        // Get remote ephemeral
        match hs.get_re() {
            Some(re) => {
                state.re = Some(Vec::from(re.as_slice()));
            }
            None => {
                return Err("Failed to get remote ephemeral".to_string());
            }
        }

        // Write handshake message 2
        let payload = match hs.write_message_vec(&[]) {
            Ok(output) => output,
            Err(e) => {
                return Err(format!("Failed to write handshake message 2: {}", e));
            }
        };

        // Get cipher keys
        let (cipher_key_in, cipher_key_out) = hs.get_ciphers();
        let (key_out, _) = cipher_key_out.extract();
        let (key_in, _) = cipher_key_in.extract();

        // Update state to established
        state.state = BleSessionState::Established;
        state.cipher_out = Some(key_out.as_slice().to_vec());
        state.cipher_in = Some(key_in.as_slice().to_vec());
        state.nonce_out = 0;
        state.nonce_in_highest = 0;

        let session_id = state.session_id;

        // Save session state
        self.sessions.insert(small_id.to_vec(), state);

        log::info!(
            "BLE crypto: handshake 2 created, session established, session_id: {}",
            session_id
        );

        Ok(proto_net::NoiseHandshake {
            session_id,
            message_number: 2,
            payload,
        })
    }

    /// Process incoming handshake message 2 to complete the session
    ///
    /// This is called when we receive the second handshake message (response).
    pub fn process_handshake_2(
        &mut self,
        small_id: &[u8],
        handshake: proto_net::NoiseHandshake,
    ) -> Result<(), String> {
        log::info!("BLE crypto: processing handshake 2 from {:?}", small_id);

        let state = match self.sessions.get_mut(small_id) {
            Some(s) => s,
            None => {
                return Err("No session found for handshake 2".to_string());
            }
        };

        // Verify session_id matches
        if state.session_id != handshake.session_id {
            return Err(format!(
                "Session ID mismatch: expected {}, got {}",
                state.session_id, handshake.session_id
            ));
        }

        // Verify we're in the correct state
        if state.state != BleSessionState::HandshakePending {
            return Err("Session not in HandshakePending state".to_string());
        }

        // Rebuild handshake state for initiator
        let pattern = noise_protocol::patterns::noise_kk();
        let prologue: Vec<u8> = Vec::new();
        let e = <X25519 as DH>::Key::from_slice(&state.e);

        let mut hs: HandshakeState<X25519, ChaCha20Poly1305, Sha256> = HandshakeState::new(
            pattern,
            true,
            prologue.as_slice(),
            Some(U8Array::from_slice(&state.s)),
            Some(e),
            Some(U8Array::from_slice(&state.rs)),
            None,
        );

        // Set message index to 1 (we've already done message 0)
        hs.set_index(1);

        // Read handshake message 2
        match hs.read_message_vec(&handshake.payload) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("Failed to read handshake message 2: {}", e));
            }
        }

        // Get remote ephemeral
        match hs.get_re() {
            Some(re) => {
                state.re = Some(Vec::from(re.as_slice()));
            }
            None => {
                return Err("Failed to get remote ephemeral".to_string());
            }
        }

        // Get cipher keys (note: order is reversed for initiator)
        let (cipher_key_out, cipher_key_in) = hs.get_ciphers();
        let (key_out, _) = cipher_key_out.extract();
        let (key_in, _) = cipher_key_in.extract();

        // Update state to established
        state.state = BleSessionState::Established;
        state.cipher_out = Some(key_out.as_slice().to_vec());
        state.cipher_in = Some(key_in.as_slice().to_vec());
        state.nonce_out = 0;
        state.nonce_in_highest = 0;

        log::info!(
            "BLE crypto: session established (initiator), session_id: {}",
            state.session_id
        );

        Ok(())
    }

    /// Encrypt a message for transport
    ///
    /// Returns the encrypted transport message ready for sending.
    pub fn encrypt(
        &mut self,
        small_id: &[u8],
        plaintext: Vec<u8>,
    ) -> Result<proto_net::EncryptedBleTransport, String> {
        let state = match self.sessions.get_mut(small_id) {
            Some(s) => s,
            None => {
                return Err("No session found for encryption".to_string());
            }
        };

        // Verify session is established
        if state.state != BleSessionState::Established {
            return Err("Session not established".to_string());
        }

        let cipher_key = state
            .cipher_out
            .as_ref()
            .ok_or("No cipher key available")?;

        let nonce = state.nonce_out;

        // Create cipher and encrypt
        let mut cipher: CipherState<ChaCha20Poly1305> =
            CipherState::new(cipher_key.as_slice(), nonce);

        let ciphertext = cipher.encrypt_vec(plaintext.as_slice());

        // Increment nonce
        state.nonce_out += 1;

        log::trace!(
            "BLE crypto: encrypted message, session_id: {}, nonce: {}",
            state.session_id,
            nonce
        );

        Ok(proto_net::EncryptedBleTransport {
            session_id: state.session_id,
            nonce,
            ciphertext,
        })
    }

    /// Decrypt an incoming encrypted message
    ///
    /// Returns the decrypted plaintext.
    pub fn decrypt(
        &mut self,
        small_id: &[u8],
        encrypted: proto_net::EncryptedBleTransport,
    ) -> Result<Vec<u8>, String> {
        let state = match self.sessions.get_mut(small_id) {
            Some(s) => s,
            None => {
                return Err("No session found for decryption".to_string());
            }
        };

        // Verify session is established
        if state.state != BleSessionState::Established {
            return Err("Session not established".to_string());
        }

        // Verify session_id matches
        if state.session_id != encrypted.session_id {
            return Err(format!(
                "Session ID mismatch: expected {}, got {}",
                state.session_id, encrypted.session_id
            ));
        }

        let cipher_key = state
            .cipher_in
            .as_ref()
            .ok_or("No cipher key available")?;

        // Create cipher with the received nonce
        let mut cipher: CipherState<ChaCha20Poly1305> =
            CipherState::new(cipher_key.as_slice(), encrypted.nonce);

        // Decrypt
        let plaintext = cipher
            .decrypt_vec(encrypted.ciphertext.as_slice())
            .map_err(|_| "Decryption failed")?;

        // Update highest nonce seen
        if encrypted.nonce > state.nonce_in_highest {
            state.nonce_in_highest = encrypted.nonce;
        }

        log::trace!(
            "BLE crypto: decrypted message, session_id: {}, nonce: {}",
            state.session_id,
            encrypted.nonce
        );

        Ok(plaintext)
    }

    /// Clean up session when a node becomes unavailable
    pub fn on_node_unavailable(&mut self, small_id: &[u8]) {
        if self.sessions.remove(small_id).is_some() {
            log::info!("BLE crypto: session removed for unavailable node {:?}", small_id);
        }
    }

    /// Get the number of active sessions
    #[allow(dead_code)]
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Create initial crypto state for handshake
    fn create_crypto_state<D>(
        initiator: bool,
        keys: &libp2p::identity::Keypair,
        remote_key: PublicKey,
    ) -> Option<BleCryptoState>
    where
        D: DH,
    {
        // Generate random session ID
        let mut rng = rand::rng();
        let session_id: u32 = rng.random();

        // Convert our ED25519 key to X25519 (montgomery form)
        let private_key = Crypto25519::private_key_to_montgomery(keys.clone())?;

        // Convert remote ED25519 public key to X25519
        let remote_public_key = Crypto25519::public_key_to_montgomery(remote_key)?;

        // Generate ephemeral key
        let e = D::genkey();

        let state = BleCryptoState {
            session_id,
            state: BleSessionState::HandshakePending,
            initiator,
            s: private_key,
            rs: Vec::from(remote_public_key.to_bytes()),
            e: Vec::from(e.as_slice()),
            re: None,
            cipher_out: None,
            cipher_in: None,
            nonce_out: 0,
            nonce_in_highest: 0,
        };

        Some(state)
    }
}

impl Default for BleCryptoModule {
    fn default() -> Self {
        Self::new()
    }
}

/// BleCrypto - Global state wrapper for backward compatibility
///
/// This struct provides static methods that access the global BLE_CRYPTO state.
/// For new code, prefer using `BleCryptoModule` directly.
pub struct BleCrypto;

impl BleCrypto {
    /// Initialize the BLE crypto module (global state)
    pub fn init() {
        let crypto = BleCryptoModule::new();
        BLE_CRYPTO.set(RwLock::new(crypto));
        log::info!("BLE crypto module initialized");
    }

    /// Check if a session is established for the given small_id
    pub fn is_session_established(small_id: &[u8]) -> bool {
        let crypto = BLE_CRYPTO.get().read().unwrap();
        crypto.is_session_established(small_id)
    }

    /// Get session ID for a given small_id (if session exists)
    #[allow(dead_code)]
    pub fn get_session_id(small_id: &[u8]) -> Option<u32> {
        let crypto = BLE_CRYPTO.get().read().unwrap();
        crypto.get_session_id(small_id)
    }

    /// Initiate a handshake with a remote node
    pub fn initiate_handshake(small_id: &[u8], remote_id: PeerId) -> Option<proto_net::NoiseHandshake> {
        let mut crypto = BLE_CRYPTO.get().write().unwrap();
        crypto.initiate_handshake(small_id, remote_id)
    }

    /// Process incoming handshake message 1 and generate response
    pub fn process_handshake_1(
        small_id: &[u8],
        handshake: proto_net::NoiseHandshake,
        remote_id: PeerId,
    ) -> Result<proto_net::NoiseHandshake, String> {
        let mut crypto = BLE_CRYPTO.get().write().unwrap();
        crypto.process_handshake_1(small_id, handshake, remote_id)
    }

    /// Process incoming handshake message 2 to complete the session
    pub fn process_handshake_2(
        small_id: &[u8],
        handshake: proto_net::NoiseHandshake,
    ) -> Result<(), String> {
        let mut crypto = BLE_CRYPTO.get().write().unwrap();
        crypto.process_handshake_2(small_id, handshake)
    }

    /// Encrypt a message for transport
    pub fn encrypt(
        small_id: &[u8],
        plaintext: Vec<u8>,
    ) -> Result<proto_net::EncryptedBleTransport, String> {
        let mut crypto = BLE_CRYPTO.get().write().unwrap();
        crypto.encrypt(small_id, plaintext)
    }

    /// Decrypt an incoming encrypted message
    pub fn decrypt(
        small_id: &[u8],
        encrypted: proto_net::EncryptedBleTransport,
    ) -> Result<Vec<u8>, String> {
        let mut crypto = BLE_CRYPTO.get().write().unwrap();
        crypto.decrypt(small_id, encrypted)
    }

    /// Clean up session when a node becomes unavailable
    pub fn on_node_unavailable(small_id: &[u8]) {
        let mut crypto = BLE_CRYPTO.get().write().unwrap();
        crypto.on_node_unavailable(small_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_equality() {
        assert_eq!(BleSessionState::HandshakePending, BleSessionState::HandshakePending);
        assert_eq!(BleSessionState::Established, BleSessionState::Established);
        assert_ne!(BleSessionState::HandshakePending, BleSessionState::Established);
    }

    #[test]
    fn test_new_module_has_no_sessions() {
        let module = BleCryptoModule::new();
        assert_eq!(module.session_count(), 0);
        assert!(!module.is_session_established(&[1, 2, 3, 4]));
    }
}
