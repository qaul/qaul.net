// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Debug module functions

use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto { include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.debug.rs"); }

/// debug module function handling
pub struct Debug {}

impl Debug {

    /// Process received RPC message
    /// 
    /// Decodes received protobuf encoded binary RPC message
    /// of the debug module.
    pub fn rpc(data: Vec<u8>) {
        match proto::Debug::decode(&data[..]) {
            Ok(debug) => {
                match debug.message {
                    Some(proto::debug::Message::HeartbeatResponse(_heartbeat)) => {
                        // print confirmation
                        println!("Heartbeat response received");
                    },
                    Some(proto::debug::Message::StoragePathResponse(storage_path_response)) => {
                        // printout path
                        println!("Storage Path: {}", storage_path_response.storage_path);
                    },
                    _ => {
                        log::error!("unprocessable RPC debug message");
                    },
                }    
            },
            Err(error) => {
                log::error!("{:?}", error);
            },
        }
    }
}