// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Process RPC Messages
//!
//! The RPC messages are defined in the protobuf format.
//! The format is then translated to rust program code.

pub mod authentication;
pub mod debug;
pub mod sys;

use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use std::sync::RwLock;

use prost::Message;

use crate::connections::ble::Ble;
use crate::connections::Connections;
use crate::connections::{internet::Internet, lan::Lan};
use crate::node::user_accounts::UserAccounts;
use crate::node::Node;
use crate::router::users::Users;
use crate::router::Router;
use crate::services::chat::Chat;
use crate::services::chat::ChatFile;
use crate::services::dtn::Dtn;
use crate::services::feed::Feed;
use crate::services::group::Group;
use debug::Debug;

#[cfg(feature = "rtc")]
use crate::services::rtc::Rtc;

use proto::{Modules, QaulRpc};
/// Import protobuf message definition
pub use qaul_proto::qaul_rpc as proto;

/// counter of received messages
/// this is for bug fixing only
pub struct MessageCounter {
    count: i32,
}

/// Instance-based RPC state.
/// Replaces the global EXTERN_SEND_COUNT, EXTERN_RECEIVE, EXTERN_SEND,
/// LIBQAUL_SEND statics for multi-instance use.
pub struct RpcState {
    /// Message counter (for debugging).
    pub send_count: RwLock<MessageCounter>,
    /// Sending end for external → libqaul direction.
    pub extern_send: Sender<Vec<u8>>,
    /// Receiving end for external → libqaul direction.
    pub extern_receive: Receiver<Vec<u8>>,
    /// Sending end for libqaul → external direction.
    pub libqaul_send: Sender<Vec<u8>>,
    /// Receiving end for libqaul → external direction.
    pub libqaul_receive: Receiver<Vec<u8>>,
}

impl RpcState {
    /// Create a new RpcState with fresh channels.
    pub fn new() -> Self {
        let (libqaul_send, extern_receive) = unbounded();
        let (extern_send, libqaul_receive) = unbounded();
        Self {
            send_count: RwLock::new(MessageCounter { count: 0 }),
            extern_send,
            extern_receive,
            libqaul_send,
            libqaul_receive,
        }
    }

    /// Send RPC message from external to libqaul (instance method).
    pub fn send_to_libqaul(&self, binary_message: Vec<u8>) {
        if let Err(err) = self.extern_send.send(binary_message) {
            log::error!("{:?}", err);
        }
    }

    /// Receive RPC message from libqaul to external (instance method).
    pub fn receive_from_libqaul(&self) -> Result<Vec<u8>, TryRecvError> {
        self.extern_receive.try_recv()
    }

    /// Send RPC message from libqaul to external (instance method).
    pub fn send_to_extern(&self, message: Vec<u8>) {
        if let Err(err) = self.libqaul_send.send(message) {
            log::error!("{:?}", err);
        }
    }

    /// Send a wrapped RPC message to external (instance method).
    pub fn send_message(&self, data: Vec<u8>, module: i32, request_id: String, user_id: Vec<u8>) {
        let buf = build_rpc_message(data, module, request_id, user_id);
        self.send_to_extern(buf);
    }

    /// Increase message counter (instance method).
    pub fn increase_message_counter(&self) {
        let mut counter = self.send_count.write().unwrap();
        counter.count = counter.count + 1;
    }
}

/// RPC Module - wrapper for instance-based RPC channel management
///
/// This struct provides instance-based access to RPC channels.
/// The channels are created during initialization and the receiver
/// is stored in the `Libqaul` struct.
pub struct RpcModule {
    /// Whether RPC module has been initialized
    initialized: bool,
}

impl RpcModule {
    /// Create a new RpcModule (instance-based)
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Check if RPC module is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Mark as initialized
    pub fn set_initialized(&mut self) {
        self.initialized = true;
    }
}

impl Default for RpcModule {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a protobuf-encoded RPC message container (shared inner logic).
fn build_rpc_message(data: Vec<u8>, module: i32, request_id: String, user_id: Vec<u8>) -> Vec<u8> {
    let proto_message = proto::QaulRpc {
        module,
        request_id,
        user_id,
        data,
    };

    let mut buf = Vec::with_capacity(proto_message.encoded_len());
    proto_message
        .encode(&mut buf)
        .expect("Vec<u8> provides capacity as needed");
    buf
}

/// Handling of RPC messages of libqaul (global state - for backward compatibility)
pub struct Rpc {}

impl Rpc {
    /// Initialize RPC module.
    /// State is now owned by QaulState, so this is a no-op.
    pub fn init() {
        // State is created by QaulState::new(); nothing to do here.
    }

    /// send rpc message from the outside to the inside
    /// of the worker thread of libqaul.
    pub fn send_to_libqaul(state: &crate::QaulState, binary_message: Vec<u8>) {
        if let Err(err) = state.rpc.extern_send.send(binary_message) {
            log::error!("{:?}", err);
        }
    }

    /// check the receiving rpc channel if there
    /// are new messages from inside libqaul for
    /// the outside.
    pub fn receive_from_libqaul(state: &crate::QaulState) -> Result<Vec<u8>, TryRecvError> {
        state.rpc.extern_receive.try_recv()
    }

    /// get the number of messages in the receiving cue
    pub fn receive_from_libqaul_queue_length(state: &crate::QaulState) -> usize {
        state.rpc.extern_receive.len()
    }

    /// send an rpc message from inside libqaul thread
    /// to the extern.
    pub fn send_to_extern(state: &crate::QaulState, message: Vec<u8>) {
        if let Err(err) = state.rpc.libqaul_send.send(message) {
            log::error!("{:?}", err);
        }
    }

    /// Process received binary protobuf encoded RPC message
    ///
    /// This function will decode the message from the binary
    /// protobuf format to rust structures and send it to
    /// the module responsible.
    pub async fn process_received_message(
        state: &crate::QaulState,
        data: Vec<u8>,
        lan: Option<&mut Lan>,
        internet: Option<&mut Internet>,
    ) {
        Self::increase_message_counter(state);

        match QaulRpc::decode(&data[..]) {
            Ok(message) => {
                match Modules::try_from(message.module) {
                    Ok(Modules::Node) => {
                        Self::increase_message_counter(state);
                        Node::rpc(state, message.data, lan, internet, message.request_id);
                    }
                    Ok(Modules::Rpc) => {
                        log::trace!("Message Modules::Rpc received");
                        // TODO: authorisation
                    }
                    Ok(Modules::Useraccounts) => {
                        UserAccounts::rpc(state, message.data, message.user_id, message.request_id);
                    }
                    Ok(Modules::Users) => {
                        let rs = state.get_router();
                        Users::rpc(state, &rs, message.data, message.user_id, message.request_id);
                    }
                    Ok(Modules::Router) => {
                        let rs = state.get_router();
                        Router::rpc(state, &rs, message.data, message.request_id);
                    }
                    Ok(Modules::Feed) => {
                        Feed::rpc(
                            state,
                            message.data,
                            message.user_id,
                            lan,
                            internet,
                            message.request_id,
                        );
                    }
                    Ok(Modules::Connections) => {
                        Connections::rpc(state, message.data, internet, message.request_id);
                    }
                    Ok(Modules::Ble) => {
                        Ble::rpc(state, message.data, message.request_id);
                    }
                    Ok(Modules::Debug) => {
                        Debug::rpc(state, message.data, message.user_id, message.request_id);
                    }
                    Ok(Modules::Chat) => {
                        Chat::rpc(
                            state,
                            message.data,
                            message.user_id,
                            lan,
                            internet,
                            message.request_id,
                        );
                    }
                    Ok(Modules::Chatfile) => {
                        log::trace!("Message Modules::Chatfile received");
                        ChatFile::rpc(state, message.data, message.user_id, message.request_id).await;
                    }
                    Ok(Modules::Group) => {
                        log::trace!("Message Modules::Group received");
                        Group::rpc(state, message.data, message.user_id, message.request_id);
                    }
                    Ok(Modules::Rtc) => {
                        #[cfg(feature = "rtc")]
                        {
                            log::trace!("Message Modules::Rtc received");
                            Rtc::rpc(state, message.data, message.user_id, message.request_id);
                        }
                        #[cfg(not(feature = "rtc"))]
                        {
                            log::warn!("Received Modules::Rtc message, but the 'rtc' feature is not enabled");
                        }
                    }
                    Ok(Modules::Dtn) => {
                        log::trace!("Message Modules::Group received");
                        Dtn::rpc(state, message.data, message.user_id, message.request_id);
                    }
                    Ok(Modules::Auth) => {
                        log::trace!("Auth message received in CLI");
                        authentication::Authentication::rpc(
                            state,
                            message.data,
                            message.user_id,
                            message.request_id,
                        );
                    }
                    Ok(Modules::None) => {
                        log::error!("Message Modules::None received");
                    }
                    Err(_) => {
                        log::error!("Message module undefined");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    /// sends an RPC message to the outside
    pub fn send_message(state: &crate::QaulState, data: Vec<u8>, module: i32, request_id: String, user_id: Vec<u8>) {
        let buf = build_rpc_message(data, module, request_id, user_id);
        Self::send_to_extern(state, buf);
    }

    /// get message count of all messages sent to libqaul
    ///
    /// This function is for bug fixing only,
    /// it changes and can be removed anytime.
    /// Please don't use it for anything serious.
    pub fn send_rpc_count(state: &crate::QaulState) -> i32 {
        let counter = state.rpc.send_count.read().unwrap();
        counter.count
    }

    /// increase message counter by one, of all messages sent to libqaul
    ///
    /// This function is for bug fixing only,
    /// it changes and can be removed anytime.
    /// Please don't use it for anything serious.
    pub fn increase_message_counter(state: &crate::QaulState) {
        let mut counter = state.rpc.send_count.write().unwrap();
        counter.count = counter.count + 1;
    }
}
