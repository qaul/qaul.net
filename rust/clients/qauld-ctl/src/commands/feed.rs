use crate::{
    cli::FeedSubcmd,
    commands::RpcCommand,
    proto::{feed, Feed, FeedMessageRequest, Modules, SendMessage},
};
use prost::Message;

/// request feed list via rpc
fn request_feed_list(last_index: u64) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
    // create feed list request message
    let proto_message = Feed {
        message: Some(feed::Message::Request(FeedMessageRequest {
            last_received: Vec::new(),
            last_index,
        })),
    };

    // encode message
    let mut buf = Vec::with_capacity(proto_message.encoded_len());
    proto_message
        .encode(&mut buf)
        .expect("Vec<u8> provides capacity as needed");

    // send message
    Ok((buf, Modules::Feed))
}

impl RpcCommand for FeedSubcmd {
    fn expects_response(&self) -> bool {
        !matches!(&self, FeedSubcmd::Send { .. })
    }

    fn encode_request(&self) -> Result<(Vec<u8>, Modules), Box<dyn std::error::Error>> {
        match &self {
            FeedSubcmd::Send { message } => {
                let proto_message = Feed {
                    message: Some(feed::Message::Send(SendMessage {
                        content: message.to_string(),
                    })),
                };

                // encode message
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");

                Ok((buf, Modules::Feed))
            }
            FeedSubcmd::List { feed_message_id } => match feed_message_id {
                Some(id) => request_feed_list(*id),
                None => request_feed_list(0),
            },
        }
    }
    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let feed = Feed::decode(data)?;
        match feed.message {
            Some(feed::Message::Received(feed_list)) => {
                // List header
                println!("====================================");
                println!("Received Feed Messages");
                println!("------------------------------------");

                // print all messages in the feed list
                for message in feed_list.feed_message {
                    print! {"[{}] ", message.index};
                    println!("Time Sent - {}", message.time_sent);
                    println!("Timestamp Sent - {}", message.timestamp_sent);
                    println!("Time Received - {}", message.time_received);
                    println!("Timestamp Received - {}", message.timestamp_received);
                    println!("Message ID {}", message.message_id_base58);
                    println!("From {}", message.sender_id_base58);
                    println!("\t{}", message.content);
                    println!("");
                }
            }
            _ => {
                log::error!("unprocessable RPC feed message");
            }
        };
        Ok(())
    }
}
