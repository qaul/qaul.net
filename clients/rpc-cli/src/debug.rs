//! # Node module functions

use libqaul;

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
}