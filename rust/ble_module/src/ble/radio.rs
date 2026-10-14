// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use crate::{
    ble::{
        ble_uuids::{main_service_uuid, QAUL_ID_ADVERT_BYTES, QAUL_MANUFACTURER_ID},
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
    gatt::local::ApplicationHandle,
    Adapter, Address, Session,
};
use bytes::Bytes;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    time::Duration,
};
use tokio::sync::mpsc;

/// Requested advert power. Mirrors Android's TX_POWER_HIGH/MAX, the controller clamps it
const ADVERT_TX_POWER_DBM: i16 = 20;

/// Advert interval. Mirrors Android's ADVERTISE_MODE_LOW_LATENCY, which
/// Android's NOMINAL_ADVERT_INTERVAL_MS also assumes.
const ADVERT_INTERVAL: Duration = Duration::from_millis(100);

/// What the radio side tells the loop. One channel for all of them, so the loop sees one ordered stream.
#[derive(Debug)]
pub enum RadioEvent {
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
    /// Scanner saw this advert, pass to engine for connection decision making
    Advert {
        address: Address,
        id_prefix:  [u8; QAUL_ID_ADVERT_BYTES], 
        rssi: i16
    },

    /// The phone's notify pipe failed. May repeat, so the loop must ignore a
    /// `Gone` for an unknown address.
    Gone { address: Address },
}

/// How the engine names a link on Linux: the phones address and our role on it. 
/// The same phone could be linked in both roles at once, so the address alone isn't unique.
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
}

impl Peer for PeerKey {
    fn role(&self) -> Role {
        self.role
    }
}

/// Everything that exists only while BLE is on. Dropping it takes the
/// service off air.
pub struct Running {
    _session: Session,
    adapter: Adapter,
    _advert: AdvertisementHandle,
    _app: ApplicationHandle,
    pub events: mpsc::Receiver<RadioEvent>,
    pub outboxes: HashMap<PeerKey, mpsc::Sender<Vec<u8>>>,
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

        let advert = adapter.advertise(le_advertisement).await?;

        /***** Setup GATT server *******/

        let (events_tx, events_rx) = tokio::sync::mpsc::channel::<RadioEvent>(64);

        let app = serve(&adapter, qaul_id, events_tx.clone()).await?;

        /***** Start scanning *******/

        scan(&adapter, events_tx).await?;

        Ok(Running{
            _session: session,
            adapter,
            _advert: advert,
            _app: app,
            events: events_rx,
            outboxes: HashMap::new(),
            engine: Engine::new(our_id),
        })
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



   