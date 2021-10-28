// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Run Libqaul in an own Thread
//! 
//! Start libqaul in an own thread and communicate
//! via a sync mpsc queues into and from this thread.
//! 
//! This setup is to decouple the GUI thread from 
//! libqaul. 
//! The communication will happen via protobuf rpc messages.

use crossbeam_channel::TryRecvError;
use futures::executor::block_on;
use std::{
    thread,
};
use directories::ProjectDirs;

use crate::rpc::Rpc;

/// C API module
mod c;

/// android module
/// The module only compiled, when the compile target is android.
#[cfg(target_os = "android")]
mod android;

/// start libqaul in an own thread
/// 
/// Provide the location for storage, all data of qaul will be saved there.
pub fn start(storage_path: String) {
    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            // start libqaul
            crate::start(storage_path).await;
        }
    ));    
}

/// start libqaul on a desktop platform (Linux, Mac, Windows)
/// 
/// It will automatically define the path to the common OS specific
/// configuration and data location.
/// 
/// The locations are:
/// 
/// * Linux: /home/USERNAME/.config/qaul
/// * MacOS: /Users/USERNAME/Library/Application Support/net.qaul.qaul
///   * in flutter app: /Users/USERNAME/Library/Containers/net.qaul.qaulApp/Application Support/net.qaul.qaul
/// * Windows: C:\Users\USERNAME\AppData\Roaming\qaul\qaul\config
pub fn start_desktop() {
    log::info!("start_desktop");
    // create path
    if let Some(proj_dirs) = ProjectDirs::from("net", "qaul", "qaul") {
        // get path
        let path = proj_dirs.config_dir();

        log::info!("configuration path: {:?}", path);

        // check if path already exists
        if !path.exists() {
            log::info!("create path");

            // create path if it does not exist
            std::fs::create_dir_all(path).unwrap();
        }

        log::info!("start libqaul");

        // start the library with the path
        start(path.to_str().unwrap().to_string());
    } else {
        log::error!("Configuration path couldn't be created.");
    }
}

/// start libqaul for android
/// here for debugging and testing
/// 
/// Hand over the path on the file system
/// where the app is allowed to store data.
pub fn start_android(storage_path: String) {
    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            // start libqaul
            crate::start_android(storage_path).await;
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
