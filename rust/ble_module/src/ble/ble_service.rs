use std::{cell::RefCell, collections::HashMap, error::Error};

use async_std::{channel::Sender, prelude::*};
use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::{local::*, CharacteristicReader},
    Adapter, AdapterEvent, Address, Device, Session,
};
use bytes::Bytes;
use futures::FutureExt;
use futures_concurrency::stream::Merge;

use crate::ble::ble_uuids::main_service_uuid;
use crate::ble::ble_uuids::msg_char;
use crate::ble::ble_uuids::msg_service_uuid;
use crate::ble::ble_uuids::read_char;
use crate::{
    ble::utils::mac_to_string,
    rpc::{proto_sys::*, utils::*},
};

pub enum QaulBleService {
    Idle(IdleBleService),
    Started(StartedBleService),
}

enum QaulBleHandle {
    AdvertisementHandle(AdvertisementHandle),
    AppHandle(ApplicationHandle),
}

pub struct StartedBleService {
    ble_handles: Vec<QaulBleHandle>,
    adapter: Adapter,
    session: Session,
    device_block_list: Vec<Address>,
    address_lookup: RefCell<HashMap<Vec<u8>, Address>>,
    stop_handle: Sender<bool>,
}

pub struct IdleBleService {
    ble_handles: Vec<QaulBleHandle>,
    adapter: Adapter,
    session: Session,
    device_block_list: Vec<Address>,
    address_lookup: RefCell<HashMap<Vec<u8>, Address>>,
}

enum BleMainLoopEvent {
    Stop,
    MessageReceived((Vec<u8>, Address)),
    MainCharEvent(CharacteristicControlEvent),
    MsgCharEvent(CharacteristicControlEvent),
    DeviceDiscovered(Device),
}

impl IdleBleService {
    /// Initialize a new BleService
    /// Gets default Bluetooth adapter and initializes a Bluer session
    pub async fn new() -> Result<QaulBleService, Box<dyn Error>> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        Ok(QaulBleService::Idle(IdleBleService {
            ble_handles: vec![],
            adapter,
            session,
            device_block_list: vec![],
            address_lookup: RefCell::new(HashMap::new()),
        }))
    }
}

impl IdleBleService {
    pub async fn advertise_scan_listen(
        mut self,
        qaul_id: Bytes,
        advert_mode: Option<i16>,
    ) -> QaulBleService {
        // ==================================================================================
        // ------------------------- SET UP ADVERTISEMENT -----------------------------------
        // ==================================================================================

        let advertisement = Advertisement {
            service_uuids: vec![main_service_uuid()].into_iter().collect(),
            tx_power: advert_mode,
            discoverable: Some(true),
            local_name: Some("qaul.net".to_string()),
            ..Default::default()
        };

        match self.adapter.advertise(advertisement).await {
            Ok(handle) => self
                .ble_handles
                .push(QaulBleHandle::AdvertisementHandle(handle)),
            Err(err) => {
                error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        debug!(
            "Advertising qaul main BLE service at UUID {}",
            main_service_uuid()
        );

        // ==================================================================================
        // ------------------------- SET UP APPLICATION -------------------------------------
        // ==================================================================================

        let (_, main_service_handle) = service_control();
        let (main_chara_ctrl, main_chara_handle) = characteristic_control();

        let main_service = Service {
            uuid: main_service_uuid(),
            primary: true,
            characteristics: vec![Characteristic {
                uuid: read_char(),
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        let value = qaul_id.clone();
                        async move {
                            debug!("Read request {:?} with value {:x?}", &req, &value);
                            Ok(value.to_vec())
                        }
                        .boxed()
                    }),
                    ..Default::default()
                }),
                control_handle: main_chara_handle,
                ..Default::default()
            }],
            control_handle: main_service_handle,
            ..Default::default()
        };

        let (_, msg_service_handle) = service_control();
        let (msg_chara_ctrl, msg_chara_handle) = characteristic_control();

        let msg_service = Service {
            uuid: msg_service_uuid(),
            primary: true,
            characteristics: vec![Characteristic {
                uuid: msg_char(),
                write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Io,
                    ..Default::default()
                }),
                control_handle: msg_chara_handle,
                ..Default::default()
            }],
            control_handle: msg_service_handle,
            ..Default::default()
        };

        let app = Application {
            services: vec![main_service, msg_service],
            ..Default::default()
        };

        match self.adapter.serve_gatt_application(app).await {
            Ok(handle) => self.ble_handles.push(QaulBleHandle::AppHandle(handle)),
            Err(err) => {
                error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        // ==================================================================================
        // --------------------------------- SCAN -------------------------------------------
        // ==================================================================================

        let device_stream = match self.adapter.discover_devices().await {
            Ok(addr_stream) => addr_stream.filter_map(|evt| match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    if self.device_block_list.contains(&addr) {
                        return None;
                    }
                    match self.adapter.device(addr) {
                        Ok(device) => Some(BleMainLoopEvent::DeviceDiscovered(device)),
                        Err(_) => None,
                    }
                }
                _ => None,
            }),
            Err(err) => {
                error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        // ==================================================================================
        // --------------------------------- MAIN BLE LOOP ----------------------------------
        // ==================================================================================

        let (stop_tx, stop_rx) = async_std::channel::bounded::<bool>(1);

        let (message_tx, message_rx) = async_std::channel::bounded::<BleMainLoopEvent>(32);
        let stop_stream = stop_rx.map(|_| BleMainLoopEvent::Stop);
        let main_evt_stream = main_chara_ctrl.map(BleMainLoopEvent::MainCharEvent);
        let msg_evt_stream = msg_chara_ctrl.map(BleMainLoopEvent::MsgCharEvent);

        let mut merged_ble_streams = (
            stop_stream,
            main_evt_stream,
            msg_evt_stream,
            device_stream,
            message_rx,
        )
            .merge();

        while let Some(evt) = merged_ble_streams.next().await {
            match evt {
                BleMainLoopEvent::Stop => {
                    info!("Received stop signal, stopping advertising, scanning, and listening.");
                    break;
                }
                BleMainLoopEvent::MessageReceived(e) => {
                    info!(
                        "Received {} bytes of data from {}",
                        e.0.len(),
                        mac_to_string(&e.1)
                    );
                    send_direct_received(e.1 .0.to_vec(), e.0)
                }
                BleMainLoopEvent::MainCharEvent(_e) => {
                    // TODO: should main character events be sent to the UI?
                }
                BleMainLoopEvent::MsgCharEvent(e) => match e {
                    CharacteristicControlEvent::Write(write) => {
                        if let Ok(reader) = write.accept() {
                            let message_tx = message_tx.clone();
                            let _ = self.spawn_msg_listener(reader, message_tx);
                        }
                    }
                    CharacteristicControlEvent::Notify(_) => (),
                },
                BleMainLoopEvent::DeviceDiscovered(device) => {
                    match self.on_device_discovered(&device).await {
                        Ok(msg_receivers) => {
                            for rec in msg_receivers {
                                let message_tx = message_tx.clone();
                                let _ = self.spawn_msg_listener(rec, message_tx);
                            }
                        }
                        Err(err) => {
                            error!("{:#?}", err);
                        }
                    }
                }
            }
        }

        QaulBleService::Started(StartedBleService {
            ble_handles: self.ble_handles,
            adapter: self.adapter,
            session: self.session,
            device_block_list: self.device_block_list,
            stop_handle: stop_tx,
            address_lookup: self.address_lookup,
        })
    }

    async fn on_device_discovered(
        &self,
        device: &Device,
    ) -> Result<Vec<CharacteristicReader>, Box<dyn Error>> {
        let mut msg_receivers: Vec<CharacteristicReader> = vec![];

        let stringified_addr = mac_to_string(&device.address());
        let uuids = device.uuids().await?.unwrap_or_default();
        trace!(
            "Discovered device {} with service UUIDs {:?}",
            &stringified_addr,
            &uuids
        );

        if !uuids.contains(&main_service_uuid()) {
            return Ok(msg_receivers);
        }
        debug!("Discovered qaul bluetooth device {}", &stringified_addr);

        if !device.is_connected().await? {
            device.connect().await?;
            info!("Connected to device {}", &stringified_addr);
        }

        for service in device.services().await? {
            let service_uuid = service.uuid().await?;
            if service_uuid != main_service_uuid() {
                continue;
            }
            for char in service.characteristics().await? {
                let flags = char.flags().await?;
                if flags.notify || flags.indicate {
                    msg_receivers.push(char.notify_io().await?);
                    info!(
                        "Setting up notification for characteristic {} of device {}",
                        char.uuid().await?,
                        &stringified_addr
                    );
                } else if flags.read && char.uuid().await? == read_char() {
                    let remote_qaul_id = char.read().await?;
                    self.address_lookup
                        .borrow_mut()
                        .insert(remote_qaul_id.clone(), device.address());
                    let rssi = device.rssi().await?.unwrap_or(999) as i32;
                    send_device_found(remote_qaul_id, rssi)
                }
            }
        }
        Ok(msg_receivers)
    }

    async fn spawn_msg_listener(
        &self,
        reader: CharacteristicReader,
        message_tx: Sender<BleMainLoopEvent>,
    ) {
        async_std::task::spawn(async move {
            while let Some(msg) = reader.recv().await.ok() {
                if message_tx.receiver_count() != 0 {
                    break;
                }
                let _ = message_tx
                    .send(BleMainLoopEvent::MessageReceived((
                        msg,
                        reader.device_address(),
                    )))
                    .await
                    .map_err(|err| error!("{:#?}", err));
            }
        });
    }
}

impl StartedBleService {
    pub async fn direct_send(
        &self,
        direct_send_request: &BleDirectSend,
    ) -> Result<(), Box<dyn Error>> {
        let addr_loopup = self.address_lookup.borrow();
        let recipient = addr_loopup
            .get(&direct_send_request.receiver_id)
            .ok_or("Could not find a device address for the given qaul ID!")?;
        let stringified_addr = mac_to_string(&recipient);
        let device = self.adapter.device(recipient.to_owned())?;

        if !device.is_connected().await? {
            device.connect().await?;
            info!("Connected to device {}", &stringified_addr);
        }

        for service in device.services().await? {
            if service.uuid().await? == msg_service_uuid() {
                for chara in service.characteristics().await? {
                    if chara.uuid().await? == msg_char() {
                        chara.write(&direct_send_request.data).await?;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn stop(self) -> QaulBleService {
        if let Err(err) = self.stop_handle.send(true).await {
            error!("Failed to stop bluetooth service: {:#?}", &err);
            send_stop_unsuccessful(err.to_string());
            return QaulBleService::Started(self);
        }

        for handle in self.ble_handles {
            drop(handle)
        }

        send_stop_successful();

        QaulBleService::Idle(IdleBleService {
            ble_handles: vec![],
            adapter: self.adapter,
            session: self.session,
            device_block_list: self.device_block_list,
            address_lookup: self.address_lookup,
        })
    }
}

pub async fn get_device_info() -> Result<(), Box<dyn Error>> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    let has_multiple_adv_support = adapter
        .supported_advertising_features()
        .await?
        .unwrap()
        .contains(&bluer::adv::PlatformFeature::HardwareOffload);
    let max_adv_length = adapter
        .supported_advertising_capabilities()
        .await?
        .map(|caps| caps.max_advertisement_length)
        .unwrap_or(30);
    let this_device = BleDeviceInfo {
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
    let response = BleInfoResponse {
        device: Some(this_device),
    };
    send_ble_sys_msg(ble::Message::InfoResponse(response));
    Ok(())
}
