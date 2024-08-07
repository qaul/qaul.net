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
use state::InitCell;
use std::{
    collections::BTreeMap,
    convert::TryInto,
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::RwLock,
};

use super::ChatStorage;
use crate::services::messaging::{self, Messaging, MessagingServiceType};
use crate::storage::database::DataBase;
use crate::utilities::timestamp;
use crate::utilities::timestamp::Timestamp;
use crate::{
    node::user_accounts::{UserAccount, UserAccounts},
    router::users::Users,
    services::group::{self, group_id::GroupId, Group, GroupStorage},
};
use crate::{rpc::Rpc, services::group::GroupManage};

/// Import protobuf message definition generated by
/// the rust module prost-build.
pub mod proto_rpc {
    include!("qaul.rpc.chatfile.rs");
}
pub mod proto_net {
    include!("qaul.net.chatfile.rs");
}

/// Size of the biggest file data package
pub const DEF_PACKAGE_SIZE: u32 = 64000;

/// mutable state of all file
static ALLFILES: InitCell<RwLock<AllFiles>> = InitCell::new();

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
}

impl UserFiles {
    /// get file history
    pub fn get_filehistory(&self, file_id: u64) -> Option<FileHistory> {
        // get invite
        match self.histories.get(file_id.to_be_bytes().to_vec()) {
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
            .range(first_key.to_be_bytes().to_vec()..last_key.to_be_bytes().to_vec());

        result
    }

    /// save file history
    pub fn save_filehistory(&self, file_id: u64, file_history: FileHistory) {
        // save file history into data base
        let file_history_bytes = bincode::serialize(&file_history).unwrap();
        if let Err(e) = self
            .histories
            .insert(file_id.to_be_bytes().to_vec(), file_history_bytes)
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
    fn get_chunk_key(file_id: &Vec<u8>, index: u32) -> Vec<u8> {
        let mut index_bytes = index.to_be_bytes().to_vec();
        let mut key_bytes = file_id.to_owned();
        key_bytes.append(&mut index_bytes);
        key_bytes
    }

    /// get file chunk DB key range
    ///
    /// returns a key tuple, which can be used to
    /// retrieve all file chunks for a certain file id from DB
    ///
    /// (first_key, last_key)
    fn get_chunk_key_range(file_id: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        let first_key = Self::get_chunk_key(file_id, 0);
        let last_key = Self::get_chunk_key(file_id, u32::MAX);
        (first_key, last_key)
    }

    /// save file chunk
    pub fn save_file_chunk(&self, file_id: u64, index: u32, data: Vec<u8>) {
        // get chunk key
        let key = Self::get_chunk_key(&file_id.to_be_bytes().to_vec(), index);

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

    /// count file chunks
    ///
    /// Count how many chunks of a file we already have in the data base
    pub fn count_file_chunks(&self, file_id: &Vec<u8>) -> usize {
        // get key range
        let (first_key, last_key) = Self::get_chunk_key_range(file_id);

        // get results from data base
        let result = self.file_chunks.range(first_key..last_key);

        result.count()
    }

    /// get all file chunks for a specific id
    pub fn get_file_chunks(&self, file_id: &Vec<u8>) -> sled::Iter {
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
    pub file_size: u32,
    /// file sent
    pub sent_at: u64,
    /// file received
    pub received_at: u64,
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

pub struct ChatFile {}
/// File sharing module to process transfer, receive and RPC commands
impl ChatFile {
    /// initialize chat file module
    pub fn init() {
        // create file history state
        let all_files = AllFiles {
            db_ref: BTreeMap::new(),
        };
        ALLFILES.set(RwLock::new(all_files));
    }

    /// File history is stored based on the users account id.
    /// This function getting history table based on the users account id.
    fn get_db_ref(user_id: &PeerId) -> UserFiles {
        // check if user data exists
        {
            // get chat state
            let all_files = ALLFILES.get().read().unwrap();

            // check if user ID is in map
            if let Some(user_files) = all_files.db_ref.get(&user_id.to_bytes()) {
                return UserFiles {
                    histories: user_files.histories.clone(),
                    file_chunks: user_files.file_chunks.clone(),
                };
            }
        }

        // create user data if it does not exist
        let user_files = Self::create_userfiles(user_id);

        // return chat_user structure
        UserFiles {
            histories: user_files.histories.clone(),
            file_chunks: user_files.file_chunks.clone(),
        }
    }

    /// create [user => file history] when it does not exist
    fn create_userfiles(user_id: &PeerId) -> UserFiles {
        // get user data base
        let db = DataBase::get_user_db(user_id.clone());

        // open trees
        let histories: sled::Tree = db.open_tree("chat_file").unwrap();
        let file_chunks: sled::Tree = db.open_tree("file_chunks").unwrap();

        let user_files = UserFiles {
            histories,
            file_chunks,
        };

        // get chat state for writing
        let mut all_files = ALLFILES.get().write().unwrap();

        // add user to state
        all_files
            .db_ref
            .insert(user_id.to_bytes(), user_files.clone());

        // return structure
        user_files
    }

    /// Update file message confirmation in data base
    pub fn update_confirmation(
        account_id: PeerId,
        receiver_id: PeerId,
        file_id: u64,
        received_at: u64,
    ) {
        log::trace!("update confirmation");

        // get db reference
        let user_files = ChatFile::get_db_ref(&account_id);

        // get file history
        if let Some(mut file_history) = user_files.get_filehistory(file_id) {
            // update reception & check if user finished
            if file_history.reception_confirmed(receiver_id) {
                // update chat message
                ChatStorage::update_confirmation(
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
    fn create_file_path(account_id: PeerId, file_id: u64, file_extension: &str) -> PathBuf {
        // create path to file storage directory
        let account_storage_path = crate::storage::Storage::get_account_path(account_id);
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
        files_storage_path.join(file_name.clone())
    }

    /// Getting file histories from table.
    /// This function is called from RPC command (file history [offset limit])
    pub fn file_history(
        user_account: &UserAccount,
        history_req: &proto_rpc::FileHistoryRequest,
    ) -> Vec<FileHistory> {
        // get DB references
        let db_ref = Self::get_db_ref(&user_account.id);

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
                    let file_history: FileHistory = bincode::deserialize(&message).unwrap();
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
        user_account: &UserAccount,
        group_id: &Vec<u8>,
        path_name: String,
        description: String,
    ) -> Result<bool, String> {
        // get group
        let group;
        match GroupStorage::get_group(user_account.id, group_id.to_owned()) {
            Some(v) => group = v,
            None => {
                match GroupId::from_bytes(group_id) {
                    Ok(direct_group) => {
                        // check if the group is a direct group
                        match direct_group.is_direct(user_account.id.clone()) {
                            Some(remote_q8id) => {
                                // get remote user
                                match Users::get_user_id_by_q8id(remote_q8id) {
                                    Some(remote_id) => {
                                        // create group
                                        group = GroupManage::create_new_direct_chat_group(
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
        match File::open(path_name.clone()) {
            Ok(f) => Some(file = f),
            Err(_e) => {
                return Err("file open error".to_string());
            }
        };

        let size = file.metadata().unwrap().len() as u32;
        if size == 0 {
            return Err("file size is zero".to_string());
        }

        // get file name
        let path = Path::new(path_name.as_str());
        let mut extension = "".to_string();

        if let Some(ext) =
            Self::get_extension_from_filename(path.file_name().unwrap().to_str().unwrap())
        {
            extension = ext.to_string();
        }

        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        // create file id
        let file_id = Self::generate_file_id(
            group_id,
            &user_account.id.to_bytes(),
            file_name.clone(),
            size,
        );

        // get file path
        let file_path = Self::create_file_path(user_account.id, file_id, extension.as_str());

        // TODO: start in new async thread here

        // copy file
        if let Err(e) = fs::copy(path_name.clone(), file_path) {
            log::error!("copy file error {}", e.to_string());
        }

        // create messages
        let mut mesage_count = 1 + size / DEF_PACKAGE_SIZE;
        if size % DEF_PACKAGE_SIZE > 0 {
            mesage_count = mesage_count + 1;
        }

        // create message ID
        let message_id = group::GroupManage::get_new_message_id(&user_account.id, group_id);

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
        };

        let info = proto_net::ChatFileContainer {
            message: Some(proto_net::chat_file_container::Message::FileInfo(
                file_info.clone(),
            )),
        };

        // send message to all group members
        Self::send_filecontainer_to_group(
            user_account,
            &group,
            &message_id,
            timestamp,
            info.encode_to_vec(),
        );

        // create group ID object
        let groupid = GroupId::from_bytes(group_id).unwrap();

        // save file state to data base
        let file_history = FileHistory {
            group_id: group_id.to_owned(),
            sender_id: user_account.id.to_bytes(),
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
        };

        let db_ref = Self::get_db_ref(&user_account.id);

        // save file history to data base
        let file_history_bytes = bincode::serialize(&file_history).unwrap();
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
            left_size = left_size - read_size;

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
                        data: buffer[0..(read_size as usize)].iter().cloned().collect(),
                    },
                )),
            };

            // send message to all group members
            Self::send_filecontainer_to_group(
                user_account,
                &group,
                &message_id,
                timestamp,
                data.encode_to_vec(),
            );

            chunk_index = chunk_index + 1;
        }

        // set file status to sent
        ChatStorage::udate_status(
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
        user_account: &UserAccount,
        sender_id: &PeerId,
        group_id: &GroupId,
        file_history: &FileHistory,
        status: super::rpc_proto::MessageStatus,
    ) {
        log::trace!("save_filemsg_in_chat");

        // create chat file message content
        let chat_filecontent = super::rpc_proto::FileContent {
            file_id: file_history.file_id,
            file_name: file_history.file_name.clone(),
            file_extension: file_history.file_extension.clone(),
            file_size: file_history.file_size,
            file_description: file_history.file_description.clone(),
        };
        let chat_message = super::rpc_proto::ChatContentMessage {
            message: Some(
                super::rpc_proto::chat_content_message::Message::FileContent(chat_filecontent),
            ),
        };

        // save file message to chat
        ChatStorage::save_message(
            &user_account.id,
            &group_id,
            &sender_id,
            &file_history.message_id,
            file_history.sent_at,
            chat_message.clone(),
            status,
        );
    }

    /// Pack a FileContainer message and send it to all Group members
    fn send_filecontainer_to_group(
        user_account: &UserAccount,
        group: &Group,
        message_id: &Vec<u8>,
        timestamp: u64,
        data: Vec<u8>,
    ) {
        // pack file container into common message
        let common_message = messaging::proto::CommonMessage {
            message_id: message_id.clone(),
            group_id: group.id.clone(),
            sent_at: timestamp,
            payload: Some(messaging::proto::common_message::Payload::FileMessage(
                messaging::proto::FileMessage { content: data },
            )),
        };

        let message = messaging::proto::Messaging {
            message: Some(messaging::proto::messaging::Message::CommonMessage(
                common_message.clone(),
            )),
        };

        // send to all members
        for user_id in group.members.keys() {
            let receiver = PeerId::from_bytes(&user_id.clone()).unwrap();
            if receiver == user_account.id {
                continue;
            }

            if let Err(error) = Messaging::pack_and_send_message(
                user_account,
                &receiver,
                message.encode_to_vec(),
                MessagingServiceType::ChatFile,
                message_id,
                true,
            ) {
                log::error!("sending file message error {}", error);
            }
        }
    }

    /// Generate File id
    fn generate_file_id(group_id: &Vec<u8>, sender: &Vec<u8>, file_name: String, size: u32) -> u64 {
        let mut name_bytes = file_name.as_bytes().to_vec();
        let mut size_bytes = size.to_be_bytes().to_vec();
        let mut time_bytes = timestamp::Timestamp::get_timestamp().to_be_bytes().to_vec();
        let mut key_bytes = group_id.clone();
        let mut sender_bytes = sender.clone();

        key_bytes.append(&mut sender_bytes);
        key_bytes.append(&mut name_bytes);
        key_bytes.append(&mut size_bytes);
        key_bytes.append(&mut time_bytes);
        crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO).checksum(&key_bytes)
    }

    /// Try to store the file
    ///
    /// This function will check if the file is fully downloaded,
    /// and will initiate the storage process if yes.
    fn try_store_file(
        user_account: &UserAccount,
        user_files: UserFiles,
        file_history: FileHistory,
    ) {
        // check how many chunks have been downloaded
        let count = user_files.count_file_chunks(&file_history.file_id.to_be_bytes().to_vec());

        log::trace!(
            "received {} chunks of {}",
            count + 1,
            file_history.message_count
        );

        // if we downloaded all chunks, save it to file if we received the file info
        if (count + 1) as u32 == file_history.message_count {
            log::trace!("store_file");

            Self::store_file(user_account, user_files, file_history);
        }
    }

    /// Store a completely downloaded file
    fn store_file(user_account: &UserAccount, user_files: UserFiles, file_history: FileHistory) {
        // get all chunks from data base
        let iterator = user_files.get_file_chunks(&file_history.file_id.to_be_bytes().to_vec());

        // create file
        let file_path = Self::create_file_path(
            user_account.id,
            file_history.file_id,
            &file_history.file_extension,
        );

        // open a file in write mode
        let mut file: File;
        match File::create(file_path.clone()) {
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
                        log::error!("file storing failed {}", e.to_string());
                    }
                }
                Err(e) => log::error!("{}", e),
            }
        }

        // set file status to received
        ChatStorage::udate_status(
            &user_account.id,
            &file_history.message_id,
            super::rpc_proto::MessageStatus::Received,
        );
    }

    /// process chat file data message
    fn process_data_message(
        user_account: &UserAccount,
        _sender_id: PeerId,
        _group_id: Vec<u8>,
        file_data: proto_net::ChatFileData,
    ) {
        // get DB references
        let user_files = Self::get_db_ref(&user_account.id);

        // save file chunk in DB
        user_files.save_file_chunk(file_data.file_id, file_data.start_index, file_data.data);

        // check if we already received the file info
        match user_files.get_filehistory(file_data.file_id) {
            Some(file_history) => {
                // create file once everything has been received
                Self::try_store_file(user_account, user_files, file_history);
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
        user_account: &UserAccount,
        sender_id: PeerId,
        group_id: Vec<u8>,
        message_id: Vec<u8>,
        sent_at: u64,
        file_info: proto_net::ChatFileInfo,
    ) {
        // get db
        let user_files = Self::get_db_ref(&user_account.id);

        // check if it already exists in DB
        let file_history;
        if let Some(my_file_history) = user_files.get_filehistory(file_info.file_id) {
            file_history = my_file_history;

            // update fields
        } else {
            file_history = FileHistory {
                group_id: group_id.clone(),
                sender_id: sender_id.to_bytes(),
                file_id: file_info.file_id,
                message_id: message_id.clone(),
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
            };
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
            user_account,
            &sender_id,
            &groupid,
            &file_history,
            super::rpc_proto::MessageStatus::Receiving,
        );

        // create file once everything has been received
        Self::try_store_file(user_account, user_files, file_history);
    }

    /// process chat file container message from network
    pub fn process_net_chatfilecontainer(
        sender_id: PeerId,
        user_account: UserAccount,
        group_id: Vec<u8>,
        message_id: Vec<u8>,
        sent_at: u64,
        data: &Vec<u8>,
    ) {
        // decode protobuf file message container
        match proto_net::ChatFileContainer::decode(&data[..]) {
            Ok(messaging) => match messaging.message {
                Some(proto_net::chat_file_container::Message::FileInfo(file_info)) => {
                    Self::process_info_message(
                        &user_account,
                        sender_id,
                        group_id,
                        message_id,
                        sent_at,
                        file_info,
                    );
                }
                Some(proto_net::chat_file_container::Message::FileData(file_data)) => {
                    Self::process_data_message(&user_account, sender_id, group_id, file_data);
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
    pub async fn rpc(data: Vec<u8>, user_id: Vec<u8>) {
        let account_id = PeerId::from_bytes(&user_id).unwrap();

        match proto_rpc::ChatFile::decode(&data[..]) {
            Ok(chatfile) => {
                match chatfile.message {
                    Some(proto_rpc::chat_file::Message::SendFileRequest(send_req)) => {
                        let user_account = UserAccounts::get_by_id(account_id).unwrap();

                        if let Err(e) = Self::send(
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
                        match UserAccounts::get_by_id(account_id) {
                            Some(account) => user_account = account,
                            None => {
                                log::error!("user account not found");
                                return;
                            }
                        }

                        let list = Self::file_history(&user_account, &history_req);

                        let mut histories: Vec<proto_rpc::FileHistoryEntry> = vec![];
                        for entry in list {
                            let file_entry = proto_rpc::FileHistoryEntry {
                                file_id: entry.file_id,
                                file_name: entry.file_name.clone(),
                                file_extension: entry.file_extension.clone(),
                                file_size: entry.file_size,
                                file_description: entry.file_description.clone(),
                                time: entry.sent_at,
                                sender_id: bs58::encode(entry.sender_id).into_string(),
                                group_id: uuid::Uuid::from_bytes(
                                    entry.group_id.try_into().unwrap(),
                                )
                                .to_string(),
                            };
                            histories.push(file_entry);
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
                            buf,
                            crate::rpc::proto::Modules::Chatfile.into(),
                            "".to_string(),
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
