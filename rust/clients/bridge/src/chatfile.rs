// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # ChatFile module functions

use super::rpc::Rpc;
use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.chatfile.rs");
}

/// Chat file module function handling
pub struct ChatFile {}

impl ChatFile {
    /// send file via rpc
    pub fn send_file(group_id: Vec<u8>, file_name: String, description: String) {
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
                        log::info!("====================================");
                        log::info!("File Sharing Histories");
                        log::info!("------------------------------------");
                        log::info!(
                            "offset={}, limit={}, total={}",
                            proto_file_history.offset,
                            proto_file_history.limit,
                            proto_file_history.total
                        );

                        // print all messages in the feed list
                        for entry in proto_file_history.histories {
                            log::info!("[{}] - {}", entry.file_id, entry.file_name);
                            log::info!("\t Time: {}, SenderId: {}", entry.time, entry.sender_id);
                            log::info!("\t Group Id: {}", entry.group_id);
                            log::info!(
                                "\t FileSize: {}, Description: {}",
                                entry.file_size,
                                entry.file_description
                            );
                            log::info!("");
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
