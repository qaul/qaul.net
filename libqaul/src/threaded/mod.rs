//! # Run Libqaul in an own Thread
//! 
//! Start libqaul in an own thread and communicate
//! via a sync mpsc queues into and from this thread.
//! 
//! This setup is to decouple the GUI thread from 
//! libqaul. 
//! The communication will happen via protbuf rpc messages.

use std::{
    sync::mpsc::{Receiver, RecvError},
};


mod api;


/// send rpc message from the outside to the inside 
/// of the worker thread of libqaul.
pub fn send_rpc_to_libqaul(binary_message: Vec<u8>) {
    
}


/// check the receiving rpc channel if there
/// are new messages from inside libqaul for 
/// the outside.
pub fn receive_rpc_from_libqaul() -> Result<Vec<u8>, RecvError> {
    Ok(vec![0])
}


/// start libqaul in an own thread
pub fn start() {

}

/// initialize libqaul inside the thread 
/// and poll all the necessary modules
async fn start_libqaul_threaded_init(rpc_receiver: Receiver<Vec<u8>>) -> () {

}


/// send an rpc message from inside libqaul thread
/// to the extern.
pub fn send_rpc_to_extern(message: Vec<u8>) {
    
}
