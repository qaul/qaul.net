// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Feed module functions

use super::rpc::Rpc;
use prost::Message;
use tokio::runtime::Runtime;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.feed.rs");
}

use crate::{relay_bot::{MATRIX_CLIENT, MATRIX_CONFIG}, configuration::MatrixConfiguration};

use matrix_sdk::{
    room::Room,
    ruma::{
        events::{room::message::MessageEventContent, AnyMessageEventContent},
        RoomId,
    },
};

/// feed module function handling
pub struct Feed {}

impl Feed {

    /// request feed list via rpc
    pub fn request_feed_list(last_index: u64) {
        // create feed list request message
        let proto_message = proto::Feed {
            message: Some(proto::feed::Message::Request(
                proto::FeedMessageRequest{
                    last_received: Vec::new(),
                    last_index,
                }
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Feed.into(), "".to_string());
    }

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the feed module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Feed::decode(&data[..]) {
            Ok(feed) => {
                match feed.message {
                    Some(proto::feed::Message::Received(proto_feedlist)) => {
                        // The configuration Object from matrix.yaml
                        let mut config = MATRIX_CONFIG.get().write().unwrap();
                        let last_index_matrix = config.feed.last_index;
                        // print all messages in the feed list
                        for message in proto_feedlist.feed_message {
                            print!{"[{}] ", message.index};
                            println!("Time Sent - {}", message.time_sent);
                            println!("Timestamp Sent - {}", message.timestamp_sent);
                            println!("Time Received - {}", message.time_received);
                            println!("Timestamp Received - {}", message.timestamp_received);
                            println!("Message ID {}", message.message_id_base58);
                            println!("From {}", message.sender_id_base58);
                            println!("\t{}", message.content);
                            println!("");
                            if message.index> last_index_matrix {
                                Self::matrix_send(message.content);
                                config.feed.last_index = message.index;
                                MatrixConfiguration::save(config.clone());
                            }      
                        }
                        // MATRIX_CONFIG.set(config.clone().into()) is not helping to save;
                        MATRIX_CONFIG.set(config.clone().into());
                    }
                    _ => {
                        log::error!("unprocessable RPC feed message");
                    },
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }

    fn matrix_send(message: String) {
        // Get the Room based on RoomID from the client information
        let matrix_client = MATRIX_CLIENT.get();
        let room_id = RoomId::try_from("!nGnOGFPgRafNcUAJJA:matrix.org").unwrap();
        let room = matrix_client.get_room(&room_id).unwrap();
        // Check if the room is already joined or not
        if let Room::Joined(room) = room {
            // Build the message content to send to matrix
            let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                message,
            ));
 
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                // Sends messages into the matrix room
                room.send(content, None).await.unwrap();
            });
        }
    }
}
