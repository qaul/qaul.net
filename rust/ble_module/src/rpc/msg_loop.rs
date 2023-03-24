use std::error::Error;

use async_std::task::spawn;
use bytes::Bytes;

use crate::{
    ble::ble_service::{get_device_info, QaulBleService},
    rpc::{
        proto_sys::ble::Message::*,
        utils::{
            send_ble_sys_msg, send_direct_send_error, send_direct_send_success,
            send_result_already_running, send_result_not_running, send_start_successful,
            send_stop_successful,
        },
        SysRpcReceiver,
    },
};

use super::BleRpc;

pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    mut ble_service: QaulBleService,
    internal_sender: async_std::channel::Sender<Vec<u8>>,
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
                log::debug!("Received 'sys' message: {:#?}", msg);
                if msg.message.is_none() {
                    continue;
                }
                match msg.message.unwrap() {
                    StartRequest(req) => match ble_service {
                        QaulBleService::Idle(svc) => {
                            let qaul_id = Bytes::from(req.qaul_id);
                            ble_service = svc
                                .advertise_scan_listen(qaul_id, None, internal_sender.clone())
                                .await;
                            log::debug!(
                                "Set up advertisement and scan filter, entering BLE main loop."
                            );
                            send_start_successful(&mut local_sender_handle);
                        }
                        QaulBleService::Started(_) => {
                            log::warn!(
                                "Received Start Request, but bluetooth service is already running!"
                            );
                            send_result_already_running(&mut local_sender_handle)
                        }
                    },
                    StopRequest(_) => match ble_service {
                        QaulBleService::Started(svc) => {
                            ble_service = svc.stop(&mut local_sender_handle).await;
                        }
                        QaulBleService::Idle(_) => {
                            log::warn!(
                                "Received Stop Request, but bluetooth service is not running!"
                            );
                            send_stop_successful(&mut local_sender_handle); // Is this really a success case?
                        }
                    },
                    DirectSend(req) => match ble_service {
                        QaulBleService::Started(ref mut svc) => {
                            let receiver_id = req.receiver_id.clone();
                            match svc.direct_send(req).await {
                                Ok(_) => {
                                    send_direct_send_success(receiver_id, &mut local_sender_handle)
                                }
                                Err(err) => send_direct_send_error(
                                    receiver_id,
                                    err.to_string(),
                                    &mut local_sender_handle,
                                ),
                            }
                        }
                        QaulBleService::Idle(_) => {
                            log::warn!("Received Direct Send Request, but bluetooth service is not running!");
                            send_result_not_running(&mut local_sender_handle)
                        }
                    },
                    InfoRequest(_) => {
                        let mut sender_handle_clone = internal_sender.clone();
                        spawn(async move {
                            match get_device_info().await {
                                Ok(info) => {
                                    send_ble_sys_msg(InfoResponse(info), &mut sender_handle_clone)
                                }
                                Err(err) => log::error!("Error getting device info: {:#?}", &err),
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
