// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Search
//!
//! Manages per-account full-text search indexes for chat messages.
//! State is owned by `QaulState.services.chat.search`.

use libp2p::PeerId;
use prost::Message;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::rpc_proto;
use super::ChatStorage;
use crate::search::{Search, Searchable};
use crate::storage::Storage;
use crate::QaulState;

/// Per-instance chat search state.
///
/// Holds one [`Search`] index per user account, lazily opened on first use.
pub struct ChatSearchState {
    /// search indexes accessible by user account
    pub indexes: RwLock<BTreeMap<Vec<u8>, Search>>,
}

impl ChatSearchState {
    pub fn new() -> Self {
        Self {
            indexes: RwLock::new(BTreeMap::new()),
        }
    }
}

/// Stateless namespace for chat-search operations.
///
/// All methods take `&QaulState` and access the per-instance search state at
/// `state.services.chat.search`.
pub struct ChatSearch {}

/// A chat message wrapped for indexing via the [`Searchable`] trait.
///
/// Only text messages (ChatContent) produce a `ChatSearchableMessage`;
/// file transfers and group events are skipped.
pub struct ChatSearchableMessage {
    id: String,
    content: String,
}

impl Searchable for ChatSearchableMessage {
    fn id(&self) -> &str {
        &self.id
    }
    fn content(&self) -> &str {
        &self.content
    }
}

impl ChatSearchableMessage {
    /// Attempts to create a searchable message from a ChatMessage proto.
    ///
    /// Returns `None` if the message has no text content (e.g. file transfer, group event)
    /// or if the content bytes fail to decode.
    pub fn from_chat_message(msg: &rpc_proto::ChatMessage) -> Option<Self> {
        // Skip messages with empty content or empty message_id
        if msg.content.is_empty() || msg.message_id.is_empty() {
            return None;
        }

        let content_message = rpc_proto::ChatContentMessage::decode(&msg.content[..]).ok()?;

        match content_message.message? {
            rpc_proto::chat_content_message::Message::ChatContent(chat_content) => {
                if chat_content.text.is_empty() {
                    return None;
                }
                Some(Self {
                    id: bs58::encode(&msg.message_id).into_string(),
                    content: chat_content.text,
                })
            }
            // File transfers and group events are not indexed
            _ => None,
        }
    }
}

/// Search result with enough data for display without a second lookup.
pub struct ChatSearchResult {
    pub message_id: Vec<u8>,
    pub group_id: Vec<u8>,
    pub sender_id: Vec<u8>,
    pub content: String,
    pub sent_at: u64,
}

impl ChatSearch {
    /// Lazily opens (or creates) the search index for the given account.
    ///
    /// Returns `true` if the index was freshly created, signaling that
    /// existing messages should be batch-indexed.
    pub fn get_or_create(state: &QaulState, account_id: &PeerId) -> bool {
        let key = account_id.to_bytes();

        // Check if already open
        {
            let indexes = state.services.chat.search.indexes.read().unwrap();
            if indexes.contains_key(&key) {
                return false;
            }
        }

        // Build the index path: {account_path}/search/chat/
        let account_path = Storage::get_account_path(state, account_id.to_owned());
        let index_path = account_path.join("search").join("chat");
        let path_str = index_path.to_str().unwrap();

        match Search::new(path_str) {
            Ok(search) => {
                let is_fresh = search.is_fresh();

                let mut indexes = state.services.chat.search.indexes.write().unwrap();
                // Re-check inside the write lock to avoid clobbering a concurrently-inserted index.
                if indexes.contains_key(&key) {
                    return false;
                }
                indexes.insert(key, search);

                is_fresh
            }
            Err(e) => {
                log::error!("failed to create chat search index: {}", e);
                false
            }
        }
    }

    /// Index a single chat message and commit immediately.
    pub fn index_message(state: &QaulState, account_id: &PeerId, item: &impl Searchable) {
        let key = account_id.to_bytes();

        // Ensure the index exists
        Self::get_or_create(state, account_id);

        let mut indexes = state.services.chat.search.indexes.write().unwrap();
        if let Some(search) = indexes.get_mut(&key) {
            if let Err(e) = search.index(item) {
                log::error!("chat search index error: {}", e);
                return;
            }
            if let Err(e) = search.commit() {
                log::error!("chat search commit error: {}", e);
            }
        }
    }

    /// Index a batch of messages and commit once at the end.
    pub fn index_message_batch(
        state: &QaulState,
        account_id: &PeerId,
        items: &[ChatSearchableMessage],
    ) {
        let key = account_id.to_bytes();

        // Ensure the index exists
        Self::get_or_create(state, account_id);

        let mut indexes = state.services.chat.search.indexes.write().unwrap();
        if let Some(search) = indexes.get_mut(&key) {
            if let Err(e) = search.index_many(items) {
                log::error!("chat search batch index error: {}", e);
                return;
            }
            if let Err(e) = search.commit() {
                log::error!("chat search batch commit error: {}", e);
            }
        }
    }

    /// Search chat messages for the given account.
    ///
    /// Returns results with full message data for display.
    pub fn search(state: &QaulState, account_id: &PeerId, query: &str) -> Vec<ChatSearchResult> {
        let key = account_id.to_bytes();

        // Ensure the index exists
        Self::get_or_create(state, account_id);

        let indexes = state.services.chat.search.indexes.read().unwrap();
        let search = match indexes.get(&key) {
            Some(s) => s,
            None => return vec![],
        };

        // The reconstruct closure looks up the full message from ChatStorage
        // using the message_id (stored as base58 in the search index).
        let account_id_owned = account_id.to_owned();
        match search.search(query, |message_id_b58| {
            Self::reconstruct_result(state, &account_id_owned, message_id_b58)
        }) {
            Ok(results) => results,
            Err(e) => {
                log::error!("chat search error: {}", e);
                vec![]
            }
        }
    }

    /// Reconstruct a ChatSearchResult from a message_id base58 string.
    ///
    /// Looks up the raw ChatMessage in ChatStorage, decodes the content,
    /// and returns a result suitable for display.
    fn reconstruct_result(
        state: &QaulState,
        account_id: &PeerId,
        message_id_b58: &str,
    ) -> Option<ChatSearchResult> {
        // Decode the base58 message_id back to bytes
        let message_id_bytes = bs58::decode(message_id_b58).into_vec().ok()?;

        // Look up the message in ChatStorage
        let msg = ChatStorage::get_message_by_id(state, account_id.to_owned(), &message_id_bytes)?;

        // Decode the text content
        let content_message = rpc_proto::ChatContentMessage::decode(&msg.content[..]).ok()?;
        let text = match content_message.message? {
            rpc_proto::chat_content_message::Message::ChatContent(c) => c.text,
            _ => return None,
        };

        Some(ChatSearchResult {
            message_id: msg.message_id,
            group_id: msg.group_id,
            sender_id: msg.sender_id,
            content: text,
            sent_at: msg.sent_at,
        })
    }
}
