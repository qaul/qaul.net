// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use bytes::Bytes;
use std::error::Error;

use crate::{
    ble::ble_service::{get_device_info, IdleBleService, QaulBleService},
    rpc::{proto_sys::ble::Message::*, proto_sys::*, utils::*, SysRpcReceiver},
};

use super::BleRpc;

/// Manages all sys messages defined in the 'ble.proto' file.
pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    internal_sender: BleResultSender,
) -> Result<(), Box<dyn Error>> {
    let mut local_sender_handle = internal_sender.clone();
    loop {
        let evt = rpc_receiver.recv().await;
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
                        let device_id = req.device_id;
                        let ble_service = match IdleBleService::new(&device_id).await {
                            Ok(svc) => svc,
                            Err(err) => {
                                log::error!("BLE device '{}' unavailable: {}", device_id, err);
                                local_sender_handle.send_start_error(
                                    BleError::DeviceUnavailable,
                                    format!("BLE device '{}' unavailable: {}", device_id, err),
                                );
                                continue;
                            }
                        };

                        match ble_service {
                            QaulBleService::Idle(svc) => {
                                let internal_sender_1 = local_sender_handle.clone();
                                let qaul_id = Bytes::from(req.qaul_id);
                                let handle = tokio::task::spawn_local(async move {
                                    let ble_service = svc
                                        .advertise_scan_listen(
                                            qaul_id,
                                            None,
                                            internal_sender_1,
                                            rpc_receiver,
                                        )
                                        .await;

                                    match ble_service {
                                        QaulBleService::Idle(_) => {
                                            log::error!(
                                                "Failed to start BLE module (advertisement or GATT setup failed)"
                                            );
                                            local_sender_handle.send_start_error(
                                                BleError::UnknownError,
                                                "Failed to configure BLE advertisement or GATT application".into(),
                                            );
                                        }
                                        QaulBleService::Started(svc) => {
                                            log::info!(
                                                "BLE service started successfully on device '{}'",
                                                device_id
                                            );
                                            local_sender_handle.send_start_successful();
                                            svc.spawn_handles().await;
                                        }
                                    }
                                });
                                let _ = handle.await;
                                break;
                            }
                            QaulBleService::Started(_) => {
                                log::warn!(
                                    "Received Start Request, but bluetooth service is already running!"
                                );
                                local_sender_handle.send_result_already_running();
                            }
                        }
                    }
                    // This streams were mearged into IdleBleService stream.
                    // The events are recieved by the main loop and handled there.
                    StopRequest(_) => {}
                    DirectSend(_) => {}
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
    Ok(())
}
