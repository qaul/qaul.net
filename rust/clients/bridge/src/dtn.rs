// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group module functions

use prost::Message;
/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.dtn.rs");
}

/// Group module function handling
pub struct Dtn {}

impl Dtn {

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
