// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Storage Handling
//!
//! Saves and retrieves groups from data base.

use libp2p::PeerId;
use sled;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::{Group, GroupInvited};
use crate::storage::database::DataBase;

/// Group DB links for user account
#[derive(Clone)]
pub struct GroupAccountDb {
    /// group DB reference
    /// bincode of `Group`
    pub groups: sled::Tree,
    /// invited DB ref
    /// bincode of `GroupInvited`
    pub invited: sled::Tree,
}

/// qaul Chat Conversation Storage
pub struct GroupStorage {
    /// data base tree references accessible
    /// by user account
    db_ref: BTreeMap<Vec<u8>, GroupAccountDb>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum FlushMode {
    Immediate,
    Deferred,
}

/// Instance-based group storage state.
/// Replaces the global GROUPSTORAGE static for multi-instance use.
pub struct GroupStorageState {
    /// Group storage inner state.
    pub inner: RwLock<GroupStorage>,
}

impl GroupStorageState {
    /// Create a new empty GroupStorageState.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(GroupStorage {
                db_ref: BTreeMap::new(),
            }),
        }
    }

    /// Get DB refs for user account (instance method).
    /// Takes an explicit `sled::Db` instead of calling `DataBase::get_user_db()`.
    pub fn get_db_ref(&self, account_id: PeerId, db: &sled::Db) -> GroupAccountDb {
        // check if user account data already exists
        {
            let group_storage = self.inner.read().unwrap();
            if let Some(group_account_db) = group_storage.db_ref.get(&account_id.to_bytes()) {
                return GroupAccountDb {
                    groups: group_account_db.groups.clone(),
                    invited: group_account_db.invited.clone(),
                };
            }
        }

        // create group account db entry if it does not exist
        self.create_groupaccountdb(account_id, db)
    }

    /// Create group account db entry when it does not exist (instance method).
    fn create_groupaccountdb(&self, account_id: PeerId, db: &sled::Db) -> GroupAccountDb {
        let groups: sled::Tree = db.open_tree("groups").unwrap();
        let invited: sled::Tree = db.open_tree("invited").unwrap();

        let group_account_db = GroupAccountDb { groups, invited };

        let mut group_storage = self.inner.write().unwrap();
        group_storage
            .db_ref
            .insert(account_id.to_bytes(), group_account_db.clone());

        group_account_db
    }
}

impl GroupStorage {
    /// Initialize Group Storage
    ///
    /// No-op: the state is now owned by `QaulState` and initialized there.
    pub fn init() {
        // State already exists in QaulState.services.groups
    }

    /// get DB refs for user account
    pub fn get_db_ref(state: &crate::QaulState, account_id: PeerId) -> GroupAccountDb {
        // check if user account data exists
        {
            // get group state
            let group_storage = state.services.groups.inner.read().unwrap();

            // check if user account ID is in map
            if let Some(group_account_db) = group_storage.db_ref.get(&account_id.to_bytes()) {
                return GroupAccountDb {
                    groups: group_account_db.groups.clone(),
                    invited: group_account_db.invited.clone(),
                };
            }
        }

        // create group account db entry if it does not exist
        let group_account_db = Self::create_groupaccountdb(state, account_id);

        // return group_account_db structure
        GroupAccountDb {
            groups: group_account_db.groups.clone(),
            invited: group_account_db.invited.clone(),
        }
    }

    /// Flush all group-related trees for an account.
    pub fn flush_account(state: &crate::QaulState, account_id: &PeerId) {
        let db_ref = Self::get_db_ref(state, account_id.to_owned());
        Self::maybe_flush_tree(&db_ref.groups, FlushMode::Immediate, "Error groups flush");
        Self::maybe_flush_tree(&db_ref.invited, FlushMode::Immediate, "Error invited flush");
    }

    /// create group account db entry when it does not exist
    fn create_groupaccountdb(state: &crate::QaulState, account_id: PeerId) -> GroupAccountDb {
        // get user data base
        let db = DataBase::get_user_db(state, account_id);

        // open trees
        let groups: sled::Tree = db.open_tree("groups").unwrap();
        let invited: sled::Tree = db.open_tree("invited").unwrap();

        let group_account_db = GroupAccountDb { groups, invited };

        // get group storage for writing
        let mut group_storage = state.services.groups.inner.write().unwrap();

        // add user to state
        group_storage
            .db_ref
            .insert(account_id.to_bytes(), group_account_db.clone());

        // return structure
        group_account_db
    }

    /// get a group from data base
    pub fn get_group(state: &crate::QaulState, account_id: PeerId, group_id: &[u8]) -> Option<Group> {
        // get DB ref
        let db_ref = Self::get_db_ref(state, account_id);

        // get group
        match db_ref.groups.get(group_id) {
            Ok(Some(group_bytes)) => {
                let group: Group = bincode::deserialize(&group_bytes).unwrap();
                return Some(group);
            }
            Ok(None) => return None,
            Err(e) => log::error!("{}", e),
        }

        None
    }

    /// Load, mutate and save a group in one place.
    ///
    /// Returns `None` if the group does not exist.
    pub fn with_group_mut<R>(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &[u8],
        mutate: impl FnOnce(&mut Group) -> R,
    ) -> Option<R> {
        let mut group = Self::get_group(state, account_id.to_owned(), group_id)?;
        let result = mutate(&mut group);
        Self::save_group(state, account_id.to_owned(), group);
        Some(result)
    }

    /// Like `with_group_mut`, but skips saving when the closure returns an error.
    pub fn try_with_group_mut<R, E>(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &[u8],
        mutate: impl FnOnce(&mut Group) -> Result<R, E>,
    ) -> Result<Option<R>, E> {
        let Some(mut group) = Self::get_group(state, account_id.to_owned(), group_id) else {
            return Ok(None);
        };

        let result = mutate(&mut group)?;
        Self::save_group(state, account_id.to_owned(), group);
        Ok(Some(result))
    }

    /// Check if a group exists in the data base
    #[allow(dead_code)]
    pub fn group_exists(state: &crate::QaulState, account_id: PeerId, group_id: &[u8]) -> bool {
        // get DB ref
        let db_ref = Self::get_db_ref(state, account_id);

        // check id group exists
        match db_ref.groups.contains_key(group_id) {
            Ok(exists) => {
                return exists;
            }
            Err(e) => log::error!("{}", e),
        }

        false
    }

    /// Save a group into the data base
    ///
    /// This function overwrites an already existing group entry or
    /// creates a new one.
    pub fn save_group(state: &crate::QaulState, account_id: PeerId, group: Group) {
        Self::save_group_with_mode(state, account_id, group, FlushMode::Immediate);
    }

    /// Save a group without flushing.
    ///
    /// Useful when batching several writes in one operation.
    #[allow(dead_code)]
    pub fn save_group_deferred(state: &crate::QaulState, account_id: PeerId, group: Group) {
        Self::save_group_with_mode(state, account_id, group, FlushMode::Deferred);
    }

    fn save_group_with_mode(state: &crate::QaulState, account_id: PeerId, group: Group, flush_mode: FlushMode) {
        // get DB ref
        let db_ref = Self::get_db_ref(state, account_id);

        // save group in data base
        let group_bytes = bincode::serialize(&group).unwrap();
        if let Err(e) = db_ref.groups.insert(group.id.clone(), group_bytes) {
            log::error!("Error saving group to data base: {}", e);
        }

        Self::maybe_flush_tree(&db_ref.groups, flush_mode, "Error groups flush");
    }

    /// Update Last Chat Message sent to this Group
    pub fn group_update_last_chat_message(
        state: &crate::QaulState,
        account_id: PeerId,
        group_id: Vec<u8>,
        sender_id: PeerId,
        message: Vec<u8>,
        received_at: u64,
    ) {
        Self::group_update_last_chat_message_with_mode(
            state,
            account_id,
            group_id,
            sender_id,
            message,
            received_at,
            FlushMode::Immediate,
        );
    }

    /// Update last chat message information without flushing the group tree.
    pub fn group_update_last_chat_message_deferred(
        state: &crate::QaulState,
        account_id: PeerId,
        group_id: Vec<u8>,
        sender_id: PeerId,
        message: Vec<u8>,
        received_at: u64,
    ) {
        Self::group_update_last_chat_message_with_mode(
            state,
            account_id,
            group_id,
            sender_id,
            message,
            received_at,
            FlushMode::Deferred,
        );
    }

    fn group_update_last_chat_message_with_mode(
        state: &crate::QaulState,
        account_id: PeerId,
        group_id: Vec<u8>,
        sender_id: PeerId,
        message: Vec<u8>,
        received_at: u64,
        flush_mode: FlushMode,
    ) {
        log::debug!("group_update_last_chat_message");

        if let Some(mut group) = Self::get_group(state, account_id, &group_id) {
            // update values
            group.last_message_sender_id = sender_id.to_bytes();
            group.last_message_at = received_at;
            group.last_message_data = message;

            // check if it is us who is sending the message
            if sender_id != account_id {
                group.unread_messages += 1;
            }
            Self::save_group_with_mode(state, account_id, group, flush_mode);
        } else {
            log::error!("group_update_last_chat group not found");
        }
    }

    /// Clear Unread Message Counter
    pub fn group_clear_unread(state: &crate::QaulState, account_id: PeerId, group_id: Vec<u8>) {
        log::debug!("group_clear_unread");

        if Self::with_group_mut(state, &account_id, &group_id, |group| {
            group.unread_messages = 0;
        })
        .is_none()
        {
            log::error!("group_clear_unread group not found");
        }
    }

    /// get invite
    pub fn get_invite(state: &crate::QaulState, account_id: PeerId, group_id: &[u8]) -> Option<GroupInvited> {
        // get DB ref
        let db_ref = Self::get_db_ref(state, account_id);

        // get invite
        match db_ref.invited.get(group_id) {
            Ok(Some(invite_bytes)) => {
                let invite: GroupInvited = bincode::deserialize(&invite_bytes).unwrap();
                return Some(invite);
            }
            Ok(None) => return None,
            Err(e) => log::error!("{}", e),
        }

        None
    }

    /// Save a group invite into the data base
    ///
    /// This function overwrites an already existing invite entry for
    /// the same group or creates a new one.
    pub fn save_invite(state: &crate::QaulState, account_id: PeerId, invite: GroupInvited) {
        Self::save_invite_with_mode(state, account_id, invite, FlushMode::Immediate);
    }

    /// Save a group invite without flushing.
    #[allow(dead_code)]
    pub fn save_invite_deferred(state: &crate::QaulState, account_id: PeerId, invite: GroupInvited) {
        Self::save_invite_with_mode(state, account_id, invite, FlushMode::Deferred);
    }

    fn save_invite_with_mode(state: &crate::QaulState, account_id: PeerId, invite: GroupInvited, flush_mode: FlushMode) {
        // get DB ref
        let db_ref = Self::get_db_ref(state, account_id);

        // save group invite in data base
        let invite_bytes = bincode::serialize(&invite).unwrap();
        if let Err(e) = db_ref.invited.insert(invite.group.id.clone(), invite_bytes) {
            log::error!("Error saving group invite to data base: {}", e);
        }

        Self::maybe_flush_tree(&db_ref.invited, flush_mode, "Error invited flush");
    }

    /// Remove a group invite from the data base
    #[allow(dead_code)]
    pub fn remove_invite(state: &crate::QaulState, account_id: PeerId, group_id: &[u8]) {
        Self::remove_invite_with_mode(state, account_id, group_id, FlushMode::Immediate);
    }

    /// Remove a group invite without flushing.
    #[allow(dead_code)]
    pub fn remove_invite_deferred(state: &crate::QaulState, account_id: PeerId, group_id: &[u8]) {
        Self::remove_invite_with_mode(state, account_id, group_id, FlushMode::Deferred);
    }

    fn remove_invite_with_mode(state: &crate::QaulState, account_id: PeerId, group_id: &[u8], flush_mode: FlushMode) {
        // get DB ref
        let db_ref = Self::get_db_ref(state, account_id);

        // remove group invite from data base
        if let Err(e) = db_ref.invited.remove(group_id) {
            log::error!("Error removing group invite from data base: {}", e);
        }

        Self::maybe_flush_tree(&db_ref.invited, flush_mode, "Error invited flush");
    }

    fn maybe_flush_tree(tree: &sled::Tree, flush_mode: FlushMode, error_context: &str) {
        if matches!(flush_mode, FlushMode::Deferred) {
            return;
        }

        if let Err(e) = tree.flush() {
            log::error!("{}: {}", error_context, e);
        }
    }
}
