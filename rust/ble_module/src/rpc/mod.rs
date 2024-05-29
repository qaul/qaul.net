// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC Module
//!
//! Listens to incoming RPC messages on the `qaul.sys.ble` channel.

// TODO: remove local import, import instead from libqaul
pub mod proto_sys {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.sys.ble.rs");
}
pub mod msg_loop;
pub mod utils;

use async_trait::async_trait;
use prost::Message;
use state::InitCell;

use proto_sys::Ble;

/// sender of the mpsc channel: libqaul ---> ble_module
static EXTERN_SEND: InitCell<async_std::channel::Sender<Vec<u8>>> = InitCell::new();

#[async_trait]
pub trait SysRpcReceiver {
    async fn recv(&mut self) -> Option<Ble>;
}

pub struct BleRpc {
    receiver: async_std::channel::Receiver<Vec<u8>>,
}

#[async_trait]
impl SysRpcReceiver for BleRpc {
    async fn recv(&mut self) -> Option<Ble> {
        self.receiver
            .recv()
            .await
            .ok()
            .map(&process_received_message)
            .flatten()
    }
}

/// Initialize RPC module
/// Create the sending and receiving channels and persist them across threads.
/// Return the receiver for the channel libqaul ---> ble_module
pub fn init() -> BleRpc {
    // create channels
    let (libqaul_send, ble_rec) = async_std::channel::bounded(32);
    // save to state
    EXTERN_SEND.set(libqaul_send);

    // return ble receiver
    BleRpc { receiver: ble_rec }
}

/// send sys message libqaul ---> ble_module
#[allow(dead_code)]
pub fn send_to_ble_module(binary_message: Vec<u8>) {
    if let Err(err) = EXTERN_SEND
        .try_get()
        .ok_or("Sender libqaul ---> ble_module not yet initialized!")
        .map(|sender| sender.try_send(binary_message))
    {
        log::error!("{:?}", err);
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
