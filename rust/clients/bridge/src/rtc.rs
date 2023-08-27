// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # ChatFile module functions

use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rtc.rs");
}
mod proto_net {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.net.rtc.rs");
}

/// GrouChat module function handling
pub struct Rtc {}

impl Rtc {

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the group chat module.
    pub fn rpc(data: Vec<u8>) {
        match proto::RtcRpc::decode(&data[..]) {
            Ok(rtc) => {
                match rtc.message {
                    Some(proto::rtc_rpc::Message::RtcSessionResponse(resp)) => {
                        println!("====================================");
                        println!("Session was created");
                        println!("\tgroup id: {}", bs58::encode(resp.group_id).into_string());
                    }
                    Some(proto::rtc_rpc::Message::RtcIncoming(resp)) => {
                        //
                        println!("====================================");
                        println!("Rtc Incoming");
                        println!("\tgroup id: {}", bs58::encode(resp.group_id).into_string());

                        match proto_net::RtcContent::decode(&resp.content[..])
                            .unwrap()
                            .content
                        {
                            Some(proto_net::rtc_content::Content::ChatContent(chat_content)) => {
                                println!("\tchat content: {}", chat_content.content);
                            }
                            _ => {}
                        }
                    }
                    Some(proto::rtc_rpc::Message::RtcSessionListResponse(resp)) => {
                        // List sessions
                        println!("=============List Of Sessions=================");
                        for session in resp.sessions {
                            println!("group id: {}", bs58::encode(session.group_id).into_string());
                            println!(
                                "\ttype: {} , state: {} , created at: {}",
                                session.session_type, session.state, session.created_at
                            );
                        }
                    }
                    _ => {
                        log::error!("unprocessable RPC RTC message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
