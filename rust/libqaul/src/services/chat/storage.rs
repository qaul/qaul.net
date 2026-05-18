// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Conversations Storage Functions
//!
//! Saves and retrieves the chat conversations
//! and their overview from the data base.

use libp2p::PeerId;
use sled;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::rpc_proto;
use crate::services::group::{group_id::GroupId, GroupStorage};
use crate::storage::database::DataBase;
use crate::utilities::timestamp::Timestamp;
use prost::Message;

/// chat DB references per user account
#[derive(Clone)]
pub struct ChatAccountDb {
    /// messages sled data base tree
    ///
    /// value: Vec<u8> bincode of rpc_proto::ChatMessage
    pub messages: sled::Tree,
    /// message id => db key
    ///
    /// value: Vec<u8> of db key
    pub message_ids: sled::Tree,
}

/// qaul Chat Conversation Storage
pub struct ChatStorage {
    /// data base tree references accessible
    /// by user account
    db_ref: BTreeMap<Vec<u8>, ChatAccountDb>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum FlushMode {
    Immediate,
    Deferred,
}

/// Instance-based chat storage state.
/// Replaces the global CHAT static for multi-instance use.
pub struct ChatState {
    /// Chat storage inner state.
    pub inner: RwLock<ChatStorage>,
    /// Per-account full-text search indexes.
    pub search: super::search::ChatSearchState,
}

impl ChatState {
    /// Create a new empty ChatState.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(ChatStorage {
                db_ref: BTreeMap::new(),
            }),
            search: super::search::ChatSearchState::new(),
        }
    }

    /// Get DB refs for user account (instance method).
    /// Takes an explicit `sled::Db` instead of calling `DataBase::get_user_db()`.
    pub fn get_db_ref(&self, account_id: PeerId, db: &sled::Db) -> ChatAccountDb {
        {
            let chat = self.inner.read().unwrap();
            if let Some(chat_user) = chat.db_ref.get(&account_id.to_bytes()) {
                return ChatAccountDb {
                    messages: chat_user.messages.clone(),
                    message_ids: chat_user.message_ids.clone(),
                };
            }
        }

        self.create_chatuser(account_id, db)
    }

    /// Create user data when it does not exist (instance method).
    fn create_chatuser(&self, account_id: PeerId, db: &sled::Db) -> ChatAccountDb {
        let messages: sled::Tree = db.open_tree("chat_messages").unwrap();
        let message_ids: sled::Tree = db.open_tree("chat_message_ids").unwrap();

        let chat_user = ChatAccountDb {
            messages,
            message_ids,
        };

        let mut chat = self.inner.write().unwrap();
        chat.db_ref.insert(account_id.to_bytes(), chat_user.clone());

        chat_user
    }
}

impl ChatStorage {
    /// initialize chat storage
    pub fn init() {
        // State is already created by QaulState; nothing to do here.
    }

    /// Flush all chat-related trees for an account.
    pub fn flush_account(state: &crate::QaulState, account_id: &PeerId) {
        let db_ref = Self::get_db_ref(state, account_id.to_owned());
        Self::maybe_flush_tree(
            &db_ref.messages,
            FlushMode::Immediate,
            "Error chat messages flush",
        );
        Self::maybe_flush_tree(
            &db_ref.message_ids,
            FlushMode::Immediate,
            "Error chat message_ids flush",
        );
    }

    fn maybe_flush_tree(tree: &sled::Tree, flush_mode: FlushMode, error_context: &str) {
        if matches!(flush_mode, FlushMode::Deferred) {
            return;
        }

        if let Err(e) = tree.flush() {
            log::error!("{}: {}", error_context, e);
        }
    }

    fn save_chat_message_record(
        db_ref: &ChatAccountDb,
        db_key: &[u8],
        chat_message: &rpc_proto::ChatMessage,
        flush_mode: FlushMode,
    ) {
        let chat_message_bytes = bincode::serialize(chat_message).unwrap();
        if let Err(e) = db_ref.messages.insert(db_key, chat_message_bytes) {
            log::error!("Error saving chat message to data base: {}", e);
            return;
        }

        Self::maybe_flush_tree(&db_ref.messages, flush_mode, "Error chat messages flush");
    }

    fn save_message_id_mapping(
        db_ref: &ChatAccountDb,
        message_id: &[u8],
        db_key: &[u8],
        flush_mode: FlushMode,
    ) {
        if let Err(e) = db_ref.message_ids.insert(message_id, db_key) {
            log::error!("Error saving chat messageid to data base: {}", e);
            return;
        }

        Self::maybe_flush_tree(
            &db_ref.message_ids,
            flush_mode,
            "Error chat message_ids flush",
        );
    }

    fn mutate_chat_message_by_id(
        db_ref: &ChatAccountDb,
        message_id: &[u8],
        flush_mode: FlushMode,
        mutate: impl FnOnce(&mut rpc_proto::ChatMessage),
    ) {
        let Some(key) = db_ref.message_ids.get(message_id).unwrap() else {
            return;
        };
        let Some(chat_msg_fromdb) = db_ref.messages.get(&key).unwrap() else {
            return;
        };

        let mut chat_message: rpc_proto::ChatMessage =
            bincode::deserialize(&chat_msg_fromdb).unwrap();
        mutate(&mut chat_message);

        Self::save_chat_message_record(db_ref, &key, &chat_message, flush_mode);
    }

    /// check if messages exists
    #[allow(dead_code)]
    pub fn messages_exist(state: &crate::QaulState, user_id: &PeerId, message_ids: &Vec<Vec<u8>>) -> bool {
        // get data base of user account
        let db_ref = Self::get_db_ref(state, user_id.clone());
        for id in message_ids {
            if !db_ref.message_ids.contains_key(id).unwrap() {
                return false;
            }
        }
        true
    }

    /// remove messages by ids
    #[allow(dead_code)]
    pub fn remove_messages(state: &crate::QaulState, user_id: &PeerId, message_ids: &Vec<Vec<u8>>) {
        let db_ref = Self::get_db_ref(state, user_id.clone());
        for id in message_ids {
            match db_ref.message_ids.get(id) {
                Ok(opt_key) => {
                    if let Some(db_key) = opt_key {
                        if let Err(_e) = db_ref.messages.remove(&db_key) {
                            log::error!("remove message error!");
                        }
                    }
                    if let Err(_e) = db_ref.message_ids.remove(id) {
                        log::error!("remove message id error!");
                    }
                }
                _ => {}
            }
        }

        Self::maybe_flush_tree(
            &db_ref.messages,
            FlushMode::Immediate,
            "message storing error",
        );
        Self::maybe_flush_tree(
            &db_ref.message_ids,
            FlushMode::Immediate,
            "message ids storing error",
        );
    }

    /// Save a Chat Message
    ///
    /// This general function saves a chat message to data base,
    /// writes it into the chat overview.
    ///
    /// The message ID is optional.
    /// If there is a chat message ID set, it also references it in the
    /// chat id data base.
    pub fn save_message(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &GroupId,
        sender_id: &PeerId,
        message_id: &[u8],
        sent_at: u64,
        content: super::rpc_proto::ChatContentMessage,
        status: rpc_proto::MessageStatus,
    ) {
        Self::save_message_with_mode(
            state,
            account_id,
            group_id,
            sender_id,
            message_id,
            sent_at,
            content,
            status,
            FlushMode::Immediate,
        );
    }

    /// Save a chat message without flushing the chat trees.
    ///
    /// Useful when multiple writes are performed in sequence.
    #[allow(dead_code)]
    pub fn save_message_deferred(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &GroupId,
        sender_id: &PeerId,
        message_id: &[u8],
        sent_at: u64,
        content: super::rpc_proto::ChatContentMessage,
        status: rpc_proto::MessageStatus,
    ) {
        Self::save_message_with_mode(
            state,
            account_id,
            group_id,
            sender_id,
            message_id,
            sent_at,
            content,
            status,
            FlushMode::Deferred,
        );
    }

    fn save_message_with_mode(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &GroupId,
        sender_id: &PeerId,
        message_id: &[u8],
        sent_at: u64,
        content: super::rpc_proto::ChatContentMessage,
        status: rpc_proto::MessageStatus,
        flush_mode: FlushMode,
    ) {
        log::trace!("chat save_message");

        // get data base of user account
        let db_ref = Self::get_db_ref(state, account_id.clone());

        // check if message_id already exists
        // this protects the double saving of incoming messages
        if !message_id.is_empty() {
            if db_ref.message_ids.contains_key(message_id).unwrap() {
                log::warn!("chat message already exists");
                return;
            }
        }

        let group_id_bytes = group_id.to_bytes();
        let content_bytes = content.encode_to_vec();

        // create received at timestamp
        let received_at = Timestamp::get_timestamp();

        // update last message
        match flush_mode {
            FlushMode::Immediate => GroupStorage::group_update_last_chat_message(
                state,
                account_id.to_owned(),
                group_id_bytes.clone(),
                sender_id.to_owned(),
                content_bytes.clone(),
                received_at,
            ),
            FlushMode::Deferred => GroupStorage::group_update_last_chat_message_deferred(
                state,
                account_id.to_owned(),
                group_id_bytes.clone(),
                sender_id.to_owned(),
                content_bytes.clone(),
                received_at,
            ),
        }

        // get next index
        let index = Self::get_next_db_index(db_ref.clone(), &group_id_bytes);

        // create data base key
        let db_key = Self::get_db_key_from_vec(&group_id_bytes, index);

        // create chat message
        let chat_message = rpc_proto::ChatMessage {
            index,
            sender_id: sender_id.to_bytes(),
            message_id: message_id.to_vec(),
            status: status as i32,
            message_reception_confirmed: Vec::new(),
            group_id: group_id_bytes,
            sent_at,
            received_at,
            content: content_bytes,
        };

        Self::save_chat_message_record(&db_ref, &db_key, &chat_message, flush_mode);

        // save message id in data base
        if !message_id.is_empty() {
            Self::save_message_id_mapping(&db_ref, message_id, &db_key, flush_mode);
        }

        // index the message for full-text search (text content only)
        if let Some(searchable) =
            super::search::ChatSearchableMessage::from_chat_message(&chat_message)
        {
            super::search::ChatSearch::index_message(state, account_id, &searchable);
        }
    }

    /// updating chat message status as confirmed
    pub fn update_confirmation(
        state: &crate::QaulState,
        account_id: PeerId,
        receiver_id: PeerId,
        message_id: &[u8],
        received_at: u64,
    ) {
        Self::update_confirmation_with_mode(
            state,
            account_id,
            receiver_id,
            message_id,
            received_at,
            FlushMode::Immediate,
        );
    }

    /// Update confirmation without flushing the messages tree.
    #[allow(dead_code)]
    pub fn update_confirmation_deferred(
        state: &crate::QaulState,
        account_id: PeerId,
        receiver_id: PeerId,
        message_id: &[u8],
        received_at: u64,
    ) {
        Self::update_confirmation_with_mode(
            state,
            account_id,
            receiver_id,
            message_id,
            received_at,
            FlushMode::Deferred,
        );
    }

    fn update_confirmation_with_mode(
        state: &crate::QaulState,
        account_id: PeerId,
        receiver_id: PeerId,
        message_id: &[u8],
        received_at: u64,
        flush_mode: FlushMode,
    ) {
        let db_ref = Self::get_db_ref(state, account_id);
        let receiver_id_bytes = receiver_id.to_bytes();

        Self::mutate_chat_message_by_id(&db_ref, message_id, flush_mode, |chat_msg| {
            chat_msg.status = rpc_proto::MessageStatus::Confirmed as i32;
            chat_msg.received_at = received_at;

            // TODO: check if receiver already exists
            chat_msg
                .message_reception_confirmed
                .push(rpc_proto::MessageReceptionConfirmed {
                    user_id: receiver_id_bytes,
                    confirmed_at: received_at,
                });

            // TODO: check if it was received by everyone
            //       set received_by_all flag if yes
        });
    }

    /// update message status
    pub fn udate_status(
        state: &crate::QaulState,
        account_id: &PeerId,
        message_id: &[u8],
        status: super::rpc_proto::MessageStatus,
    ) {
        Self::udate_status_with_mode(state, account_id, message_id, status, FlushMode::Immediate);
    }

    /// Update message status without flushing the messages tree.
    #[allow(dead_code)]
    pub fn udate_status_deferred(
        state: &crate::QaulState,
        account_id: &PeerId,
        message_id: &[u8],
        status: super::rpc_proto::MessageStatus,
    ) {
        Self::udate_status_with_mode(state, account_id, message_id, status, FlushMode::Deferred);
    }

    fn udate_status_with_mode(
        state: &crate::QaulState,
        account_id: &PeerId,
        message_id: &[u8],
        status: super::rpc_proto::MessageStatus,
        flush_mode: FlushMode,
    ) {
        let db_ref = Self::get_db_ref(state, account_id.to_owned());
        Self::mutate_chat_message_by_id(&db_ref, message_id, flush_mode, |chat_msg| {
            chat_msg.status = status as i32;
        });
    }

    /// Look up a single message by its message_id.
    ///
    /// Returns `None` if the message_id is not found.
    pub fn get_message_by_id(
        state: &crate::QaulState,
        account_id: PeerId,
        message_id: &[u8],
    ) -> Option<rpc_proto::ChatMessage> {
        let db_ref = Self::get_db_ref(state, account_id);
        let db_key = db_ref.message_ids.get(message_id).ok()??;
        let message_bytes = db_ref.messages.get(&db_key).ok()??;
        bincode::deserialize(&message_bytes).ok()
    }

    /// Get chat messages of a specific conversation from data base
    pub fn get_messages(state: &crate::QaulState, account_id: PeerId, group_id: Vec<u8>) -> rpc_proto::ChatConversationList {
        // create empty messages list
        let mut message_list: Vec<rpc_proto::ChatMessage> = Vec::new();

        if let Ok(group_id_typed) = GroupId::from_bytes(&group_id) {
            // get database references for this user account
            let db_ref = Self::get_db_ref(state, account_id);

            // create message keys
            let (first_key, last_key) = Self::get_db_key_range(group_id_typed.as_slice());

            // iterate over all values in chat_messages db
            for res in db_ref
                .messages
                .range(first_key.as_slice()..last_key.as_slice())
            {
                match res {
                    Ok((_id, message_bytes)) => {
                        let message: rpc_proto::ChatMessage =
                            bincode::deserialize(&message_bytes).unwrap();
                        message_list.push(message);
                    }
                    Err(e) => {
                        log::error!("get_messages error: {}", e);
                    }
                }
            }

            // clear unread messages from group
            GroupStorage::group_clear_unread(state, account_id, group_id.clone());
        }

        rpc_proto::ChatConversationList {
            group_id,
            message_list,
        }
    }

    /// get DB key range for a group id
    ///
    /// returns a key tuple, which can be used to
    /// retrieve all messages for a user ID from the DB:
    ///
    /// (first_key, last_key)
    fn get_db_key_range(group_id: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let first_key = Self::get_db_key_from_vec(group_id, 0);
        let last_key = Self::get_db_key_from_vec(group_id, 0xFFFFFFFFFFFFFFFF); // = 4294967295
        (first_key, last_key)
    }

    /// create DB key from group id
    fn get_db_key_from_vec(group_id: &[u8], index: u64) -> Vec<u8> {
        let mut index_bytes = index.to_be_bytes().to_vec();
        let mut key_bytes = group_id.to_vec();
        key_bytes.append(&mut index_bytes);
        key_bytes
    }

    /// get nex db_index key
    fn get_next_db_index(db_ref: ChatAccountDb, group_id: &[u8]) -> u64 {
        // get biggest existing index
        let search_key = Self::get_db_key_from_vec(group_id, u64::MAX);
        let result = db_ref.messages.get_lt(search_key);
        if let Ok(Some((_key, value))) = result {
            // check if result is really of the same group
            let chat_message: rpc_proto::ChatMessage = bincode::deserialize(&value).unwrap();
            if group_id == chat_message.group_id.as_slice() {
                return chat_message.index + 1;
            }
        }

        return 0;
    }

    /// get user account data base tree references
    fn get_db_ref(state: &crate::QaulState, account_id: PeerId) -> ChatAccountDb {
        // check if user account data exists
        {
            // get chat state
            let chat = state.services.chat.inner.read().unwrap();

            // check if user account ID is in map
            if let Some(chat_user) = chat.db_ref.get(&account_id.to_bytes()) {
                return ChatAccountDb {
                    messages: chat_user.messages.clone(),
                    message_ids: chat_user.message_ids.clone(),
                };
            }
        }

        // create user data if it does not exist
        let chat_user = Self::create_chatuser(state, account_id);

        // return chat_user structure
        ChatAccountDb {
            messages: chat_user.messages.clone(),
            message_ids: chat_user.message_ids.clone(),
        }
    }

    /// create user data when it does not exist
    fn create_chatuser(state: &crate::QaulState, account_id: PeerId) -> ChatAccountDb {
        // get user data base
        let db = DataBase::get_user_db(state, account_id);

        // open trees
        let messages: sled::Tree = db.open_tree("chat_messages").unwrap();
        let message_ids: sled::Tree = db.open_tree("chat_message_ids").unwrap();

        let chat_user = ChatAccountDb {
            messages,
            message_ids,
        };

        // get chat state for writing
        {
            let mut chat = state.services.chat.inner.write().unwrap();
            chat.db_ref.insert(account_id.to_bytes(), chat_user.clone());
        }

        // Initialize search index for this account.
        // If the index is fresh, batch-index all existing messages.
        let is_fresh = super::search::ChatSearch::get_or_create(state, &account_id);
        if is_fresh {
            Self::batch_index_existing_messages(state, &account_id, &chat_user);
        }

        // return structure
        chat_user
    }

    /// Batch-index all existing chat messages into the search index.
    ///
    /// Called once when a fresh search index is created for an account
    /// (first upgrade to a version with search, or after index deletion).
    fn batch_index_existing_messages(
        state: &crate::QaulState,
        account_id: &PeerId,
        db_ref: &ChatAccountDb,
    ) {
        let mut items: Vec<super::search::ChatSearchableMessage> = Vec::new();

        for res in db_ref.messages.iter() {
            match res {
                Ok((_key, value)) => {
                    if let Ok(msg) = bincode::deserialize::<rpc_proto::ChatMessage>(&value) {
                        if let Some(searchable) =
                            super::search::ChatSearchableMessage::from_chat_message(&msg)
                        {
                            items.push(searchable);
                        }
                    }
                }
                Err(e) => log::error!("batch index read error: {}", e),
            }
        }

        if !items.is_empty() {
            log::info!(
                "batch indexing {} existing chat messages for search",
                items.len()
            );
            super::search::ChatSearch::index_message_batch(state, account_id, &items);
        }
    }
}
