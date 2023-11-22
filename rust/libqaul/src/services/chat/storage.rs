// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Conversations Storage Functions
//!
//! Saves and retrieves the chat conversations
//! and their overview from the data base.

use libp2p::PeerId;
use sled_extensions::{bincode::Tree, DbExt};
use state::InitCell;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::rpc_proto;
use crate::services::group::{group_id::GroupId, GroupStorage};
use crate::storage::database::DataBase;
use crate::utilities::timestamp::Timestamp;
use prost::Message;

/// mutable state of chat messages
static CHAT: InitCell<RwLock<ChatStorage>> = InitCell::new();

/// chat DB references per user account
#[derive(Clone)]
pub struct ChatAccountDb {
    // messages sled data base tree
    pub messages: Tree<rpc_proto::ChatMessage>,
    // message id => db key
    pub message_ids: Tree<Vec<u8>>,
}

/// qaul Chat Conversation Storage
pub struct ChatStorage {
    /// data base tree references accessible
    /// by user account
    db_ref: BTreeMap<Vec<u8>, ChatAccountDb>,
}

impl ChatStorage {
    /// initialize chat storage
    pub fn init() {
        // create chat storage
        let chat = ChatStorage {
            db_ref: BTreeMap::new(),
        };
        CHAT.set(RwLock::new(chat));
    }

    /// check if messages exists
    #[allow(dead_code)]
    pub fn messages_exist(user_id: &PeerId, message_ids: &Vec<Vec<u8>>) -> bool {
        // get data base of user account
        let db_ref = Self::get_db_ref(user_id.clone());
        for id in message_ids {
            if !db_ref.message_ids.contains_key(id).unwrap() {
                return false;
            }
        }
        true
    }

    /// get messages by ids
    #[allow(dead_code)]
    pub fn get_messages_by_id(user_id: &PeerId, message_ids: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut res: Vec<Vec<u8>> = vec![];

        let db_ref = Self::get_db_ref(user_id.clone());
        for id in message_ids {
            match db_ref.message_ids.get(id) {
                Ok(opt_key) => {
                    if let Some(db_key) = opt_key {
                        match db_ref.messages.get(&db_key) {
                            Ok(opt_msg) => {
                                if let Some(msg) = opt_msg {
                                    res.push(msg.content.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        res
    }

    /// remove messages by ids
    #[allow(dead_code)]
    pub fn remove_messages(user_id: &PeerId, message_ids: &Vec<Vec<u8>>) {
        let db_ref = Self::get_db_ref(user_id.clone());
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

        if let Err(_e) = db_ref.messages.flush() {
            log::error!("message storing error!");
        }
        if let Err(_e) = db_ref.message_ids.flush() {
            log::error!("message ids storing error!");
        }
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
        account_id: &PeerId,
        group_id: &GroupId,
        sender_id: &PeerId,
        message_id: &Vec<u8>,
        sent_at: u64,
        content: super::rpc_proto::ChatContentMessage,
        status: rpc_proto::MessageStatus,
    ) {
        log::trace!("chat save_message");

        // get data base of user account
        let db_ref = Self::get_db_ref(account_id.clone());

        // check if message_id already exists
        // this protects the double saving of incoming messages
        if message_id.len() > 0 {
            if db_ref.message_ids.contains_key(message_id).unwrap() {
                log::warn!("chat message already exists");
                return;
            }
        }

        // create received at timestamp
        let received_at = Timestamp::get_timestamp();

        // update last message
        GroupStorage::group_update_last_chat_message(
            account_id.to_owned(),
            group_id.to_bytes(),
            sender_id.to_owned(),
            content.encode_to_vec(),
            received_at,
        );

        // get next index
        let index = Self::get_next_db_index(db_ref.clone(), &group_id.to_bytes());

        // create data base key
        let db_key = Self::get_db_key_from_vec(&group_id.to_bytes(), index);

        // create chat message
        let chat_message = rpc_proto::ChatMessage {
            index,
            sender_id: sender_id.to_bytes(),
            message_id: message_id.clone(),
            status: status as i32,
            message_reception_confirmed: Vec::new(),
            group_id: group_id.to_bytes(),
            sent_at,
            received_at,
            content: content.encode_to_vec(),
        };

        // save message in data base
        if let Err(e) = db_ref.messages.insert(db_key.clone(), chat_message) {
            log::error!("Error saving chat message to data base: {}", e);
        }
        // flush trees to disk
        if let Err(e) = db_ref.messages.flush() {
            log::error!("Error chat messages flush: {}", e);
        }

        // save message id in data base
        if message_id.len() > 0 {
            if let Err(e) = db_ref
                .message_ids
                .insert(message_id.clone(), db_key.clone())
            {
                log::error!("Error saving chat messageid to data base: {}", e);
            }
            // flush trees to disk
            if let Err(e) = db_ref.message_ids.flush() {
                log::error!("Error chat message_ids flush: {}", e);
            }
        }
    }

    /// updating chat message status as confirmed
    pub fn update_confirmation(
        account_id: PeerId,
        receiver_id: PeerId,
        message_id: &Vec<u8>,
        received_at: u64,
    ) {
        // get data base of user account
        let db_ref = Self::get_db_ref(account_id.clone());
        if let Some(key) = db_ref.message_ids.get(message_id).unwrap() {
            if let Some(mut chat_msg) = db_ref.messages.get(&key).unwrap() {
                chat_msg.status = rpc_proto::MessageStatus::Confirmed as i32;
                chat_msg.received_at = received_at;

                // TODO: check if receiver already exists

                // receiving user
                let confirmation = rpc_proto::MessageReceptionConfirmed {
                    user_id: receiver_id.to_bytes(),
                    confirmed_at: received_at,
                };
                chat_msg.message_reception_confirmed.push(confirmation);

                // TODO: check if it was received by everyone
                //       set received_by_all flag if yes

                // save message in data base
                if let Err(e) = db_ref.messages.insert(key.clone(), chat_msg) {
                    log::error!("Error saving chat message to data base: {}", e);
                }
                // flush trees to disk
                if let Err(e) = db_ref.messages.flush() {
                    log::error!("Error chat messages flush: {}", e);
                }
            }
        }
    }

    /// update message status
    pub fn udate_status(
        account_id: &PeerId,
        message_id: &Vec<u8>,
        status: super::rpc_proto::MessageStatus,
    ) {
        // get data base of user account
        let db_ref = Self::get_db_ref(account_id.to_owned());
        if let Some(key) = db_ref.message_ids.get(message_id).unwrap() {
            if let Some(mut chat_msg) = db_ref.messages.get(&key).unwrap() {
                chat_msg.status = status as i32;

                // save message in data base
                if let Err(e) = db_ref.messages.insert(key.clone(), chat_msg) {
                    log::error!("Error saving chat message to data base: {}", e);
                }
                // flush trees to disk
                if let Err(e) = db_ref.messages.flush() {
                    log::error!("Error chat messages flush: {}", e);
                }
            }
        }
    }

    /// Get chat messages of a specific conversation from data base
    pub fn get_messages(account_id: PeerId, group_id: Vec<u8>) -> rpc_proto::ChatConversationList {
        // create empty messages list
        let mut message_list: Vec<rpc_proto::ChatMessage> = Vec::new();

        if group_id.len() == 16 {
            // get database references for this user account
            let db_ref = Self::get_db_ref(account_id);

            // create message keys
            let (first_key, last_key) = Self::get_db_key_range(&group_id.clone());

            // iterate over all values in chat_messages db
            for res in db_ref
                .messages
                .range(first_key.as_slice()..last_key.as_slice())
            {
                match res {
                    Ok((_id, message)) => {
                        message_list.push(message);
                    }
                    Err(e) => {
                        log::error!("get_messages error: {}", e);
                    }
                }
            }

            // clear unread messages from group
            GroupStorage::group_clear_unread(account_id, group_id.clone());
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
    fn get_db_key_range(group_id: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        let first_key = Self::get_db_key_from_vec(group_id, 0);
        let last_key = Self::get_db_key_from_vec(group_id, 0xFFFFFFFFFFFFFFFF); // = 4294967295
        (first_key, last_key)
    }

    /// create DB key from group id
    fn get_db_key_from_vec(group_id: &Vec<u8>, index: u64) -> Vec<u8> {
        let mut index_bytes = index.to_be_bytes().to_vec();
        let mut key_bytes = group_id.clone();
        key_bytes.append(&mut index_bytes);
        key_bytes
    }

    /// get nex db_index key
    fn get_next_db_index(db_ref: ChatAccountDb, group_id: &Vec<u8>) -> u64 {
        // get biggest existing index
        let search_key = Self::get_db_key_from_vec(group_id, u64::MAX);
        let result = db_ref.messages.get_lt(search_key);
        if let Ok(Some((_key, value))) = result {
            // check if result is really of the same group
            if group_id.to_owned() == value.group_id {
                return value.index + 1;
            }
        }

        return 0;
    }

    /// get user account data base tree references
    fn get_db_ref(account_id: PeerId) -> ChatAccountDb {
        // check if user account data exists
        {
            // get chat state
            let chat = CHAT.get().read().unwrap();

            // check if user account ID is in map
            if let Some(chat_user) = chat.db_ref.get(&account_id.to_bytes()) {
                return ChatAccountDb {
                    messages: chat_user.messages.clone(),
                    message_ids: chat_user.message_ids.clone(),
                };
            }
        }

        // create user data if it does not exist
        let chat_user = Self::create_chatuser(account_id);

        // return chat_user structure
        ChatAccountDb {
            messages: chat_user.messages.clone(),
            message_ids: chat_user.message_ids.clone(),
        }
    }

    /// create user data when it does not exist
    fn create_chatuser(account_id: PeerId) -> ChatAccountDb {
        // get user data base
        let db = DataBase::get_user_db(account_id);

        // open trees
        let messages: Tree<rpc_proto::ChatMessage> = db.open_bincode_tree("chat_messages").unwrap();
        let message_ids: Tree<Vec<u8>> = db.open_bincode_tree("chat_message_ids").unwrap();

        let chat_user = ChatAccountDb {
            messages,
            message_ids,
        };

        // get chat state for writing
        let mut chat = CHAT.get().write().unwrap();

        // add user to state
        chat.db_ref.insert(account_id.to_bytes(), chat_user.clone());

        // return structure
        chat_user
    }
}
