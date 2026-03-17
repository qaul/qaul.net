// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC Debug Messages
//!
//! Messages to debug libqaul

use super::Rpc;
use crate::storage::configuration::Configuration;
use crate::storage::Storage;
use prost::Message;

/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_debug as proto;

/// RPC Debugging Module
pub struct Debug {}

impl Debug {
    /// Process incoming RPC request messages for debug module
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, _user_id: Vec<u8>, request_id: String) {
        match proto::Debug::decode(&data[..]) {
            Ok(debug) => {
                match debug.message {
                    Some(proto::debug::Message::HeartbeatRequest(_heartbeat_request)) => {
                        // create and return heartbeat message
                        let proto_message = proto::Debug {
                            message: Some(proto::debug::Message::HeartbeatResponse(
                                proto::HeartbeatResponse {},
                            )),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            state,
                            buf,
                            crate::rpc::proto::Modules::Debug.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto::debug::Message::Panic(_panic)) => {
                        // panic
                        log::error!("Libqaul will panic");
                        panic!("Libqaul panics for debugging reasons");
                    }
                    Some(proto::debug::Message::LogToFile(log_to_file)) => {
                        if log_to_file.enable {
                            // start log
                            state.filelogger.enable(true);
                            if Configuration::get_debug_log(state) == false {
                                Configuration::enable_debug_log(state, true);
                                Configuration::save(state);
                                log::info!("starting debug log..");
                            }
                        } else {
                            // stop log
                            if Configuration::get_debug_log(state) == true {
                                Configuration::enable_debug_log(state, false);
                                Configuration::save(state);
                                log::info!("stop debug log..");
                            }
                            state.filelogger.enable(false);
                        }
                    }
                    Some(proto::debug::Message::StoragePathRequest(_storage_path_request)) => {
                        // create and return storage path response message
                        let path = Storage::get_path(state);
                        let proto_message = proto::Debug {
                            message: Some(proto::debug::Message::StoragePathResponse(
                                proto::StoragePathResponse { storage_path: path },
                            )),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            state,
                            buf,
                            crate::rpc::proto::Modules::Debug.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    _ => {
                        log::error!("Unhandled RPC Debug Message");
                    }
                }
            }
            Err(e) => {
                log::error!("{:?}", e);
            }
        }
    }
}
