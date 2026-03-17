// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
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

#[cfg(target_os = "android")]
use crate::api::android::Android;

/// Instance-based SYS RPC state.
/// Replaces the global EXTERN_RECEIVE, EXTERN_SEND, LIBQAUL_SEND statics
/// for multi-instance use.
pub struct SysRpcState {
    /// Sending end for external → libqaul direction.
    pub extern_send: Sender<Vec<u8>>,
    /// Receiving end for libqaul → external direction.
    pub extern_receive: Receiver<Vec<u8>>,
    /// Sending end for libqaul → external direction.
    pub libqaul_send: Sender<Vec<u8>>,
    /// Receiving end for external → libqaul direction.
    pub libqaul_receive: Receiver<Vec<u8>>,
}

impl SysRpcState {
    /// Create a new SysRpcState with fresh channels.
    pub fn new() -> Self {
        let (libqaul_send, extern_receive) = unbounded();
        let (extern_send, libqaul_receive) = unbounded();
        Self {
            extern_send,
            extern_receive,
            libqaul_send,
            libqaul_receive,
        }
    }

    /// Send SYS message from external to libqaul (instance method).
    pub fn send_to_libqaul(&self, binary_message: Vec<u8>) {
        if let Err(err) = self.extern_send.send(binary_message) {
            log::error!("{:?}", err);
        }
    }

    /// Receive SYS message from libqaul to external (instance method).
    pub fn receive_from_libqaul(&self) -> Result<Vec<u8>, TryRecvError> {
        self.extern_receive.try_recv()
    }

    /// Send SYS message from libqaul to external (instance method).
    pub fn send_to_extern(&self, message: Vec<u8>) {
        if let Err(err) = self.libqaul_send.send(message) {
            log::error!("{:?}", err);
        }
    }
}

/// Handling of SYS messages of libqaul
pub struct Sys {}

impl Sys {
    /// Access the global SysRpcState from QaulState.
    fn state() -> &'static SysRpcState {
        &crate::QaulState::global().sys
    }

    /// Initialize SYS module.
    /// State is now owned by QaulState, so this is a no-op.
    pub fn init() {
        // State is created by QaulState::new(); nothing to do here.
    }

    /// send sys message from the outside to the inside
    /// of the worker thread of libqaul.
    pub fn send_to_libqaul(binary_message: Vec<u8>) {
        if let Err(err) = Self::state().extern_send.send(binary_message) {
            log::error!("{:?}", err);
        }
    }

    /// check the receiving sys channel if there
    /// are new messages from inside libqaul for
    /// the outside.
    pub fn receive_from_libqaul() -> Result<Vec<u8>, TryRecvError> {
        Self::state().extern_receive.try_recv()
    }

    /// send an rpc message from inside libqaul thread
    /// to the extern.
    #[allow(dead_code)]
    pub fn send_to_extern(message: Vec<u8>) {
        if let Err(err) = Self::state().libqaul_send.send(message) {
            log::error!("{:?}", err);
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
        // send to linux BLE module
        #[cfg(all(target_os = "linux", feature = "ble"))]
        ble_module::rpc::send_to_ble_module(data);

        // send to android BLE module
        #[cfg(target_os = "android")]
        Android::send_to_android(data);
    }
}
