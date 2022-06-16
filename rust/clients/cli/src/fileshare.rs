// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # FileShare module functions

use prost::Message;
use super::rpc::Rpc;
use std::fmt;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.filesharing.rs"); }

/// FileShare module function handling
pub struct FileShare {}

impl FileShare {
    /// CLI command interpretation
    /// 
    /// The CLI commands of FileShare module are processed here
    pub fn cli(command: &str) {
        match command {
            // send file
            cmd if cmd.starts_with("send ") => {
                let command_string = cmd.strip_prefix("send ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(conversation_id_str) = iter.next() {
                    // convert conversation id from string to binary version
                    match Self::id_string_to_bin(conversation_id_str.to_string()) {
                        Ok(conversation_id) => {
                            if let Some(file_path_name) = iter.next() {

                                let descr = match iter.next() {
                                    Some(description)=>{description.to_string()},
                                    _ =>{ "".to_string()}
                                };

                                log::info!("send file peerid= {}, file={}, descr={}", conversation_id_str, file_path_name, descr);

                                Self::send_file(conversation_id, file_path_name.to_string(), descr);
                            }else{
                                log::error!("file pathname is not given");
                            }
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                }
                else {
                    log::error!("chat send command incorrectly formatted");
                }
            },

            // request fileShare history list
            cmd if cmd.starts_with("history ") => {
                let command_string = cmd.strip_prefix("history ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                let mut offset: i32 = 0;
                let mut limit: i32 = 10;                

                if let Some(offset_str) = iter.next() {
                    offset = offset_str.to_string().parse().unwrap();
                    if let Some(limit_str) = iter.next() {
                        limit = limit_str.to_string().parse().unwrap();
                    }
                }
                Self::send_file_history_commnad(offset as u32, limit as u32);

            },
            // unknown command
            _ => log::error!("unknown file command"),
        }
    }

    /// Convert Conversation ID from String to Binary
    fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
        // check length
        if id.len() < 52 {
            return Err("Conversation ID not long enough".to_string());
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

    /// send file via rpc
    fn send_file(conversation_id: Vec<u8>, file_name: String, description:String) {
        // create file send message
        let proto_message = proto::FileSharing {
            message: Some(proto::file_sharing::Message::SendFileRequest(
                proto::SendFileRequest{
                    path_name: file_name.clone(),
                    conversation_id: conversation_id.clone(),
                    description: description.clone()
                }
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Fileshare.into(), "".to_string());
    }

    /// send file history list command via rpc
    fn send_file_history_commnad(offset: u32, limit: u32){
        // create file history message
        let proto_message = proto::FileSharing {
            message: Some(proto::file_sharing::Message::FileHistory(
                proto::FileHistoryRequest{
                    offset,
                    limit,
                }
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Fileshare.into(), "".to_string());        
    }

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the file share module.
    pub fn rpc(data: Vec<u8>) {
        match proto::FileSharing::decode(&data[..]) {
            Ok(file_share) => {
                match file_share.message {
                    Some(proto::file_sharing::Message::FileHistoryResponse(proto_file_history)) => {
                        // List header
                        println!("====================================");
                        println!("File Sharing Histories");
                        println!("------------------------------------");
                        println!("offset={}, limit={}, total={}", proto_file_history.offset, proto_file_history.limit, proto_file_history.total);

                        // print all messages in the feed list
                        for entry in proto_file_history.histories {

                            if entry.sent{
                                println!("===> [{}] - {}", entry.file_id, entry.file_name);
                            }else{
                                println!("<=== [{}] - {}", entry.file_id, entry.file_name);
                            }                            
                            println!("\t Time: {}, PeerId: {}", entry.time, entry.peer_id);
                            println!("\t FileSize: {}, Description: {}", entry.file_size, entry.file_descr);
                            println!("");
                        }
                    }
                    _ => {
                        log::error!("unprocessable RPC file message");
                    },
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }
}