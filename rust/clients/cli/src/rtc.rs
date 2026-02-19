// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # ChatFile module functions

use super::rpc::Rpc;
use prost::Message;
use std::fmt;

use qaul_proto::qaul_net_rtc as proto_net;
/// protobuf RPC definition
use qaul_proto::qaul_rpc_rtc as proto;

/// GrouChat module function handling
pub struct Rtc {}

impl Rtc {
    /// CLI command interpretation
    ///
    /// The CLI commands of RTC module are processed here
    pub fn cli(command: &str) {
        match command {
            // rtc session request
            cmd if cmd.starts_with("request ") => {
                let command_string = cmd.strip_prefix("request ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(user_id_str) = iter.next() {
                    match Self::id_string_to_bin(user_id_str.to_string()) {
                        Ok(user_id) => {
                            Self::request_session(user_id);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("rtc request command incorrectly formatted");
                }
            }

            // rtc session accept
            cmd if cmd.starts_with("accept ") => {
                let command_string = cmd.strip_prefix("accept ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::id_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            Self::accept_session(group_id);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("rtc accept command incorrectly formatted");
                }
            }

            // rtc session accept
            cmd if cmd.starts_with("decline ") => {
                let command_string = cmd.strip_prefix("decline ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::id_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            Self::decline_session(group_id);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("rtc decline command incorrectly formatted");
                }
            }

            // rtc session accept
            cmd if cmd.starts_with("end ") => {
                let command_string = cmd.strip_prefix("end ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::id_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            Self::end_session(group_id);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("rtc end command incorrectly formatted");
                }
            }

            // rtc send message
            cmd if cmd.starts_with("send ") => {
                let command_string = cmd.strip_prefix("send ").unwrap().to_string();
                let mut iter = command_string.split_whitespace();

                if let Some(group_id_str) = iter.next() {
                    match Self::id_string_to_bin(group_id_str.to_string()) {
                        Ok(group_id) => {
                            if let Some(message_str) = iter.next() {
                                Self::send_session(group_id, message_str.to_string());
                            } else {
                                log::error!("rtc send command no contents");
                            }
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            return;
                        }
                    }
                } else {
                    log::error!("rtc send command incorrectly formatted");
                }
            }

            // rtc send message
            cmd if cmd.starts_with("list") => {
                Self::list_session();
            }

            // unknown command
            _ => log::error!("unknown group command"),
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

    /// session request
    fn request_session(user_id: Vec<u8>) {
        // create group send message
        let proto_message = proto::RtcRpc {
            message: Some(proto::rtc_rpc::Message::RtcSessionRequest(
                proto::RtcSessionRequest {
                    group_id: user_id.clone(),
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Rtc.into(), "".to_string());
    }

    /// accept session
    fn accept_session(group_id: Vec<u8>) {
        // create group send message
        let proto_message = proto::RtcRpc {
            message: Some(proto::rtc_rpc::Message::RtcSessionManagement(
                proto::RtcSessionManagement {
                    group_id: group_id.clone(),
                    option: 1,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Rtc.into(), "".to_string());
    }

    /// decline session
    fn decline_session(group_id: Vec<u8>) {
        // decline session message
        let proto_message = proto::RtcRpc {
            message: Some(proto::rtc_rpc::Message::RtcSessionManagement(
                proto::RtcSessionManagement {
                    group_id: group_id.clone(),
                    option: 2,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Rtc.into(), "".to_string());
    }

    /// end session
    fn end_session(group_id: Vec<u8>) {
        // end session message
        let proto_message = proto::RtcRpc {
            message: Some(proto::rtc_rpc::Message::RtcSessionManagement(
                proto::RtcSessionManagement {
                    group_id: group_id.clone(),
                    option: 3,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Rtc.into(), "".to_string());
    }

    /// send session
    fn send_session(group_id: Vec<u8>, message: String) {
        let rtc_content = proto_net::RtcContent {
            content: Some(proto_net::rtc_content::Content::ChatContent(
                proto_net::RtcChatContent {
                    content: message.clone(),
                },
            )),
        };

        // session message
        let mut buf0 = Vec::with_capacity(rtc_content.encoded_len());
        rtc_content
            .encode(&mut buf0)
            .expect("Vec<u8> provides capacity as needed");

        let proto_message = proto::RtcRpc {
            message: Some(proto::rtc_rpc::Message::RtcOutgoing(proto::RtcOutgoing {
                group_id: group_id.clone(),
                content: buf0,
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Rtc.into(), "".to_string());
    }

    /// session list
    fn list_session() {
        let proto_message = proto::RtcRpc {
            message: Some(proto::rtc_rpc::Message::RtcSessionListRequest(
                proto::RtcSessionListRequest {},
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, super::rpc::proto::Modules::Rtc.into(), "".to_string());
    }

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
