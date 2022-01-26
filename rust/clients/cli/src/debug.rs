// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Node module functions

use libqaul;
use super::rpc::Rpc;

/// node module function handling
pub struct Debug {}

impl Debug {
    /// CLI command interpretation
    /// 
    /// The CLI commands of node module are processed here
    pub fn cli(command: &str) {
        match command {
            // node functions
            cmd if cmd.starts_with("rpc sent") => {
                Self::rpc_sent();
            },
            // node functions
            cmd if cmd.starts_with("rpc queued") => {
                Self::rpc_queued();
            },
            // let libqaul panic
            cmd if cmd.starts_with("panic") => {
                Self::panic();
            },
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

    /// send a debugging message to libqaul that
    /// let's it panic.
    fn panic() {
        // send empty debug message
        Rpc::send_message(Vec::new(), super::rpc::proto::Modules::Debug.into(), "".to_string());
    }
}