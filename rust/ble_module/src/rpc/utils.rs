use async_std::channel::Sender;

use super::proto_sys::{self, BleDeviceDiscovered, BleError, BleStartResult};

pub fn send_ble_sys_msg(msg: proto_sys::ble::Message, sender: &mut Sender<Vec<u8>>) {
    let mut buf = Vec::with_capacity(msg.encoded_len());
    msg.encode(&mut buf);
    if let Err(err) = sender.try_send(buf) {
        log::error!("{:?}", err);
    }
}

pub fn send_result_already_running(sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::StartResult(BleStartResult {
            success: true,
            error_reason: BleError::UnknownError.into(),
            error_message: "Received start request, but BLE service is already running!".into(),
        }),
        sender,
    );
}

pub fn send_result_not_running(sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::StartResult(BleStartResult {
            success: true,
            error_reason: BleError::UnknownError.into(),
            error_message: "Received direct send request, but BLE service is not yet running!"
                .into(),
        }),
        sender,
    );
}

pub fn send_start_successful(sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::StartResult(BleStartResult {
            success: true,
            error_reason: 0,
            error_message: "".into(),
        }),
        sender,
    )
}

pub fn send_device_found(qaul_id: Vec<u8>, rssi: i32, sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::DeviceDiscovered(BleDeviceDiscovered { qaul_id, rssi }),
        sender,
    )
}

pub fn send_direct_received(from: Vec<u8>, data: Vec<u8>, sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::DirectReceived(proto_sys::BleDirectReceived { from, data }),
        sender,
    )
}

pub fn send_direct_send_success(id: Vec<u8>, sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::DirectSendResult(proto_sys::BleDirectSendResult {
            id,
            success: true,
            error_message: "".into(),
        }),
        sender,
    )
}

pub fn send_direct_send_error(id: Vec<u8>, error_message: String, sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::DirectSendResult(proto_sys::BleDirectSendResult {
            id,
            success: false,
            error_message,
        }),
        sender,
    )
}

pub fn send_stop_unsuccessful(error_message: String, sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::StopResult(proto_sys::BleStopResult {
            success: false,
            error_reason: 0,
            error_message,
        }),
        sender,
    )
}

pub fn send_stop_successful(sender: &mut Sender<Vec<u8>>) {
    send_ble_sys_msg(
        proto_sys::ble::Message::StopResult(proto_sys::BleStopResult {
            success: true,
            error_reason: 0,
            error_message: "".into(),
        }),
        sender,
    )
}
