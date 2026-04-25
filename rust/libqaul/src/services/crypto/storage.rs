// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Storage of the Crypto Sessions State
//!
//! Handling of the data base access for the crypto handshake and session state.

use libp2p::PeerId;
use sled;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::CryptoState;
use crate::services::messaging::proto;
use crate::storage::database::DataBase;

/// Group DB links for user account
#[derive(Clone)]
pub struct CryptoAccount {
    /// user crypto session state storage
    ///
    /// value: bincode of `CryptoState`
    pub state: sled::Tree,
    /// unprocessable out of order handshake
    /// state messages
    ///
    /// value: bincode of `proto::Encrypted`
    pub cache: sled::Tree,
}

impl CryptoAccount {
    /// create state db key for state
    ///
    /// The db key for a specific session is a Vec<u8> of:
    /// {remote_id}{session_id}
    fn create_state_key(remote_id: PeerId, session_id: u32) -> Vec<u8> {
        let mut remote_id_bytes = remote_id.to_bytes();
        let mut session_id_bytes = session_id.to_be_bytes().to_vec();
        remote_id_bytes.append(&mut session_id_bytes);
        remote_id_bytes
    }

    /// create state db search range
    fn create_state_key_range(remote_id: PeerId) -> (Vec<u8>, Vec<u8>) {
        let first_key = Self::create_state_key(remote_id, 0);
        let last_key = Self::create_state_key(remote_id, u32::MAX);

        (first_key, last_key)
    }

    /// Create cache storage key
    ///
    /// The db key for the cache messages is:
    /// {remote_id}{session_id}{nonce}
    fn create_cache_key(remote_id: PeerId, session_id: u32, nonce: u64) -> Vec<u8> {
        let mut nonce_bytes = nonce.to_be_bytes().to_vec();
        let mut session_key = Self::create_state_key(remote_id, session_id);
        session_key.append(&mut nonce_bytes);
        session_key
    }

    /// Create cache storage key range
    #[allow(dead_code)]
    fn create_cache_key_range(remote_id: PeerId, session_id: u32) -> (Vec<u8>, Vec<u8>) {
        let first_key = Self::create_cache_key(remote_id, session_id, 0);
        let last_key = Self::create_cache_key(remote_id, session_id, u64::MAX);

        (first_key, last_key)
    }

    /// get currently active CryptoState from db
    pub fn get_state(&self, remote_id: PeerId) -> Option<CryptoState> {
        // get key range
        let (first_key, last_key) = Self::create_state_key_range(remote_id);

        // create return value
        let mut state_option: Option<CryptoState> = None;

        // get results from data base
        let iterator = self.state.range(first_key..last_key);

        for result in iterator {
            match result {
                Ok((_key, crypto_state_bytes)) => {
                    let crypto_state: CryptoState =
                        match bincode::deserialize(&crypto_state_bytes) {
                            Ok(v) => v,
                            Err(e) => {
                                log::error!("Error deserializing crypto state: {}", e);
                                continue;
                            }
                        };
                    match crypto_state.state {
                        super::CryptoProcessState::HalfOutgoing => {
                            state_option = Some(crypto_state)
                        }
                        super::CryptoProcessState::HalfIncoming => return Some(crypto_state),
                        super::CryptoProcessState::Transport => return Some(crypto_state),
                    }
                }
                Err(e) => log::error!("{}", e),
            }
        }

        state_option
    }

    /// get a specific CryptoState by ID
    pub fn get_state_by_id(&self, remote_id: PeerId, session_id: u32) -> Option<CryptoState> {
        // create db key
        let key = Self::create_state_key(remote_id, session_id);

        // get result from data base
        match self.state.get(key) {
            Ok(Some(crypto_state_bytes)) => {
                let crypto_state: CryptoState = match bincode::deserialize(&crypto_state_bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Error deserializing crypto state by id: {}", e);
                        return None;
                    }
                };
                return Some(crypto_state);
            }
            Ok(None) => return None,
            Err(e) => log::error!("{}", e),
        }

        None
    }

    /// save CryptoState to db
    pub fn save_state(&self, remote_id: PeerId, session_id: u32, crypto_state: CryptoState) {
        // create key
        let key = Self::create_state_key(remote_id, session_id);

        // save message in data base
        let crypto_state_bytes = match bincode::serialize(&crypto_state) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Error serializing crypto state: {}", e);
                return;
            }
        };
        if let Err(e) = self.state.insert(key, crypto_state_bytes) {
            log::error!("Error handshake to db: {}", e);
        }

        // flush trees to disk
        if let Err(e) = self.state.flush() {
            log::error!("Error db flush: {}", e);
        }
    }

    /// save an incoming, out of order message to cache
    pub fn save_cache_message(
        &self,
        remote_id: PeerId,
        session_id: u32,
        nonce: u64,
        message: proto::Encrypted,
    ) {
        // create key
        let key = Self::create_cache_key(remote_id, session_id, nonce);

        // save message in data base
        let message_bytes = match bincode::serialize(&message) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Error serializing cache message: {}", e);
                return;
            }
        };
        if let Err(e) = self.cache.insert(key, message_bytes) {
            log::error!("Error handshake to db: {}", e);
        }

        // flush trees to disk
        if let Err(e) = self.cache.flush() {
            log::error!("Error db flush: {}", e);
        }
    }
}

/// Crypto Module Storage
///
/// This contains all references to the DB tree
pub struct CryptoStorage {
    /// data base tree references accessible
    /// by user account
    db_ref: BTreeMap<Vec<u8>, CryptoAccount>,
}

/// Instance-based crypto storage state.
/// Replaces the global CRYPTOSTORAGE static for multi-instance use.
pub struct CryptoStorageState {
    /// Crypto storage inner state.
    pub inner: RwLock<CryptoStorage>,
}

impl CryptoStorageState {
    /// Create a new empty CryptoStorageState.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(CryptoStorage {
                db_ref: BTreeMap::new(),
            }),
        }
    }

    /// Get DB refs for user account (instance method).
    /// Takes an explicit `sled::Db` instead of calling `DataBase::get_user_db()`.
    pub fn get_db_ref(&self, account_id: PeerId, db: &sled::Db) -> CryptoAccount {
        {
            let crypto_storage = self.inner.read().unwrap();
            if let Some(crypto_account_db) = crypto_storage.db_ref.get(&account_id.to_bytes()) {
                return CryptoAccount {
                    state: crypto_account_db.state.clone(),
                    cache: crypto_account_db.cache.clone(),
                };
            }
        }

        self.create_cryptoaccountdb(account_id, db)
    }

    /// Create crypto account db entry when it does not exist (instance method).
    fn create_cryptoaccountdb(&self, account_id: PeerId, db: &sled::Db) -> CryptoAccount {
        let state: sled::Tree = db.open_tree("crypto_state").unwrap();
        let cache: sled::Tree = db.open_tree("crypto_cache").unwrap();

        let crypto_account = CryptoAccount { state, cache };

        let mut crypto_storage = self.inner.write().unwrap();
        crypto_storage
            .db_ref
            .insert(account_id.to_bytes(), crypto_account.clone());

        crypto_account
    }
}

impl CryptoStorage {
    /// Initialize the crypto storage
    ///
    /// No-op: the state is now owned by `QaulState` and initialized there.
    pub fn init() {
        // State already exists in QaulState.services.crypto
    }

    /// get DB refs for user account
    pub fn get_db_ref(state: &crate::QaulState, account_id: PeerId) -> CryptoAccount {
        // check if user account data exists
        {
            // get crypto state
            let crypto_storage = state.services.crypto.inner.read().unwrap();

            // check if user account ID is in map
            if let Some(crypto_account_db) = crypto_storage.db_ref.get(&account_id.to_bytes()) {
                return CryptoAccount {
                    state: crypto_account_db.state.clone(),
                    cache: crypto_account_db.cache.clone(),
                };
            }
        }

        // create crypto account db entry if it does not exist
        let crypto_account = Self::create_groupaccountdb(state, account_id);

        // return crypto_account_db structure
        crypto_account.clone()
    }

    /// create group account db entry when it does not exist
    fn create_groupaccountdb(state: &crate::QaulState, account_id: PeerId) -> CryptoAccount {
        // get user data base
        let db = DataBase::get_user_db(state, account_id);

        // open trees
        let state_tree: sled::Tree = match db.open_tree("crypto_state") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("failed to open crypto_state tree: {}", e);
                return CryptoAccount {
                    state: db.open_tree("__fallback_crypto_state").expect("fallback tree"),
                    cache: db.open_tree("__fallback_crypto_cache").expect("fallback tree"),
                };
            }
        };
        let cache: sled::Tree = match db.open_tree("crypto_cache") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("failed to open crypto_cache tree: {}", e);
                return CryptoAccount {
                    state: state_tree,
                    cache: db.open_tree("__fallback_crypto_cache").expect("fallback tree"),
                };
            }
        };

        let crypto_account = CryptoAccount { state: state_tree, cache };

        // get crypto storage for writing
        let mut crypto_storage = state.services.crypto.inner.write().unwrap();

        // add user to state
        crypto_storage
            .db_ref
            .insert(account_id.to_bytes(), crypto_account.clone());

        // return structure
        crypto_account
    }
}
