//! Serves qaul's GATT application: writes arrive by callback, notifications
//! go out over one IO pipe per subscribed phone.

use bluer::{
    Adapter, gatt::local::{
        Application, ApplicationHandle, Characteristic, CharacteristicControl, CharacteristicControlEvent, CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod, Service, characteristic_control,
    },
};
use bytes::Bytes;
use futures::{FutureExt, StreamExt};
use tokio::{
    sync::mpsc,
};

use crate::ble::{
    ble_uuids::{main_service_uuid, msg_char},
    radio::{send_frames, PeerKey, RadioEvent},
};

use super::ble_uuids::{psm_char, read_char};

/// PSM_CHAR value: -1 means "no L2CAP channel offered", matching Android.
const NO_L2CAP_PSM: i32 = -1;

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

/// MSG_CHAR: carries frames in both directions. Writes arrive through a
/// callback; the returned control hands out each phone's notify pipe.
///
/// Writes use a callback, not an IO pipe: needs investigation but the first write when the I/O
/// pipe was set up seemed to rarely succeed, which is the qaul id being sent so not ideal.
fn msg_characteristic(events: mpsc::Sender<RadioEvent>) -> (Characteristic, CharacteristicControl) {
    let (control, handle) = characteristic_control();
    let characteristic = Characteristic {
        uuid: msg_char(),
        write: Some(CharacteristicWrite {
                write: true,
                write_without_response: true,
                method: CharacteristicWriteMethod::Fun(Box::new(move |bytes, req| {
                    let events = events.clone();
                    async move {
                        if !bytes.is_empty() {
                            let peer = PeerKey::peripheral(req.device_address);
                            let _ = events.send(RadioEvent::FrameReceived { peer, bytes }).await;
                        }
                        Ok(())
                    }
                    .boxed()
                })),
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
    events: mpsc::Sender<RadioEvent>,
) -> bluer::Result<ApplicationHandle> {
    let read = read_characteristic(qaul_id);
    let psm = psm_characteristic();
    let (msg, control) = msg_characteristic(events.clone());

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
async fn run_control(mut control: CharacteristicControl, events: mpsc::Sender<RadioEvent>) {
    while let Some(evt) = control.next().await {
        match evt {
            // Only sent for CharacteristicWriteMethod::Io, writes use a callback.
            CharacteristicControlEvent::Write(_) => {}
            CharacteristicControlEvent::Notify(writer) => {
                let peer = PeerKey::peripheral(writer.device_address());
                let (outbox, outbox_rx) = mpsc::channel(64);
                let subscribed = RadioEvent::Subscribed {
                    peer,
                    mtu: writer.mtu(),
                    outbox,
                };
                if events.send(subscribed).await.is_err() {
                    break;
                }
                tokio::task::spawn_local(send_frames(peer, writer, outbox_rx, events.clone()));
            }
        }
    }
}