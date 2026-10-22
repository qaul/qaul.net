//! Connects to a phone as GATT client (central) and gets its pipes.

use std::{error::Error, time::Duration};

use ble_protocol::QAUL_ID_BYTES;
use bluer::{
    gatt::{CharacteristicReader, CharacteristicWriter},
    Adapter, Address,
};
use tokio::{sync::mpsc, time::timeout};

use crate::ble::{
    ble_uuids::{main_service_uuid, msg_char, read_char},
    radio::{send_frames, PeerKey, RadioEvent},
};

/// Time limit for the whole connect chain. Mirrors Android's SETUP_TIMEOUT_MS.
const SETUP_TIMEOUT: Duration = Duration::from_secs(15);

/// Try to connect to `address` and get its reader, writer and read its qaul_id.
async fn setup(adapter: &Adapter, address: Address) -> Result<(CharacteristicReader, CharacteristicWriter, [u8; QAUL_ID_BYTES]), Box<dyn Error>> {
    let device = adapter.device(address)?;

    if !device.is_connected().await? {
        device.connect().await?;
    }

    // BlueZ discovers services on connect, so services() will wait for it to finish
    let (mut msg, mut read) = (None, None);
    for service in device.services().await? {
        if service.uuid().await? != main_service_uuid() {
            continue;
        }
        for char in service.characteristics().await? {
            let uuid = char.uuid().await?;
            if uuid == msg_char() {
                msg = Some(char);
            } else if uuid == read_char() {
                read = Some(char);
            }
        }
    }
    let (Some(msg), Some(read)) = (msg, read) else {
        return Err("qaul service is missing MSG_CHAR or READ_CHAR".into());
    };
    let reader = msg.notify_io().await?;
    let writer = msg.write_io().await?;

    let id = read.read().await?;
    let id: [u8; QAUL_ID_BYTES] = id
        .as_slice()
        .try_into()
        .map_err(|_| format!("qaul id must be {} bytes, got {}", QAUL_ID_BYTES, id.len()))?;

    Ok((reader, writer, id))
}

/// Connect to `address` as central. The msg_loop will hear either: Subscribed + PeerIdRead on success or ConnectFailed otherwise.
pub fn connect(adapter: Adapter, address: Address, events: mpsc::Sender<RadioEvent>) {
    let peer = PeerKey::central(address);
    tokio::task::spawn_local(async move {
        let (reader, writer, id) = match timeout(SETUP_TIMEOUT, setup(&adapter, address)).await {
            Ok(Ok(pipes)) => pipes,
            Ok(Err(err)) => return fail(&adapter, peer, &events, &err.to_string()).await,
            Err(_) => return fail(&adapter, peer, &events, "timed out").await,
        };

        let (outbox, outbox_rx) = mpsc::channel(64);
        let mtu = reader.mtu();
        if events.send(RadioEvent::Subscribed { peer, mtu, outbox }).await.is_err()
            || events.send(RadioEvent::PeerIdRead { peer, id }).await.is_err()
        {
            return; //the loop is gone
        }
        tokio::task::spawn_local(send_frames(peer, writer, outbox_rx, events.clone()));
        recv_frames(peer, reader, events).await;
    });
}

/// Disconnect a connection and tell msg_loop, so the engine frees its connect slot.
async fn fail(adapter: &Adapter, peer: PeerKey, events: &mpsc::Sender<RadioEvent>, reason: &str) {
    log::warn!("connect to {} failed: {}", peer.address, reason);
    if let Ok(device) = adapter.device(peer.address) {
        let _ = device.disconnect().await;
    }
    let _ = events.send(RadioEvent::ConnectFailed { peer }).await;
}

/// Passes each notification from the phone to the loop. An empty read means
/// the pipe was closed, so the connection is gone.
async fn recv_frames(peer: PeerKey, reader: CharacteristicReader, events: mpsc::Sender<RadioEvent>) {
    loop {
        let bytes = match reader.recv().await {
            Ok(bytes) if !bytes.is_empty() => bytes,
            _ => break,
        };
        if events.send(RadioEvent::FrameReceived { peer, bytes }).await.is_err() {
            return;
        }
    }
    let _ = events.send(RadioEvent::Gone { peer }).await;
}
