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

use super::{CryptoProcessState, CryptoState};
use crate::services::messaging::proto;
use crate::storage::database::DataBase;

/// Per-peer Noise session rotation state.
///
/// There is at most one row per `remote_id` in the `rotation_meta`
/// sled tree. It tracks which session is currently primary and, if a
/// rotation is in progress or an old session is still draining, the
/// draining session plus any in-flight initiation the local node owns.
///
/// Collision resolution rule: if this node has
/// `pending_initiated_session_id = Some(mine)` and a
/// `RotateHandshakeFirst` arrives from the peer, the **lower
/// `PeerId` wins** (a fixed symmetric tie-break). If the peer's
/// PeerId is lower, this node abandons its own initiation (deletes
/// the pending `CryptoState` row) and responds; otherwise it ignores
/// the incoming frame and waits for the peer to reply to ours.
///
/// The old session is retired by **draining its in-flight traffic by
/// nonce** — no wall-clock. Each direction has its own nonce counter,
/// so the peer declares its final nonce for the direction this node
/// receives (`draining_recv_target`), and this node retires the old
/// session only once it has received every nonce up to that value.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RotationMeta {
    /// The session_id that is currently used for outbound traffic.
    pub primary_session_id: u32,
    /// Set when this node has sent a `RotateHandshakeFirst` for which
    /// no `RotateHandshakeSecond` has been received yet. The
    /// corresponding `CryptoState` lives in the `state` tree in
    /// `HalfOutgoing`. `None` when no rotation is in flight locally.
    pub pending_initiated_session_id: Option<u32>,
    /// The previous session_id whose `cipher_in` is still accepted
    /// for incoming messages while it drains. `None` when no session
    /// is draining.
    pub draining_session_id: Option<u32>,
    /// The peer's declared final nonce on the draining session in the
    /// direction *this node receives*. The draining session is
    /// retired once every nonce up to this value has been received.
    /// `None` until learned: the initiator learns it from
    /// `RotateHandshakeSecond`; the responder learns it from
    /// `RotateHandshakeFinal`. While `None` the session keeps
    /// draining (never prematurely retired).
    #[serde(default)]
    pub draining_recv_target: Option<u64>,
    /// This node's `highest_index_nonce_in` on the draining session
    /// captured at cut-over. The drain bitmap only needs to cover the
    /// in-flight tail `(base, target]`, not the whole session history,
    /// so it stays small.
    #[serde(default)]
    pub draining_recv_base: u64,
    /// Bitmap of received nonces on the draining session above
    /// `draining_recv_base`. Bit `i` set means nonce
    /// `draining_recv_base + 1 + i` has been received.
    #[serde(default)]
    pub draining_recv_seen: Vec<u8>,
    /// The most recent session_id that finished draining and was
    /// retired. Kept so the decrypt path can tell "message arrived
    /// after the session drained" from "brand-new unknown session".
    ///
    /// Only the *last* retirement is remembered; subsequent rotations
    /// overwrite it. A pair of peers rotating faster than messages can
    /// be redelivered will eventually hit a truly unknown session id,
    /// which the decrypt path handles as a cold re-key.
    #[serde(default)]
    pub last_retired_session_id: Option<u32>,
}

impl RotationMeta {
    /// Construct a fresh meta for a peer that has a single primary
    /// session and no rotation activity.
    pub fn primary_only(primary_session_id: u32) -> Self {
        RotationMeta {
            primary_session_id,
            ..Default::default()
        }
    }

    /// Record that `nonce` was received on the draining session.
    /// Nonces at or below `draining_recv_base` were already received
    /// before cut-over and are ignored.
    pub fn mark_drain_received(&mut self, nonce: u64) {
        if nonce <= self.draining_recv_base {
            return;
        }
        let bit = nonce - self.draining_recv_base - 1;
        let byte = (bit / 8) as usize;
        if byte >= self.draining_recv_seen.len() {
            self.draining_recv_seen.resize(byte + 1, 0);
        }
        self.draining_recv_seen[byte] |= 1u8 << (bit % 8);
    }

    /// Whether every nonce in `(draining_recv_base, target]` has been
    /// received, i.e. the draining session has fully drained. Returns
    /// `false` while `draining_recv_target` is still unknown.
    pub fn drain_complete(&self) -> bool {
        let target = match self.draining_recv_target {
            Some(t) => t,
            None => return false,
        };
        // Peer's final nonce was already received before cut-over.
        if target <= self.draining_recv_base {
            return true;
        }
        // Every nonce base+1..=target must have its bit set.
        for nonce in (self.draining_recv_base + 1)..=target {
            let bit = nonce - self.draining_recv_base - 1;
            let byte = (bit / 8) as usize;
            let set = self
                .draining_recv_seen
                .get(byte)
                .map(|b| b & (1u8 << (bit % 8)) != 0)
                .unwrap_or(false);
            if !set {
                return false;
            }
        }
        true
    }

    /// Test helper: whether `nonce`'s bit is set in the drain bitmap.
    #[cfg(test)]
    pub fn drain_nonce_seen(&self, nonce: u64) -> bool {
        if nonce <= self.draining_recv_base {
            return false;
        }
        let bit = nonce - self.draining_recv_base - 1;
        let byte = (bit / 8) as usize;
        self.draining_recv_seen
            .get(byte)
            .map(|b| b & (1u8 << (bit % 8)) != 0)
            .unwrap_or(false)
    }

    /// Clear all draining-session bookkeeping, recording `drained_id`
    /// as the most recent retirement so the decrypt path can detect
    /// late arrivals on it.
    pub fn clear_drain(&mut self, drained_id: u32) {
        self.draining_session_id = None;
        self.draining_recv_target = None;
        self.draining_recv_base = 0;
        self.draining_recv_seen = Vec::new();
        self.last_retired_session_id = Some(drained_id);
    }
}

/// On-disk shape of `CryptoState` before the handshake-extras
/// `pre_*` fields were appended. Kept (field order and types must
/// never change) so rows written by pre-extras builds keep decoding;
/// see `CryptoAccount::decode_state`.
#[derive(Serialize, Deserialize)]
struct LegacyCryptoState {
    session_id: u32,
    state: CryptoProcessState,
    initiator: bool,
    s: Vec<u8>,
    rs: Vec<u8>,
    e: Vec<u8>,
    re: Option<Vec<u8>>,
    cipher_out: Option<Vec<u8>>,
    index_nonce_out: u64,
    cipher_in: Option<Vec<u8>>,
    highest_index_nonce_in: u64,
    out_of_order_indexes: bool,
}

impl LegacyCryptoState {
    /// Upgrade to the current shape; the handshake-extras fields
    /// start at their defaults (no pre-cipher captured, nothing
    /// seen or accounted).
    fn into_current(self) -> CryptoState {
        CryptoState {
            session_id: self.session_id,
            state: self.state,
            initiator: self.initiator,
            s: self.s,
            rs: self.rs,
            e: self.e,
            re: self.re,
            cipher_out: self.cipher_out,
            index_nonce_out: self.index_nonce_out,
            cipher_in: self.cipher_in,
            highest_index_nonce_in: self.highest_index_nonce_in,
            out_of_order_indexes: self.out_of_order_indexes,
            pre_cipher_out: None,
            pre_index_out: 0,
            pre_cipher_in: None,
            pre_index_in_highest: 0,
            pre_index_in_seen: Vec::new(),
            pre_bytes_accounted: 0,
            established_at: 0,
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

    /// Return the session_ids of all `Transport`-state sessions held
    /// for `remote_id`. Used by the cold re-key path to find prior
    /// sessions that a fresh handshake supersedes.
    pub fn transport_session_ids(&self, remote_id: PeerId) -> Vec<u32> {
        let (first_key, last_key) = Self::create_state_key_range(remote_id);
        let mut out = Vec::new();
        for result in self.state.range(first_key..last_key) {
            if let Ok((_key, bytes)) = result {
                if let Ok(cs) = bincode::deserialize::<CryptoState>(&bytes) {
                    if matches!(cs.state, super::CryptoProcessState::Transport) {
                        out.push(cs.session_id);
                    }
                }
            }
        }
        out
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

    /// Decode a `CryptoState` row from its bincode bytes.
    ///
    /// Tries the current shape first. On failure, falls back to the
    /// pre-handshake-extras shape (rows written before the `pre_*`
    /// fields existed): bincode is not self-describing, so
    /// `#[serde(default)]` on trailing fields does not make old rows
    /// decode — without this fallback every session established
    /// before the upgrade would be lost. Migrated rows get the
    /// extras fields at their defaults and are rewritten in the new
    /// shape on the next `save_state`.
    fn decode_state(crypto_state_bytes: &[u8]) -> Option<CryptoState> {
        match bincode::deserialize::<CryptoState>(crypto_state_bytes) {
            Ok(v) => Some(v),
            Err(current_shape_error) => {
                match bincode::deserialize::<LegacyCryptoState>(crypto_state_bytes) {
                    Ok(legacy) => Some(legacy.into_current()),
                    Err(legacy_shape_error) => {
                        log::error!(
                            "Error deserializing crypto state: {} (legacy fallback: {})",
                            current_shape_error,
                            legacy_shape_error
                        );
                        None
                    }
                }
            }
        }
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
                    let crypto_state: CryptoState = match Self::decode_state(&crypto_state_bytes) {
                        Some(v) => v,
                        None => continue,
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
                return Self::decode_state(&crypto_state_bytes);
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

    /// Build a `CryptoAccount` backed by anonymous in-memory sled
    /// databases. Tests use this to exercise the storage-shaped APIs
    /// (`save_state`, `get_state`, `get_state_by_id`) without
    /// touching the daemon's `QaulState.database` and without any
    /// disk I/O. Each call returns an independent account.
    #[cfg(test)]
    pub fn test_account() -> CryptoAccount {
        use sled::Config;
        let state_db = Config::new().temporary(true).open().unwrap();
        let cache_db = Config::new().temporary(true).open().unwrap();
        CryptoAccount {
            state: state_db.open_tree("crypto_state").unwrap(),
            cache: cache_db.open_tree("crypto_cache").unwrap(),
        }
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
        let state_tree: sled::Tree = match db.open_tree("crypto_state") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("failed to open crypto_state tree: {}", e);
                return CryptoAccount {
                    state: db
                        .open_tree("__fallback_crypto_state")
                        .expect("fallback tree"),
                    cache: db
                        .open_tree("__fallback_crypto_cache")
                        .expect("fallback tree"),
                    rotation_meta: db
                        .open_tree("__fallback_rotation_meta")
                        .expect("fallback tree"),
                };
            }
        };
        let cache: sled::Tree = match db.open_tree("crypto_cache") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("failed to open crypto_cache tree: {}", e);
                return CryptoAccount {
                    state: state_tree,
                    cache: db
                        .open_tree("__fallback_crypto_cache")
                        .expect("fallback tree"),
                    rotation_meta: db
                        .open_tree("__fallback_rotation_meta")
                        .expect("fallback tree"),
                };
            }
        };
        let rotation_meta: sled::Tree = match db.open_tree("rotation_meta") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("failed to open rotation_meta tree: {}", e);
                return CryptoAccount {
                    state: state_tree,
                    cache,
                    rotation_meta: db
                        .open_tree("__fallback_rotation_meta")
                        .expect("fallback tree"),
                };
            }
        };

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
            ..Default::default()
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

#[cfg(test)]
mod legacy_state_decode_tests {
    //! Rows written by builds that predate the handshake-extras
    //! `pre_*` fields on `CryptoState` must still load. bincode is
    //! not self-describing, so `#[serde(default)]` on the new
    //! trailing fields does NOT make old rows decode — without an
    //! explicit legacy fallback, every pre-existing session is lost
    //! on upgrade (decode fails with "unexpected end of file").
    use super::*;
    use crate::services::crypto::CryptoProcessState;
    use libp2p::identity::Keypair;
    use serde::{Deserialize, Serialize};

    /// On-disk shape of `CryptoState` before the handshake-extras
    /// fields were appended. Serialize-only: the tests use it to
    /// produce byte-exact legacy rows.
    #[derive(Serialize, Deserialize)]
    struct PreExtrasCryptoState {
        session_id: u32,
        state: CryptoProcessState,
        initiator: bool,
        s: Vec<u8>,
        rs: Vec<u8>,
        e: Vec<u8>,
        re: Option<Vec<u8>>,
        cipher_out: Option<Vec<u8>>,
        index_nonce_out: u64,
        cipher_in: Option<Vec<u8>>,
        highest_index_nonce_in: u64,
        out_of_order_indexes: bool,
    }

    fn fresh_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    /// Insert a legacy-shaped row directly into the state tree,
    /// bypassing `save_state` (which would write the new shape).
    fn insert_legacy_row(acct: &CryptoAccount, remote_id: PeerId, session_id: u32) {
        let legacy = PreExtrasCryptoState {
            session_id,
            state: CryptoProcessState::Transport,
            initiator: true,
            s: vec![1; 32],
            rs: vec![2; 32],
            e: vec![3; 32],
            re: Some(vec![4; 32]),
            cipher_out: Some(vec![5; 32]),
            index_nonce_out: 1000,
            cipher_in: Some(vec![6; 32]),
            highest_index_nonce_in: 999,
            out_of_order_indexes: false,
        };
        let bytes = match bincode::serialize(&legacy) {
            Ok(v) => v,
            Err(e) => panic!("serializing legacy row failed: {}", e),
        };
        let key = CryptoAccount::create_state_key(remote_id, session_id);
        if let Err(e) = acct.state.insert(key, bytes) {
            panic!("inserting legacy row failed: {}", e);
        }
    }

    /// A session stored by a pre-extras build must still be found by
    /// `get_state_by_id`, with the extras fields at their defaults.
    #[test]
    fn legacy_row_loads_via_get_state_by_id() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        insert_legacy_row(&acct, remote, 42);

        let loaded = match acct.get_state_by_id(remote, 42) {
            Some(v) => v,
            None => panic!("legacy CryptoState row no longer decodes — established sessions are lost on upgrade"),
        };
        assert_eq!(loaded.session_id, 42);
        assert_eq!(loaded.index_nonce_out, 1000);
        assert_eq!(loaded.highest_index_nonce_in, 999);
        assert_eq!(loaded.cipher_out, Some(vec![5; 32]));
        // extras fields come up at their defaults
        assert_eq!(loaded.pre_cipher_out, None);
        assert_eq!(loaded.pre_index_out, 0);
        assert_eq!(loaded.pre_cipher_in, None);
        assert_eq!(loaded.pre_index_in_highest, 0);
        assert!(loaded.pre_index_in_seen.is_empty());
        assert_eq!(loaded.pre_bytes_accounted, 0);
    }

    /// Same through `get_state` (the active-session scan used by the
    /// messaging send/receive paths).
    #[test]
    fn legacy_row_loads_via_get_state() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        insert_legacy_row(&acct, remote, 7);

        let loaded = match acct.get_state(remote) {
            Some(v) => v,
            None => panic!("legacy CryptoState row no longer decodes via get_state"),
        };
        assert_eq!(loaded.session_id, 7);
        assert!(matches!(loaded.state, CryptoProcessState::Transport));
    }

    /// Rows written in the current shape keep decoding unchanged.
    #[test]
    fn current_shape_row_still_loads() {
        let acct = CryptoStorage::test_account();
        let remote = fresh_peer();
        let state = CryptoState {
            session_id: 9,
            state: CryptoProcessState::Transport,
            initiator: false,
            s: vec![],
            rs: vec![],
            e: vec![],
            re: None,
            cipher_out: None,
            index_nonce_out: 3,
            cipher_in: None,
            highest_index_nonce_in: 2,
            out_of_order_indexes: false,
            pre_cipher_out: Some(vec![7; 32]),
            pre_index_out: 5,
            pre_cipher_in: None,
            pre_index_in_highest: 4,
            pre_index_in_seen: vec![0b1011],
            pre_bytes_accounted: 4096,
        };
        acct.save_state(remote, 9, state);

        let loaded = match acct.get_state_by_id(remote, 9) {
            Some(v) => v,
            None => panic!("current-shape row failed to decode"),
        };
        assert_eq!(loaded.pre_cipher_out, Some(vec![7; 32]));
        assert_eq!(loaded.pre_index_out, 5);
        assert_eq!(loaded.pre_index_in_seen, vec![0b1011]);
        assert_eq!(loaded.pre_bytes_accounted, 4096);
    }
}
