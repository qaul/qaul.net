// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Search
//!
//! Manages per-account full-text search indexes for chat groups (frontend: "chat rooms").
//! State is owned by `QaulState.services.groups.search`.
//!
//! Groups are the third searchable entity after chat messages and users, and the first to
//! use the ranked engine config: results are ordered by text relevance, with the group's
//! `last_message_at` breaking ties so more recently active rooms surface first.
//!
//! ## The direct-chat naming nuance
//!
//! Group chats carry their own `name`. Direct chats do not — a direct chat is indexed under
//! the *partner's* username (the member whose id is not the account's), resolved through the
//! router's users table. Partner names are effectively immutable after discovery, so this
//! does not go stale in practice; if a partner is not yet known, the room is indexed with
//! empty content and self-heals on the next message (see [`GroupSearchable::from_group`]).

use libp2p::PeerId;

use super::{Group, GroupStorage};
use crate::router::users::Users;
use crate::search::{Search, SearchConfig, Searchable};
use crate::storage::Storage;
use crate::utilities::qaul_id::QaulId;
use crate::QaulState;

/// Stateless namespace for group-search operations.
///
/// All methods take `&QaulState` and access the per-instance search state at
/// `state.services.groups.search`.
pub struct GroupSearch {}

/// A group wrapped for indexing via the [`Searchable`] trait.
///
/// `content` is the group name for group chats, or the partner's username for direct chats.
/// `last_message_at` feeds the ranked engine's recency tiebreak.
pub struct GroupSearchable {
    id: String,
    content: String,
    last_message_at: u64,
}

impl Searchable for GroupSearchable {
    fn id(&self) -> &str {
        &self.id
    }
    fn content(&self) -> &str {
        &self.content
    }
    fn ranking_key(&self) -> Option<u64> {
        Some(self.last_message_at)
    }
}

impl GroupSearchable {
    /// Build a searchable record from a stored [`Group`].
    ///
    /// For direct chats the indexed text is the partner's username; if the partner is not
    /// yet in the users table the content is left empty (best-effort — it self-heals once a
    /// message arrives and re-indexing runs).
    pub fn from_group(state: &QaulState, account_id: &PeerId, group: &Group) -> Self {
        let content = if group.is_direct_chat {
            Self::resolve_partner_name(state, account_id, group).unwrap_or_default()
        } else {
            group.name.clone()
        };

        Self {
            id: bs58::encode(&group.id).into_string(),
            content,
            last_message_at: group.last_message_at,
        }
    }

    /// Find the direct-chat partner's display name via the router's users table.
    fn resolve_partner_name(
        state: &QaulState,
        account_id: &PeerId,
        group: &Group,
    ) -> Option<String> {
        let account_bytes = account_id.to_bytes();
        // Members are keyed by full user-id bytes; the partner is the one that is not us.
        let partner_id = group
            .members
            .keys()
            .find(|member_id| **member_id != account_bytes)?;
        let q8id = QaulId::bytes_as_q8id(partner_id);

        let router = state.router.read().unwrap();
        Users::get_name_by_q8id(&router, q8id)
    }
}

impl GroupSearch {
    /// Lazily opens (or creates) the ranked search index for the given account.
    ///
    /// Returns `true` if the index was freshly created, signalling that existing groups
    /// should be batch-indexed.
    pub fn get_or_create(state: &QaulState, account_id: &PeerId) -> bool {
        let key = account_id.to_bytes();

        // Check if already open.
        {
            let indexes = state.services.groups.search.read().unwrap();
            if indexes.contains_key(&key) {
                return false;
            }
        }

        // Build the index path: {account_path}/search/groups/
        let account_path = Storage::get_account_path(state, account_id.to_owned());
        let index_path = account_path.join("search").join("groups");
        let Some(path_str) = index_path.to_str() else {
            log::error!("failed to create group search index: invalid path");
            return false;
        };

        match Search::new(path_str, SearchConfig::ranked()) {
            Ok(search) => {
                let is_fresh = search.is_fresh();

                let mut indexes = state.services.groups.search.write().unwrap();
                // Re-check inside the write lock to avoid clobbering a concurrent insert.
                if indexes.contains_key(&key) {
                    return false;
                }
                indexes.insert(key, search);

                is_fresh
            }
            Err(e) => {
                log::error!("failed to create group search index: {}", e);
                false
            }
        }
    }

    /// Stage a single group for indexing without committing.
    ///
    /// Used by the save hook: a `Deferred` save stages and lets a later flush commit, while
    /// an `Immediate` save follows this with [`GroupSearch::commit`].
    pub fn stage(state: &QaulState, account_id: &PeerId, item: &GroupSearchable) {
        let key = account_id.to_bytes();
        Self::get_or_create(state, account_id);

        let mut indexes = state.services.groups.search.write().unwrap();
        if let Some(search) = indexes.get_mut(&key) {
            if let Err(e) = search.index(item) {
                log::error!("group search index error: {}", e);
            }
        }
    }

    /// Commit any staged changes for the account, making them visible to searches.
    pub fn commit(state: &QaulState, account_id: &PeerId) {
        let key = account_id.to_bytes();

        let mut indexes = state.services.groups.search.write().unwrap();
        if let Some(search) = indexes.get_mut(&key) {
            if let Err(e) = search.commit() {
                log::error!("group search commit error: {}", e);
            }
        }
    }

    /// Stage a batch of groups and commit once. Used for the fresh-index backfill.
    pub fn index_group_batch(state: &QaulState, account_id: &PeerId, items: &[GroupSearchable]) {
        let key = account_id.to_bytes();
        Self::get_or_create(state, account_id);

        let mut indexes = state.services.groups.search.write().unwrap();
        if let Some(search) = indexes.get_mut(&key) {
            if let Err(e) = search.index_many(items) {
                log::error!("group search batch index error: {}", e);
                return;
            }
            if let Err(e) = search.commit() {
                log::error!("group search batch commit error: {}", e);
            }
        }
    }

    /// Search an account's groups, ranked by relevance then recency.
    ///
    /// Returns reconstructed [`GroupInfo`](super::proto_rpc::GroupInfo) records **in ranked
    /// order**. Pagination is the caller's responsibility and must not re-sort, or the
    /// relevance/recency ordering is lost.
    ///
    /// `limit` bounds how many matches tantivy returns; callers pass the live group count
    /// to fetch all matches and paginate afterwards.
    pub fn search(
        state: &QaulState,
        account_id: &PeerId,
        query: &str,
        limit: Option<usize>,
    ) -> Vec<super::proto_rpc::GroupInfo> {
        let key = account_id.to_bytes();
        Self::get_or_create(state, account_id);

        let indexes = state.services.groups.search.read().unwrap();
        let search = match indexes.get(&key) {
            Some(s) => s,
            None => return vec![],
        };

        let account_id_owned = account_id.to_owned();
        match search.search(query, limit, |group_id_b58| {
            Self::reconstruct_result(state, &account_id_owned, group_id_b58)
        }) {
            Ok(results) => results,
            Err(e) => {
                log::error!("group search error: {}", e);
                vec![]
            }
        }
    }

    /// Reconstruct a `GroupInfo` from a base58-encoded group id.
    ///
    /// Returns `None` if the group no longer exists in storage (the index may briefly
    /// reference a removed group); such results are silently dropped.
    fn reconstruct_result(
        state: &QaulState,
        account_id: &PeerId,
        group_id_b58: &str,
    ) -> Option<super::proto_rpc::GroupInfo> {
        let group_id_bytes = bs58::decode(group_id_b58).into_vec().ok()?;
        let group = GroupStorage::get_group(state, account_id.to_owned(), &group_id_bytes)?;
        Some(super::manage::group_to_info(group))
    }
}
