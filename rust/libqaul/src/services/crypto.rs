// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # End to End Encryption from user to user
//! 
//! This module provides the end to end encryption functionality
//! for the messaging service.
//! 
//! The cryptography is based on the Noise protocol.
//! qaul uses the `Noise_KK_25519_ChaChaPoly_SHA256` pattern.

use libp2p::PeerId;
use libp2p::identity::{Keypair, PublicKey};
use prost::Message;
use sled_extensions::{
    DbExt,
    bincode::Tree,
};
use serde::{Serialize, Deserialize};
use ed25519_dalek;
use x25519_dalek;
use curve25519_dalek;
use sha2::{Digest, Sha512};
use noise_protocol::{
    HandshakeState,
    CipherState,
    DH,
    Hash,
    Cipher,
    U8Array,
};
use noise_rust_crypto::{
    X25519,
    ChaCha20Poly1305,
    Sha256,
};

use super::messaging::{
    Messaging,
    proto,
};
use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::storage::database::DataBase;
use crate::router::users::Users;

/// The State Data of the Noise Protocol
#[derive(Clone, Serialize, Deserialize)]
pub struct CryptoState {
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
    /// A full roundtrip has been done.
    /// We have all cryptographic data.
    /// But we are still missing the confirmation of the other party.
    Full,
    /// The full roundtrip has been verified.
    /// From now on, no half encrypted messages are allowed anymore.
    Verified,
}

/// Crypto Module State
/// 
/// This contains all references to the DB tree
pub struct Crypto {}

impl Crypto {
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
    /// The function returns the encrypted data and the nonce.
    pub fn encrypt(data: Vec<u8>, user_account: UserAccount, remote_id: PeerId) -> (Option<Vec<u8>>, u64)
    {
        Self::encrypt_noise_kk::<X25519, ChaCha20Poly1305, Sha256, &[u8]>(data, user_account, remote_id)
    }

    /// Encrypt a Message
    /// 
    /// Encrypt an outgoing message.
    fn encrypt_noise_kk<D, C, H, P>(data: Vec<u8>, user_account: UserAccount, remote_id: PeerId) -> (Option<Vec<u8>>, u64)
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let mut state: CryptoState;
        let mut message: Option<Vec<u8>> = None;
        let mut nonce: u64 = 0;
        
        // get or build noise state
        if let Some(saved_state) = Self::get_handshake_state(user_account.clone(), remote_id) {
            // noise state exists
            state = saved_state;

            // if incoming crypto state is half done, send the hand shake response
            match state.state {
                CryptoProcessState::HalfIncoming => {
                    // we need to send the second handshake message
                    log::info!("Send Handshake Response");

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
                        },
                    }
        
                    // get ciphers & put them to state
                    let (cipher_key_in, cipher_key_out) = handshake.get_ciphers();
                    let (key_out, _nonce_out) = cipher_key_out.extract();
                    let (key_in, _nonce_in) = cipher_key_in.extract();

                    state.state = CryptoProcessState::Verified;
                    state.cipher_in = Some(key_in.as_slice().to_vec());
                    state.highest_index_nonce_in = 0;
                    state.cipher_out = Some(key_out.as_slice().to_vec());
                    state.index_nonce_out = 0;

                    // save crypto state to data base
                    Self::save_handshake_state(user_account, remote_id, state);
                },
                CryptoProcessState::Verified => {
                    log::info!("Encrypt with full encryption");

                    // the handshake has been done, we can encrypt messages
                    nonce = state.index_nonce_out;

                    // create cipher
                    let mut cipher: CipherState<C> = CipherState::new(state.clone().cipher_out.unwrap().as_slice(), nonce);

                    // encrypt message
                    message = Some(cipher.encrypt_vec(data.as_slice()));

                    // save new nonce to state
                    state.index_nonce_out = nonce +1;
                    Self::save_handshake_state(user_account, remote_id, state);
                },
                _ => {
                    log::error!("unexpected handshake state");
                },
            }
        }
        else {
            // no saved state available: generate crypto state
            log::info!("Initiate Crypto Handshake");

            // get receivers public key
            let remote_key: PublicKey;
            match Users::get_pub_key(&remote_id) {
                Some(key) => remote_key = key,
                None => {
                    log::error!("No key found for user {:?}", remote_id);
                    return (None, 0);
                },
            }
            
            // create crypto state
            state = Self::create_crypto_state::<D>(true, user_account.clone(), remote_key);

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
                    return (message, nonce);
                },
            }

            // save state to data base
            state.e = e.as_slice().to_vec();
            Self::save_handshake_state(user_account, remote_id, state);
        }

        (message, nonce)
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
    /// The function returns the decrypted data.
    pub fn decrypt(data: Vec<u8>, nonce: u64, user_account_id: PeerId, remote_id: PeerId) -> Option<Vec<u8>> {
        Self::decrypt_noise_kk::<X25519, ChaCha20Poly1305, Sha256, &[u8]>(data, nonce, user_account_id, remote_id)
    }

    /// Decrypt a Message
    /// 
    /// Decrypt an incoming message.
    fn decrypt_noise_kk<D, C, H, P>(data: Vec<u8>, nonce: u64, user_account_id: PeerId, remote_id: PeerId) -> Option<Vec<u8>>
    where
        D: DH,
        C: Cipher,
        H: Hash,
        P: AsRef<[u8]>,
    {
        let mut state: CryptoState;
        let mut message: Option<Vec<u8>> = None;

        // get user account
        let user_account: UserAccount;
        if let Some(result) = UserAccounts::get_by_id(user_account_id){
            user_account = result;
        } else {
            log::error!("user account not found");
            return None;
        }

        // check if there is already an entry
        if let Some(saved_state) = Self::get_handshake_state(user_account.clone(), remote_id) {
            state = saved_state;

            // if incoming crypto state is half done, send the hand shake response
            match state.state {
                CryptoProcessState::HalfOutgoing => {
                    // this should be the responders hand shake message.
                    log::info!("Receiving handshake confirmation");

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
                        },
                    }

                    // get remote ephemeral
                    match handshake.get_re() {
                        Some(re) => {
                            // put it to state
                            state.re = Some(Vec::from(re.as_slice()));
                        },
                        None => {
                            // TODO: remove state
                            return None;
                        },
                    }

                    // get ciphers & put them to state
                    let (cipher_key_out, cipher_key_in) = handshake.get_ciphers();
                    let (key_out, nonce_out) = cipher_key_out.extract();
                    let (key_in, nonce_in) = cipher_key_in.extract();

                    log::info!("handshake initiation finished: noce out: {}, in: {}", nonce_out, nonce_in);

                    state.state = CryptoProcessState::Verified;
                    state.cipher_in = Some(key_in.as_slice().to_vec());
                    state.highest_index_nonce_in = 0;
                    state.cipher_out = Some(key_out.as_slice().to_vec());
                    state.index_nonce_out = 0;

                    // save state to data base
                    Self::save_handshake_state(user_account.clone(), remote_id, state);
                },
                CryptoProcessState::Verified => {
                    // we had a successful handshake and are in transport state
                    log::info!("Decrypting with full encryption");

                    // create cipher
                    let mut cipher: CipherState<C> = CipherState::new(state.cipher_in.clone().unwrap().as_slice(), nonce);

                    // decrypt message
                    match cipher.decrypt_vec(data.as_slice()) {
                        Ok(decrypted) => {
                            message = Some(decrypted);

                            state.highest_index_nonce_in = nonce;
                            Self::save_handshake_state(user_account, remote_id, state);
                        },
                        Err(_) => {
                            log::error!("decryption error");
                            return None;
                        },
                    }
                },
                _ => {
                    log::error!("unexpected handshake state");
                },
            }
        }
        else {
            log::info!("Incoming handshake");

            // no saved state available: generate crypto state

            // get receivers public key
            let remote_key: PublicKey;
            match Users::get_pub_key(&remote_id) {
                Some(key) => remote_key = key,
                None => {
                    log::error!("No key found for user {:?}", remote_id);
                    return None;
                },
            }

            // create initial crypto state
            state = Self::create_crypto_state::<D>(false, user_account.clone(), remote_key);

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
                Ok(encrypted) => message = Some(encrypted),
                Err(e) => {
                    log::error!("{}", e);
                    return None;
                },
            }

            // get remote ephemeral
            match handshake.get_re() {
                Some(re) => {
                    // put it to state
                    state.re = Some(Vec::from(re.as_slice()));
                },
                None => {
                    // TODO: remove state
                    return None;
                },
            }

            // save state to data base
            Self::save_handshake_state(user_account.clone(), remote_id, state);

            // send response to handshake message
            Self::send_crypto_service_message(&user_account, remote_id);
        }

        message
    }

    /// Create CryptoState during handshake phase
    /// for outgoing or incoming 
    fn create_crypto_state<D>(initiator: bool, user_account: UserAccount, remote_key: PublicKey) -> CryptoState
    where 
        D: DH,
    {
        // create private key
        let private_key = Self::private_key_to_montgomery(user_account.keys).unwrap();

        // create public key
        let remote_public_key = Self::public_key_to_montgomery(remote_key).unwrap();

        // create new ephemeral key
        let e = D::genkey();

        // create CryptoState structure
        let rs_vec: Vec<u8> = Vec::from(remote_public_key.to_bytes());
        let e_vec: Vec<u8> = Vec::from(e.as_slice());
        let process_state: CryptoProcessState;
        if initiator {
            process_state = CryptoProcessState::HalfOutgoing;
        }
        else {
            process_state = CryptoProcessState::HalfIncoming;
        }

        let state = CryptoState {
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
        };

        state
    }

    /// Convert ed25519 private key to montgomery form
    /// 
    /// libqaul's Keypair is an `ed25519::KeyPair`
    /// To get to the montgomery form, the following actions are done:
    /// 
    /// 1) Get the `SecretKey` from the KeyPair structure.
    /// 2) Convert the `EdwardsPoint` to a `MontgomeryPoint` by hashing it with sha512.
    /// 3) This gives us the bytes of the curve25519_dalek secret key in the Montgomery form to be used with for the X25519 Diffie-Hellman.
    /// 
    /// ## References
    /// 
    /// * libqaul's `Keypair` = [`libp2p::identity::ed25519::Keypair`](https://docs.rs/libp2p/latest/libp2p/identity/enum.Keypair.html)
    /// * `EdwardsPoint` = [`curve25519_dalek::edwards::EdwardsPoint`](https://doc.dalek.rs/curve25519_dalek/edwards/struct.EdwardsPoint.html)
    /// * `MontgomeryPoint` = [`curve25519_dalek::montgomery::MontgomeryPoint`](https://doc.dalek.rs/curve25519_dalek/montgomery/struct.MontgomeryPoint.html)
    /// * [`x25519_dalek::PublicKey`](https://doc.dalek.rs/x25519_dalek/struct.PublicKey.html)
    /// 
    pub fn private_key_to_montgomery(key: Keypair) -> Option<Vec<u8>> {
        // get ed25519 keypair
        if let Keypair::Ed25519(ed25519_keypair) = key {
            // get dalek keypair as bytes
            //
            // unfortunately the dalek keypair is private in the 
            // libp2p keypair structure, therefore we have to
            // make the detour via the bytes.
            let ed25519_dalek_keypair_bytes = ed25519_keypair.encode();

            // create dalek ed25519 keypair
            if let Ok(ed25519_dalek_keypair) = ed25519_dalek::Keypair::from_bytes(&ed25519_dalek_keypair_bytes) {
                // get dalek secret key as bytes
                let ed25519_dalek_secret_bytes = ed25519_dalek_keypair.secret.to_bytes();

                // transform into dalek curve25519 secret key as bytes
                let mut curve25519_dalek_secret: [u8; 32] = [0; 32];
                let hash = Sha512::digest(ed25519_dalek_secret_bytes.as_ref());
                curve25519_dalek_secret.copy_from_slice(&hash[..32]);
        
                return Some(curve25519_dalek_secret.to_vec());
            }
        }

        None
    }

    /// Convert ed25519 public key to montgomery form
    /// 
    /// Libqaul's PublicKey is a `CompressedEdwardsY` point from the `curve25519_dalek` library.
    /// To get to the montgomery form, the following actions are done:
    /// 
    /// 1) Decompress `CompressedEdwardsY` point, which returns an `EdwardsPoint`.
    /// 2) Convert the `EdwardsPoint` to a `MontgomeryPoint`
    /// 3) The bytes of the `MontgomeryPoint` are equal to the `x25519_dalek::PublicKey`
    /// 
    /// ## References
    /// 
    /// * libqaul's `PublicKey` = [`libp2p::identity::ed25519::PublicKey`](https://docs.rs/libp2p/latest/libp2p/identity/ed25519/struct.PublicKey.html)
    /// * `EdwardsPoint` = [`curve25519_dalek::edwards::EdwardsPoint`](https://doc.dalek.rs/curve25519_dalek/edwards/struct.EdwardsPoint.html)
    /// * `CompressedEdwardsY` = [`curve25519_dalek::edwards::CompressedEdwardsY`](https://doc.dalek.rs/curve25519_dalek/edwards/struct.CompressedEdwardsY.html)
    /// * `MontgomeryPoint` = [`curve25519_dalek::montgomery::MontgomeryPoint`](https://doc.dalek.rs/curve25519_dalek/montgomery/struct.MontgomeryPoint.html)
    /// * [`x25519_dalek::PublicKey`](https://doc.dalek.rs/x25519_dalek/struct.PublicKey.html)
    /// 
    pub fn public_key_to_montgomery(key: PublicKey) -> Option<x25519_dalek::PublicKey> {
        // get ed25519 structure
        if let PublicKey::Ed25519(ed25519_pub) = key {
            // convert to dalek public key in bytes form
            let dalek_pub_bytes = ed25519_pub.encode();

            // generate Montgomery form
            // x25519_dalek::PublicKey internal is private, we have to go via bytes
            let montgomery_bytes = curve25519_dalek::edwards::CompressedEdwardsY(dalek_pub_bytes)
                .decompress()
                .expect("An Ed25519 public key is a valid point by construction.")
                .to_montgomery()
                .0;

            return Some(x25519_dalek::PublicKey::from(montgomery_bytes));
        }

        None
    }

    /// Get User Crypto Handshake State
    fn get_handshake_state(user_account: UserAccount, remote_id: PeerId) -> Option<CryptoState> {
        // get data base of user account
        let db = DataBase::get_user_db(user_account.id);
        
        // get data base tree
        let state_tree: Tree<CryptoState> = db.open_bincode_tree("crypto_state").unwrap();

        // check if handshake state does exist
        match state_tree.get(remote_id.to_bytes()) {
            Ok(state) => return state,
            Err(e) => log::error!("{}", e),
        }

        None
    }

    /// Save Updated Crypto Handshake State
    fn save_handshake_state(user_account: UserAccount, remote_id: PeerId, crypto_state: CryptoState) {
        // get data base of user account
        let db = DataBase::get_user_db(user_account.id);
        
        // get data base tree
        let state_tree: Tree<CryptoState> = db.open_bincode_tree("crypto_state").unwrap();

        // save message in data base
        if let Err(e) = state_tree.insert(remote_id.to_bytes(), crypto_state) {
            log::error!("Error handshake to db: {}", e);
        }

        // flush trees to disk
        if let Err(e) = state_tree.flush() {
            log::error!("Error db flush: {}", e);
        }
    }

    /// Send crypto service message
    fn send_crypto_service_message(user_account: &UserAccount, receiver: PeerId) {
        // pack message
        let send_message = proto::Messaging{
            message: Some(proto::messaging::Message::CryptoService(
                proto::CryptoService {
                }
            )),
        };

        // encode chat message
        let mut message_buf = Vec::with_capacity(send_message.encoded_len());
        send_message.encode(&mut message_buf).expect("Vec<u8> provides capacity as needed");
        log::info!("message_buf len {}", message_buf.len());

        // send message via messaging
        match Messaging::pack_and_send_message(user_account, &receiver, &message_buf, None, false) {
            Ok(_) => {},
            Err(e) => log::error!("{}", e),
        }
    }
}
