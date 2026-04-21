// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Storage of the Crypto Sessions State
//!
//! Handling of the data base access for the crypto handshake and session state.

use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use sled;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::CryptoState;
use crate::services::messaging::proto;
use crate::storage::database::DataBase;

/// Per-peer Noise session rotation state.
///
/// There is at most one row per `remote_id` in the `rotation_meta`
/// sled tree. It tracks which session is currently primary and, if a
/// rotation is in progress or a grace window is open, the draining
/// session plus any in-flight initiation the local node owns.
///
/// Collision resolution rule (per Phase 1 design): if this node has
/// `pending_initiated_session_id = Some(mine)` and a
/// `RotateHandshakeFirst { new_session_id: incoming }` arrives, the
/// lower numeric session id wins. If `incoming < mine`, this node
/// abandons its own initiation (deletes the pending `CryptoState`
/// row) and responds to `incoming`. Otherwise this node ignores
/// `incoming` and waits for the remote to observe our lower
/// session id and reply with `RotateHandshakeSecond`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RotationMeta {
    /// The session_id that is currently used for outbound traffic.
    pub primary_session_id: u32,
    /// Set when this node has sent a `RotateHandshakeFirst` for which
    /// no `RotateHandshakeSecond` has been received yet. The
    /// corresponding `CryptoState` lives in the `state` tree in
    /// `HalfOutgoing`. `None` when no rotation is in flight locally.
    pub pending_initiated_session_id: Option<u32>,
    /// The previous session_id whose `cipher_in` is still accepted
    /// for incoming messages. `None` outside a grace window.
    pub draining_session_id: Option<u32>,
    /// Unix-ms deadline after which the draining session must be
    /// retired. `None` outside a grace window.
    pub draining_until: Option<u64>,
    /// Remaining inbound message budget on the draining session.
    /// Counts down in the decrypt path until it hits zero. `None`
    /// outside a grace window.
    pub draining_remaining_volume: Option<u64>,
}

impl RotationMeta {
    /// Construct a fresh meta for a peer that has a single primary
    /// session and no rotation activity.
    pub fn primary_only(primary_session_id: u32) -> Self {
        RotationMeta {
            primary_session_id,
            pending_initiated_session_id: None,
            draining_session_id: None,
            draining_until: None,
            draining_remaining_volume: None,
        }
    }
}

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
    /// per-peer session-rotation metadata
    ///
    /// key: `remote_id.to_bytes()`
    /// value: bincode of `RotationMeta`
    pub rotation_meta: sled::Tree,
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
                        bincode::deserialize(&crypto_state_bytes).unwrap();
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
                let crypto_state: CryptoState = bincode::deserialize(&crypto_state_bytes).unwrap();
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
        let crypto_state_bytes = bincode::serialize(&crypto_state).unwrap();
        if let Err(e) = self.state.insert(key, crypto_state_bytes) {
            log::error!("Error handshake to db: {}", e);
        }

        // flush trees to disk
        if let Err(e) = self.state.flush() {
            log::error!("Error db flush: {}", e);
        }
    }

    /// fetch the rotation meta for a peer, or `None` if no row exists.
    pub fn get_rotation_meta(&self, remote_id: PeerId) -> Option<RotationMeta> {
        match self.rotation_meta.get(remote_id.to_bytes()) {
            Ok(Some(bytes)) => match bincode::deserialize::<RotationMeta>(&bytes) {
                Ok(meta) => Some(meta),
                Err(e) => {
                    log::error!("rotation_meta deserialize: {}", e);
                    None
                }
            },
            Ok(None) => None,
            Err(e) => {
                log::error!("rotation_meta read: {}", e);
                None
            }
        }
    }

    /// Write (or replace) the rotation meta for a peer. Flushes the tree.
    pub fn save_rotation_meta(&self, remote_id: PeerId, meta: &RotationMeta) {
        let key = remote_id.to_bytes();
        let bytes = match bincode::serialize(meta) {
            Ok(b) => b,
            Err(e) => {
                log::error!("rotation_meta serialize: {}", e);
                return;
            }
        };
        if let Err(e) = self.rotation_meta.insert(key, bytes) {
            log::error!("rotation_meta insert: {}", e);
        }
        if let Err(e) = self.rotation_meta.flush() {
            log::error!("rotation_meta flush: {}", e);
        }
    }

    /// Remove the rotation meta row for a peer, e.g. after the
    /// draining session has expired and been zeroized.
    pub fn delete_rotation_meta(&self, remote_id: PeerId) {
        if let Err(e) = self.rotation_meta.remove(remote_id.to_bytes()) {
            log::error!("rotation_meta remove: {}", e);
        }
        if let Err(e) = self.rotation_meta.flush() {
            log::error!("rotation_meta flush: {}", e);
        }
    }

    /// Delete a `CryptoState` row by (remote_id, session_id).
    ///
    /// Used when a rotation is abandoned (collision with a lower
    /// incoming session_id) or when a draining session's grace
    /// window has expired and its ciphers have been zeroized.
    pub fn delete_state(&self, remote_id: PeerId, session_id: u32) {
        let key = Self::create_state_key(remote_id, session_id);
        if let Err(e) = self.state.remove(key) {
            log::error!("crypto_state remove: {}", e);
        }
        if let Err(e) = self.state.flush() {
            log::error!("crypto_state flush: {}", e);
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
        let message_bytes = bincode::serialize(&message).unwrap();
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
                    rotation_meta: crypto_account_db.rotation_meta.clone(),
                };
            }
        }

        self.create_cryptoaccountdb(account_id, db)
    }

    /// Create crypto account db entry when it does not exist (instance method).
    fn create_cryptoaccountdb(&self, account_id: PeerId, db: &sled::Db) -> CryptoAccount {
        let state_tree: sled::Tree = db.open_tree("crypto_state").unwrap();
        let cache: sled::Tree = db.open_tree("crypto_cache").unwrap();
        let rotation_meta: sled::Tree = db.open_tree("rotation_meta").unwrap();

        let crypto_account = CryptoAccount {
            state: state_tree,
            cache,
            rotation_meta,
        };

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
                    rotation_meta: crypto_account_db.rotation_meta.clone(),
                };
            }
        }

        // create crypto account db entry if it does not exist
        let crypto_account = Self::create_groupaccountdb(state, account_id);

        // return crypto_account_db structure
        crypto_account.clone()
    }

    /// Create an in-memory `CryptoAccount` for tests only.
    ///
    /// Opens three temporary sled databases (one each for the
    /// `crypto_state`, `crypto_cache`, and `rotation_meta` trees)
    /// and returns a `CryptoAccount` backed by them. Does not touch
    /// the global `CRYPTOSTORAGE` state, so tests can build isolated
    /// accounts without initialising the wider libqaul stack.
    #[cfg(test)]
    pub fn test_account() -> CryptoAccount {
        use sled::Config;
        let state_db = Config::new().temporary(true).open().unwrap();
        let cache_db = Config::new().temporary(true).open().unwrap();
        let meta_db = Config::new().temporary(true).open().unwrap();
        CryptoAccount {
            state: state_db.open_tree("crypto_state").unwrap(),
            cache: cache_db.open_tree("crypto_cache").unwrap(),
            rotation_meta: meta_db.open_tree("rotation_meta").unwrap(),
        }
    }

    /// create group account db entry when it does not exist
    fn create_groupaccountdb(state: &crate::QaulState, account_id: PeerId) -> CryptoAccount {
        // get user data base
        let db = DataBase::get_user_db(state, account_id);

        // open trees
        let state_tree: sled::Tree = db.open_tree("crypto_state").unwrap();
        let cache: sled::Tree = db.open_tree("crypto_cache").unwrap();
        let rotation_meta: sled::Tree = db.open_tree("rotation_meta").unwrap();

        let crypto_account = CryptoAccount {
            state: state_tree,
            cache,
            rotation_meta,
        };

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

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    fn fresh_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    // Save, load, overwrite, delete on the rotation_meta tree. No
    // global state — uses the in-memory `test_account` helper.
    #[test]
    fn rotation_meta_roundtrip() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        assert!(acct.get_rotation_meta(remote).is_none());

        let meta = RotationMeta::primary_only(0xAAAA_BBBB);
        acct.save_rotation_meta(remote, &meta);
        assert_eq!(acct.get_rotation_meta(remote), Some(meta.clone()));

        // overwrite
        let meta2 = RotationMeta {
            primary_session_id: 0xAAAA_BBBB,
            pending_initiated_session_id: Some(0x1111_2222),
            draining_session_id: None,
            draining_until: None,
            draining_remaining_volume: None,
        };
        acct.save_rotation_meta(remote, &meta2);
        assert_eq!(acct.get_rotation_meta(remote), Some(meta2));

        acct.delete_rotation_meta(remote);
        assert!(acct.get_rotation_meta(remote).is_none());
    }

    // Meta rows are keyed per-peer; one peer's row must not leak
    // into another peer's result.
    #[test]
    fn rotation_meta_keyed_per_peer() {
        let acct = CryptoStorage::test_account();
        let alice = fresh_peer();
        let bob = fresh_peer();
        acct.save_rotation_meta(alice, &RotationMeta::primary_only(1));
        acct.save_rotation_meta(bob, &RotationMeta::primary_only(2));
        assert_eq!(
            acct.get_rotation_meta(alice).map(|m| m.primary_session_id),
            Some(1)
        );
        assert_eq!(
            acct.get_rotation_meta(bob).map(|m| m.primary_session_id),
            Some(2)
        );
    }
}
