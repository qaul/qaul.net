// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul BLE Module for Linux
//!
//! qaul BLE module for Linux

use crate::ble::ble_service::IdleBleService;
use futures::executor::block_on;
use rpc::msg_loop::listen_for_sys_msgs;
use std::thread;

mod ble;
pub mod rpc;

/// initialize and start the ble_module in an own thread
pub fn init() {
    // Spawn new thread
    thread::spawn(move || {
        block_on(async move {
            // start BLE module main loop
            main_loop().await;
        })
    });
}

/// Start the setup and main loop of this library
async fn main_loop() {
    let rpc_receiver = rpc::init();
    let ble_service = IdleBleService::new().await.unwrap_or_else(|err| {
        log::error!("{:#?}", err);
        std::process::exit(1);
    });

    listen_for_sys_msgs(rpc_receiver, ble_service)
        .await
        .unwrap_or_else(|err| {
            log::error!("{:#?}", err);
            std::process::exit(1);
        });
}
