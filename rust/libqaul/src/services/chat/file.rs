// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat File Transfer
//!
//! Sending files via the chat messenger to other users and groups.
//! The chat file messages use the messaging service.

use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled;
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::RwLock,
};

use super::ChatStorage;
use crate::services::messaging::{self, MessagingServiceType};
use crate::storage::database::DataBase;
use crate::utilities::timestamp;
use crate::utilities::timestamp::Timestamp;
use crate::{
    node::user_accounts::{UserAccount, UserAccounts},
    router::users::Users,
    services::group::{self, group_id::GroupId, Group, GroupStorage},
};
use crate::{rpc::Rpc, services::group::GroupManage};

pub use qaul_proto::qaul_net_chatfile as proto_net;
/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_chatfile as proto_rpc;

/// Size of the biggest file data package
pub const DEF_PACKAGE_SIZE: u32 = 64000;

/// Structure to management for file histories based on the each user_id.
pub struct AllFiles {
    pub db_ref: BTreeMap<Vec<u8>, UserFiles>,
}

/// User file histories structure
#[derive(Clone)]
pub struct UserFiles {
    /// history table
    ///
    /// key: file ID
    /// value: bincode of `FileHistory`
    pub histories: sled::Tree,
    /// file data chunks
    ///
    /// Storage of incoming chunks until receiving is completed.
    ///
    /// key: file_id & chunk_index
    /// value: `Vec<u8>`
    pub file_chunks: sled::Tree,
    /// per-file envelope content keys (group-file envelope encryption)
    ///
    /// Holds received `file_key`s awaiting their body, plus locally
    /// generated keys for sent files that may need relay. Entries
    /// expire by `GroupFiles.envelope_ttl_seconds`.
    ///
    /// key: file_id (u64 big-endian)
    /// value: bincode of `FileKey`
    pub file_keys: sled::Tree,
}

/// A stored per-file envelope content key.
///
/// Either received in a `FileKeyEnvelope` (awaiting the body) or
/// generated locally for a file we sent (kept for possible relay to
/// late joiners). `body_digest` binds the key to exactly one body.
#[derive(Clone, Serialize, Deserialize)]
pub struct FileKey {
    pub file_id: u64,
    pub group_id: Vec<u8>,
    /// 32-byte ChaCha20-Poly1305 content key.
    pub file_key: Vec<u8>,
    /// SHA-256 of the encrypted body this key decrypts.
    pub body_digest: Vec<u8>,
    /// Wall-clock ms when this entry was stored; drives TTL expiry.
    pub stored_at: u64,
    /// True for keys we generated locally (sent files), which we may
    /// relay; false for keys received from a sender.
    pub locally_generated: bool,
}

impl AllFiles {
    /// Remove cached sled::Tree handles for a user account.
    ///
    /// This must be called before closing the user's sled database
    /// so that all Arc references to the Db are released.
    pub fn remove_account(state: &crate::QaulState, account_id: PeerId) {
        let mut all_files = state.services.chat_files.inner.write().unwrap();
        all_files.db_ref.remove(&account_id.to_bytes());
    }
}

impl UserFiles {
    /// get file history
    pub fn get_filehistory(&self, file_id: u64) -> Option<FileHistory> {
        // get invite
        match self.histories.get(file_id.to_be_bytes()) {
            Ok(None) => log::warn!("file history empty"),
            Ok(Some(file_history_bytes)) => match bincode::deserialize(&file_history_bytes) {
                Ok(file_history) => return Some(file_history),
                Err(e) => log::error!("{}", e),
            },
            Err(e) => log::error!("{}", e),
        }

        None
    }

    /// get file history iterator
    pub fn get_filehistory_iterator(&self) -> sled::Iter {
        // get key range
        let first_key: u64 = 0;
        let last_key: u64 = u64::MAX;

        // get results from data base
        let result = self
            .histories
            .range(first_key.to_be_bytes()..last_key.to_be_bytes());

        result
    }

    /// save file history
    pub fn save_filehistory(&self, file_id: u64, file_history: FileHistory) {
        // save file history into data base
        let file_history_bytes = match bincode::serialize(&file_history) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!("Error serializing file history: {}", e);
                return;
            }
        };
        if let Err(e) = self
            .histories
            .insert(file_id.to_be_bytes(), file_history_bytes)
        {
            log::error!("Error saving file history to data base: {}", e);
            return;
        }
        // flush trees to disk
        if let Err(e) = self.histories.flush() {
            log::error!("Error file history flush: {}", e);
        }
    }

    /// create a db chunk key
    fn get_chunk_key(file_id: &[u8], index: u32) -> Vec<u8> {
        let mut key_bytes = Vec::with_capacity(file_id.len() + std::mem::size_of::<u32>());
        key_bytes.extend_from_slice(file_id);
        key_bytes.extend_from_slice(&index.to_be_bytes());
        key_bytes
    }

    /// get file chunk DB key range
    ///
    /// returns a key tuple, which can be used to
    /// retrieve all file chunks for a certain file id from DB
    ///
    /// (first_key, last_key)
    fn get_chunk_key_range(file_id: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let first_key = Self::get_chunk_key(file_id, 0);
        let last_key = Self::get_chunk_key(file_id, u32::MAX);
        (first_key, last_key)
    }

    /// save file chunk
    pub fn save_file_chunk(&self, file_id: u64, index: u32, data: Vec<u8>) {
        // get chunk key
        let file_id_bytes = file_id.to_be_bytes();
        let key = Self::get_chunk_key(&file_id_bytes, index);

        log::trace!("save file chunk {} with key: {:?}", index, key);

        // save file chunk into data base
        if let Err(e) = self.file_chunks.insert(key, data) {
            log::error!("Error saving file chunk to data base: {}", e);
            return;
        }

        // flush trees to disk
        if let Err(e) = self.file_chunks.flush() {
            log::error!("Error file history flush: {}", e);
        }
    }

    /// Store an envelope content key for `file_id` (received or locally
    /// generated). Idempotent: storing the same `file_id` overwrites.
    pub fn save_file_key(&self, key: &FileKey) {
        let bytes = match bincode::serialize(key) {
            Ok(b) => b,
            Err(e) => {
                log::error!("Error serializing file key: {}", e);
                return;
            }
        };
        if let Err(e) = self.file_keys.insert(key.file_id.to_be_bytes(), bytes) {
            log::error!("Error saving file key: {}", e);
            return;
        }
        if let Err(e) = self.file_keys.flush() {
            log::error!("Error file_keys flush: {}", e);
        }
    }

    /// Fetch the envelope content key for `file_id`, if present.
    pub fn get_file_key(&self, file_id: u64) -> Option<FileKey> {
        match self.file_keys.get(file_id.to_be_bytes()) {
            Ok(Some(bytes)) => match bincode::deserialize(&bytes) {
                Ok(k) => Some(k),
                Err(e) => {
                    log::error!("Error deserializing file key: {}", e);
                    None
                }
            },
            Ok(None) => None,
            Err(e) => {
                log::error!("Error reading file key: {}", e);
                None
            }
        }
    }

    /// Remove (and zeroize via drop) the stored key for `file_id`,
    /// e.g. once the body is decrypted and exposed to the user.
    pub fn delete_file_key(&self, file_id: u64) {
        if let Err(e) = self.file_keys.remove(file_id.to_be_bytes()) {
            log::error!("Error removing file key: {}", e);
        }
        if let Err(e) = self.file_keys.flush() {
            log::error!("Error file_keys flush: {}", e);
        }
    }

    /// Drop any stored file keys older than `ttl_ms` (by `stored_at`).
    /// Bounds the `file_keys` tree even when bodies never arrive.
    pub fn prune_expired_file_keys(&self, now_ms: u64, ttl_ms: u64) {
        let mut expired: Vec<u64> = Vec::new();
        for item in self.file_keys.iter() {
            if let Ok((_k, bytes)) = item {
                if let Ok(key) = bincode::deserialize::<FileKey>(&bytes) {
                    if now_ms.saturating_sub(key.stored_at) > ttl_ms {
                        expired.push(key.file_id);
                    }
                }
            }
        }
        for file_id in expired {
            self.delete_file_key(file_id);
        }
    }

    /// count file chunks
    ///
    /// Count how many chunks of a file we already have in the data base
    pub fn count_file_chunks(&self, file_id: &[u8]) -> usize {
        // get key range
        let (first_key, last_key) = Self::get_chunk_key_range(file_id);

        // get results from data base
        let result = self.file_chunks.range(first_key..last_key);

        result.count()
    }

    /// get all file chunks for a specific id
    pub fn get_file_chunks(&self, file_id: &[u8]) -> sled::Iter {
        // get key range
        let (first_key, last_key) = Self::get_chunk_key_range(file_id);

        // get results from data base
        let result = self.file_chunks.range(first_key..last_key);

        result
    }
}

/// File State
#[derive(Serialize, Deserialize, Clone)]
pub enum FileState {
    /// We are in the process of sending this file
    Sending,
    /// File has been sent to another user
    Sent,
    /// File reception has been confirmed
    Confirmed,
    /// Confirmed by all recipients
    ConfirmedByAll,
    /// Receiving
    Receiving,
    /// File successfully received
    Received,
}

/// User Reception Tracking
#[derive(Serialize, Deserialize, Clone)]
pub struct ReceptionTracking {
    pub received: bool,
    pub package_count: u32,
}

/// File history structure, this structure is stored into DB
#[derive(Serialize, Deserialize, Clone)]
pub struct FileHistory {
    /// group id
    pub group_id: Vec<u8>,
    /// sender id
    pub sender_id: Vec<u8>,
    /// file id
    pub file_id: u64,
    /// message id
    pub message_id: Vec<u8>,
    /// start index
    pub start_index: u32,
    /// sender id
    pub message_count: u32,
    /// chunk size
    pub chunk_size: u32,
    /// file_state
    pub file_state: FileState,
    /// reception_tracking
    pub reception_tracking: BTreeMap<Vec<u8>, ReceptionTracking>,
    /// file name
    pub file_name: String,
    /// file description
    pub file_description: String,
    /// file extension
    pub file_extension: String,
    /// file size in bytes
    // TODO: u32 limits files to 4GB; consider u64 for large file support
    pub file_size: u32,
    /// file sent
    pub sent_at: u64,
    /// file received
    pub received_at: u64,
    /// Group-file envelope marker + binding. Non-empty means this file
    /// is envelope-encrypted: the stored chunks are the body ciphertext
    /// (one per-file key, delivered via FileKeyEnvelope), and this is
    /// its SHA-256 digest. Empty = legacy per-recipient encryption.
    #[serde(default)]
    pub body_digest: Vec<u8>,
}

impl FileHistory {
    /// the reception of a file message has successfully been confirmed
    ///
    /// the function returns a boolean that indicates, whether the user finished receiving
    /// {user completed}
    pub fn reception_confirmed(&mut self, receiver_id: PeerId) -> bool {
        let key = receiver_id.to_bytes();
        if let Some(tracking) = self.reception_tracking.get_mut(&key) {
            tracking.package_count = tracking.package_count + 1;
            log::trace!("package_count {}", tracking.package_count);

            // check if user has received all messages
            if tracking.package_count >= self.message_count {
                tracking.received = true;
                self.file_state = FileState::Confirmed;

                // check if all users have received all messages
                let mut done = true;
                for (_, v) in &self.reception_tracking {
                    if v.received == false {
                        done = false;
                        break;
                    }
                }

                // set file reception state
                if done {
                    self.file_state = FileState::ConfirmedByAll;
                }

                return true;
            }
        }

        false
    }
}

/// Instance-based file history state.
/// Replaces the global ALLFILES static for multi-instance use.
pub struct ChatFileState {
    /// File history inner state.
    pub inner: RwLock<AllFiles>,
}

impl ChatFileState {
    /// Create a new empty ChatFileState.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(AllFiles {
                db_ref: BTreeMap::new(),
            }),
        }
    }

    /// Get DB refs for user account (instance method).
    /// Takes an explicit `sled::Db` instead of calling `DataBase::get_user_db()`.
    pub fn get_db_ref(&self, user_id: &PeerId, db: &sled::Db) -> UserFiles {
        {
            let all_files = self.inner.read().unwrap();
            if let Some(user_files) = all_files.db_ref.get(&user_id.to_bytes()) {
                return UserFiles {
                    histories: user_files.histories.clone(),
                    file_chunks: user_files.file_chunks.clone(),
                    file_keys: user_files.file_keys.clone(),
                };
            }
        }

        self.create_userfiles(user_id, db)
    }

    /// Create user file data when it does not exist (instance method).
    fn create_userfiles(&self, user_id: &PeerId, db: &sled::Db) -> UserFiles {
        let histories: sled::Tree = db.open_tree("chat_file").unwrap();
        let file_chunks: sled::Tree = db.open_tree("file_chunks").unwrap();
        let file_keys: sled::Tree = db.open_tree("file_keys").unwrap();

        let user_files = UserFiles {
            histories,
            file_chunks,
            file_keys,
        };

        let mut all_files = self.inner.write().unwrap();
        all_files
            .db_ref
            .insert(user_id.to_bytes(), user_files.clone());

        user_files
    }
}

pub struct ChatFile {}
/// File sharing module to process transfer, receive and RPC commands
impl ChatFile {
    /// initialize chat file module
    pub fn init() {
        // State is already created by QaulState; nothing to do here.
    }

    /// File history is stored based on the users account id.
    /// This function getting history table based on the users account id.
    fn get_db_ref(state: &crate::QaulState, user_id: &PeerId) -> UserFiles {
        // check if user data exists
        {
            // get chat file state
            let all_files = state.services.chat_files.inner.read().unwrap();

            // check if user ID is in map
            if let Some(user_files) = all_files.db_ref.get(&user_id.to_bytes()) {
                return UserFiles {
                    histories: user_files.histories.clone(),
                    file_chunks: user_files.file_chunks.clone(),
                    file_keys: user_files.file_keys.clone(),
                };
            }
        }

        // create user data if it does not exist
        let user_files = Self::create_userfiles(state, user_id);

        // return chat_user structure
        UserFiles {
            histories: user_files.histories.clone(),
            file_chunks: user_files.file_chunks.clone(),
            file_keys: user_files.file_keys.clone(),
        }
    }

    /// create [user => file history] when it does not exist
    fn create_userfiles(state: &crate::QaulState, user_id: &PeerId) -> UserFiles {
        // get user data base
        let db = DataBase::get_user_db(state, user_id.clone());

        // open trees
        let histories: sled::Tree = match db.open_tree("chat_file") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Error opening chat_file tree: {}", e);
                return UserFiles {
                    histories: db.open_tree("__fallback_chat_file").expect("fallback tree"),
                    file_chunks: db.open_tree("__fallback_file_chunks").expect("fallback tree"),
                    file_keys: db.open_tree("__fallback_file_keys").expect("fallback tree"),
                };
            }
        };
        let file_chunks: sled::Tree = match db.open_tree("file_chunks") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Error opening file_chunks tree: {}", e);
                return UserFiles {
                    histories,
                    file_chunks: db.open_tree("__fallback_file_chunks").expect("fallback tree"),
                    file_keys: db.open_tree("__fallback_file_keys").expect("fallback tree"),
                };
            }
        };
        let file_keys: sled::Tree = match db.open_tree("file_keys") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Error opening file_keys tree: {}", e);
                return UserFiles {
                    histories,
                    file_chunks,
                    file_keys: db.open_tree("__fallback_file_keys").expect("fallback tree"),
                };
            }
        };

        let user_files = UserFiles {
            histories,
            file_chunks,
            file_keys,
        };

        // get chat file state for writing
        let mut all_files = state.services.chat_files.inner.write().unwrap();

        // add user to state
        all_files
            .db_ref
            .insert(user_id.to_bytes(), user_files.clone());

        // return structure
        user_files
    }

    /// Update file message confirmation in data base
    pub fn update_confirmation(
        state: &crate::QaulState,
        account_id: PeerId,
        receiver_id: PeerId,
        file_id: u64,
        received_at: u64,
    ) {
        log::trace!("update confirmation");

        // get db reference
        let user_files = ChatFile::get_db_ref(state, &account_id);

        // get file history
        if let Some(mut file_history) = user_files.get_filehistory(file_id) {
            // update reception & check if user finished
            if file_history.reception_confirmed(receiver_id) {
                // update chat message
                ChatStorage::update_confirmation(
                    state,
                    account_id,
                    receiver_id,
                    &file_history.message_id,
                    received_at,
                );
            }

            // save file history
            user_files.save_filehistory(file_id, file_history);
        }
    }

    /// getting file extension from given filename
    fn get_extension_from_filename(filename: &str) -> Option<&str> {
        Path::new(filename).extension().and_then(OsStr::to_str)
    }

    /// Create and return the file path for a file
    fn create_file_path(state: &crate::QaulState, account_id: PeerId, file_id: u64, file_extension: &str) -> PathBuf {
        // create path to file storage directory
        let account_storage_path = crate::storage::Storage::get_account_path(state, account_id);
        let files_storage_path = account_storage_path.join("files");

        // create file directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&files_storage_path) {
            log::error!("creating folder error {}", e.to_string());
        }

        // create file name
        let mut file_name = file_id.to_string();
        if file_extension.len() > 0 {
            file_name.push_str(".");
            file_name.push_str(&file_extension);
        }

        // create file path
        files_storage_path.join(file_name)
    }

    fn file_content_from_history(file_history: &FileHistory) -> super::rpc_proto::FileContent {
        super::rpc_proto::FileContent {
            file_id: file_history.file_id,
            file_name: file_history.file_name.clone(),
            file_extension: file_history.file_extension.clone(),
            file_size: file_history.file_size,
            file_description: file_history.file_description.clone(),
        }
    }

    fn rpc_history_entry_from_file_history(entry: &FileHistory) -> proto_rpc::FileHistoryEntry {
        let group_id = GroupId::from_bytes(&entry.group_id)
            .map(|id| id.to_string())
            .unwrap_or_else(|_| GroupId::slice_to_string(&entry.group_id));

        proto_rpc::FileHistoryEntry {
            file_id: entry.file_id,
            file_name: entry.file_name.clone(),
            file_extension: entry.file_extension.clone(),
            file_size: entry.file_size,
            file_description: entry.file_description.clone(),
            time: entry.sent_at,
            sender_id: bs58::encode(&entry.sender_id).into_string(),
            group_id,
        }
    }

    fn file_history_from_info_message(
        sender_id: &PeerId,
        group_id: &[u8],
        message_id: &[u8],
        sent_at: u64,
        file_info: &proto_net::ChatFileInfo,
    ) -> FileHistory {
        FileHistory {
            group_id: group_id.to_vec(),
            sender_id: sender_id.to_bytes(),
            file_id: file_info.file_id,
            message_id: message_id.to_vec(),
            start_index: file_info.start_index,
            message_count: file_info.message_count,
            chunk_size: file_info.data_chunk_size,
            file_state: FileState::Receiving,
            reception_tracking: BTreeMap::new(),
            file_name: file_info.file_name.clone(),
            file_description: file_info.file_description.clone(),
            file_extension: file_info.file_extension.clone(),
            file_size: file_info.file_size,
            sent_at,
            received_at: Timestamp::get_timestamp(),
            body_digest: file_info.body_digest.clone(),
        }
    }

    /// Getting file histories from table.
    /// This function is called from RPC command (file history [offset limit])
    pub fn file_history(
        state: &crate::QaulState,
        user_account: &UserAccount,
        history_req: &proto_rpc::FileHistoryRequest,
    ) -> Vec<FileHistory> {
        // get DB references
        let db_ref = Self::get_db_ref(state, &user_account.id);

        let mut histories: Vec<FileHistory> = vec![];

        // loop through results
        let mut counter: u32 = 0;

        let iterator = db_ref.get_filehistory_iterator();
        for history_result in iterator {
            match history_result {
                Ok((_id, message)) => {
                    // stop when we reached our limit
                    if counter >= history_req.offset + history_req.limit {
                        break;
                    }

                    // check if we collect the result
                    let file_history: FileHistory = match bincode::deserialize(&message) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("Error deserializing file history: {}", e);
                            continue;
                        }
                    };
                    if counter >= history_req.offset {
                        histories.push(file_history);
                    }

                    counter = counter + 1;
                }
                Err(e) => log::error!("{}", e),
            }
        }

        histories
    }

    /// send a file from RPC to users
    fn send(
        state: &crate::QaulState,
        user_account: &UserAccount,
        group_id: &[u8],
        path_name: String,
        description: String,
    ) -> Result<bool, String> {
        // get group
        let group;
        match GroupStorage::get_group(state, user_account.id, group_id) {
            Some(v) => group = v,
            None => {
                match GroupId::from_bytes(group_id) {
                    Ok(direct_group) => {
                        // check if the group is a direct group
                        match direct_group.is_direct(user_account.id.clone()) {
                            Some(remote_q8id) => {
                                // get remote user
                                let rs = state.get_router();
                                match Users::get_user_id_by_q8id(&rs, &remote_q8id) {
                                    Some(remote_id) => {
                                        // create group
                                        group = GroupManage::create_new_direct_chat_group(
                                            state,
                                            &user_account.id,
                                            &remote_id,
                                        )
                                    }
                                    None => {
                                        return Err(
                                            "remote id of direct group not found".to_string()
                                        )
                                    }
                                }
                            }
                            None => {
                                return Err("Group does not exist and is not direct".to_string())
                            }
                        }
                    }
                    Err(_) => return Err("Group does not exist".to_string()),
                }
            }
        }

        let timestamp = Timestamp::get_timestamp();

        let mut file: File;
        match File::open(&path_name) {
            Ok(f) => Some(file = f),
            Err(_e) => {
                return Err("file open error".to_string());
            }
        };

        let size = match file.metadata() {
            Ok(m) => m.len() as u32,
            Err(e) => {
                return Err(format!("file metadata error: {}", e));
            }
        };
        if size == 0 {
            return Err("file size is zero".to_string());
        }

        // get file name
        let path = Path::new(path_name.as_str());
        let mut extension = "".to_string();

        let path_file_name = match path.file_name().and_then(|f| f.to_str()) {
            Some(name) => name,
            None => {
                return Err("unable to get file name from path".to_string());
            }
        };

        if let Some(ext) = Self::get_extension_from_filename(path_file_name) {
            extension = ext.to_string();
        }

        let file_name = path_file_name.to_string();

        // create file id
        let user_id_bytes = user_account.id.to_bytes();
        let file_id = Self::generate_file_id(group_id, &user_id_bytes, &file_name, size);

        // get file path
        let file_path = Self::create_file_path(state, user_account.id, file_id, extension.as_str());

        // TODO: start in new async thread here

        // copy file
        if let Err(e) = fs::copy(&path_name, file_path) {
            log::error!("copy file error {}", e);
        }

        // create messages
        let mut mesage_count = 1 + size / DEF_PACKAGE_SIZE;
        if size % DEF_PACKAGE_SIZE > 0 {
            mesage_count += 1;
        }

        // create message ID
        let message_id = group::GroupManage::get_new_message_id(state, &user_account.id, group_id);

        // Envelope mode: encrypt the body once under a per-file key and
        // distribute that key per-member, instead of encrypting the
        // whole file once per recipient. Gated on config; capability
        // negotiation is a follow-up.
        let envelope_enabled = crate::storage::configuration::Configuration::get(state)
            .group_files
            .envelope_enabled;
        if envelope_enabled {
            return Self::send_envelope(
                state,
                user_account,
                &group,
                group_id,
                file_id,
                &file_name,
                &extension,
                size,
                &description,
                &message_id,
                timestamp,
                &path_name,
            );
        }

        // 1. file info message
        let file_info = proto_net::ChatFileInfo {
            file_id,
            file_name: file_name.clone(),
            file_extension: extension.clone(),
            file_size: size,
            file_description: description.clone(),
            start_index: 0,
            message_count: mesage_count,
            data_chunk_size: DEF_PACKAGE_SIZE,
            // legacy per-recipient path: not envelope-encrypted
            body_digest: Vec::new(),
        };

        let info = proto_net::ChatFileContainer {
            message: Some(proto_net::chat_file_container::Message::FileInfo(
                file_info.clone(),
            )),
        };

        // send message to all group members
        Self::send_filecontainer_to_group(
            state,
            user_account,
            &group,
            &message_id,
            timestamp,
            info.encode_to_vec(),
        );

        // create group ID object
        let groupid = match GroupId::from_bytes(group_id) {
            Ok(id) => id,
            Err(e) => {
                log::error!("Error parsing group id: {}", e);
                return Err("invalid group id".to_string());
            }
        };

        // save file state to data base
        let file_history = FileHistory {
            group_id: group_id.to_vec(),
            sender_id: user_id_bytes,
            file_id,
            message_id: message_id.clone(),
            start_index: file_info.start_index,
            message_count: file_info.message_count,
            chunk_size: DEF_PACKAGE_SIZE,
            file_state: FileState::Sending,
            reception_tracking: BTreeMap::new(),
            file_name: file_name.clone(),
            file_description: description.clone(),
            file_extension: extension.clone(),
            file_size: size,
            sent_at: timestamp,
            received_at: 0,
            body_digest: Vec::new(),
        };

        let db_ref = Self::get_db_ref(state, &user_account.id);

        // save file history to data base
        let file_history_bytes = match bincode::serialize(&file_history) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!("Error serializing file history: {}", e);
                return Err("failed to serialize file history".to_string());
            }
        };
        if let Err(e) = db_ref
            .histories
            .insert(&file_id.to_be_bytes(), file_history_bytes)
        {
            log::error!("Error saving file history to data base: {}", e);
        } else {
            if let Err(e) = db_ref.histories.flush() {
                log::error!("Error when flushing data base to disk: {}", e);
            }
        }

        // save file message to chat conversation
        Self::save_filemsg_in_chat(
            state,
            user_account,
            &user_account.id,
            &groupid,
            &file_history,
            super::rpc_proto::MessageStatus::Sending,
        );

        // 2. file data message
        // read file contents and create and send FileData messages
        let mut buffer: [u8; DEF_PACKAGE_SIZE as usize] = [0; DEF_PACKAGE_SIZE as usize];
        let mut left_size = size;
        let mut chunk_index: u32 = 0;

        while left_size > 0 {
            let mut read_size = left_size;
            if left_size > DEF_PACKAGE_SIZE {
                read_size = DEF_PACKAGE_SIZE;
            };
            left_size -= read_size;

            if let Err(e) = file.read(&mut buffer) {
                return Err(e.to_string());
            }

            // pack chat file container
            let data = proto_net::ChatFileContainer {
                message: Some(proto_net::chat_file_container::Message::FileData(
                    proto_net::ChatFileData {
                        file_id,
                        start_index: chunk_index,
                        message_count: mesage_count,
                        data: buffer[0..(read_size as usize)].to_vec(),
                    },
                )),
            };

            // send message to all group members
            Self::send_filecontainer_to_group(
                state,
                user_account,
                &group,
                &message_id,
                timestamp,
                data.encode_to_vec(),
            );

            chunk_index += 1;
        }

        // set file status to sent
        ChatStorage::udate_status(
            state,
            &user_account.id,
            &message_id,
            super::rpc_proto::MessageStatus::Sent,
        );

        Ok(true)
    }

    /// Save File Message in Chat Conversation
    ///
    /// Creates a chat message and saves it to the chat db.
    fn save_filemsg_in_chat(
        state: &crate::QaulState,
        user_account: &UserAccount,
        sender_id: &PeerId,
        group_id: &GroupId,
        file_history: &FileHistory,
        status: super::rpc_proto::MessageStatus,
    ) {
        log::trace!("save_filemsg_in_chat");

        // create chat file message content
        let chat_message = super::rpc_proto::ChatContentMessage {
            message: Some(
                super::rpc_proto::chat_content_message::Message::FileContent(
                    Self::file_content_from_history(file_history),
                ),
            ),
        };

        // save file message to chat
        ChatStorage::save_message(
            state,
            &user_account.id,
            &group_id,
            &sender_id,
            &file_history.message_id,
            file_history.sent_at,
            chat_message,
            status,
        );
    }

    /// Pack a FileContainer message and send it to all Group members
    fn send_filecontainer_to_group(
        state: &crate::QaulState,
        user_account: &UserAccount,
        group: &Group,
        message_id: &[u8],
        timestamp: u64,
        data: Vec<u8>,
    ) {
        // pack file container into common message
        let common_message = messaging::proto::CommonMessage {
            message_id: message_id.to_vec(),
            group_id: group.id.clone(),
            sent_at: timestamp,
            payload: Some(messaging::proto::common_message::Payload::FileMessage(
                messaging::proto::FileMessage { content: data },
            )),
        };

        let message = messaging::proto::Messaging {
            message: Some(messaging::proto::messaging::Message::CommonMessage(
                common_message,
            )),
        };
        let message_bytes = message.encode_to_vec();

        Group::send_to_remote_members(
            state,
            user_account,
            group,
            &message_bytes,
            MessagingServiceType::ChatFile,
            message_id,
            "sending file message error",
        );
    }

    /// Like [`send_filecontainer_to_group`] but distributes the file
    /// container to members **without** per-recipient session
    /// encryption (the body is already encrypted under the per-file
    /// key). The Envelope is still signed.
    fn send_filecontainer_to_group_plain(
        state: &crate::QaulState,
        user_account: &UserAccount,
        group: &Group,
        message_id: &[u8],
        timestamp: u64,
        data: Vec<u8>,
    ) {
        let common_message = messaging::proto::CommonMessage {
            message_id: message_id.to_vec(),
            group_id: group.id.clone(),
            sent_at: timestamp,
            payload: Some(messaging::proto::common_message::Payload::FileMessage(
                messaging::proto::FileMessage { content: data },
            )),
        };
        let message = messaging::proto::Messaging {
            message: Some(messaging::proto::messaging::Message::CommonMessage(
                common_message,
            )),
        };
        let message_bytes = message.encode_to_vec();

        Group::send_to_remote_members_plain(
            state,
            user_account,
            group,
            &message_bytes,
            message_id,
            "sending plain file message error",
        );
    }

    /// Envelope-encrypted group file send.
    ///
    /// Encrypts the body once under a fresh per-file key, distributes
    /// the (already-encrypted) body to members via the signed-but-not-
    /// session-encrypted "plain" path, and fans out the per-file key to
    /// each member under their per-peer session (`FileKeyEnvelope`).
    ///
    /// NOTE: file metadata (name/size/description) currently travels in
    /// the clear at the service layer (per the proposal). Encrypting
    /// the FileInfo per-recipient is a noted follow-up.
    fn send_envelope(
        state: &crate::QaulState,
        user_account: &UserAccount,
        group: &Group,
        group_id: &[u8],
        file_id: u64,
        file_name: &str,
        extension: &str,
        size: u32,
        description: &str,
        message_id: &[u8],
        timestamp: u64,
        path_name: &str,
    ) -> Result<bool, String> {
        // read the whole body and encrypt it once under a fresh key
        let body = match fs::read(path_name) {
            Ok(b) => b,
            Err(e) => return Err(format!("file read error: {}", e)),
        };
        let file_key = super::file_envelope::generate_file_key();
        let body_ct = super::file_envelope::encrypt_body(&file_key, &body);
        let digest = super::file_envelope::body_digest(&body_ct);

        // store the key locally (for possible relay to late joiners)
        let db_ref = Self::get_db_ref(state, &user_account.id);
        db_ref.save_file_key(&FileKey {
            file_id,
            group_id: group_id.to_vec(),
            file_key: file_key.clone(),
            body_digest: digest.clone(),
            stored_at: timestamp,
            locally_generated: true,
        });

        // chunk count over the ciphertext body, using the SAME
        // convention as the legacy sender: message_count is
        // (actual chunks + 1), because the receiver completes on
        // `(count + 1) == message_count`.
        let ct_len = body_ct.len() as u32;
        let mut message_count = 1 + ct_len / DEF_PACKAGE_SIZE;
        if ct_len % DEF_PACKAGE_SIZE > 0 {
            message_count += 1;
        }

        // 1. file info (carries the envelope marker / body digest)
        let file_info = proto_net::ChatFileInfo {
            file_id,
            file_name: file_name.to_string(),
            file_extension: extension.to_string(),
            file_size: size, // original plaintext size, for display
            file_description: description.to_string(),
            start_index: 0,
            message_count,
            data_chunk_size: DEF_PACKAGE_SIZE,
            body_digest: digest.clone(),
        };
        let info = proto_net::ChatFileContainer {
            message: Some(proto_net::chat_file_container::Message::FileInfo(
                file_info.clone(),
            )),
        };
        Self::send_filecontainer_to_group_plain(
            state,
            user_account,
            group,
            message_id,
            timestamp,
            info.encode_to_vec(),
        );

        // persist file history + chat message
        let groupid = match GroupId::from_bytes(group_id) {
            Ok(id) => id,
            Err(e) => return Err(format!("invalid group id: {}", e)),
        };
        let file_history = FileHistory {
            group_id: group_id.to_vec(),
            sender_id: user_account.id.to_bytes(),
            file_id,
            message_id: message_id.to_vec(),
            start_index: 0,
            message_count,
            chunk_size: DEF_PACKAGE_SIZE,
            file_state: FileState::Sending,
            reception_tracking: BTreeMap::new(),
            file_name: file_name.to_string(),
            file_description: description.to_string(),
            file_extension: extension.to_string(),
            file_size: size,
            sent_at: timestamp,
            received_at: 0,
            body_digest: digest.clone(),
        };
        db_ref.save_filehistory(file_id, file_history.clone());
        Self::save_filemsg_in_chat(
            state,
            user_account,
            &user_account.id,
            &groupid,
            &file_history,
            super::rpc_proto::MessageStatus::Sending,
        );

        // 2. body ciphertext chunks via the plain (sign-only) path
        let mut chunk_index: u32 = 0;
        for chunk in body_ct.chunks(DEF_PACKAGE_SIZE as usize) {
            let data = proto_net::ChatFileContainer {
                message: Some(proto_net::chat_file_container::Message::FileData(
                    proto_net::ChatFileData {
                        file_id,
                        start_index: chunk_index,
                        message_count,
                        data: chunk.to_vec(),
                    },
                )),
            };
            Self::send_filecontainer_to_group_plain(
                state,
                user_account,
                group,
                message_id,
                timestamp,
                data.encode_to_vec(),
            );
            chunk_index += 1;
        }

        // 3. fan out the per-file key to each remote member under their
        //    per-peer session (small, encrypted, confidential)
        let sender_bytes = user_account.id.to_bytes();
        for member in group.members.values() {
            let member_id = match PeerId::from_bytes(&member.user_id) {
                Ok(id) => id,
                Err(_) => continue,
            };
            if member_id == user_account.id {
                continue;
            }
            let envelope_bytes =
                crate::services::crypto::sessionmanager::CryptoSessionManager::create_file_key_envelope_message(
                    file_id,
                    group_id.to_vec(),
                    file_key.clone(),
                    digest.clone(),
                    sender_bytes.clone(),
                    timestamp,
                );
            let mut env_msg_id = vec![0u8; 16];
            {
                use rand::Rng;
                rand::rng().fill(&mut env_msg_id[..]);
            }
            if let Err(e) = messaging::Messaging::pack_and_send_message(
                state,
                user_account,
                &member_id,
                envelope_bytes,
                MessagingServiceType::Crypto,
                &env_msg_id,
                true,
            ) {
                log::error!("failed sending FileKeyEnvelope to {}: {}", member_id.to_base58(), e);
            }
        }

        // mark sent
        ChatStorage::udate_status(
            state,
            &user_account.id,
            message_id,
            super::rpc_proto::MessageStatus::Sent,
        );

        Ok(true)
    }

    /// Generate File id
    fn generate_file_id(group_id: &[u8], sender: &[u8], file_name: &str, size: u32) -> u64 {
        let size_bytes = size.to_be_bytes();
        let time_bytes = timestamp::Timestamp::get_timestamp().to_be_bytes();
        let mut key_bytes = Vec::with_capacity(
            group_id.len() + sender.len() + file_name.len() + size_bytes.len() + time_bytes.len(),
        );
        key_bytes.extend_from_slice(group_id);
        key_bytes.extend_from_slice(sender);
        key_bytes.extend_from_slice(file_name.as_bytes());
        key_bytes.extend_from_slice(&size_bytes);
        key_bytes.extend_from_slice(&time_bytes);
        crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO).checksum(&key_bytes)
    }

    /// Try to store the file
    ///
    /// This function will check if the file is fully downloaded,
    /// and will initiate the storage process if yes.
    fn try_store_file(
        state: &crate::QaulState,
        user_account: &UserAccount,
        user_files: UserFiles,
        file_history: FileHistory,
    ) {
        // check how many chunks have been downloaded
        let file_id_bytes = file_history.file_id.to_be_bytes();
        let count = user_files.count_file_chunks(&file_id_bytes);

        log::trace!(
            "received {} chunks of {}",
            count + 1,
            file_history.message_count
        );

        // if we downloaded all chunks, save it to file if we received the file info
        if (count + 1) as u32 == file_history.message_count {
            log::trace!("store_file");

            Self::store_file(state, user_account, user_files, file_history);
        }
    }

    /// Store a completely downloaded file
    fn store_file(state: &crate::QaulState, user_account: &UserAccount, user_files: UserFiles, file_history: FileHistory) {
        // get all chunks from data base
        let file_id_bytes = file_history.file_id.to_be_bytes();
        let iterator = user_files.get_file_chunks(&file_id_bytes);

        // create file
        let file_path = Self::create_file_path(
            state,
            user_account.id,
            file_history.file_id,
            &file_history.file_extension,
        );

        // Envelope-encrypted file: the chunks are body ciphertext. We
        // need the per-file key (delivered separately via
        // FileKeyEnvelope) to decrypt. If it hasn't arrived yet, hold —
        // process_file_key_envelope will re-trigger assembly once it
        // does. Do NOT write ciphertext to disk as if it were plaintext.
        if !file_history.body_digest.is_empty() {
            let key_entry = match user_files.get_file_key(file_history.file_id) {
                Some(k) => k,
                None => {
                    log::info!(
                        "envelope file {} fully received but key not yet present — waiting",
                        file_history.file_id
                    );
                    return;
                }
            };

            // reassemble the body ciphertext (iterator is in chunk-index
            // order because keys are file_id ‖ index big-endian)
            let mut body_ct: Vec<u8> = Vec::new();
            for result in iterator {
                match result {
                    Ok((_key, chunk)) => body_ct.extend_from_slice(&chunk),
                    Err(e) => {
                        log::error!("reading file chunk failed: {}", e);
                        return;
                    }
                }
            }

            // verify the digest binds this body to the key's envelope
            let digest = super::file_envelope::body_digest(&body_ct);
            if digest != file_history.body_digest || digest != key_entry.body_digest {
                log::error!(
                    "envelope file {} body digest mismatch — dropping",
                    file_history.file_id
                );
                return;
            }

            // decrypt the body once
            let plaintext = match super::file_envelope::decrypt_body(&key_entry.file_key, &body_ct) {
                Some(pt) => pt,
                None => {
                    log::error!("envelope file {} body decryption failed", file_history.file_id);
                    return;
                }
            };

            match File::create(&file_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&plaintext) {
                        log::error!("file storing failed {}", e);
                        return;
                    }
                }
                Err(e) => {
                    log::error!("file path error: {}", e);
                    return;
                }
            }

            ChatStorage::udate_status(
                state,
                &user_account.id,
                &file_history.message_id,
                super::rpc_proto::MessageStatus::Received,
            );
            return;
        }

        // Legacy path: chunks are plaintext, write them directly.
        // open a file in write mode
        let mut file: File;
        match File::create(&file_path) {
            Ok(my_file) => file = my_file,
            Err(e) => {
                log::error!("file path error: {}", e);
                return;
            }
        }

        // loop over all chunks and write them to the data base
        for result in iterator {
            match result {
                Ok((_key, chunk)) => {
                    // write chunk to file
                    if let Err(e) = file.write(&chunk) {
                        log::error!("file storing failed {}", e);
                    }
                }
                Err(e) => log::error!("{}", e),
            }
        }

        // set file status to received
        ChatStorage::udate_status(
            state,
            &user_account.id,
            &file_history.message_id,
            super::rpc_proto::MessageStatus::Received,
        );
    }

    /// Process an incoming `FileKeyEnvelope`: store the per-file key,
    /// then (if the body is already complete) assemble + decrypt the
    /// file. Idempotent — a duplicate envelope just overwrites the key.
    pub fn process_file_key_envelope(
        state: &crate::QaulState,
        user_account: &UserAccount,
        _sender_id: &PeerId,
        envelope: qaul_proto::qaul_net_crypto::FileKeyEnvelope,
    ) {
        let user_files = Self::get_db_ref(state, &user_account.id);
        user_files.save_file_key(&FileKey {
            file_id: envelope.file_id,
            group_id: envelope.group_id,
            file_key: envelope.file_key,
            body_digest: envelope.body_digest,
            stored_at: Timestamp::get_timestamp(),
            locally_generated: false,
        });

        // if the body (and its info) already arrived, finish now
        if let Some(file_history) = user_files.get_filehistory(envelope.file_id) {
            Self::try_store_file(state, user_account, user_files, file_history);
        } else {
            log::trace!(
                "stored file key for {} — awaiting body",
                envelope.file_id
            );
        }
    }

    /// process chat file data message
    fn process_data_message(
        state: &crate::QaulState,
        user_account: &UserAccount,
        _sender_id: PeerId,
        _group_id: Vec<u8>,
        file_data: proto_net::ChatFileData,
    ) {
        // get DB references
        let user_files = Self::get_db_ref(state, &user_account.id);

        // save file chunk in DB
        user_files.save_file_chunk(file_data.file_id, file_data.start_index, file_data.data);

        // check if we already received the file info
        match user_files.get_filehistory(file_data.file_id) {
            Some(file_history) => {
                // create file once everything has been received
                Self::try_store_file(state, user_account, user_files, file_history);
            }
            None => {
                log::warn!("haven't received file info message yet");
                // TODO: create stub file history file
                /*
                // create a file history stub if nothing was found
                let file_history = FileHistory {
                    group_id: todo!(),
                    sender_id: todo!(),
                    file_id: todo!(),
                    message_id: todo!(),
                    start_index: todo!(),
                    message_count: todo!(),
                    chunk_size: todo!(),
                    file_state: todo!(),
                    reception_tracking: todo!(),
                    file_name: todo!(),
                    file_description: todo!(),
                    file_extension: todo!(),
                    file_size: todo!(),
                    sent_at: todo!(),
                    received_at: todo!(),
                };

                user_files.save_filehistory(file_id, file_history);
                 */
            }
        }
    }

    /// process chat file info message
    fn process_info_message(
        state: &crate::QaulState,
        user_account: &UserAccount,
        sender_id: PeerId,
        group_id: Vec<u8>,
        message_id: Vec<u8>,
        sent_at: u64,
        file_info: proto_net::ChatFileInfo,
    ) {
        // get db
        let user_files = Self::get_db_ref(state, &user_account.id);

        // check if it already exists in DB
        let file_history;
        if let Some(my_file_history) = user_files.get_filehistory(file_info.file_id) {
            file_history = my_file_history;

            // update fields
        } else {
            file_history = Self::file_history_from_info_message(
                &sender_id,
                &group_id,
                &message_id,
                sent_at,
                &file_info,
            );
        }

        // save to file history
        user_files.save_filehistory(file_info.file_id, file_history.clone());

        // create group id
        let groupid;
        match GroupId::from_bytes(&group_id) {
            Ok(result) => groupid = result,
            Err(e) => {
                log::error!("{}", e);
                return;
            }
        }

        // save message to chat
        Self::save_filemsg_in_chat(
            state,
            user_account,
            &sender_id,
            &groupid,
            &file_history,
            super::rpc_proto::MessageStatus::Receiving,
        );

        // create file once everything has been received
        Self::try_store_file(state, user_account, user_files, file_history);
    }

    /// process chat file container message from network
    pub fn process_net_chatfilecontainer(
        state: &crate::QaulState,
        sender_id: PeerId,
        user_account: UserAccount,
        group_id: Vec<u8>,
        message_id: Vec<u8>,
        sent_at: u64,
        data: &[u8],
    ) {
        // decode protobuf file message container
        match proto_net::ChatFileContainer::decode(data) {
            Ok(messaging) => match messaging.message {
                Some(proto_net::chat_file_container::Message::FileInfo(file_info)) => {
                    Self::process_info_message(
                        state,
                        &user_account,
                        sender_id,
                        group_id,
                        message_id,
                        sent_at,
                        file_info,
                    );
                }
                Some(proto_net::chat_file_container::Message::FileData(file_data)) => {
                    Self::process_data_message(state, &user_account, sender_id, group_id, file_data);
                }
                None => {
                    log::error!(
                        "file share message from {} was empty",
                        sender_id.to_base58()
                    )
                }
            },
            Err(e) => {
                log::error!(
                    "Error decoding ChatFile Message from {} to {}: {}",
                    sender_id.to_base58(),
                    user_account.id.to_base58(),
                    e
                );
            }
        }
    }

    /// Process incoming RPC request messages for file sharing module
    pub async fn rpc(state: &crate::QaulState, data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        let account_id = match PeerId::from_bytes(&user_id) {
            Ok(id) => id,
            Err(e) => {
                log::error!("Error parsing user id: {:?}", e);
                return;
            }
        };

        match proto_rpc::ChatFile::decode(&data[..]) {
            Ok(chatfile) => {
                match chatfile.message {
                    Some(proto_rpc::chat_file::Message::SendFileRequest(send_req)) => {
                        let user_account = match UserAccounts::get_by_id(state, account_id) {
                            Some(account) => account,
                            None => {
                                log::error!("user account not found for file send");
                                return;
                            }
                        };

                        if let Err(e) = Self::send(
                            state,
                            &user_account,
                            &send_req.group_id,
                            send_req.path_name,
                            send_req.description,
                        ) {
                            log::error!("file rpc send file failed {}", e.to_string());
                        }
                    }
                    Some(proto_rpc::chat_file::Message::FileHistory(history_req)) => {
                        log::trace!("lib->file->history");

                        // get user account
                        let user_account;
                        match UserAccounts::get_by_id(state,account_id) {
                            Some(account) => user_account = account,
                            None => {
                                log::error!("user account not found");
                                return;
                            }
                        }

                        let list = Self::file_history(state, &user_account, &history_req);

                        let mut histories = Vec::with_capacity(list.len());
                        for entry in &list {
                            histories.push(Self::rpc_history_entry_from_file_history(entry));
                        }

                        // pack message
                        let proto_message = proto_rpc::ChatFile {
                            message: Some(proto_rpc::chat_file::Message::FileHistoryResponse(
                                proto_rpc::FileHistoryResponse {
                                    offset: history_req.offset,
                                    limit: history_req.limit,
                                    total: histories.len() as u64,
                                    histories,
                                },
                            )),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            state,
                            buf,
                            crate::rpc::proto::Modules::Chatfile.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    _ => {
                        log::error!("Unhandled Protobuf File Message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}

#[cfg(test)]
mod file_key_store_tests {
    use super::{FileKey, UserFiles};

    /// Build an in-memory `UserFiles` backed by temporary sled trees.
    fn test_user_files() -> UserFiles {
        let db = sled::Config::new().temporary(true).open().unwrap();
        UserFiles {
            histories: db.open_tree("chat_file").unwrap(),
            file_chunks: db.open_tree("file_chunks").unwrap(),
            file_keys: db.open_tree("file_keys").unwrap(),
        }
    }

    fn key(file_id: u64, stored_at: u64) -> FileKey {
        FileKey {
            file_id,
            group_id: vec![1, 2, 3],
            file_key: vec![7u8; 32],
            body_digest: vec![9u8; 32],
            stored_at,
            locally_generated: false,
        }
    }

    #[test]
    fn save_get_delete_roundtrip() {
        let uf = test_user_files();
        assert!(uf.get_file_key(42).is_none());
        uf.save_file_key(&key(42, 1000));
        let got = uf.get_file_key(42).expect("stored key");
        assert_eq!(got.file_id, 42);
        assert_eq!(got.file_key, vec![7u8; 32]);
        assert_eq!(got.body_digest, vec![9u8; 32]);
        uf.delete_file_key(42);
        assert!(uf.get_file_key(42).is_none());
    }

    #[test]
    fn prune_drops_only_expired() {
        let uf = test_user_files();
        uf.save_file_key(&key(1, 1_000)); // old
        uf.save_file_key(&key(2, 9_000)); // fresh
        // now = 10_000, ttl = 5_000 → entry 1 (age 9000) expired,
        // entry 2 (age 1000) kept.
        uf.prune_expired_file_keys(10_000, 5_000);
        assert!(uf.get_file_key(1).is_none(), "expired key pruned");
        assert!(uf.get_file_key(2).is_some(), "fresh key kept");
    }
}
