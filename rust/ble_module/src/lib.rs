// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul BLE Module for Linux
//!
//! qaul BLE module for Linux
#[macro_use]
extern crate log;
use crate::ble::ble_service::IdleBleService;
use async_std::{stream::StreamExt, task::spawn};
use rpc::{msg_loop::listen_for_sys_msgs, utils::BleResultSender, BleRpc};
use std::thread;
use tokio::runtime;

mod ble;
pub mod rpc;

/// initialize and start the ble_module in an own thread
pub fn init(sys_rpc_callback: Box<dyn FnMut(Vec<u8>) + Send>) {
    let rpc_receiver = rpc::init();

    // Spawn new thread
    thread::spawn(move || {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create BLE module tokio runtime!");

        rt.block_on(async move {
            // start BLE module main loop
            main_loop(sys_rpc_callback, rpc_receiver).await;
        });

    });
}

/// Start the setup and main loop of this library
async fn main_loop(mut sys_rpc_callback: Box<dyn FnMut(Vec<u8>) + Send>, ble_rpc_receiver: BleRpc) {
    let ble_service = IdleBleService::new().await.unwrap_or_else(|err| {
        error!("{:#?}", err);
        std::process::exit(1);
    });

    let (tx, mut rx) = async_std::channel::unbounded::<Vec<u8>>();

    spawn(async move {
        while let Some(result) = rx.next().await {
            sys_rpc_callback(result)
        }
    });

    listen_for_sys_msgs(ble_rpc_receiver, ble_service, BleResultSender::new(tx))
        .await
        .unwrap_or_else(|err| {
            error!("{:#?}", err);
            std::process::exit(1);
        });
}
