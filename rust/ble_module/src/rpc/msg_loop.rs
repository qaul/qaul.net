use async_std::task::spawn;
use bytes::Bytes;
use std::error::Error;

use crate::{
    ble::ble_service::{get_device_info, QaulBleService},
    rpc::{proto_sys::ble::Message::*, utils::BleResultSender, SysRpcReceiver},
};

use super::BleRpc;

/// Manages all sys messages defined in the 'ble.proto' file.
pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    ble_service: QaulBleService,
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
                    StartRequest(req) => match ble_service {
                        QaulBleService::Idle(svc) => {
                            let internal_sender_1 = local_sender_handle.clone();
                            let qaul_id = Bytes::from(req.qaul_id);
                            let handle = async_std::task::spawn(async move {
                                let ble_service = svc
                                    .advertise_scan_listen(
                                        qaul_id,
                                        None,
                                        internal_sender_1,
                                        rpc_receiver.clone(),
                                    )
                                    .await;
                                log::info!("BLE Service started successfully");

                                match ble_service {
                                    QaulBleService::Idle(_) => {
                                        log::error!("Error occured in configuring BLE module");
                                    }
                                    QaulBleService::Started(svc) => {
                                        svc.spawn_handles().await;
                                    }
                                }
                                local_sender_handle.send_start_successful();
                            });
                            handle.await;
                            break;
                        }
                        QaulBleService::Started(_) => {
                            log::warn!(
                                "Received Start Request, but bluetooth service is already running!"
                            );
                            local_sender_handle.send_result_already_running()
                        }
                    },
                    // This streams were mearged into IdleBleService stream.
                    // The events are recieved by the main loop and handled there.
                    StopRequest(_) => {}
                    DirectSend(_) => {}
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
    }
    Ok(())
}
