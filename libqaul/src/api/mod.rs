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

mod c;


/// start libqaul in an own thread
pub fn start_threaded() {
    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            // start libqaul
            crate::start().await;
        }
    ));

    // wait until initialized ...
    // TODO: make it better!
    //       wait in RPC until initialized
    std::thread::sleep(Duration::from_millis(5000));
}

/// send an RPC message to libqaul
pub fn send_rpc(binary_message: Vec<u8>) {
    Rpc::send_to_libqaul(binary_message);
}

/// receive a RPC message from libqaul
pub fn receive_rpc() -> Result<Vec<u8>, TryRecvError> {
    Rpc::receive_from_libqaul()
}
