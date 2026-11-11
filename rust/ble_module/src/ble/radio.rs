// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use crate::{
    ble::{
        ble_uuids::{main_service_uuid, QAUL_ID_ADVERT_BYTES, QAUL_MANUFACTURER_ID},
        gatt_client,
        gatt_server::serve,
        scanner::scan,
    },
    rpc::proto_sys::{BleDeviceInfo, BleInfoResponse},
};
use ble_protocol::{
    engine::{Engine, Peer, Role},
    QAUL_ID_BYTES,
};
use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::{local::ApplicationHandle, CharacteristicWriter},
    Adapter, Address, Session,
};
use bytes::Bytes;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    time::Duration,
};
use tokio::{
    net::UnixDatagram, sync::{mpsc, watch}, time::Instant,
};

/// Requested advert power. Mirrors Android's TX_POWER_HIGH/MAX, the controller clamps it
const ADVERT_TX_POWER_DBM: i16 = 20;

/// Advert interval. Mirrors Android's ADVERTISE_MODE_LOW_LATENCY, which
/// Android's NOMINAL_ADVERT_INTERVAL_MS also assumes.
const ADVERT_INTERVAL: Duration = Duration::from_millis(100);

/// Wait after a failed advertise before trying again.
const ADVERT_RETRY: Duration = Duration::from_secs(5);

/// Every Nth frame is traced: the send task waits until BlueZ has put it, and every frame before it,
/// on air before reporting FrameSent. BlueZ only sends a Write Request or an Indication once its
/// write queue (commands and notifications) is empty, so the reply means the queue has drained.
/// Without it FrameSent only means "handed to bluetoothd", which queues without limit.
/// Bounds what sits ahead of a control frame to N frames, costing 1–2 idle connection events per tracer.
pub(crate) const WRITE_TRACER_EVERY: usize = 20;

/// Measures throughput between tracers. A tracers reply means bluetoothds queue has drained,
/// so the time from a batchs first frame to its tracers reply is how long those bytes took on air inluding the tracer response time.
/// Starts at the first frame, not at the last tracer, so time with an empty outbox doesn't count.
#[derive(Default)]
pub(crate) struct BatchTimer {
    start: Option<Instant>,
    frames: usize,
    bytes: usize,
}

impl BatchTimer {
    /// Call just before handing a frame to BlueZ.
    pub(crate) fn on_frame(&mut self, len: usize) {
        self.start.get_or_insert_with(Instant::now);
        self.frames += 1;
        self.bytes += len;
    }

    /// Call once a tracers reply is in: logs the batch and starts a new one.
    /// Only full batches are logged, a short one doesnt send enough to get an accurate reading.
    pub(crate) fn finish(&mut self, peer: PeerKey) {
        let Some(start) = self.start.take() else { return };
        let frames = std::mem::take(&mut self.frames);
        let bytes = std::mem::take(&mut self.bytes);
        if frames < WRITE_TRACER_EVERY {
            return;
        }
        let elapsed = start.elapsed();
        let kb_s = bytes as f64 / elapsed.as_secs_f64().max(1e-6) / 1000.0;
        log::info!(
            "{} ({:?}) tracer: {} frames, {} B in {} ms = {:.1} KB/s",
            peer.address, peer.role, frames, bytes, elapsed.as_millis(), kb_s
        );
    }
}

/// How a link's frames reach its writer task: `transmit` queues here and the
/// task does the awaiting writes. Each platforms radio defines its own. Mostly needed for linux
pub type Outbox = mpsc::Sender<Vec<u8>>;

/// What the radio side tells the loop. One channel for all of them, so the loop sees one ordered stream.
#[derive(Debug)]
pub enum RadioEvent {
    /// The peer wrote one frame to MSG_CHAR. Can arrive before `Subscribed`,
    /// because Android writes SEND_ID first.
    FrameReceived { peer: PeerKey, bytes: Vec<u8> },
    /// A frame was sent to the radio, tell engine so that the next may be retrieved from this peer's link
    FrameSent { peer: PeerKey },
    /// The peer subscribed to notifications on MSG_CHAR. Frames sent on
    /// `outbox` are notified to it by its sender task.
    Subscribed {
        peer: PeerKey,
        mtu: usize,
        outbox: Outbox,
    },
    /// Scanner saw this advert, pass to engine for connection decision making
    Advert {
        address: Address,
        id_prefix:  [u8; QAUL_ID_ADVERT_BYTES], 
        rssi: i16
    },

    /// We read the peer's qaul ID from its READ_CHAR (central only).
    PeerIdRead { peer: PeerKey, id: [u8; QAUL_ID_BYTES] },
    /// A connect we started didn't produce a link (central only).
    ConnectFailed { peer: PeerKey },
    /// The peer's sending or recieving pipe failed.
    Gone { peer: PeerKey },
}

/// How the engine names a link on Linux: the peer address and our role on it. 
/// The same peer could be linked in both roles at once, so the address alone isn't unique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerKey {
    pub address: Address,
    pub role: Role,
}

impl PeerKey {
    /// A phone connected to our GATT server.
    pub fn peripheral(address: Address) -> Self {
        Self { address, role: Role::Peripheral }
    }

    /// A phone we connected to as GATT client.
    pub fn central(address: Address) -> Self {
        Self { address, role: Role::Central }
    }
}

impl Peer for PeerKey {
    fn role(&self) -> Role {
        self.role
    }
}

/// Notifies each frame from a peers outbox on its MSG_CHAR pipe, in order (peripheral role).
/// With a `tracer` from the central subscribing to TRACER_CHAR, every WRITE_TRACER_EVERY frames it waits
/// for a confirmed indication before reporting FrameSent. 
/// Ends when the loop drops the outbox, or reports `Gone` when a pipe fails.
pub(crate) async fn send_frames(
    peer: PeerKey,
    writer: CharacteristicWriter,
    tracer: Option<UnixDatagram>,
    mut outbox_rx: mpsc::Receiver<Vec<u8>>,
    events: mpsc::Sender<RadioEvent>,
) {
    let mut frames_sent: usize = 0;
    let mut batch = BatchTimer::default();
    while let Some(frame) = outbox_rx.recv().await {
        batch.on_frame(frame.len());
        if let Err(err) = writer.send(&frame).await {
            log::warn!("{} ({:?}) send failed: {}", peer.address, peer.role, err);
            let _ = events.send(RadioEvent::Gone { peer }).await;
            break;
        }
        frames_sent += 1;

        if let Some(tracer) = &tracer {
            if frames_sent % WRITE_TRACER_EVERY == 0 {
                if let Err(err) = await_tracer(tracer).await {
                    log::warn!("{} ({:?}) tracer failed: {}", peer.address, peer.role, err);
                    let _ = events.send(RadioEvent::Gone { peer }).await;
                    break;
                }
                batch.finish(peer);
            }
        }
        let _ = events.send(RadioEvent::FrameSent { peer }).await;
    }
}

/// Sends one indication on TRACER_CHAR and waits for the centrals confirmation. A zero length read means BlueZ closed it.
async fn await_tracer(tracer: &UnixDatagram) -> std::io::Result<()> {
    tracer.send(&[1]).await?;
    let mut confirmation = [0u8; 1];
    match tracer.recv(&mut confirmation).await? {
        0 => Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "tracer pipe closed")),
        _ => Ok(()),
    }
}

/// Everything that exists only while BLE is on. Dropping it takes the
/// service off air.
pub struct Running {
    _session: Session,
    adapter: Adapter,
    advertisement: Advertisement,
    /// `None` while stopped: at the connection cap, or after a centrals connect ends it.
    advert: Option<AdvertisementHandle>,
    /// After a failed advertise, not before this.
    advert_retry_at: Option<Instant>,
    _app: ApplicationHandle,
    pub events: mpsc::Receiver<RadioEvent>,
    /// Handed to each central connect we start, so its tasks report on events
    events_tx: mpsc::Sender<RadioEvent>,
    /// True stops the scanner.
    scan_paused: watch::Sender<bool>,
    outboxes: HashMap<PeerKey, Outbox>,
    /// The protocol state. Lives here so a restart always begins with no links.
    pub engine: Engine<PeerKey>,
}

impl Running {
    pub async fn start(device_id: &str, qaul_id: Bytes) -> Result<Running, Box<dyn Error>> {
        // Checked first, so a bad ID fails before anything goes on air.
        let our_id: [u8; QAUL_ID_BYTES] = qaul_id[..]
            .try_into()
            .map_err(|_| format!("qaul id must be {} bytes, got {}", QAUL_ID_BYTES, qaul_id.len()))?;

        let session = bluer::Session::new().await?;
        let agent = bluer::agent::Agent {
            request_default: false,
            ..Default::default()
        };
        let _ = session.register_agent(agent).await?;
        // Find the adapter matching the requested device_id
        let adapter = Self::find_adapter_by_id(&session, device_id).await?;
        adapter.set_powered(true).await?;

        /******* Setup Advertiser *********/

        // A truncated qaul ID for pre connection decisions, role tiebreaker, ignore list etc
        let mut manufacturer_data = BTreeMap::new();
        manufacturer_data.insert(
            QAUL_MANUFACTURER_ID,
            qaul_id
                .iter()
                .take(QAUL_ID_ADVERT_BYTES)
                .copied()
                .collect::<Vec<u8>>(),
        );

        let le_advertisement = Advertisement {
            advertisement_type: bluer::adv::Type::Peripheral,
            service_uuids: vec![main_service_uuid()].into_iter().collect(),
            manufacturer_data,
            discoverable: Some(true),
            tx_power: Some(ADVERT_TX_POWER_DBM),
            min_interval: Some(ADVERT_INTERVAL),
            max_interval: Some(ADVERT_INTERVAL),
            ..Default::default()
        };

        let advert = adapter.advertise(le_advertisement.clone()).await?;

        /***** Setup GATT server *******/

        let (events_tx, events_rx) = tokio::sync::mpsc::channel::<RadioEvent>(64);

        let app = serve(&adapter, qaul_id, events_tx.clone()).await?;

        /***** Start scanning *******/

        let (scan_paused, scan_paused_rx) = watch::channel(false);
        scan(&adapter, events_tx.clone(), scan_paused_rx).await?;

        Ok(Running{
            _session: session,
            adapter,
            advertisement: le_advertisement,
            advert: Some(advert),
            advert_retry_at: None,
            _app: app,
            events: events_rx,
            events_tx,
            scan_paused,
            outboxes: HashMap::new(),
            engine: Engine::new(our_id, gossip_seq_seed()),
        })
    }

    /// Start connecting to `peer` as central. The result comes back on `events`.
    pub fn connect(&self, peer: PeerKey) {
        gatt_client::connect(self.adapter.clone(), peer.address, self.events_tx.clone());
    }

    /// No scanning while one of our connects is in flight, from Connect until Subscribed or ConnectFailed. 
    /// Measured: the kernel holds the connect until blueZ's scan round ends which can be up to 10s, then it 
    /// takes 20–100 ms. Only tells the scanner when it changes.
    pub fn update_scan_pause(&self) {
        let pause = self.engine.is_connecting();
        self.scan_paused.send_if_modified(|paused: &mut bool| {
            let changed = *paused != pause;
            *paused = pause;
            changed
        });
    }

    /// A central connected through our advert, which ends it: the controller terminates the advertising
    /// set, unlike Android which restarts it. Drops the dead handle so update_advertising puts a new one
    /// on air. Without this there can be at most one peripheral connection.
    fn advert_ended(&mut self) {
        self.advert = None;
    }

    /// Advertise only while the engine has room for another link. Pause BleAdvertiser at `MAX_CONNECTIONS`, so centrals stop connecting to a node that would refuse them.
    /// Only acts when that changes, or after advert_ended.
    pub async fn update_advertising(&mut self) {
        let room = self.engine.has_room();
        match (room, self.advert.is_some()) {
            (false, true) => {
                self.advert = None;
                log::info!("at connection cap, advertising stopped");
            }
            (true, false) => {
                // After a failure wait before retrying, so a refusing controller isn't hammered on every event. TODO: investigate linux add/scan restart caps
                if self.advert_retry_at.is_some_and(|at| Instant::now() < at) {
                    return;
                }
                match self.adapter.advertise(self.advertisement.clone()).await {
                    Ok(handle) => {
                        self.advert = Some(handle);
                        self.advert_retry_at = None;
                        log::info!("advertising");
                    }
                    Err(err) => {
                        self.advert_retry_at = Some(Instant::now() + ADVERT_RETRY);
                        log::warn!("advertising failed, retrying in {:?}: {}", ADVERT_RETRY, err);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn transmit(&self, peer: PeerKey, bytes: Vec<u8>) {
        // A dropped chunk is recovered by MissingChunk, liveness catches a stuck peer.
        match self.outboxes.get(&peer) {
            Some(outbox) => {
                if let Err(err) = outbox.try_send(bytes) {
                    log::warn!("dropped frame to {:?}: {}", peer, err);
                }
            }
            None => log::warn!("dropped frame to {:?}: no outbox", peer),
        }
    }

    /// A link came up: keep its outbox for use for transmit. When a central connects to us,
    /// its connect also ends our advert so update_advertising will put it back if theres still room.
    pub fn peer_ready(&mut self, peer: PeerKey, outbox: Outbox) {
        if peer.role == Role::Peripheral {
            self.advert_ended();
        }
        self.outboxes.insert(peer, outbox);
    }

    /// Stop talking to `peer`: end its send task and have BlueZ drop the
    /// connection, which closing our notify pipe alone doesn't do.
    ///
    /// Note: does this drop the whole BLE connection to that address? so a link
    /// in the other role over the same connection goes with it?
    pub fn disconnect(&mut self, peer: PeerKey) {
        self.outboxes.remove(&peer);
        let adapter = self.adapter.clone();
        let address = peer.address;
        tokio::task::spawn_local(async move {
            if let Err(err) = adapter.remove_device(address).await {
                log::debug!("remove_device {}: {}", address, err);
            }
        });
    }

    /// Find a Bluetooth adapter whose address matches `device_id`.
    /// Falls back to the default adapter if `device_id` is empty.
    async fn find_adapter_by_id(
        session: &Session,
        device_id: &str,
    ) -> Result<Adapter, Box<dyn Error>> {
        if device_id.is_empty() {
            return Ok(session.default_adapter().await?);
        }

        let adapter_names = session.adapter_names().await?;
        for name in &adapter_names {
            let adapter = session.adapter(name)?;
            let addr = format!("{}", adapter.address().await?);
            if addr == device_id {
                return Ok(adapter);
            }
        }

        Err(format!("BLE device with ID '{}' not found", device_id).into())
    }
}

/// Where our neighbour list seq starts: the clock in whole seconds, so it moves ahead across restarts.
/// A random start would be behind half the time, and peers would drop our lists as stale until their entry for us expires 
fn gossip_seq_seed() -> u16 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs() as u16)
}

/// Check if bluetooth is powered on by the device.
pub async fn is_ble_enabled() -> bool {
    let session = match bluer::Session::new().await {
        Ok(s) => s,
        Err(e) => {
            log::warn!("Failed to create BLE session: {}", e);
            return false;
        }
    };
    let adapter = match session.default_adapter().await {
        Ok(a) => a,
        Err(e) => {
            log::warn!("No default BLE adapter found: {}", e);
            return false;
        }
    };
    adapter.is_powered().await.unwrap_or(false)
}

pub async fn get_device_info() -> Result<BleInfoResponse, Box<dyn Error>> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let mut devices = Vec::new();


    for adapter_name in adapter_names {
        let adapter = session.adapter(&adapter_name)?;
        let has_multiple_adv_support = adapter
            .supported_advertising_features()
            .await?
            .map_or(false, |feats| {
                feats.contains(&bluer::adv::PlatformFeature::HardwareOffload)
            });
        // bluer fails with MissingKey("MinTxPower") on controllers without
        // tx power control (e.g. BT 4.2), so treat an error as "unknown".
        let max_adv_length = adapter
            .supported_advertising_capabilities()
            .await
            .ok()
            .flatten()
            .map(|caps| caps.max_advertisement_length)
            .unwrap_or(30);
        let device_info = BleDeviceInfo {
            ble_support: true,
            id: format!("{}", adapter.address().await?),
            name: adapter.name().into(),
            bluetooth_on: adapter.is_powered().await?,
            adv_extended: max_adv_length > 31,
            adv_extended_bytes: max_adv_length as u32,
            le_2m: false,                   // TODO: provide actual value
            le_coded: false,                // TODO: provide actual value
            le_audio: false,                // TODO: provide actual value
            le_periodic_adv_support: false, // TODO: provide actual value
            le_multiple_adv_support: has_multiple_adv_support,
            offload_filter_support: false, // TODO: provide actual value
            offload_scan_batching_support: false, // TODO: provide actual value
        };
        devices.push(device_info);
    }


    let response = BleInfoResponse {
        device: None,
        devices: devices,
    };
    Ok(response)
}



   