// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group module functions

use super::rpc::Rpc;
use prost::Message;
use std::fmt;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.dtn.rs");
}

/// Group module function handling
pub struct Dtn {}

impl Dtn {
    /// CLI command interpretation
    ///
    /// The CLI commands of dtn are processed here
    pub fn cli(command: &str) {
        match command {
            // state
            cmd if cmd.starts_with("state") => {
                Self::dtn_state();
            }
            // config
            cmd if cmd.starts_with("config") => {
                Self::dtn_config();
            }
            // add user
            cmd if cmd.starts_with("add ") => match cmd.strip_prefix("add ") {
                Some(user_id_str) => {
                    if let Ok(id) = Self::id_string_to_bin(user_id_str.to_string()) {
                        Self::dtn_add_user(id);
                    } else {
                        log::error!("invalid user id");
                    }
                }
                None => {
                    log::error!("invalid command parameter");
                }
            },
            // remove user
            cmd if cmd.starts_with("remove ") => match cmd.strip_prefix("remove ") {
                Some(user_id_str) => {
                    if let Ok(id) = Self::id_string_to_bin(user_id_str.to_string()) {
                        Self::dtn_remove_user(id);
                    } else {
                        log::error!("invalid user id");
                    }
                }
                None => {
                    log::error!("invalid command parameter");
                }
            },
            // set maximum storage size
            cmd if cmd.starts_with("size ") => match cmd.strip_prefix("size ") {
                Some(total_size_str) => {
                    if let Ok(total_size) = total_size_str.parse::<u32>() {
                        Self::dtn_total_size(total_size);
                    } else {
                        log::error!("invalid storage size");
                    }
                }
                None => {
                    log::error!("invalid command parameter");
                }
            },
            // unknown command
            _ => log::error!("unknown dtn command"),
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

    /// dtn state
    fn dtn_state() {
        // create group send message
        let proto_message = proto::Dtn {
            message: Some(proto::dtn::Message::DtnStateRequest(
                proto::DtnStateRequest {},
            )),
        };
        // send message
        Rpc::send_message(
            proto_message.encode_to_vec(),
            super::rpc::proto::Modules::Dtn.into(),
            "".to_string(),
        );
    }

    /// dtn state
    fn dtn_config() {
        // create group send message
        let proto_message = proto::Dtn {
            message: Some(proto::dtn::Message::DtnConfigRequest(
                proto::DtnConfigRequest {},
            )),
        };
        // send message
        Rpc::send_message(
            proto_message.encode_to_vec(),
            super::rpc::proto::Modules::Dtn.into(),
            "".to_string(),
        );
    }

    /// dtn add user
    fn dtn_add_user(user_id: Vec<u8>) {
        // create group send message
        let proto_message = proto::Dtn {
            message: Some(proto::dtn::Message::DtnAddUserRequest(
                proto::DtnAddUserRequest { user_id },
            )),
        };
        // send message
        Rpc::send_message(
            proto_message.encode_to_vec(),
            super::rpc::proto::Modules::Dtn.into(),
            "".to_string(),
        );
    }

    /// dtn remove user
    fn dtn_remove_user(user_id: Vec<u8>) {
        // create group send message
        let proto_message = proto::Dtn {
            message: Some(proto::dtn::Message::DtnRemoveUserRequest(
                proto::DtnRemoveUserRequest { user_id },
            )),
        };
        // send message
        Rpc::send_message(
            proto_message.encode_to_vec(),
            super::rpc::proto::Modules::Dtn.into(),
            "".to_string(),
        );
    }

    /// dtn total_size
    fn dtn_total_size(total_size: u32) {
        // create group send message
        let proto_message = proto::Dtn {
            message: Some(proto::dtn::Message::DtnSetTotalSizeRequest(
                proto::DtnSetTotalSizeRequest { total_size },
            )),
        };
        // send message
        Rpc::send_message(
            proto_message.encode_to_vec(),
            super::rpc::proto::Modules::Dtn.into(),
            "".to_string(),
        );
    }

    /// Process received RPC message
    ///
    /// Decodes received protobuf encoded binary RPC message
    /// of the group chat module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Dtn::decode(&data[..]) {
            Ok(dtn) => match dtn.message {
                Some(proto::dtn::Message::DtnStateResponse(dtn_state)) => {
                    println!("====================================");
                    println!("DTN State");
                    println!("\tUsed Storage Size: {} MB", dtn_state.used_size);
                    println!("\tDTN Messages: {}", dtn_state.dtn_message_count);
                    println!("\tUnconfirmed Messages: {}", dtn_state.unconfirmed_count);
                }
                Some(proto::dtn::Message::DtnConfigResponse(dtn_config)) => {
                    println!("====================================");
                    println!("DTN Options");
                    println!("\tMaximum Storage Size: {} MB", dtn_config.total_size);
                    println!("\tUsers");
                    for user in dtn_config.users {
                        println!("\t\t{}", bs58::encode(user).into_string());
                    }
                }
                Some(proto::dtn::Message::DtnAddUserResponse(dtn_add_user_resp)) => {
                    println!("====================================");
                    println!("DTN Add User");
                    if dtn_add_user_resp.status {
                        println!("\tSuccess");
                    } else {
                        println!("\tFailed");
                        println!("\t{}", dtn_add_user_resp.message);
                    }
                }
                Some(proto::dtn::Message::DtnRemoveUserResponse(dtn_remove_user_resp)) => {
                    println!("====================================");
                    println!("DTN Add User");
                    if dtn_remove_user_resp.status {
                        println!("\tSuccess");
                    } else {
                        println!("\tFailed");
                        println!("\t{}", dtn_remove_user_resp.message);
                    }
                }
                Some(proto::dtn::Message::DtnSetTotalSizeResponse(resp)) => {
                    println!("====================================");
                    println!("DTN Add User");
                    if resp.status {
                        println!("\tSuccess");
                    } else {
                        println!("\tFailed");
                        println!("\t{}", resp.message);
                    }
                }
                _ => {
                    log::error!("unprocessable RPC group chat message");
                }
            },
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
