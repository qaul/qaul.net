// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use crate::{
    ble::{
        ble_uuids::{main_service_uuid, QAUL_ID_ADVERT_BYTES, QAUL_MANUFACTURER_ID},
        gatt_server::{serve, ServerEvent},
    },
    rpc::proto_sys::{BleDeviceInfo, BleInfoResponse},
};
use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::local::ApplicationHandle,
    Adapter, Address, Session,
};
use bytes::Bytes;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
    time::Duration,
};
use tokio::sync::mpsc;

/// Requested advert power. Mirrors Android's TX_POWER_HIGH/MAX, the controller clamps it
const ADVERT_TX_POWER_DBM: i16 = 20;

/// Advert interval. Mirrors Android's ADVERTISE_MODE_LOW_LATENCY, which
/// Android's NOMINAL_ADVERT_INTERVAL_MS also assumes.
const ADVERT_INTERVAL: Duration = Duration::from_millis(100);

/// Everything that exists only while BLE is on. Dropping it takes the
/// service off air.
pub struct Running {
    _session: Session,
    adapter: Adapter,
    _advert: AdvertisementHandle,
    _app: ApplicationHandle,
    pub events: mpsc::Receiver<ServerEvent>,
    pub outboxes: HashMap<Address, mpsc::Sender<Vec<u8>>>,
}

impl Running {
    pub async fn start(device_id: &str, qaul_id: Bytes) -> Result<Running, Box<dyn Error>> {
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

        let (server_tx, server_rx) = tokio::sync::mpsc::channel::<ServerEvent>(64);

        let app = serve(&adapter, qaul_id, server_tx).await?;

        Ok(Running{
            _session: session,
            adapter,
            _advert: advert,
            _app: app,
            events: server_rx,
            outboxes: HashMap::new(),
        })
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


/// Scan filter for qaul adverts. Unused until the central role is ported.
#[allow(dead_code)]
fn get_filter() -> bluer::DiscoveryFilter {
    let mut qaul_uuids = HashSet::new();
    qaul_uuids.insert(main_service_uuid());
    bluer::DiscoveryFilter {
        uuids: qaul_uuids,
        transport: bluer::DiscoveryTransport::Le,
        ..Default::default()
    }
}


   