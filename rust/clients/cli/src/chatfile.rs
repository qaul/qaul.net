// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # ChatFile module functions

use super::rpc::Rpc;
use prost::Message;
use std::fmt;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chatfile.rs");
}

/// Chat file module function handling
pub struct ChatFile {}

impl ChatFile {
    /// CLI command interpretation
    ///
    /// The CLI commands of the chat file module are processed here
    pub fn cli(command: &str) {
        match command {
            // send file
            cmd if cmd.starts_with("send ") => {
                let command_string = cmd.strip_prefix("send ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    let group_id;
                    // convert group id from string to binary version
                    match Self::id_string_to_bin(group_id_str.to_string()) {
                        Ok(id) => {
                            group_id = id.clone();
                        }
                        Err(_e) => match Self::uuid_string_to_bin(group_id_str.to_string()) {
                            Ok(id) => {
                                group_id = id.clone();
                            }
                            _ => {
                                log::error!("Invalid group id");
                                return;
                            }
                        },
                    }

                    if let Some(file_path_name) = iter.next() {
                        let descr = match iter.next() {
                            Some(description) => description.to_string(),
                            _ => "".to_string(),
                        };

                        log::trace!(
                            "send file to group={}, file path={}, description={}",
                            group_id_str,
                            file_path_name,
                            descr
                        );

                        Self::send_file(group_id, file_path_name.to_string(), descr);
                    } else {
                        log::error!("file pathname is not given");
                    }
                } else {
                    log::error!("chat send command incorrectly formatted");
                }
            }

            // request chat file history list
            cmd if cmd.starts_with("history") => {
                let mut offset: i32 = 0;
                let mut limit: i32 = 10;

                if cmd.starts_with("history ") {
                    let command_string = cmd.strip_prefix("history ").unwrap().to_string();
                    let mut iter = command_string.split_whitespace();

                    if let Some(offset_str) = iter.next() {
                        offset = offset_str.to_string().parse().unwrap();
                        if let Some(limit_str) = iter.next() {
                            limit = limit_str.to_string().parse().unwrap();
                        }
                    }
                }

                Self::send_file_history_command(offset as u32, limit as u32);
            }
            // unknown command
            _ => log::error!("unknown file command"),
        }
    }

    /// Convert Group ID from String to Binary
    fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
        // check length
        if id.len() < 52 {
            return Err("Group ID not long enough".to_string());
        }

        // convert input
        match bs58::decode(id).into_vec() {
            Ok(id_bin) => Ok(id_bin),
            Err(e) => {
                let err = fmt::format(format_args!("{}", e));
                Err(err)
            }
        }
    }

    /// Convert Group ID from String to Binary
    fn uuid_string_to_bin(id_str: String) -> Result<Vec<u8>, String> {
        match uuid::Uuid::parse_str(id_str.as_str()) {
            Ok(id) => Ok(id.as_bytes().to_vec()),
            _ => Err("invalid group id".to_string()),
        }
    }

    /// send file via rpc
    fn send_file(group_id: Vec<u8>, file_name: String, description: String) {
        // create file send message
        let proto_message = proto::ChatFile {
            message: Some(proto::chat_file::Message::SendFileRequest(
                proto::SendFileRequest {
                    path_name: file_name.clone(),
                    group_id: group_id.clone(),
                    description: description.clone(),
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
            super::rpc::proto::Modules::Chatfile.into(),
            "".to_string(),
        );
    }

    /// send file history list command via rpc
    fn send_file_history_command(offset: u32, limit: u32) {
        // create file history message
        let proto_message = proto::ChatFile {
            message: Some(proto::chat_file::Message::FileHistory(
                proto::FileHistoryRequest { offset, limit },
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
            super::rpc::proto::Modules::Chatfile.into(),
            "".to_string(),
        );
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the file share module.
    pub fn rpc(data: Vec<u8>) {
        match proto::ChatFile::decode(&data[..]) {
            Ok(file_share) => {
                match file_share.message {
                    Some(proto::chat_file::Message::FileHistoryResponse(proto_file_history)) => {
                        // List header
                        println!("====================================");
                        println!("File Sharing Histories");
                        println!("------------------------------------");
                        println!(
                            "offset={}, limit={}, total={}",
                            proto_file_history.offset,
                            proto_file_history.limit,
                            proto_file_history.total
                        );

                        // print all messages in the feed list
                        for entry in proto_file_history.histories {
                            println!("[{}] - {}", entry.file_id, entry.file_name);
                            println!("\t Time: {}, SenderId: {}", entry.time, entry.sender_id);
                            println!("\t Group Id: {}", entry.group_id);
                            println!(
                                "\t FileSize: {}, Description: {}",
                                entry.file_size, entry.file_description
                            );
                            println!("");
                        }
                    }
                    _ => {
                        log::error!("unprocessable RPC file message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
