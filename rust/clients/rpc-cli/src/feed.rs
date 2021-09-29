//! # Feed module functions

use prost::Message;
use super::rpc::Rpc;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.feed.rs"); }

/// feed module function handling
pub struct Feed {}

impl Feed {
    /// CLI command interpretation
    /// 
    /// The CLI commands of feed module are processed here
    pub fn cli(command: &str) {
        match command {
            // send feed message
            cmd if cmd.starts_with("send ") => {
                Self::send_feed_message(cmd.strip_prefix("send ").unwrap().to_string());
            },
            // request feed list
            cmd if cmd.starts_with("list") => {
                Self::request_feed_list();
            },
            // unknown command
            _ => log::error!("unknown feed command"),
        }
    }

    /// create and send feed message via rpc
    fn send_feed_message(message_text: String) {
        // create feed send message
        let proto_message = proto::Feed {
            message: Some(proto::feed::Message::Send(
                proto::SendMessage{
                    content: message_text,
                }
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Feed.into(), "".to_string());
    }

    /// request feed list via rpc
    fn request_feed_list() {
        // create feed list request message
        let proto_message = proto::Feed {
            message: Some(proto::feed::Message::Request(
                proto::FeedMessageRequest{
                    last_received: Vec::new(),
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
                        // List header
                        println!("====================================");
                        println!("Received Feed Messages");
                        println!("------------------------------------");

                        // print all messages in the feed list
                        for message in proto_feedlist.feed_message {
                            println!("{}", message.time_sent);
                            println!("Message ID {}", message.message_id_base58);
                            println!("From {}", message.sender_id_base58);
                            println!("\t{}", message.content);
                            println!("");
                        }
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
}