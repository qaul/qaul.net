use async_std::task::spawn;
use bytes::Bytes;
use std::{clone, error::Error};

use crate::{
    ble::ble_service::{get_device_info, QaulBleService},
    rpc::{proto_sys::ble::Message::*, utils::BleResultSender, SysRpcReceiver},
};

use super::BleRpc;

/// Manages all sys messages defined in the 'ble.proto' file.
pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    mut ble_service: QaulBleService,
    internal_sender: BleResultSender,
) -> Result<(), Box<dyn Error>> {
    let mut local_sender_handle = internal_sender.clone();
    loop {
        // async_std::task::spawn(async move {
        let evt = rpc_receiver.recv().await;
        log::info!("Received event: ",);
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
                    StartRequest(req) => match ble_service {
                        QaulBleService::Idle(svc) => {
                            let mut internal_sender_1 = local_sender_handle.clone();
                            // let mut internal_sender_2 = local_sender_handle.clone();
                            // let
                            let qaul_id = Bytes::from(req.qaul_id);
                            // let handle =
                            //  async_std::task::spawn(async move {
                            // let mut
                            ble_service = svc
                                .advertise_scan_listen(qaul_id, None, internal_sender_1.clone())
                                .await;
                            log::info!("BLE Service started successfully");
                            match ble_service {
                                QaulBleService::Idle(_) => {
                                    log::error!("Error occured in configuring BLE module");
                                }
                                QaulBleService::Started(ref mut svc) => {
                                    // ble_service = QaulBleService::Started(svc);
                                    //  async_std::task::spawn(async move {
                                    // ble_service = 
                                        svc.spawn_handles().await;
                                        // svc.join_handles.await;
                                    // });
                                }
                            }
                            internal_sender_1.send_start_successful();
                            //     ble_service
                            // });
                            // ble_service = handle.await;
                            log::info!("BLE Service started successfully");
                            // continue;
                            // break;
                        }
                        QaulBleService::Started(_) => {
                            log::warn!(
                                "Received Start Request, but bluetooth service is already running!"
                            );
                            local_sender_handle.send_result_already_running()
                            // continue;
                        }
                    },
                    StopRequest(_) => match ble_service {
                        QaulBleService::Started(svc) => {
                            log::info!("Received Stop Request");
                            ble_service = svc.stop(&mut local_sender_handle).await;
                        }
                        QaulBleService::Idle(_) => {
                            log::warn!(
                                "Received Stop Request, but bluetooth service is not running!"
                            );
                            local_sender_handle.send_stop_successful(); // Is this really a success case?
                        }
                    },
                    DirectSend(req) => match ble_service {
                        QaulBleService::Started(ref mut svc) => {
                            log::info!("Received Direct Send Request: {:#?}", req);
                            let receiver_id = req.receiver_id.clone();
                            match svc.direct_send(req).await {
                                Ok(_) => local_sender_handle.send_direct_send_success(receiver_id),
                                Err(err) => local_sender_handle
                                    .send_direct_send_error(receiver_id, err.to_string()),
                            }
                        }
                        QaulBleService::Idle(_) => {
                            log::info!("Received Direct Send Request, but bluetooth service is not running!");
                            local_sender_handle.send_result_not_running()
                        }
                    },
                    InfoRequest(_) => {
                        let mut sender_handle_clone = internal_sender.clone();
                        spawn(async move {
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
        // });
    }
    Ok(())
}
