//! Serves a Bluetooth GATT application using the IO programming model.

use bluer::{
    Adapter, Address, gatt::{
        CharacteristicReader, CharacteristicWriter, local::{
            Application, ApplicationHandle, Characteristic, CharacteristicControl, CharacteristicControlEvent, CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod, Service, characteristic_control,
        },
    },
};
use bytes::Bytes;
use futures::{FutureExt, StreamExt};
use tokio::{
    sync::mpsc,
};

use crate::ble::ble_uuids::{main_service_uuid, msg_char};

use super::ble_uuids::{psm_char, read_char};

/// PSM_CHAR value: -1 means "no L2CAP channel offered", matching Android.
const NO_L2CAP_PSM: i32 = -1;



/// What the GATT server tells the event loop about each connected phone.
#[derive(Debug)]
pub enum ServerEvent {
    /// The phone wrote one frame to MSG_CHAR. Can arrive before `Subscribed`,
    /// because Android writes SEND_ID first.
    FrameReceived { address: Address, bytes: Vec<u8> },
    /// The phone subscribed to notifications on MSG_CHAR. Frames sent on
    /// `outbox` are notified to it by its sender task.
    Subscribed {
        address: Address,
        mtu: usize,
        outbox: mpsc::Sender<Vec<u8>>,
    },
    /// The phone's reader ended or its sender failed in osme way. Both sides may report
    /// it, so the loop must ignore a `Gone` for an unknown address.
    Gone { address: Address },
}

fn read_characteristic(qaul_id: Bytes) -> Characteristic {
    Characteristic {
        uuid: read_char(),
        read: Some(CharacteristicRead {
            read: true,
            fun: Box::new(move |req| {
                let value = qaul_id.clone();
                async move {
                    log::debug!("READ_CHAR read by {}", req.device_address);
                    Ok(value.to_vec())
                }
                .boxed()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// PSM_CHAR: read this to decide whether to open an L2CAP
/// channel. INCOMPLETE
fn psm_characteristic() -> Characteristic {
    Characteristic {
        uuid: psm_char(),
        read: Some(CharacteristicRead {
            read: true,
            fun: Box::new(|_req| async move { Ok(NO_L2CAP_PSM.to_be_bytes().to_vec()) }.boxed()),
            ..Default::default()
        }),
        ..Default::default()
    }
} 

/// MSG_CHAR: carries frames in both directions, write and notify, the returned control is what we check for incoming data.
fn msg_characteristic() -> (Characteristic, CharacteristicControl) {
    let (control, handle) = characteristic_control();
    let characteristic = Characteristic {
        uuid: msg_char(),
        write: Some(CharacteristicWrite {
                write: true,
                write_without_response: true,
                method: CharacteristicWriteMethod::Io,
                ..Default::default()}),
        notify: Some(CharacteristicNotify {
                    notify: true,
                    method: CharacteristicNotifyMethod::Io,
                    ..Default::default()
                }),
        control_handle: handle,
        ..Default::default()
    };
    (characteristic, control)
}

/// Serve the qaul GATT service on `adapter` and report each phone's activity
/// on `events`. Dropping the returned handle removes the service.
pub async fn serve(
    adapter: &Adapter,
    qaul_id: Bytes,
    events: mpsc::Sender<ServerEvent>,
) -> bluer::Result<ApplicationHandle> {
    let read = read_characteristic(qaul_id);
    let psm = psm_characteristic();
    let (msg, control) = msg_characteristic();

    let app = Application {
        services: vec![Service {
            uuid: main_service_uuid(),
            primary: true,
            characteristics: vec![read,
            psm,
            msg
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;
    tokio::task::spawn_local(run_control(control, events));
    Ok(app_handle)
}

/// Takes each phone's line off of MSG_CHAR's control stream and starts a task
/// per pipe. Ends when the GATT application is removed or the loop is gone.
async fn run_control(mut control: CharacteristicControl, events: mpsc::Sender<ServerEvent>) {
    while let Some(evt) = control.next().await {
        match evt {
            CharacteristicControlEvent::Write(req) => {
                let address = req.device_address();
                match req.accept() {
                    Ok(reader) => {
                        tokio::task::spawn_local(read_frames(address, reader, events.clone()));
                    }
                    Err(err) => log::warn!("MSG_CHAR write from {} not accepted: {}", address, err),
                }
            }
            CharacteristicControlEvent::Notify(writer) => {
                let address = writer.device_address();
                let (outbox, outbox_rx) = mpsc::channel(64);
                let subscribed = ServerEvent::Subscribed {
                    address,
                    mtu: writer.mtu(),
                    outbox,
                };
                if events.send(subscribed).await.is_err() {
                    break;
                }
                tokio::task::spawn_local(send_frames(address, writer, outbox_rx, events.clone()));
            }
        }
    }
}

/// Forwards each write from one phone to the loop as a frame. A failed or
/// empty read means the phone has gone.
async fn read_frames(address: Address, reader: CharacteristicReader, events: mpsc::Sender<ServerEvent>) {
    loop {
        let bytes = match reader.recv().await {
            Ok(bytes) if !bytes.is_empty() => bytes,
            Ok(_) => break,
            Err(err) => {
                log::debug!("MSG_CHAR reader for {} ended: {}", address, err);
                break;
            }
        };
        if events.send(ServerEvent::FrameReceived { address, bytes }).await.is_err() {
            return;
        }
    }
    let _ = events.send(ServerEvent::Gone { address }).await;
}

async fn send_frames(address: Address, writer: CharacteristicWriter, mut outbox_rx: mpsc::Receiver<Vec<u8>>, events: mpsc::Sender<ServerEvent>) {
    while let Some(frame) = outbox_rx.recv().await {
        if writer.send(&frame).await.is_err() {
            let _ = events.send(ServerEvent::Gone { address }).await; // tell the engine the phone is gone
            break;
        }
    }
}