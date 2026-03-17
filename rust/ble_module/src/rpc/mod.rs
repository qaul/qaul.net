// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC Module
//!
//! Listens to incoming RPC messages on the `qaul.sys.ble` channel.

pub mod msg_loop;
pub mod utils;

use async_trait::async_trait;
use prost::Message;
pub use qaul_proto::qaul_sys_ble as proto_sys;
pub use qaul_proto::qaul_sys_ble::Ble;

#[async_trait]
pub trait SysRpcReceiver {
    async fn recv(&mut self) -> Option<Ble>;
}

pub struct BleRpc {
    pub receiver: tokio::sync::mpsc::Receiver<Vec<u8>>,
}

#[async_trait]
impl SysRpcReceiver for BleRpc {
    async fn recv(&mut self) -> Option<Ble> {
        self.receiver
            .recv()
            .await
            .and_then(process_received_message)
    }
}

/// Process received binary protobuf encoded SYS message
///
/// This function will decode the message from the binary
/// protobuf format to a rust struct and return it
pub fn process_received_message(data: Vec<u8>) -> Option<Ble> {
    match Ble::decode(&data[..]) {
        Ok(ble) => Some(ble),
        Err(err) => {
            log::error!("{:#?}", err);
            None
        }
    }
}
