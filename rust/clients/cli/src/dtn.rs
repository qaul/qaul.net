// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group module functions

use super::rpc::Rpc;
use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.dtn.rs");
}

/// Group module function handling
pub struct Dtn {}

impl Dtn {
    /// CLI command interpretation
    ///
    /// The CLI commands of group module are processed here
    pub fn cli(command: &str) {
        match command {
            // create group
            cmd if cmd.starts_with("state") => {
                Self::dtn_state();
            }
            // unknown command
            _ => log::error!("unknown group command"),
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
                    println!("\tUsed Size: {}", dtn_state.used_size);
                    println!("\tDTN Messages: {}", dtn_state.dtn_message_count);
                    println!("\tUnconfirmed Messages: {}", dtn_state.unconfirmed_count);
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
