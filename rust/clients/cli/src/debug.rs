// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Debug module functions

use super::rpc::Rpc;
use libqaul;
use prost::Message;

/// include generated protobuf RPC rust definition file
mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.debug.rs");
}

/// debug module function handling
pub struct Debug {}

impl Debug {
    /// CLI command interpretation
    ///
    /// The CLI commands of debug module are processed here
    pub fn cli(command: &str) {
        match command {
            // print sent rpc messages
            cmd if cmd.starts_with("rpc sent") => {
                Self::rpc_sent();
            }
            // print queued rpc messages
            cmd if cmd.starts_with("rpc queued") => {
                Self::rpc_queued();
            }
            // send a heartbeat
            cmd if cmd.starts_with("heartbeat") => {
                Self::heartbeat_send();
            }
            // let libqaul panic
            cmd if cmd.starts_with("panic") => {
                Self::panic();
            }
            // enable libqaul logging to file
            cmd if cmd.starts_with("log enable") => {
                Self::debug_log_enable_send();
            }
            // disable libqaul logging to file
            cmd if cmd.starts_with("log disable") => {
                Self::debug_log_disable_send();
            }
            // request storage path location
            cmd if cmd.starts_with("path") => {
                Self::debug_path_send();
            }
            // unknown command
            _ => log::error!("unknown debug command"),
        }
    }

    /// print rpc message counter of messages sent
    /// from client to libqaul
    fn rpc_sent() {
        let count = libqaul::api::send_rpc_count();
        println!("{} RPC messages sent by this client to libqaul", count);
    }

    /// print rpc message counter of queued messages
    /// in the message output of libqaul
    fn rpc_queued() {
        let count = libqaul::api::receive_rpc_queued();
        println!("{} RPC messages in libqaul's queue", count);
    }

    /// print rpc message counter of messages sent
    /// from client to libqaul
    fn heartbeat_send() {
        // create heartbeat message
        let proto_message = proto::Debug {
            message: Some(proto::debug::Message::HeartbeatRequest(
                proto::HeartbeatRequest {},
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Debug.into(),
            "".to_string(),
        );
    }

    fn debug_log_enable_send() {
        // create log enable message
        let proto_message = proto::Debug {
            message: Some(proto::debug::Message::LogToFile(proto::LogToFile {
                enable: true,
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Debug.into(),
            "".to_string(),
        );
    }

    fn debug_log_disable_send() {
        // create log enable message
        let proto_message = proto::Debug {
            message: Some(proto::debug::Message::LogToFile(proto::LogToFile {
                enable: false,
            })),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Debug.into(),
            "".to_string(),
        );
    }

    fn debug_path_send() {
        // create StoragePathRequest
        let proto_message = proto::Debug {
            message: Some(proto::debug::Message::StoragePathRequest(
                proto::StoragePathRequest {},
            )),
        };
        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Debug.into(),
            "".to_string(),
        );
    }

    /// send a debugging message to libqaul that
    /// let's it panic.
    fn panic() {
        // create panic message
        let proto_message = proto::Debug {
            message: Some(proto::debug::Message::Panic(proto::Panic {})),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            super::rpc::proto::Modules::Debug.into(),
            "".to_string(),
        );
    }

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
                    }
                    Some(proto::debug::Message::StoragePathResponse(storage_path_response)) => {
                        // printout path
                        println!("Storage Path: {}", storage_path_response.storage_path);
                    }
                    _ => {
                        log::error!("unprocessable RPC debug message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
