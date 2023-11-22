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

// use core::slice::SlicePattern;

use crate::connections::ble::Ble;
use crate::connections::{internet::Internet, lan::Lan};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use state::InitCell;

#[cfg(target_os = "android")]
use crate::api::android::Android;

/// receiving end of the mpsc channel
static EXTERN_RECEIVE: InitCell<Receiver<Vec<u8>>> = InitCell::new();
/// sending end of the mpsc channel
static EXTERN_SEND: InitCell<Sender<Vec<u8>>> = InitCell::new();
/// sending end of th mpsc channel for libqaul to send
static LIBQAUL_SEND: InitCell<Sender<Vec<u8>>> = InitCell::new();

/// Handling of SYS messages of libqaul
pub struct Sys {}

impl Sys {
    /// Initialize SYS module
    /// Create the sending and receiving channels and put them to state.
    /// Return the receiving channel for libqaul.
    pub fn init() -> Receiver<Vec<u8>> {
        // create channels
        let (libqaul_send, extern_receive) = unbounded();
        let (extern_send, libqaul_receive) = unbounded();

        // save to state
        EXTERN_RECEIVE.set(extern_receive);
        EXTERN_SEND.set(extern_send);
        LIBQAUL_SEND.set(libqaul_send.clone());

        // return libqaul receiving channel
        libqaul_receive
    }

    /// send sys message from the outside to the inside
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

    /// check the receiving sys channel if there
    /// are new messages from inside libqaul for
    /// the outside.
    pub fn receive_from_libqaul() -> Result<Vec<u8>, TryRecvError> {
        let receiver = EXTERN_RECEIVE.get().clone();
        receiver.try_recv()
    }

    /// send an rpc message from inside libqaul thread
    /// to the extern.
    #[allow(dead_code)]
    pub fn send_to_extern(message: Vec<u8>) {
        let sender = LIBQAUL_SEND.get().clone();
        match sender.send(message) {
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

    /// sends a SYS message to the outside
    #[allow(unused_variables)]
    pub fn send_message(data: Vec<u8>) {
        // send the message
        #[cfg(target_os = "android")]
        Android::send_to_android(data);
    }
}
