// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Feed module functions

use prost::Message;
use super::rpc::Rpc;
use std::fmt;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.filesharing.rs"); }

/// feed module function handling
pub struct FileShare {}

impl FileShare {
    /// CLI command interpretation
    /// 
    /// The CLI commands of feed module are processed here
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

                                log::error!("send file peerid= {}, file={}, descr={}", conversation_id_str, file_path_name, descr);

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

                //Self::send_file(cmd.strip_prefix("send ").unwrap().to_string());
            },
            // request feed list
            cmd if cmd.starts_with("send list") => {
                // match cmd.strip_prefix("list ") {
                //     Some(index_str) => {
                //         if let Ok(index) = index_str.parse::<u64>() {
                //             // request messages
                //             Self::request_feed_list(index);
                //         }
                //         else {
                //             log::error!("feed list index is not a valid number");
                //         }
                //     },
                //     None => {
                //         // request all messages
                //         Self::request_feed_list(0);
                //     }
                // }
            },
            cmd if cmd.starts_with("receive list") => {

            },
            // unknown command
            _ => log::error!("unknown feed command"),
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

    /// create and send feed message via rpc
    fn send_file(conversation_id: Vec<u8>, file_name: String, description:String) {
        // create feed send message
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

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the feed module.
    pub fn rpc(data: Vec<u8>) {
        // match proto::Feed::decode(&data[..]) {
        //     Ok(feed) => {
        //         match feed.message {
        //             Some(proto::feed::Message::Received(proto_feedlist)) => {
        //                 // List header
        //                 println!("====================================");
        //                 println!("Received Feed Messages");
        //                 println!("------------------------------------");

        //                 // print all messages in the feed list
        //                 for message in proto_feedlist.feed_message {
        //                     print!{"[{}] ", message.index};
        //                     println!("Time Sent - {}", message.time_sent);
        //                     println!("Timestamp Sent - {}", message.timestamp_sent);
        //                     println!("Time Received - {}", message.time_received);
        //                     println!("Timestamp Received - {}", message.timestamp_received);
        //                     println!("Message ID {}", message.message_id_base58);
        //                     println!("From {}", message.sender_id_base58);
        //                     println!("\t{}", message.content);
        //                     println!("");
        //                 }
        //             }
        //             _ => {
        //                 log::error!("unprocessable RPC feed message");
        //             },
        //         }    
        //     },
        //     Err(error) => {
        //         log::error!("{:?}", error);
        //     },
        // }
    }
}