// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Process SYS Messages
//!
//! The SYS messages are defined in the protobuf format.
//! They are used to send between libqaul and the host OS.
//!
//! They are used for the following modules:
//!
//! * BLE module

use crate::connections::ble::Ble;
use crate::connections::{internet::Internet, lan::Lan};
use crossbeam_channel::{unbounded, Receiver, Sender};
use state::Storage;

#[cfg(target_os = "android")]
use crate::api::android::Android;

/// sending end of the mpsc channel
static EXTERN_SEND: Storage<Sender<Vec<u8>>> = Storage::new();

/// Handling of SYS messages of libqaul
pub struct Sys {}

impl Sys {
    /// Initialize SYS module
    /// Create the sending and receiving channels and put them to state.
    /// Return the receiving channel for libqaul.
    pub fn init() -> Receiver<Vec<u8>> {
        // create channels
        //let (libqaul_send, extern_receive) = unbounded();
        let (extern_send, libqaul_receive) = unbounded();

        // save to state
        EXTERN_SEND.set(extern_send);

        // return libqaul receiving channel
        libqaul_receive
    }

    /// send SYS message from the outside to the inside
    /// of the worker thread of libqaul.
    pub fn send_to_libqaul(binary_message: Vec<u8>) {
        let sender = EXTERN_SEND.get().clone();
        match sender.send(binary_message) {
            Ok(()) => {}
            Err(err) => {
                // log error message
                log::error!("{:?}", err);
            }
        }
    }

    /// Process received binary protobuf encoded SYS message
    ///
    /// This function will decode the message from the binary
    /// protobuf format to rust structures and send it to
    /// the module responsible.
    pub fn process_received_message(
        data: Vec<u8>,
        _lan: Option<&mut Lan>,
        _internet: Option<&mut Internet>,
    ) {
        // as there is only BLE module just forward the data
        Ble::sys_received(data);
    }

    /// sends a SYS message from libqaul to the outside
    pub fn send_message(data: Vec<u8>) {
        // send to linux BLE module
        #[cfg(target_os = "linux")]
        ble_module::rpc::send_to_ble_module(data);

        // send to android BLE module
        #[cfg(target_os = "android")]
        Android::send_to_android(data);
    }
}
