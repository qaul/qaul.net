// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use super::super::BleRpc;
use super::utils::find_device_by_mac;
use crate::{
    ble::ble_uuids::{main_service_uuid, msg_char, read_char},
    ble::utils,
    gatt_protocol::{self, GattMessageReassembler},
    rpc::{process_received_message, proto_sys::ble::Message::*, proto_sys::*, utils::*},
};
use async_std::{channel::Sender, prelude::*, task::JoinHandle};
use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::{local::*, CharacteristicReader},
    Adapter, AdapterEvent, Address, Device, Session,
};
use bytes::Bytes;
use futures::FutureExt;
use futures_concurrency::stream::Merge;
use lazy_static::lazy_static;
use std::{
    cell::RefCell, collections::{HashMap, HashSet, VecDeque}, error::Error,
    sync::Mutex,
};
use tokio::io::AsyncReadExt;

lazy_static! {
    static ref HASH_MAP: Mutex<HashMap<String, VecDeque<(String, Vec<u8>, Vec<u8>)>>> =
        Mutex::new(HashMap::new());
}

pub enum QaulBleService {
    Idle(IdleBleService),
    Started(StartedBleService),
}

enum QaulBleHandle {
    AdvertisementHandle(AdvertisementHandle),
    AppHandle(ApplicationHandle),
}

pub struct StartedBleService {
    join_handle: Option<JoinHandle<IdleBleService>>,
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
    DeviceDiscovered(Device),
    RpcEvent(Vec<u8>),
}

impl IdleBleService {
    /// Initialize a new BleService.    
    /// 
    /// Gets default Bluetooth adapter and initializes a Bluer session
    pub async fn new() -> Result<QaulBleService, Box<dyn Error>> {
        let session = bluer::Session::new().await?;
        let agent = bluer::agent::Agent {
            request_default: false,
            ..Default::default()
        };
        let _ = session.register_agent(agent).await?;
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

    /// Starts BLE advertisement, scan, and listen.
    /// Manages streams for BLE events and messages.
    pub async fn advertise_scan_listen(
        mut self,
        qaul_id: Bytes,
        advert_mode: Option<i16>,
        mut internal_sender: BleResultSender,
        rpc_receiver: BleRpc,
    ) -> QaulBleService {
        // ==================================================================================
        // ------------------------- SET UP ADVERTISEMENT -----------------------------------
        // ==================================================================================

        let le_advertisement = Advertisement {
            advertisement_type: bluer::adv::Type::Peripheral,
            service_uuids: vec![
                main_service_uuid(),
            ]
            .into_iter()
            .collect(),
            tx_power: advert_mode,
            discoverable: Some(true),
            local_name: Some(utils::get_random_string(5)),
            ..Default::default()
        };
        match self.adapter.advertise(le_advertisement).await {
            Ok(handle) => self
                .ble_handles
                .push(QaulBleHandle::AdvertisementHandle(handle)),
            Err(err) => {
                log::error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };             

        // ==================================================================================
        // ------------------------- SET UP APPLICATION -------------------------------------
        // ==================================================================================

        let (_, main_service_handle) = service_control();
        let (_, main_chara_handle) = characteristic_control();
        let (msg_chara_ctrl, msg_chara_handle) = characteristic_control();

        let (cmd_tx, cmd_rx) = async_std::channel::bounded::<BleMainLoopEvent>(8);
        let (adp_send, adp_recv) = async_std::channel::unbounded::<Adapter>();

        match adp_send.try_send(self.adapter.clone()) {
            Ok(_) => {
                log::debug!("Adapter sent to channel");
            }
            Err(err) => {
                log::error!("{:#?}", err);
            }
        }  

        let msg_characterstic = Characteristic {
            uuid: msg_char(),
            write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Io,
                    ..Default::default()
                }),
            control_handle: msg_chara_handle,
            ..Default::default()
        };

        let adp = self.adapter.clone();
        let cmd_tx2 = cmd_tx.clone();
        let main_characterstic = Characteristic {
            uuid: read_char(),
            read: Some(CharacteristicRead {
                read: true,
                fun: Box::new(move |req| {
                    log::info!(
                        "Read request received from device: {:?}",
                        &(req.device_address)
                    );      

                    // Below snippet checks for device presence in ignore list(discovered devices lsit)
                    // If device is present, it updates the last found time.
                    // Else, It triggers an event to try to discover and connect to device.
                    match utils::find_ignore_device_by_mac(req.device_address) {
                        Some(_) => {
                            // utils::update_last_found(req.device_address);
                        }
                        None => {
                            let adp2 = adp.clone();
                            let cmd_tx2 = cmd_tx2.clone();
                            async_std::task::spawn(async move {
                                match adp2.device(req.device_address) {
                                    Ok(device) => {
                                        match cmd_tx2.send(BleMainLoopEvent::DeviceDiscovered(device.clone())).await {
                                            Ok(_) => {
                                                // Add a null ble_device to stop event from being sent again
                                                // Will be update in func: on_device_discovered.
                                                let ble_device = utils::BleScanDevice {
                                                    qaul_id: vec![],
                                                    rssi: 0,
                                                    mac_address: device.address(),
                                                    name: device.name().await.unwrap().unwrap_or_default(),
                                                    device,
                                                    last_found_time: utils::current_time_millis(),
                                                    is_connected: false,
                                                };
                                                utils::add_ignore_device(ble_device);
                                                log::info!("Device discovered event sent");
                                            }
                                            Err(err) => {
                                                log::error!("Error sending device discovered event: {:#?}", err);
                                            }
                                        };
                                    },
                                    Err(e) => {
                                        log::error!("Error: {:#?}", e);
                                    }
                                };
                            });
                        }
                    }
                    let value = qaul_id.clone();
                    async move { Ok(value.to_vec()) }.boxed()
                }),
                ..Default::default()
            }),
            control_handle: main_chara_handle,
            ..Default::default()
        };

        let main_service = Service {
            uuid: main_service_uuid(),
            primary: true,
            characteristics: vec![main_characterstic, msg_characterstic],
            control_handle: main_service_handle,
            ..Default::default()
        };

        let app = Application {
            services: vec![main_service],
            ..Default::default()
        };

        match self.adapter.serve_gatt_application(app).await {
            Ok(handle) => self.ble_handles.push(QaulBleHandle::AppHandle(handle)),
            Err(err) => {
                log::error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        // ==================================================================================
        // ------------------------- MAIN BLE LOOP ------------------------------------------
        // ==================================================================================

        let join_handle = async_std::task::Builder::new()
            .name("main-ble-loop".into())
            .local(async move {
                
                let adapter: Adapter = adp_recv.recv().await.unwrap_or_else(|err| {
                    log::error!("{:#?}", err);
                    self.adapter.clone()
                });

                // Set up discovery filter and start streaming the discovered devices adn out of range checker.
                let _ = adapter.set_discovery_filter(get_filter()).await;
                let mut device_result_sender = internal_sender.clone();
                let device_stream = match adapter.discover_devices().await {
                    Ok(addr_stream) => addr_stream.filter_map(|evt| match evt {
                        AdapterEvent::DeviceAdded(addr) => {
                            if self.device_block_list.contains(&addr) {
                                return None;
                            }
                            match self.adapter.device(addr) {
                                Ok(device) => {
                                    log::warn!("Discovered device {:?}", addr);
                                    Some(BleMainLoopEvent::DeviceDiscovered(device))
                                },
                                Err(_) => None,
                            }
                        },
                        AdapterEvent::DeviceRemoved(addr) => {
                            utils::find_device_by_mac(addr).map(|device| {
                                device_result_sender.send_device_unavailable(
                                    device.qaul_id.clone(),
                                    adapter.clone(),
                                    addr,
                                );
                            });
                            return None;
                        }, 
                        AdapterEvent::PropertyChanged(_) => None,
                    }),
                    Err(err) => {
                        log::error!("Error: {:#?}", err);
                        return self;
                    }
                };


                let _ = self
                    .spawn_msg_listener(internal_sender.clone(), msg_chara_ctrl)
                    .await; 
                
                // TODO: Setup out of range checker. 
                // utils::out_of_range_checker(adapter.clone(), internal_sender.clone());

                let rpc_reciever_stream = rpc_receiver.receiver.map(BleMainLoopEvent::RpcEvent);
                let mut merged_ble_streams = (
                    cmd_rx,
                    device_stream,
                    rpc_reciever_stream,
                )
                    .merge();

                'outer: loop {
                    match merged_ble_streams.next().await {
                        Some(evt) => {
                            log::debug!("Received event:");
                            match evt {
                                BleMainLoopEvent::DeviceDiscovered(device) => {
                                    match self
                                        .on_device_discovered(&device, &mut internal_sender)
                                        .await
                                    {
                                        Ok(_) => {
                                        log::info!("Device discovered response sent");
                                        // Ok(msg_receivers) => {
                                            // Was present in kotlin code but no need for the snippet below

                                            // log::debug!("Device discovered {:?}", device.name().await);
                                            // let message_tx = message_tx.clone();``
                                            // for rec in msg_receivers {
                                            //     let message_tx = message_tx.clone();
                                            // }
                                        }
                                        Err(err) => {
                                            log::error!("{:#?}", err);
                                        }
                                    }
                                },
                                BleMainLoopEvent::RpcEvent(evt) => match process_received_message(evt)  {
                                    None => {
                                        log::info!("Qaul 'sys' message channel closed. Shutting down gracefully.");
                                        break;
                                    },
                                    Some(msg) => {
                                        log::debug!("Received rpc event: ");
                                        if msg.message.is_none() {
                                            continue;
                                        }
                                        match msg.message.unwrap() {
                                            StartRequest(_) => {
                                                    log::warn!(
                                                        "Received Start Request, but bluetooth service is already running!"
                                                    );
                                            },
                                            StopRequest(_) => {
                                                log::info!("Received Stop Request");
                                                internal_sender.send_stop_successful();
                                                break 'outer;                                                
                                            },
                                            DirectSend(mut req) => {
                                                log::info!("Received Direct Send Request: ");
                                                let msg_id = String::from_utf8_lossy(&req.message_id);
                                                log::info!("Sending message with ID: {:?}", &msg_id);
                                                match self
                                                    .send_direct_message(
                                                        msg_id.to_string(),
                                                        &mut req.receiver_id,
                                                        req.sender_id,
                                                        req.data,
                                                    )
                                                    .await
                                                {
                                                    Ok(_) => {
                                                        internal_sender.send_direct_send_success(req.receiver_id);
                                                    }
                                                    Err(err) => {
                                                        log::error!("Error sending direct BLE message: {:#?}", err);
                                                        internal_sender.send_direct_send_error(req.receiver_id, err.to_string());   
                                                    }
                                                }
                                            },
                                            InfoRequest(_) => {
                                                let mut sender_handle_clone = internal_sender.clone();
                                                match get_device_info().await {
                                                    Ok(info) => {
                                                        sender_handle_clone.send_ble_sys_msg(InfoResponse(info))
                                                    }
                                                    Err(err) => {
                                                        log::error!("Error getting device info: {:#?}", &err)
                                                    }
                                                }
                                            }
                                            _ => {
                                                log::info!("Received unknown rpc event");
                                            },
                                        }
                                    }
                                }
                            }
                        },
                        None => {
                                log::info!("No event recieved.");
                        }
                    };
                }

                for handle in self.ble_handles.drain(..) {
                    drop(handle)
                }
                self
            })
            .expect("Unable to spawn BLE main loop!");

        QaulBleService::Started(StartedBleService {
            join_handle: Some(join_handle),
            cmd_handle: cmd_tx,
        })
    }

    /// Handles device discovery and validates the Qaul Services and Characterstic.
    async fn on_device_discovered(
        &self,
        device: &Device,
        sender: &mut BleResultSender,
    ) -> Result<Vec<CharacteristicReader>, Box<dyn Error>> {
        let rssi = device.rssi().await?.unwrap_or(999) as i32;
        let mut read_char_uuid_found = false;
        let mut msg_receivers: Vec<CharacteristicReader> = vec![];
        let stringified_addr = utils::mac_to_string(&device.address());
        let device_name = device.name().await.unwrap_or_default().unwrap_or_default();
        if !(device_name.len() == 5 || device_name == "qaul") {
            return Err("Not a Qaul device".into());     
        }
        log::info!(
            "Discovered device {} with name {:?}",
            &stringified_addr,
            &device_name,
        );

        let mut retries = 1;
        if !device.is_connected().await? {   
            log::warn!("Device not connected. Trying to connect...");
            loop {
                match device.connect().await {
                    Ok(()) => break,
                    Err(err) => {   
                        if retries > 0 {
                            log::error!("    Connect error: {}", &err);
                            retries -= 1;
                        } else {
                            self.adapter.remove_device(device.address()).await?;
                            return Err("Connection retries timeout.".into());
                        }
                    }
                }
            }
        }
        
        let services_discovered = device.services().await?;
        for service in services_discovered {
            if read_char_uuid_found {
                break;
            }
            let service_uuid = service.uuid().await?;
            log::info!(
                "Service UUID: {:?} for device {:?}",
                service_uuid,
                &stringified_addr   
            );
            if service_uuid != main_service_uuid() {
                continue;
            }
            for char in service.characteristics().await? {
                let flags = char.flags().await?;
                let remote_char_uuid = char.uuid().await?;

                log::info!(
                    "Characteristic flags for device {} are: {:?}  and service {} and char {}",
                    &stringified_addr,
                    &flags.read,
                    &service_uuid,
                    &remote_char_uuid,
                );
                if flags.notify || flags.indicate {
                    // Was present in Kotlin code but never used.
                    msg_receivers.push(char.notify_io().await?);
                    log::info!(
                        "Setting up notification for characteristic {} of device {}",
                        &remote_char_uuid,
                        &stringified_addr
                    );
                } else if flags.read && remote_char_uuid == read_char() {
                    let remote_qaul_id = char.read().await?;
                    let ble_device = utils::BleScanDevice {
                        qaul_id: remote_qaul_id.clone(),
                        rssi,
                        mac_address: device.address(),
                        device: device.clone(),
                        last_found_time: utils::current_time_millis(),
                        name: device.name().await.unwrap().unwrap_or_default(),
                        is_connected: false,
                    };
                    utils::add_ignore_device(ble_device.clone());
                    utils::add_device(ble_device);
                    read_char_uuid_found = true;
                    log::info!(
                        "Read characteristic found for device {} with qaul ID {:?}",
                        &stringified_addr,
                        &remote_qaul_id
                    );
                    self.address_lookup
                        .borrow_mut()
                        .insert(remote_qaul_id.clone(), device.address());

                    sender.send_device_found(remote_qaul_id, rssi);
                    break;
                }
            }
        }
        if !read_char_uuid_found {
            log::info!(
                "Read characteristic not found for device {}",
                &stringified_addr
            );
            return Err("UUIDs not found".into());
        }
        device.disconnect().await?;
        Ok(msg_receivers)
    }

    /// Async message listner which is spawned every time a new device tries to write to the msg_char().
    /// The messages are streamed in chunks of 40 or less bytes.
    /// The function handles maintains a message map for each nearby users to separate sending queues.
    async fn spawn_msg_listener(
        &self,
        internal_sender: BleResultSender,
        mut msg_chara_ctrl: CharacteristicControl,
    ) {
        let (ble_msg_sender, ble_msg_reciever) = async_std::channel::unbounded::<(Address, Vec<u8>)>();
                
        async_std::task::spawn(async move {
            let mut internal_sender = internal_sender;
            let mut reassemblers: HashMap<String, GattMessageReassembler> = HashMap::new();
            log::info!("Spawned message listener for device.");             
            loop {
                if let Ok((mac_address, buffer)) = ble_msg_reciever.recv().await{
                    if buffer.len() == 0 {
                        log::info!("Write stream from device {:?} has ended", &mac_address);
                        continue;
                    }
                    let hex_msg = utils::bytes_to_hex(&buffer);
                    log::info!("Received message: {:?} from {:?}", &hex_msg, &mac_address);
                    let stringified_addr = utils::mac_to_string(&mac_address);
                    let decode_result = {
                        let decoder = reassemblers
                            .entry(stringified_addr.clone())
                            .or_default();
                        decoder.push_chunk(&buffer)
                    };

                    match decode_result {
                        Ok(Some(message)) => {
                            internal_sender.send_direct_received(message.qaul_id, message.message);
                            reassemblers.remove(&stringified_addr);
                        }
                        Ok(None) => {}
                        Err(err) => {
                            log::error!(
                                "Error decoding GATT message from {}: {}",
                                &stringified_addr,
                                err
                            );
                            reassemblers.remove(&stringified_addr);
                        }
                    }
                }
            }
        });

        async_std::task::spawn(async move {                        
            loop {
                match msg_chara_ctrl.next().await {
                    Some(CharacteristicControlEvent::Write(write)) => {
                        let mut device_known: bool = true;
                        let mac_address = write.device_address();
                        match find_device_by_mac(mac_address) {
                            Some(_) => {
                                // utils::update_last_found(mac_address);
                            }
                            None => {
                                device_known = false;
                                log::warn!("Device not found in known devices");
                            }
                        }
                        let mut read_buf = vec![0; 20];
                        if let Ok(mut reader) = write.accept() {   
                            let k = reader.read(&mut read_buf).await;
                            match k {
                                Ok(0) => {
                                    log::debug!("Write stream from device {:?} has ended", &mac_address);
                                    continue;
                                }
                                Ok(read_len) => {
                                    if device_known { 
                                        let _ = ble_msg_sender
                                            .send((mac_address, read_buf[..read_len].to_vec()))
                                            .await;
                                    }
                                },
                                Err(err) => {
                                    log::error!("Write stream error: {}", &err);
                                }
                            }     
                        } else {
                            log::error!("Error accepting write request");
                        }
                    }
                    _ => continue,
                }
            }
        }); 
    }

    /// Sends a serialized version of message to remote device in chunks of 40 bytes or less.
    async fn send_direct_message(
        &self,
        message_id: String,
        receiver_id: &mut Vec<u8>,
        sender_id: Vec<u8>,                     
        data: Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        log::info!("Sending direct message to {:?}", &receiver_id);
        let addr_loopup = self.address_lookup.borrow();
        let mac_address = addr_loopup
            .get(receiver_id)
            .ok_or("Could not find a device address for the given qaul ID!")?;
        let stringified_addr = utils::mac_to_string(mac_address);

        let device =  match self.adapter.device(*mac_address) {
            Ok(device) => device,
            Err(err) => {
                log::error!("Error:: {:#?}", err);
                return Err("Error getting device".into());
            }
        };
        let mut hash_map = HASH_MAP.lock().unwrap();
        match hash_map.get(&stringified_addr) {
            Some(queue) => {
                let mut queue = queue.clone();
                if queue.len() < 2 {
                    queue.push_back((message_id, sender_id, data.clone()));
                } else {
                    queue.clear();
                }
                hash_map.insert(stringified_addr.clone(), queue);
            }
            None => {
                let mut queue: VecDeque<(String, Vec<u8>, Vec<u8>)> = VecDeque::new();
                queue.push_back((message_id, sender_id, data.clone()));
                hash_map.insert(stringified_addr.clone(), queue);
            }
        }

        // Messages from queue are read and broken down into packets of 40 bytes and streamed to the remote device.
        let extracted_queue = hash_map.get(&stringified_addr).clone();
        let mut message_id: String = "".to_string();
        let mut send_queue: VecDeque<Vec<u8>> = VecDeque::new();
        if let Some(queue) = extracted_queue {
            let mut queue = queue.clone();
            if !queue.is_empty() {
                let data = queue.pop_front().unwrap();
                message_id = data.0;
                let chunks = gatt_protocol::encode_direct_message_chunks(&data.1, &data.2)?;
                send_queue = chunks.into_iter().collect();
            }
        }

        // Connect to device and write into characterstic of other device 
        if !device.is_connected().await? {
            match device.connect().await {
                Ok(()) => {
                    log::error!("Device connected");
                }
                Err(err) => {
                    log::error!("    Connect error: {}", &err);
                    return Err("Connection error".into());
                }
            }
        }
        let mut read_char_found = false;
        for service in device.services().await? {
            if read_char_found {
                break;
            }
            if service.uuid().await? == main_service_uuid() {
                for chara in service.characteristics().await? {
                    if chara.uuid().await? == msg_char() {
                        utils::update_last_found(*mac_address);
                        read_char_found = true;
                        while send_queue.len() > 0 {
                            let chunk: Vec<u8>;
                            match send_queue.pop_front() {
                                Some(queue_top) => chunk = queue_top,
                                None => {
                                    log::error!("No data found in queue");
                                    break;
                                }
                            }
                            let chunk_hex = utils::bytes_to_hex(&chunk);
                            log::info!(
                                "Sending data to device {} : {:?}",
                                &stringified_addr,
                                &chunk_hex
                            );
                            match chara.write(&chunk).await {
                                Ok(()) => {
                                    log::debug!(
                                        "Data sent to device {}",
                                        &stringified_addr
                                    );
                                }
                                Err(err) => {
                                    log::error!(
                                        "Error1 sending data to device {}: {:#?}",
                                        &stringified_addr,
                                        &err
                                    );
                                }
                            };
                        }
                    }
                }
            }
        }
        if message_id != "" {
            Ok(message_id)
        } else {
            Err("Message ID not found".into())
        }
    }

    /// Check if bluetooth is powered on by the device.
    pub async fn is_ble_enabled() -> bool {
        let session = bluer::Session::new().await.unwrap();
        let adapter = session.default_adapter().await.unwrap();
        let ble_enabled: bool = adapter.is_powered().await.unwrap();
        drop(session);
        return ble_enabled;
    }
}

impl StartedBleService {
    /// Spawn the local thread created in IdleBleService
    pub async fn spawn_handles(self) {
        match self.join_handle {
            Some(join_handles) => {
                join_handles.await;
            }
            None => {
                log::error!("No handle to spawn");
            }
        }
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

fn get_filter() -> bluer::DiscoveryFilter {
    let mut qaul_uuids = HashSet::new();
    qaul_uuids.insert(main_service_uuid());
    bluer::DiscoveryFilter {
        uuids: qaul_uuids,
        transport: bluer::DiscoveryTransport::Le,
        ..Default::default()
    }
}
