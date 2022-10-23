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
use directories::ProjectDirs;
use futures::executor::block_on;
use std::collections::BTreeMap;
use std::thread;

use crate::rpc::sys::Sys;
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
    self::start_with_config(storage_path, None);
}

/// start libqaul in an own thread
///
/// * Provide the location for storage, all data of qaul will be saved there.
/// * Optionally provide some configuration options, to initially configure libqaul to your needs.
///   the following options can be provided:
///   * Internet module listening port. By default this port is randomly assigned.
pub fn start_with_config(storage_path: String, config: Option<BTreeMap<String, String>>) {
    // Spawn new thread
    thread::spawn(move || {
        block_on(async move {
            // start libqaul
            crate::start(storage_path, config).await;
        })
    });
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
    log::trace!("start_desktop");
    // create path
    if let Some(proj_dirs) = ProjectDirs::from("net", "qaul", "qaul") {
        // get path
        let path = proj_dirs.config_dir();

        log::trace!("configuration path: {:?}", path);

        // check if path already exists
        if !path.exists() {
            log::trace!("create path");

            // create path if it does not exist
            std::fs::create_dir_all(path).unwrap();
        }

        log::trace!("start libqaul");

        // start the library with the path
        self::start_with_config(path.to_str().unwrap().to_string(), None);
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
    // start libqaul in an own thread
    self::start_with_config(storage_path, None);
}

/// Check if libqaul finished initializing
///
/// The initialization of libqaul can take several seconds.
/// If you send any message before it finished initializing, libqaul will crash.
/// Wait therefore until this function returns true before sending anything to libqaul.
pub fn initialization_finished() -> bool {
    if let Some(_) = crate::INITIALIZED.try_get() {
        return true;
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

/// send a SYS message to libqaul
pub fn send_sys(binary_message: Vec<u8>) {
    Sys::send_to_libqaul(binary_message);
}

/// receive a SYS message from libqaul
pub fn receive_sys() -> Result<Vec<u8>, TryRecvError> {
    Sys::receive_from_libqaul()
}

#[cfg(target_os = "android")]
use android_logger::Config;
use log::Level;
use rifgen::rifgen_attr::*;

struct AndroidBindings;

impl AndroidBindings {
    /// Set up logging
    #[generate_interface]
    pub fn initialise_logging() {
        #[cfg(target_os = "android")]
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Trace)
                .with_tag("rust"),
        );
        log_panics::init();
        log::error!("initialised");
    }

    /// start libqaul for android
    /// here for debugging and testing
    ///
    /// Hand over the path on the file system
    /// where the app is allowed to store data.
    #[generate_interface]
    pub fn initialise_libqual(storage_path: String) {
        start_android(storage_path)
    }

    /// send a SYS message to libqaul
    #[generate_interface]
    pub fn send_sys(binary_message: &[i8]) {
        send_sys(AndroidBindings::vec_i8_into_u8(binary_message.to_vec()))
    }

    /// receive a SYS message from libqaul
    #[generate_interface]
    pub fn receive_sys() -> Vec<i8> {
        Sys::receive_from_libqaul()
            .map(AndroidBindings::vec_u8_into_i8)
            .unwrap_or_default()
    }

    fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
        // ideally we'd use Vec::into_raw_parts, but it's unstable,
        // so we have to do it manually:

        // first, make sure v's destructor doesn't free the data
        // it thinks it owns when it goes out of scope
        let mut v = std::mem::ManuallyDrop::new(v);

        // then, pick apart the existing Vec
        let p = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();

        // finally, adopt the data into a new Vec
        unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
    }
    fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {
        // ideally we'd use Vec::into_raw_parts, but it's unstable,
        // so we have to do it manually:

        // first, make sure v's destructor doesn't free the data
        // it thinks it owns when it goes out of scope
        let mut v = std::mem::ManuallyDrop::new(v);

        // then, pick apart the existing Vec
        let p = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();

        // finally, adopt the data into a new Vec
        unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
    }
}
