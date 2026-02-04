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
use std::sync::Arc;
use std::thread;

use crate::rpc::sys::Sys;
use crate::rpc::Rpc;
use crate::Libqaul;

/// C API module
mod c;

/// android module
/// The module only compiled, when the compile target is android.
#[cfg(target_os = "android")]
pub mod android;

/// Global instance holder for backward compatibility
static INSTANCE: state::InitCell<Arc<Libqaul>> = state::InitCell::new();

/// Start libqaul in a separate thread and return the instance
///
/// This is the primary way to start libqaul. It spawns the event loop
/// in a new thread and returns the `Arc<Libqaul>` instance for direct
/// access to node state, configuration, etc.
///
/// # Example
///
/// ```rust,ignore
/// let instance = libqaul::api::start_instance_in_thread(storage_path, None);
///
/// // Wait for initialization
/// while !instance.is_initialized() {
///     std::thread::sleep(Duration::from_millis(10));
/// }
///
/// // Access instance data
/// println!("Node ID: {}", instance.node_id());
///
/// // RPC communication still works via global channels
/// libqaul::api::send_rpc(message);
/// ```
pub fn start_instance_in_thread(
    storage_path: String,
    config: Option<BTreeMap<String, String>>,
) -> Arc<Libqaul> {
    // Channel to receive the instance from the spawned thread
    let (tx, rx) = crossbeam_channel::bounded(1);

    // Spawn new thread
    thread::spawn(move || {
        block_on(async move {
            // Create the instance
            let instance = crate::start_instance(storage_path, config).await;

            // Send instance back to caller
            if let Err(e) = tx.send(Arc::clone(&instance)) {
                log::error!("Failed to send instance to main thread: {:?}", e);
                return;
            }

            // Run the event loop
            instance.run().await;
        })
    });

    // Wait for and return the instance
    let instance = rx.recv().expect(
        "Failed to receive libqaul instance from thread. \
         This usually means the initialization failed. \
         Check if another process is using the database."
    );

    // Store in global for backward compatibility
    INSTANCE.set(Arc::clone(&instance));

    instance
}

/// Get the global instance (if started via start/start_with_config)
///
/// Returns None if libqaul hasn't been started yet.
pub fn get_instance() -> Option<Arc<Libqaul>> {
    INSTANCE.try_get().map(|i| Arc::clone(i))
}

/// start libqaul in an own thread
///
/// Provide the location for storage, all data of qaul will be saved there.
#[deprecated(since = "2.0.0", note = "Use start_instance_in_thread() instead for access to the instance")]
pub fn start(storage_path: String) {
    let _instance = start_instance_in_thread(storage_path, None);
}

/// start libqaul in an own thread
///
/// * Provide the location for storage, all data of qaul will be saved there.
/// * Optionally provide some configuration options, to initially configure libqaul to your needs.
///   the following options can be provided:
///   * Internet module listening port. By default this port is randomly assigned.
#[deprecated(since = "2.0.0", note = "Use start_instance_in_thread() instead for access to the instance")]
pub fn start_with_config(storage_path: String, config: Option<BTreeMap<String, String>>) {
    // Use the new instance-based approach internally
    let _instance = start_instance_in_thread(storage_path, config);
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
///
/// Returns the instance for direct access to node state.
pub fn start_desktop() -> Option<Arc<Libqaul>> {
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
        Some(start_instance_in_thread(path.to_str().unwrap().to_string(), None))
    } else {
        log::error!("Configuration path couldn't be created.");
        None
    }
}

/// start libqaul for android
/// here for debugging and testing
///
/// Hand over the path on the file system
/// where the app is allowed to store data.
///
/// Returns the instance for direct access to node state.
pub fn start_android(storage_path: String) -> Arc<Libqaul> {
    // start libqaul in an own thread
    start_instance_in_thread(storage_path, None)
}

/// Check if libqaul finished initializing
///
/// The initialization of libqaul can take several seconds.
/// If you send any message before it finished initializing, libqaul will crash.
/// Wait therefore until this function returns true before sending anything to libqaul.
pub fn initialization_finished() -> bool {
    if let Some(instance) = INSTANCE.try_get() {
        return instance.is_initialized();
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
