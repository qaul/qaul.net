// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Storage Handling
//!
//! Saves and retrieves groups from data base.

use libp2p::PeerId;
use sled_extensions::{bincode::Tree, DbExt};
use state::InitCell;
use std::collections::BTreeMap;
use std::sync::RwLock;

use crate::storage::database::DataBase;

/// mutable state of all user groups
pub static GROUPSTORAGE: InitCell<RwLock<GroupStorage>> = InitCell::new();

use super::{Group, GroupInvited};

/// Group DB links for user account
#[derive(Clone)]
pub struct GroupAccountDb {
    /// group DB reference
    pub groups: Tree<Group>,
    /// invited DB ref
    pub invited: Tree<GroupInvited>,
}

/// qaul Chat Conversation Storage
pub struct GroupStorage {
    /// data base tree references accessible
    /// by user account
    db_ref: BTreeMap<Vec<u8>, GroupAccountDb>,
}

impl GroupStorage {
    /// Initialize Group Storage
    pub fn init() {
        let group_storage = GroupStorage {
            db_ref: BTreeMap::new(),
        };
        GROUPSTORAGE.set(RwLock::new(group_storage));
    }

    /// get DB refs for user account
    pub fn get_db_ref(account_id: PeerId) -> GroupAccountDb {
        // check if user account data exists
        {
            // get chat state
            let group_storage = GROUPSTORAGE.get().read().unwrap();

            // check if user account ID is in map
            if let Some(group_account_db) = group_storage.db_ref.get(&account_id.to_bytes()) {
                return GroupAccountDb {
                    groups: group_account_db.groups.clone(),
                    invited: group_account_db.invited.clone(),
                };
            }
        }

        // create group account db entry if it does not exist
        let group_account_db = Self::create_groupaccountdb(account_id);

        // return group_account_db structure
        GroupAccountDb {
            groups: group_account_db.groups.clone(),
            invited: group_account_db.invited.clone(),
        }
    }

    /// create group account db entry when it does not exist
    fn create_groupaccountdb(account_id: PeerId) -> GroupAccountDb {
        // get user data base
        let db = DataBase::get_user_db(account_id);

        // open trees
        let groups: Tree<Group> = db.open_bincode_tree("groups").unwrap();
        let invited: Tree<GroupInvited> = db.open_bincode_tree("invited").unwrap();

        let group_account_db = GroupAccountDb { groups, invited };

        // get group storage for writing
        let mut group_storage = GROUPSTORAGE.get().write().unwrap();

        // add user to state
        group_storage
            .db_ref
            .insert(account_id.to_bytes(), group_account_db.clone());

        // return structure
        group_account_db
    }

    /// get a group from data base
    pub fn get_group(account_id: PeerId, group_id: Vec<u8>) -> Option<Group> {
        // get DB ref
        let db_ref = Self::get_db_ref(account_id);

        // get group
        match db_ref.groups.get(group_id) {
            Ok(group) => {
                return group;
            }
            Err(e) => log::error!("{}", e),
        }

        None
    }

    /// Check if a group exists in the data base
    pub fn group_exists(account_id: PeerId, group_id: Vec<u8>) -> bool {
        // get DB ref
        let db_ref = Self::get_db_ref(account_id);

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
    pub fn save_group(account_id: PeerId, group: Group) {
        // get DB ref
        let db_ref = Self::get_db_ref(account_id);

        // save group in data base
        if let Err(e) = db_ref.groups.insert(group.id.clone(), group) {
            log::error!("Error saving group to data base: {}", e);
        }
        // flush trees to disk
        if let Err(e) = db_ref.groups.flush() {
            log::error!("Error groups flush: {}", e);
        }
    }

    /// Update Last Chat Message sent to this Group
    pub fn group_update_last_chat_message(
        account_id: PeerId,
        group_id: Vec<u8>,
        sender_id: PeerId,
        message: Vec<u8>,
        received_at: u64,
    ) {
        log::debug!("group_update_last_chat_message");

        if let Some(mut group) = Self::get_group(account_id, group_id) {
            // update values
            group.last_message_sender_id = sender_id.to_bytes();
            group.last_message_at = received_at;
            group.last_message_data = message;

            // check if it is us who is sending the message
            if sender_id != account_id {
                group.unread_messages = group.unread_messages + 1;
            }

            // save group
            Self::save_group(account_id, group);
        } else {
            log::error!("group_update_last_chat group not found");
        }
    }

    /// Clear Unread Message Counter
    pub fn group_clear_unread(account_id: PeerId, group_id: Vec<u8>) {
        log::debug!("group_clear_unread");

        if let Some(mut group) = Self::get_group(account_id, group_id) {
            // clear unread value
            group.unread_messages = 0;

            // save group
            Self::save_group(account_id, group);
        } else {
            log::error!("group_clear_unread group not found");
        }
    }

    /// get invite
    pub fn get_invite(account_id: PeerId, group_id: Vec<u8>) -> Option<GroupInvited> {
        // get DB ref
        let db_ref = Self::get_db_ref(account_id);

        // get invite
        match db_ref.invited.get(group_id) {
            Ok(invite) => {
                return invite;
            }
            Err(e) => log::error!("{}", e),
        }

        None
    }

    /// Save a group invite into the data base
    ///
    /// This function overwrites an already existing invite entry for
    /// the same group or creates a new one.
    pub fn save_invite(account_id: PeerId, invite: GroupInvited) {
        // get DB ref
        let db_ref = Self::get_db_ref(account_id);

        // save group invite in data base
        if let Err(e) = db_ref.invited.insert(invite.group.id.clone(), invite) {
            log::error!("Error saving group invite to data base: {}", e);
        }
        // flush trees to disk
        if let Err(e) = db_ref.invited.flush() {
            log::error!("Error invited flush: {}", e);
        }
    }

    /// Remove a group invite from the data base
    pub fn remove_invite(account_id: PeerId, group_id: &Vec<u8>) {
        // get DB ref
        let db_ref = Self::get_db_ref(account_id);

        // remove group invite from data base
        if let Err(e) = db_ref.invited.remove(group_id) {
            log::error!("Error removing group invite from data base: {}", e);
        }
        // flush trees to disk
        if let Err(e) = db_ref.invited.flush() {
            log::error!("Error invited flush: {}", e);
        }
    }
}
