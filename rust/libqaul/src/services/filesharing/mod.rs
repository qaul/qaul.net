// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul File Sharing Service
//!
//! The File sharing service sends and receives file messages into the network.
//! The File messages carry on the Messaging service
//! Messaging(FileMessage(FileSharingContainer(FileInfo, FileData, Confirmation)))

//use bs58::decode;
use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled_extensions::{bincode::Tree, DbExt};
use state::Storage;
use std::collections::BTreeMap;
use std::{
    convert::TryInto,
    io::{Read, Write},
    sync::RwLock,
};

use super::chat::{self};
use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::rpc::Rpc;
use crate::storage::database::DataBase;
use crate::utilities::timestamp;
use crate::utilities::timestamp::Timestamp;

use super::group;
use super::messaging::proto;
use super::messaging::Messaging;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::path::Path;

/// Import protobuf message definition generated by
/// the rust module prost-build.
pub mod proto_rpc {
    include!("qaul.rpc.filesharing.rs");
}
pub mod proto_net {
    include!("qaul.net.filesharing.rs");
}

/// Structure to management for file histories based on the each user_id.
pub struct AllFiles {
    pub db_ref: BTreeMap<Vec<u8>, UserFiles>,
}

/// User file histories structure
#[derive(Clone)]
pub struct UserFiles {
    /// history table
    pub histories: Tree<FileHistory>,
    /// file ids
    pub file_ids: Tree<u64>,
    /// last file index
    pub last_file: u64,
}

/// File history structure, this structure is stored into DB
#[derive(Serialize, Deserialize, Clone)]
pub struct FileHistory {
    /// group id
    pub group_id: Vec<u8>,
    /// sender id
    pub sender_id: Vec<u8>,
    /// start index
    pub start_index: u32,
    /// sender id
    pub message_count: u32,
    /// file id
    pub id: u64,
    /// file name
    pub name: String,
    /// file description
    pub descr: String,
    /// file extension
    pub extension: String,
    /// file size in bytes
    pub size: u32,
    /// file sent or received time
    pub time: u64,
}

/// mutable state of all file
static ALLFILES: Storage<RwLock<AllFiles>> = Storage::new();

/// pub const DEF_PACKAGE_SIZE: u32 = 64000;
pub const DEF_PACKAGE_SIZE: u32 = 1000;

pub struct FileShare {}
/// File sharing module to process transfer, receive and RPC commands
impl FileShare {
    /// initialize fileshare module
    pub fn init() {
        // create file history state
        let all_files = AllFiles {
            db_ref: BTreeMap::new(),
        };
        ALLFILES.set(RwLock::new(all_files));
    }

    /// File history stored based on the user id.
    /// This function getting histrory table based on the user id.
    fn get_db_ref(user_id: &PeerId) -> UserFiles {
        // check if user data exists
        {
            // get chat state
            let all_files = ALLFILES.get().read().unwrap();

            // check if user ID is in map
            if let Some(user_files) = all_files.db_ref.get(&user_id.to_bytes()) {
                return UserFiles {
                    histories: user_files.histories.clone(),
                    file_ids: user_files.file_ids.clone(),
                    last_file: user_files.last_file,
                };
            }
        }

        // create user data if it does not exist
        let user_files = Self::create_userfiles(user_id);

        // return chat_user structure
        UserFiles {
            histories: user_files.histories.clone(),
            file_ids: user_files.file_ids.clone(),
            last_file: user_files.last_file,
        }
    }

    /// create [user => file history] when it does not exist
    fn create_userfiles(user_id: &PeerId) -> UserFiles {
        // get user data base
        let db = DataBase::get_user_db(user_id.clone());

        // open trees
        let histories: Tree<FileHistory> = db.open_bincode_tree("file_sharing").unwrap();
        let file_ids: Tree<u64> = db.open_bincode_tree("file_ids").unwrap();

        // get last key
        let last_file: u64;
        match histories.iter().last() {
            Some(Ok((ivec, _))) => {
                let i = ivec.to_vec();
                match i.try_into() {
                    Ok(arr) => {
                        last_file = u64::from_be_bytes(arr);
                    }
                    Err(e) => {
                        log::error!("couldn't convert ivec to u64: {:?}", e);
                        last_file = 0;
                    }
                }
            }
            None => {
                last_file = 0;
            }
            Some(Err(e)) => {
                log::error!("Sled feed table error: {}", e);
                last_file = 0;
            }
        }

        let user_files = UserFiles {
            histories,
            file_ids,
            last_file,
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

    /// This function is called when file transfer or receiving finished successfully.    
    fn on_completed(
        user_id: &PeerId,
        sender_id: &PeerId,
        group_id: &Vec<u8>,
        info: &proto_net::FileSharingInfo,
    ) {
        let db_ref = Self::get_db_ref(user_id);

        //check if file id already exists
        if db_ref
            .file_ids
            .contains_key(&info.file_id.to_be_bytes())
            .unwrap()
        {
            return;
        }

        let history = FileHistory {
            group_id: group_id.clone(),
            sender_id: sender_id.to_bytes(),
            start_index: info.start_index,
            message_count: info.message_count,
            id: info.file_id,
            name: info.file_name.clone(),
            descr: info.file_descr.clone(),
            extension: info.file_extension.clone(),
            size: info.file_size,
            time: timestamp::Timestamp::get_timestamp(),
        };

        let last_file = db_ref.last_file + 1;
        // save to data base
        if let Err(e) = db_ref.histories.insert(&last_file.to_be_bytes(), history) {
            log::error!("Error saving file history to data base: {}", e);
        } else {
            if let Err(e) = db_ref.histories.flush() {
                log::error!("Error when flushing data base to disk: {}", e);
            }
        }

        // save file id
        if let Err(e) = db_ref
            .file_ids
            .insert(&info.file_id.to_be_bytes(), last_file)
        {
            log::error!("Error saving file ids to data base: {}", e);
        } else {
            if let Err(e) = db_ref.file_ids.flush() {
                log::error!("Error when flushing data base to disk: {}", e);
            }
        }

        //update last_file
        let mut all_files = ALLFILES.get().write().unwrap();

        // check if user ID is in map
        if let Some(user_files) = all_files.db_ref.get_mut(&user_id.to_bytes()) {
            user_files.last_file = last_file;
        }
    }

    /// getting file extension from given filename
    fn get_extension_from_filename(filename: &str) -> Option<&str> {
        Path::new(filename).extension().and_then(OsStr::to_str)
    }

    /// Getting file histories from table.
    /// This function is called from RPC command (file history [offset limit])
    pub fn file_history(
        user_account: &UserAccount,
        history_req: &proto_rpc::FileHistoryRequest,
    ) -> (u64, Vec<FileHistory>) {
        let db_ref = Self::get_db_ref(&user_account.id);

        let mut histories: Vec<FileHistory> = vec![];

        if history_req.offset as u64 >= db_ref.last_file {
            //no histories from offset
            return (db_ref.last_file, histories);
        }

        let mut count = history_req.limit;
        if (history_req.offset + count) as u64 >= db_ref.last_file {
            count = (db_ref.last_file - (history_req.offset as u64)) as u32;
        }

        if count == 0 {
            //no histories from offset
            return (db_ref.last_file, histories);
        }

        let first_file = db_ref.last_file - ((history_req.offset + count) as u64) + 1;
        let end_file = first_file + (count as u64);
        let first_file_bytes = first_file.to_be_bytes().to_vec();
        let end_file_bytes = end_file.to_be_bytes().to_vec();

        for res in db_ref
            .histories
            .range(first_file_bytes.as_slice()..end_file_bytes.as_slice())
        {
            //for res in db_ref.histories.range(first_file_bytes.as_slice()..) {
            match res {
                Ok((_id, message)) => {
                    histories.push(message.clone());
                }
                Err(e) => {
                    log::error!("Error retrieving file history from data base: {}", e);
                }
            }
        }

        (db_ref.last_file, histories)
    }

    fn send(
        user_account: &UserAccount,
        group_id: &Vec<u8>,
        path_name: String,
        description: String,
    ) -> Result<bool, String> {
        //get group and my member
        let group;
        match group::Group::get_group(&user_account.id, group_id) {
            Ok(v) => {
                group = v;
            }
            Err(error) => {
                return Err(error);
            }
        }

        let mut my_member;
        match group.get_member(&user_account.id.to_bytes()) {
            Some(v) => {
                my_member = v.clone();
            }
            _ => {
                return Err("you are not member in this group".to_string());
            }
        }

        let mut last_index = my_member.last_message_index + 1;
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

        // copy file into files folder
        let mut file_path = crate::storage::Storage::get_path();
        if file_path.chars().last().unwrap() != '/' {
            file_path.push_str("/");
        }
        file_path.push_str(user_account.id.to_base58().as_str());
        file_path.push_str("/files/");

        if let Err(e) = fs::create_dir_all(file_path.clone()) {
            log::error!("creating folder error {}", e.to_string());
        }

        let file_id = Self::generate_file_id(
            group_id,
            &user_account.id.to_bytes(),
            file_name.clone(),
            size,
        );

        // copy file
        file_path.push_str(file_id.to_string().as_str());
        if extension.len() > 0 {
            file_path.push_str(".");
            file_path.push_str(&extension.clone().as_str());
        }
        if let Err(e) = fs::copy(path_name.clone(), file_path) {
            log::error!("copy file error {}", e.to_string());
        }

        // create messages
        let mut mesage_count = 1 + size / DEF_PACKAGE_SIZE;
        if size % DEF_PACKAGE_SIZE > 0 {
            mesage_count = mesage_count + 1;
        }

        let mut messages: Vec<proto_net::FileSharingContainer> = vec![];
        //1. fileinfo message
        let file_info = proto_net::FileSharingInfo {
            file_id,
            file_name: file_name.clone(),
            file_extension: extension.clone(),
            file_size: size,
            file_descr: description.clone(),
            start_index: last_index,
            message_count: mesage_count,
        };
        Self::on_completed(&user_account.id, &user_account.id, group_id, &file_info);

        let info = proto_net::FileSharingContainer {
            message: Some(proto_net::file_sharing_container::Message::FileInfo(
                file_info.clone(),
            )),
        };
        messages.push(info);

        //2. filedata message
        // read file contents and make FileData messages
        let mut buffer: [u8; DEF_PACKAGE_SIZE as usize] = [0; DEF_PACKAGE_SIZE as usize];
        let mut left_size = size;

        while left_size > 0 {
            let mut read_size = left_size;
            if left_size > DEF_PACKAGE_SIZE {
                read_size = DEF_PACKAGE_SIZE;
            };
            // file.by_ref().take(read_size as u64).read(&mut buffer);
            if let Err(e) = file.read(&mut buffer) {
                return Err(e.to_string());
            }

            let data = proto_net::FileSharingContainer {
                message: Some(proto_net::file_sharing_container::Message::FileData(
                    proto_net::FileSharingData {
                        start_index: last_index,
                        message_count: mesage_count,
                        data: buffer[0..(read_size as usize)].iter().cloned().collect(),
                    },
                )),
            };
            messages.push(data);
            left_size = left_size - read_size;
        }

        //pack into common messge
        let conversation_id = super::messaging::ConversationId::from_bytes(group_id).unwrap();
        let mut msgs: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        let mut stored_info = false;
        for msg in messages {
            let message_id = super::messaging::Messaging::generate_group_message_id(
                group_id,
                &user_account.id,
                last_index,
            );
            last_index = last_index + 1;

            let common_message = proto::CommonMessage {
                message_id: message_id.clone(),
                conversation_id: group_id.clone(),
                sent_at: timestamp,
                payload: Some(proto::common_message::Payload::FileMessage(
                    proto::FileMessage {
                        content: msg.encode_to_vec(),
                    },
                )),
            };

            let message = proto::Messaging {
                message: Some(proto::messaging::Message::CommonMessage(
                    common_message.clone(),
                )),
            };

            msgs.push((message_id.clone(), message.encode_to_vec()));

            //save only the first file infor maessge into chat table
            if !stored_info {
                chat::Chat::save_outgoing_message(
                    &user_account.id,
                    &user_account.id,
                    &conversation_id,
                    &message_id,
                    chat::rpc_proto::ContentType::File.try_into().unwrap(),
                    &msg.encode_to_vec(),
                    0,
                );
                stored_info = true;
            }
        }

        // update lsat index
        my_member.last_message_index = last_index;
        group::Group::update_group_member(&user_account.id, group_id, &my_member);

        // send to all members
        for user_id in group.members.keys() {
            let receiver = PeerId::from_bytes(&user_id.clone()).unwrap();
            if receiver == user_account.id {
                continue;
            }

            for (msg_id, msg) in &msgs {
                if let Err(error) = Messaging::pack_and_send_message(
                    user_account,
                    &receiver,
                    msg,
                    Some(msg_id),
                    true,
                ) {
                    log::error!("sending file message error {}", error);
                }
            }
        }

        Ok(true)
    }

    /// Send the file on the messaging service
    /// This function is called from RPC command (file send conversation_id file_path_name)
    pub fn send_to_peer(
        user_account: &UserAccount,
        send_file_req: proto_rpc::SendFileRequest,
    ) -> Result<bool, String> {
        let conversation_id;
        //direct file messge case.
        if send_file_req.conversation_id.len() > 16 {
            // create new room if no exist
            let peer_id = PeerId::from_bytes(&send_file_req.conversation_id).unwrap();
            if user_account.id == peer_id {
                return Err("You can not send file to yourself".to_string());
            }

            let group_id =
                super::messaging::ConversationId::from_peers(&user_account.id, &peer_id).unwrap();
            if !group::Group::is_group_exist(&user_account.id, &group_id.to_bytes()) {
                group::Manage::create_new_direct_chat_group(&user_account.id, &peer_id);
            }
            conversation_id = group_id.to_bytes();
        } else {
            conversation_id = send_file_req.conversation_id.clone();
        }

        Self::send(
            user_account,
            &conversation_id,
            send_file_req.path_name.clone(),
            send_file_req.description.clone(),
        )
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
        crc::crc64::checksum_iso(&key_bytes)
    }

    /// try store received file
    pub fn try_store_file(
        user_account: &UserAccount,
        sender_id: &PeerId,
        group_id: &Vec<u8>,
        start_index: u32,
        message_count: u32,
    ) -> Result<bool, String> {
        //check all messages are arrived
        let mut message_ids: Vec<Vec<u8>> = vec![];
        for i in 0..message_count {
            let msg_id = Messaging::generate_group_message_id(group_id, sender_id, start_index + i);
            message_ids.push(msg_id.clone());
        }
        if !chat::Chat::is_messages_exist(&user_account.id, &message_ids) {
            //all content are not arrived
            return Err("all content are not arrived".to_string());
        }

        //get file info message
        let info_msg =
            chat::Chat::get_messages_by_id(&user_account.id, &vec![message_ids[0].clone()]);
        if info_msg.len() == 0
        // || chat::rpc_proto::ContentType::from_i32(info_msg[0].0).unwrap()
        //     != chat::rpc_proto::ContentType::File
        {
            return Err("file info message not exists".to_string());
        }

        //decode file info
        let file_info;
        match proto_net::FileSharingContainer::decode(&info_msg[0].1[..]) {
            Ok(container) => match container.message {
                Some(proto_net::file_sharing_container::Message::FileInfo(info)) => {
                    file_info = info;
                }
                _ => {
                    return Err("file info message damaged".to_string());
                }
            },
            _ => {
                return Err("file message container was damaged".to_string());
            }
        }
        //check content
        if start_index != file_info.start_index || message_count != file_info.message_count {
            return Err("file info message mismatched".to_string());
        }

        //get all data messages
        message_ids.remove(0);
        let file_datas = chat::Chat::get_messages_by_id(&user_account.id, &message_ids);
        //check validate
        if file_datas.len() != message_ids.len() {
            return Err("there are some missed file data messages".to_string());
        }

        // check directory
        let storage_path_string = crate::storage::Storage::get_path();
        let storage_path = Path::new(&storage_path_string);
        let user_storage_path = storage_path.join(user_account.id.to_base58());
        let files_storage_path = user_storage_path.join("files");

        // create files directory if it doesn't exist yet
        if let Err(_e) = fs::create_dir_all(files_storage_path.clone()) {
            return Err("creating folder error".to_string());
        }

        // create file name
        let mut file_name = file_info.file_id.to_string();
        if file_info.file_extension.len() > 0 {
            file_name.push_str(".");
            file_name.push_str(&file_info.file_extension.as_str());
        }

        // create file path
        let file_path = files_storage_path.join(file_name);

        log::info!("save file {:?}", file_path);
        let mut file: File = File::create(file_path.clone()).unwrap();
        for (_content_type, content) in &file_datas {
            // if chat::rpc_proto::ContentType::from_i32(*content_type).unwrap()
            //     != chat::rpc_proto::ContentType::File
            // {
            //     return Err("file data message invalid content type".to_string());
            // }

            match proto_net::FileSharingContainer::decode(&content[..]) {
                Ok(container) => match container.message {
                    Some(proto_net::file_sharing_container::Message::FileData(file_data)) => {
                        if let Err(e) = file.write(&file_data.data) {
                            log::error!("file storing failed {}", e.to_string());
                        }
                    }
                    _ => {
                        return Err("file data message invalid content type".to_string());
                    }
                },
                _ => {
                    return Err("file data message container is dirty".to_string());
                }
            }
        }

        if let Err(_e) = file.flush() {
            log::error!("file service storing error");
        }

        Self::on_completed(&user_account.id, sender_id, group_id, &file_info);

        // remove file data messges
        chat::Chat::remove_messages(&user_account.id, &message_ids);

        Ok(true)
    }

    /// process filehshare container message from network
    pub fn net(sender_id: &PeerId, receiver_id: &PeerId, group_id: &Vec<u8>, data: &Vec<u8>) {
        //check receiver id is in users list
        let user;
        match UserAccounts::get_by_id(receiver_id.clone()) {
            Some(usr) => {
                user = usr;
            }
            None => {
                log::error!("no user id={}", receiver_id);
                return;
            }
        }

        match proto_net::FileSharingContainer::decode(&data[..]) {
            Ok(messaging) => match messaging.message {
                Some(proto_net::file_sharing_container::Message::FileInfo(file_info)) => {
                    if let Err(_) = Self::try_store_file(
                        &user,
                        sender_id,
                        group_id,
                        file_info.start_index,
                        file_info.message_count,
                    ) {}
                }
                Some(proto_net::file_sharing_container::Message::FileData(file_data)) => {
                    if let Err(error) = Self::try_store_file(
                        &user,
                        sender_id,
                        group_id,
                        file_data.start_index,
                        file_data.message_count,
                    ) {
                        log::error!("on file data {}", error);
                    }
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
                    "Error decoding FileSharing Message from {} to {}: {}",
                    sender_id.to_base58(),
                    receiver_id.to_base58(),
                    e
                );
            }
        }
    }

    /// Process incoming RPC request messages for file sharing module
    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>) {
        let my_user_id = PeerId::from_bytes(&user_id).unwrap();

        match proto_rpc::FileSharing::decode(&data[..]) {
            Ok(filesharing) => {
                match filesharing.message {
                    Some(proto_rpc::file_sharing::Message::SendFileRequest(send_req)) => {
                        let user_account = UserAccounts::get_by_id(my_user_id).unwrap();

                        if let Err(e) = Self::send_to_peer(&user_account, send_req) {
                            log::error!("file rpc send file failed {}", e.to_string());
                        }
                    }
                    Some(proto_rpc::file_sharing::Message::FileHistory(history_req)) => {
                        let user_account = UserAccounts::get_by_id(my_user_id).unwrap();
                        log::info!("lib->file->history");
                        let (total, list) = Self::file_history(&user_account, &history_req);

                        let mut histories: Vec<proto_rpc::FileHistoryEntry> = vec![];
                        for entry in list {
                            let file_entry = proto_rpc::FileHistoryEntry {
                                file_id: entry.id,
                                file_name: entry.name.clone(),
                                file_ext: entry.extension.clone(),
                                file_size: entry.size,
                                file_descr: entry.descr.clone(),
                                time: entry.time,
                                sender_id: bs58::encode(entry.sender_id).into_string(),
                                group_id: uuid::Uuid::from_bytes(
                                    entry.group_id.try_into().unwrap(),
                                )
                                .to_string(),
                            };
                            histories.push(file_entry);
                        }

                        // pack message
                        let proto_message = proto_rpc::FileSharing {
                            message: Some(proto_rpc::file_sharing::Message::FileHistoryResponse(
                                proto_rpc::FileHistoryResponse {
                                    offset: history_req.offset,
                                    limit: history_req.limit,
                                    total,
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
                            crate::rpc::proto::Modules::Fileshare.into(),
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
