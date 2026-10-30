//! Serves qaul's GATT application: writes arrive by callback, notifications
//! go out over one IO pipe per subscribed phone.

use std::{collections::HashMap, os::fd::FromRawFd};

use bluer::{
    Adapter, Address, gatt::{CharacteristicWriter, local::{
        Application, ApplicationHandle, Characteristic, CharacteristicControl, CharacteristicControlEvent, CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod, Service, characteristic_control,
    }},
};
use bytes::Bytes;
use futures::{FutureExt, StreamExt};
use tokio::{
    net::UnixDatagram,
    sync::mpsc,
};

use crate::ble::{
    ble_uuids::{tracer_char, main_service_uuid, msg_char},
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

/// TRACER_CHAR: indicate only. MSG_CHAR can't carry the tracer: a central enables either notifications or indications on a
/// characteristic, never both, and BlueZs server only notifies when both are set.
fn tracer_characteristic() -> (Characteristic, CharacteristicControl) {
    let (control, handle) = characteristic_control();
    let characteristic = Characteristic {
        uuid: tracer_char(),
        notify: Some(CharacteristicNotify {
            notify: false,
            indicate: true,
            method: CharacteristicNotifyMethod::Io,
            ..Default::default()
        }),
        control_handle: handle,
        ..Default::default()
    };
    (characteristic, control)
}

/// Takes the raw socket out of a TRACER_CHAR pipe. We use a socket instead of D-bus because through D-bus,
/// we dont know which link the central is confirming, but a standard writer socket also doesn't work as
/// BlueZ writes the central's confirmation back into it as one byte, and the normal bluez writer cant recieve it.
fn tracer_socket(writer: CharacteristicWriter) -> std::io::Result<UnixDatagram> {
    let fd = writer.into_raw_fd()?;
    let socket = unsafe { std::os::unix::net::UnixDatagram::from_raw_fd(fd) };
    socket.set_nonblocking(true)?;
    UnixDatagram::from_std(socket)
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
    let (msg, msg_control) = msg_characteristic(events.clone());
    let (tracer, tracer_control) = tracer_characteristic();

    let app = Application {
        services: vec![Service {
            uuid: main_service_uuid(),
            primary: true,
            characteristics: vec![read,
            psm,
            msg,
            tracer
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;
    tokio::task::spawn_local(run_control(msg_control, tracer_control, events));
    Ok(app_handle)
}

/// Takes each centrals pipes off the MSG_CHAR and TRACER_CHAR control streams and starts a send task per MSG pipe. 
/// A central subscribes to TRACER_CHAR first, so its tracer waits in `tracers` until its MSG
/// pipe arrives. a MSG pipe with no tracer waiting (something other than linux) runs untraced as it's only needed for linux peripherals.
/// Ends when the GATT application is removed or the loop is gone.
async fn run_control(
    mut msg_control: CharacteristicControl,
    mut tracer_control: CharacteristicControl,
    events: mpsc::Sender<RadioEvent>,
) {
    let mut tracers: HashMap<Address, UnixDatagram> = HashMap::new();
    loop {
        tokio::select! {
            evt = msg_control.next() => match evt {
                // Only sent for CharacteristicWriteMethod::Io, writes use a callback.
                Some(CharacteristicControlEvent::Write(_)) => {}
                Some(CharacteristicControlEvent::Notify(writer)) => {
                    let address = writer.device_address();
                    let peer = PeerKey::peripheral(address);
                    let tracer = tracers.remove(&address);
                    log::debug!("{} subscribed, {}", address, if tracer.is_some() { "traced" } else { "untraced" });
                    let (outbox, outbox_rx) = mpsc::channel(64);
                    let subscribed = RadioEvent::Subscribed {
                        peer,
                        mtu: writer.mtu(),
                        outbox,
                    };
                    if events.send(subscribed).await.is_err() {
                        break;
                    }
                    tokio::task::spawn_local(send_frames(peer, writer, tracer, outbox_rx, events.clone()));
                }
                None => break,
            },
            evt = tracer_control.next() => match evt {
                Some(CharacteristicControlEvent::Write(_)) => {}
                Some(CharacteristicControlEvent::Notify(writer)) => {
                    let address = writer.device_address();
                    match tracer_socket(writer) {
                        // Replaces a stale one left by an earlier connection that never reached MSG_CHAR.
                        Ok(socket) => { tracers.insert(address, socket); }
                        Err(err) => log::warn!("{} tracer pipe unusable: {}", address, err),
                    }
                }
                None => break,
            },
        }
    }
}