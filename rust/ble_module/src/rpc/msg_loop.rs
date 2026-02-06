use std::error::Error;

use async_std::task::spawn;
use bytes::Bytes;

use crate::{
    ble::ble_service::{get_device_info, QaulBleService},
    rpc::{
        proto_sys::ble::Message::*,
        utils::{
            send_direct_send_error, send_direct_send_success, send_result_already_running,
            send_result_not_running, send_start_successful, send_stop_successful,
        },
        SysRpcReceiver,
    },
};

use super::BleRpc;

pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    mut ble_service: QaulBleService,
) -> Result<(), Box<dyn Error>> {
    loop {
        let evt = rpc_receiver.recv().await;
        match evt {
            None => {
                info!("Qaul 'sys' message channel closed. Shutting down gracefully.");
                break;
            }
            Some(msg) => {
                debug!("Received 'sys' message: {:#?}", msg);
                if msg.message.is_none() {
                    continue;
                }
                match msg.message.unwrap() {
                    StartRequest(req) => match ble_service {
                        QaulBleService::Idle(svc) => {
                            let qaul_id = Bytes::from(req.qaul_id);
                            ble_service = svc.advertise_scan_listen(qaul_id, None).await;
                            debug!("Set up advertisement and scan filter, entering BLE main loop.");
                            send_start_successful();
                        }
                        QaulBleService::Started(_) => {
                            warn!(
                                "Received Start Request, but bluetooth service is already running!"
                            );
                            send_result_already_running()
                        }
                    },
                    StopRequest(_) => match ble_service {
                        QaulBleService::Started(svc) => {
                            ble_service = svc.stop().await;
                        }
                        QaulBleService::Idle(_) => {
                            warn!("Received Stop Request, but bluetooth service is not running!");
                            send_stop_successful(); // Is this really a success case?
                        }
                    },
                    DirectSend(req) => match ble_service {
                        QaulBleService::Started(ref svc) => match svc.direct_send(&req).await {
                            Ok(_) => send_direct_send_success(req.receiver_id),
                            Err(err) => send_direct_send_error(req.receiver_id, err.to_string()),
                        },
                        QaulBleService::Idle(_) => {
                            warn!("Received Direct Send Request, but bluetooth service is not running!");
                            send_result_not_running()
                        }
                    },
                    InfoRequest(_) => {
                        spawn(async {
                            get_device_info().await.unwrap_or_else(|err| {
                                error!("Error getting device info: {:#?}", &err)
                            })
                        });
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
