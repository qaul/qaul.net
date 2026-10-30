//! Connects to a phone as GATT client (central) and gets its pipes.

use std::{error::Error, time::Duration};

use ble_protocol::QAUL_ID_BYTES;
use bluer::{
    Adapter, Address, gatt::{CharacteristicReader, WriteOp, remote::{Characteristic, CharacteristicWriteRequest}},
};
use tokio::{sync::mpsc, time::timeout};

use crate::ble::{
    ble_uuids::{tracer_char, main_service_uuid, msg_char, read_char},
    radio::{PeerKey, RadioEvent, WRITE_TRACER_EVERY},
};

/// Time limit for the whole connect chain. Mirrors Android's SETUP_TIMEOUT_MS.
const SETUP_TIMEOUT: Duration = Duration::from_secs(15);

/// What setup hands back for a working link.
struct Pipes {
    /// Notifications from the peer on MSG_CHAR.
    reader: CharacteristicReader,
    /// MSG_CHAR itself.
    msg: Characteristic,
    /// Indications on TRACER_CHAR, if the peer has it (Linux nodes only). Held so the subscription stays.
    tracer: Option<CharacteristicReader>,
    id: [u8; QAUL_ID_BYTES],
}

/// Try to connect to `address`, subscribe to its pipes and read its qaul_id.
async fn setup(adapter: &Adapter, address: Address) -> Result<Pipes, Box<dyn Error>> {
    let device = adapter.device(address)?;

    if !device.is_connected().await? {
        device.connect().await?;
    }

    // BlueZ discovers services on connect, so services() will wait for it to finish
    let (mut msg, mut read, mut tracer) = (None, None, None);
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
            } else if uuid == tracer_char() {
                tracer = Some(char);
            }
        }
    }
    let (Some(msg), Some(read)) = (msg, read) else {
        return Err("qaul service is missing MSG_CHAR or READ_CHAR".into());
    };
    // TRACER_CHAR before MSG_CHAR: the peers server pairs them in the order they arrive and
    // will start a link without the tracer if MSG comes first
    let tracer = match tracer {
        Some(tracer) => Some(tracer.notify_io().await?),
        None => None,
    };
    let reader = msg.notify_io().await?;
    // No write_io(): we can aquire a write socket, but it can't do writes with response, which we need for the tracer

    let id = read.read().await?;
    let id: [u8; QAUL_ID_BYTES] = id
        .as_slice()
        .try_into()
        .map_err(|_| format!("qaul id must be {} bytes, got {}", QAUL_ID_BYTES, id.len()))?;

    Ok(Pipes { reader, msg, tracer, id })
}

/// Connect to `address` as central. The msg_loop will hear either: Subscribed + PeerIdRead on success or ConnectFailed otherwise.
pub fn connect(adapter: Adapter, address: Address, events: mpsc::Sender<RadioEvent>) {
    let peer = PeerKey::central(address);
    tokio::task::spawn_local(async move {
        let Pipes { reader, msg, tracer, id } = match timeout(SETUP_TIMEOUT, setup(&adapter, address)).await {
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
        if let Some(tracer) = tracer {
            tokio::task::spawn_local(drain_tracer(tracer));
        }
        tokio::task::spawn_local(send_frames_traced(peer, msg, outbox_rx, events.clone()));
        recv_frames(peer, reader, events).await;
    });
}
/// Writes each frame from the outbox to MSG_CHAR over D-Bus, in order. Every `WRITE_TRACER_EVERY` frame, is a Write Request.
/// Write_ext only returns once its Write Response is in, so FrameSent holds the engine back until everything before the traer 
/// in bluetoothd's queue has drained. The rest are Write Commands, which return immedietely once queued. This give an opportunity for fresh control queue
/// sends to be put ahead of bulk frames every tracer. The tracer adds a slight delay as it requires a round trip.
async fn send_frames_traced(peer: PeerKey, msg: Characteristic, mut outbox_rx: mpsc::Receiver<Vec<u8>>, events: mpsc::Sender<RadioEvent>) {
    let mut frames_sent: usize = 0;

    let mut command = CharacteristicWriteRequest::default();
    command.op_type = WriteOp::Command;

    let mut request = CharacteristicWriteRequest::default();
    request.op_type = WriteOp::Request;

    while let Some(frame) = outbox_rx.recv().await {
        let tracer = frames_sent % WRITE_TRACER_EVERY == 0;
        let req = if tracer { &request } else { &command };
        frames_sent += 1;

        if let Err(err) = msg.write_ext(&frame, req).await {
            log::warn!("{} ({:?}) send failed: {}", peer.address, peer.role, err);
            let _ = events.send(RadioEvent::Gone { peer }).await;
            break;
        }
        if tracer {
            log::debug!("{} write tracer ok after {} frames", peer.address, frames_sent);
        }
        // FrameSent always reported here, it just has to actually wait on the tracer, unlike normal sends which all report FrameSent 
        // instantly, before they actually get sent off the radio.
        let _ = events.send(RadioEvent::FrameSent { peer }).await;
    }
}

/// Reads and drops the peers tracer indications. BlueZ confirms them itself, we just read to  keep the
/// pipe from filling, and holding the reader keeps the subscription.
async fn drain_tracer(tracer: CharacteristicReader) {
    while let Ok(bytes) = tracer.recv().await {
        if bytes.is_empty() {
            break;
        }
    }
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

/// Disconnect a connection and tell msg_loop, so the engine frees its connect slot.
async fn fail(adapter: &Adapter, peer: PeerKey, events: &mpsc::Sender<RadioEvent>, reason: &str) {
    log::warn!("connect to {} failed: {}", peer.address, reason);
    if let Ok(device) = adapter.device(peer.address) {
        let _ = device.disconnect().await;
    }
    let _ = events.send(RadioEvent::ConnectFailed { peer }).await;
}