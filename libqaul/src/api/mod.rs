//! # Run Libqaul in an own Thread
//! 
//! Start libqaul in an own thread and communicate
//! via a sync mpsc queues into and from this thread.
//! 
//! This setup is to decouple the GUI thread from 
//! libqaul. 
//! The communication will happen via protbuf rpc messages.

use crossbeam_channel::TryRecvError;
use futures::executor::block_on;
use std::{
    thread,
    time::Duration,
};

use crate::rpc::Rpc;

/// C API module
mod c;

/// android module
/// The module only compiled, when the compile target is android.
#[cfg(target_os = "android")]
mod android;

/// start libqaul in an own thread
pub fn start() {
    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            // start libqaul
            crate::start().await;
        }
    ));    
}

/// start libqaul for android
/// here for debugging and testing
pub fn start_android() {
    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            // start libqaul
            crate::start_android().await;
        }
    ));
}

/// Check if libqaul finished initializing
/// 
/// The initialization of libqaul can take several seconds.
/// If you send any message before it finished initializing, libqaul will crash.
/// Wait therefore until this function returns true before sending anything to libqaul.
pub fn initialization_finished() -> bool {
    if let Some(_) = crate::INITIALIZED.try_get() {
        return true
    }
    
    false
}


/// send an RPC message to libqaul
pub fn send_rpc(binary_message: Vec<u8>) {
    Rpc::send_to_libqaul(binary_message);
}

/// receive a RPC message from libqaul
pub fn receive_rpc() -> Result<Vec<u8>, TryRecvError> {
    Rpc::receive_from_libqaul()
}

/// count of rpc messages to receive in the queue
pub fn receive_rpc_queued() -> usize {
    Rpc::receive_from_libqaul_queue_length()
}

/// count of sent rpc messages
pub fn send_rpc_count() -> i32 {
    Rpc::send_rpc_count()
}
