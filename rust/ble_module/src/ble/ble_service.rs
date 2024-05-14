use std::{cell::RefCell, collections::HashMap, error::Error};

use async_std::{channel::Sender, prelude::*, task::JoinHandle};
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
    join_handle: JoinHandle<IdleBleService>,
    cmd_handle: Sender<BleMainLoopEvent>,
}

pub struct IdleBleService {
    ble_handles: Vec<QaulBleHandle>,
    adapter: Adapter,
    _session: Session,
    device_block_list: Vec<Address>,
    address_lookup: RefCell<HashMap<Vec<u8>, Address>>,
}

enum BleMainLoopEvent {
    Stop,
    MessageReceived((Vec<u8>, Address)),
    MainCharEvent(CharacteristicControlEvent),
    MsgCharEvent(CharacteristicControlEvent),
    DeviceDiscovered(Device),
    SendMessage((Vec<u8>, Vec<u8>)),
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
            _session: session,
            device_block_list: vec![],
            address_lookup: RefCell::new(HashMap::new()),
        }))
    }

    pub async fn advertise_scan_listen(
        mut self,
        qaul_id: Bytes,
        advert_mode: Option<i16>,
        mut internal_sender: BleResultSender,
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
                log::error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        log::debug!(
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
                            log::debug!("Read request {:?} with value {:x?}", &req, &value);
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
                log::error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        let (cmd_tx, cmd_rx) = async_std::channel::bounded::<BleMainLoopEvent>(8);

        println!("=========Starting BLE main loop...");

        let join_handle: JoinHandle<IdleBleService> = async_std::task::Builder::new()
            .name("main-ble-loop".into())
            .local(async move {
                // ==================================================================================
                // --------------------------------- SCAN -------------------------------------------
                // ==================================================================================

                println!("============Scanning started for devices");
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
                        log::error!("{:#?}", err);
                        return self;
                    }
                };

                match async_std::task::try_current() {
                    Some(t) => println!("The name of this task is {:?}", t.name()),
                    None => println!("Not inside a task!"),
                }

                // ==================================================================================
                // --------------------------------- MAIN BLE LOOP ----------------------------------
                // ==================================================================================

                let (message_tx, message_rx) = async_std::channel::bounded::<BleMainLoopEvent>(32);
                let main_evt_stream = main_chara_ctrl.map(BleMainLoopEvent::MainCharEvent);
                let msg_evt_stream = msg_chara_ctrl.map(BleMainLoopEvent::MsgCharEvent);

                let mut merged_ble_streams = (
                    cmd_rx,
                    main_evt_stream,
                    msg_evt_stream,
                    device_stream,
                    message_rx,
                )
                    .merge();

                while let Some(evt) = merged_ble_streams.next().await {
                    match evt {
                        BleMainLoopEvent::Stop => {
                            log::info!(
                            "Received stop signal, stopping advertising, scanning, and listening."
                        );
                            break;
                        }
                        BleMainLoopEvent::SendMessage((receiver_id, data)) => {
                            match self.send_direct_message(receiver_id, data).await {
                                Ok(_) => todo!(),
                                Err(err) => {
                                    log::error!("Error sending direct BLE message: {:#?}", err)
                                }
                            }
                        }
                        BleMainLoopEvent::MessageReceived(e) => {
                            log::info!(
                                "Received {} bytes of data from {}",
                                e.0.len(),
                                mac_to_string(&e.1)
                            );
                            internal_sender.send_direct_received(e.1 .0.to_vec(), e.0)
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
                            match self
                                .on_device_discovered(&device, &mut internal_sender)
                                .await
                            {
                                Ok(msg_receivers) => {
                                    for rec in msg_receivers {
                                        let message_tx = message_tx.clone();
                                        let _ = self.spawn_msg_listener(rec, message_tx);
                                    }
                                }
                                Err(err) => {
                                    log::error!("{:#?}", err);
                                }
                            }
                        }
                    }
                }

                for handle in self.ble_handles.drain(..) {
                    drop(handle)
                }

                self
            })
            .expect("Unable to spawn BLE main loop!");
        // join_handle.await;

        // let j  = &join_handle;
        // *j.;
        QaulBleService::Started(StartedBleService {
            // join_handle,
            join_handle,
            cmd_handle: cmd_tx,
        })
        // return QaulBleService::Idle();
    }

    async fn on_device_discovered(
        &self,
        device: &Device,
        sender: &mut BleResultSender,
    ) -> Result<Vec<CharacteristicReader>, Box<dyn Error>> {
        let mut msg_receivers: Vec<CharacteristicReader> = vec![];

        let stringified_addr = mac_to_string(&device.address());
        let uuids = device.uuids().await?.unwrap_or_default();
        log::trace!(
            "Discovered device {} with service UUIDs {:?}",
            &stringified_addr,
            &uuids
        );

        if !uuids.contains(&main_service_uuid()) {
            return Ok(msg_receivers);
        }
        log::debug!("Discovered qaul bluetooth device {}", &stringified_addr);

        if !device.is_connected().await? {
            device.connect().await?;
            log::info!("Connected to device {}", &stringified_addr);
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
                    log::info!(
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
                    sender.send_device_found(remote_qaul_id, rssi)
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
                    .map_err(|err| log::error!("{:#?}", err));
            }
        });
    }

    async fn send_direct_message(
        &self,
        receiver_id: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let addr_loopup = self.address_lookup.borrow();
        let recipient = addr_loopup
            .get(&receiver_id)
            .ok_or("Could not find a device address for the given qaul ID!")?;
        let stringified_addr = mac_to_string(&recipient);
        let device = self.adapter.device(recipient.to_owned())?;

        if !device.is_connected().await? {
            device.connect().await?;
            log::info!("Connected to device {}", &stringified_addr);
        }

        for service in device.services().await? {
            if service.uuid().await? == msg_service_uuid() {
                for chara in service.characteristics().await? {
                    if chara.uuid().await? == msg_char() {
                        chara.write(&data).await?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl StartedBleService {
    pub async fn spawn_handles(self) -> QaulBleService {
        // let mut svc : JoinHandle<IdleBleService> ;
        // swap(&mut svc, self.join_handle);
        // self.join_handle.await;
        let buf: &JoinHandle<IdleBleService> = &self.join_handle;
        buf.await;

        println!("Spawning handles");

        // buf.await;
        // self.join_handle = buf;
        // QaulBleService::Idle(buf.)
        QaulBleService::Started(self::StartedBleService {
            join_handle: self.join_handle,
            cmd_handle: self.cmd_handle.clone(),
        })
    }

    pub async fn direct_send(
        &mut self,
        direct_send_request: BleDirectSend,
    ) -> Result<(), Box<dyn Error>> {
        self.cmd_handle
            .send(BleMainLoopEvent::SendMessage((
                direct_send_request.receiver_id,
                direct_send_request.data,
            )))
            .await?;
        Ok(())
    }

    pub async fn stop(self, sender: &mut BleResultSender) -> QaulBleService {
        if let Err(err) = self.cmd_handle.send(BleMainLoopEvent::Stop).await {
            log::error!("Failed to stop bluetooth service: {:#?}", &err);
            sender.send_stop_unsuccessful(err.to_string());
            return QaulBleService::Started(self);
        }

        sender.send_stop_successful();

        QaulBleService::Idle(self.join_handle.await)
    }
}

pub async fn get_device_info() -> Result<BleInfoResponse, Box<dyn Error>> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    let has_multiple_adv_support = adapter
        .supported_advertising_features()
        .await?
        .map_or(false, |feats| {
            feats.contains(&bluer::adv::PlatformFeature::HardwareOffload)
        });
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
    Ok(response)
}
