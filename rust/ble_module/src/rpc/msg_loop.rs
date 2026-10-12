// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use bytes::Bytes;
use std::{error::Error, time::Duration};
use tokio::time::{interval, MissedTickBehavior};

use crate::{
    ble::{gatt_server::ServerEvent, radio::{Running, get_device_info}},
    rpc::{proto_sys::{ble::Message::*, *}, utils::*, SysRpcReceiver},
};

use super::BleRpc;

/// The BLE module's one loop: handles libqaul's requests, the GATT server's
/// events and the tick for as long as the module runs.
pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    mut internal_sender: BleResultSender,
) -> Result<(), Box<dyn Error>> {
    let mut running: Option<Running> = None;
    let mut tick = interval(Duration::from_secs(1));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            evt = rpc_receiver.recv() => { 
                match evt {
                    None => {
                        log::info!("Qaul 'sys' message channel closed. Shutting down gracefully.");
                        break;
                    }
                    Some(msg) => {
                        if msg.message.is_none() {
                            continue;
                        }
                        match msg.message.unwrap() {
                            StartRequest(req) => {
                                if running.is_some(){
                                    log::warn!(
                                        "Received Start Request, but bluetooth service is already running!"
                                    );
                                    internal_sender.send_result_already_running();
                                    continue;
                                }
                                let device_id = req.device_id;
                                let qaul_id = Bytes::from(req.qaul_id);
                                running = match Running::start(&device_id, qaul_id).await {
                                    Ok(r) => {
                                        log::info!(
                                            "BLE service started successfully on device '{}'",
                                            device_id
                                        );
                                        internal_sender.send_start_successful();
                                        Some(r)
                                    },
                                    Err(err) => {
                                        log::error!("BLE device '{}' unavailable: {}", device_id, err);
                                        internal_sender.send_start_error(
                                            BleError::DeviceUnavailable,
                                            format!("BLE device '{}' unavailable: {}", device_id, err),
                                        );
                                        continue;
                                    }
                                };
                                
                            }
                            StopRequest(_) => {
                                if running.take().is_some() {
                                    internal_sender.send_stop_successful();
                                }
                                else {
                                    internal_sender.send_result_not_running();
                                }
                            }
                            DirectSend(req) => {
                                internal_sender.send_direct_send_error(
                                    req.message_id,
                                    "not implemented yet".into(),
                                );
                            }
                            InfoRequest(_) => {
                                let mut sender_handle_clone = internal_sender.clone();
                                tokio::task::spawn_local(async move {
                                    match get_device_info().await {
                                        Ok(info) => {
                                            sender_handle_clone.send_ble_sys_msg(InfoResponse(info))
                                        }
                                        Err(err) => {
                                            log::error!("Error getting device info: {:#?}", &err)
                                        }
                                    }
                                });
                            }
                            _ => (),
                        }
                    }
                }
            }
            evt = next_event(&mut running) => match evt {
                Some(ServerEvent::Subscribed { address, mtu, outbox }) => {
                    log::info!("{} subscribed, MTU {}", address, mtu);
                    if let Some(r) = running.as_mut() {
                        r.outboxes.insert(address, outbox);
                    }
                }
                Some(ServerEvent::FrameReceived { address, bytes }) => {
                    log::info!("{} sent {} bytes: {:02x?}", address, bytes.len(), bytes);
                }
                Some(ServerEvent::Gone { address }) => {
                    log::info!("{} gone", address);
                    if let Some(r) = running.as_mut() {
                        r.outboxes.remove(&address);
                    }
                }
                None => log::warn!("GATT server event channel closed"),
            },
            _ = tick.tick() => {}
        }
    }
    Ok(())
}

/// The next event from the GATT server, or never while BLE is stopped, so
/// the loop's `select!` always has a future for this branch.
async fn next_event(running: &mut Option<Running>) -> Option<ServerEvent> {
    match running {
        Some(r) => r.events.recv().await,
        None => std::future::pending().await,
    }
}
