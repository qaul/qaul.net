use crate::ble::ble_uuids::msg_char;
use crate::ble::ble_uuids::msg_service_uuid;
use crate::ble::ble_uuids::read_char;
use crate::ble::{ble_uuids::main_service_uuid, utils::hex_to_bytes};
use crate::{
    ble::utils,
    rpc::{proto_sys::*, utils::*},
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
use serde::{Deserialize, Serialize};
use serde_json;
use std::string;
use std::{
    cell::RefCell, collections::HashMap, collections::HashSet, collections::VecDeque, error::Error,
    sync::Mutex,
};

lazy_static! {
    static ref HASH_MAP: Mutex<HashMap<String, VecDeque<(String, Vec<u8>, Vec<u8>)>>> =
        Mutex::new(HashMap::new());
}

#[derive(Serialize, Deserialize)]
struct Message {
    #[serde(rename = "qaul_id")]
    qaulId: Option<Vec<u8>>,

    #[serde(rename = "message")]
    message: Option<Vec<u8>>,
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
    Stop,
    MessageReceived((Vec<u8>, Address)),
    MainCharEvent(CharacteristicControlEvent),
    MsgCharEvent(CharacteristicControlEvent),
    DeviceDiscovered(Device),
    SendMessage((Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)),
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

    // pub fn adapter_channel_handles() -> (Sender<Adapter>, Receiver<Adapter>){
    //     let (adp_send, adp_recv) = async_std::channel::unbounded::<Adapter>();
    //     (adp_send, adp_recv)
    // }

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
            service_uuids: vec![
                main_service_uuid(),
                msg_service_uuid(),
                msg_char(),
                read_char(),
            ]
            .into_iter()
            .collect(),
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
        let (adp_send, adp_recv) = async_std::channel::unbounded::<Adapter>();
        match adp_send.try_send(self.adapter.clone()) {
            Ok(_) => {
                log::debug!("Sent adapter to channel");
            }
            Err(err) => {
                log::error!("{:#?}", err);
            }
        }
        let (cmd_tx, cmd_rx) = async_std::channel::bounded::<BleMainLoopEvent>(8);

        let join_handle = async_std::task::Builder::new()
            .name("main-ble-loop".into())
            .local(async move {
                log::info!("Starting BLE main loop...");

                let adapter: Adapter;
                match adp_recv.recv().await {
                    Ok(adp) => {
                        log::info!("Received adapter from channel");
                        adapter = adp;
                    }
                    Err(err) => {
                        log::error!("Failed to receive adapter from channel: {:#?}", err);
                        return self;
                    }
                };

                let _ = adapter.set_discovery_filter(get_filter()).await;
                let device_stream = match adapter.discover_devices().await {
                    Ok(addr_stream) => addr_stream.filter_map(|evt| match evt {
                        AdapterEvent::DeviceAdded(addr) => match utils::find_device_by_mac(addr) {
                            Some(device) => {
                                utils::update_last_found(addr);
                                Some(BleMainLoopEvent::DeviceDiscovered(device.device.clone()))
                            }
                            None => {
                                if self.device_block_list.contains(&addr) {
                                    return None;
                                }
                                match adapter.device(addr) {
                                    Ok(device) => {
                                        utils::add_device(utils::BleScanDevice {
                                            qaul_id: vec![],
                                            rssi: 0,
                                            mac_address: addr,
                                            device: device.clone(),
                                            last_found_time: 0,
                                            name: "".to_string(), // TODO: get device name
                                            is_connected: false,
                                        });
                                        println!("{:?}", device);
                                        Some(BleMainLoopEvent::DeviceDiscovered(device))
                                    }
                                    Err(_) => None,
                                }
                            }
                        },
                        _ => None,
                    }),
                    Err(err) => {
                        log::error!("Error: {:#?}", err);
                        return self;
                    }
                };
                utils::out_of_range_checker(internal_sender.clone());
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
                        BleMainLoopEvent::SendMessage((
                            message_id,
                            receiver_id,
                            sender_id,
                            data,
                        )) => {
                            println!("===============Sending message to {:?}", receiver_id);
                            match utils::bytes_to_str(&message_id) {
                                Ok(msg_id) => {
                                    log::info!("Message ID: {:?}", msg_id);
                                    match self
                                        .send_direct_message(
                                            msg_id.to_string(),
                                            receiver_id,
                                            sender_id,
                                            data,
                                        )
                                        .await
                                    {
                                        Ok(_) => {
                                            todo!()
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "Error sending direct BLE message: {:#?}",
                                                err
                                            )
                                        }
                                    }
                                }
                                Err(err) => {
                                    log::error!(
                                        "Error converting message ID to string: {:#?}",
                                        err
                                    );
                                }
                            }
                        }
                        BleMainLoopEvent::MessageReceived(e) => {
                            log::info!(
                                "Received {} bytes of data from {}",
                                e.0.len(),
                                utils::mac_to_string(&e.1)
                            );
                            internal_sender.send_direct_received(e.1 .0.to_vec(), e.0)
                        }
                        BleMainLoopEvent::MainCharEvent(_e) => {
                            // TODO: should main character events be sent to the UI?
                        }
                        BleMainLoopEvent::MsgCharEvent(e) => match e {
                            CharacteristicControlEvent::Write(write) => {
                                let mac_address = write.device_address();
                                if let Ok(reader) = write.accept() {
                                    let message_tx = message_tx.clone();
                                    let _ =
                                        self.spawn_msg_listener(reader, message_tx, mac_address);
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
                                    log::debug!(
                                        "================== Device discovered {:?}",
                                        device.name().await
                                    );
                                    let message_tx = message_tx.clone();
                                    // let _ = self.spawn_msg_listener(main_chara_ctrl, message_tx);
                                    for rec in msg_receivers {
                                        let message_tx = message_tx.clone();
                                        // let _ = self.spawn_msg_listener(rec, message_tx);
                                    }
                                }
                                Err(err) => {
                                    log::error!("===={:#?}", err);
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

        QaulBleService::Started(StartedBleService {
            join_handle: Some(join_handle),
            cmd_handle: cmd_tx,
        })
    }

    async fn on_device_discovered(
        &self,
        device: &Device,
        sender: &mut BleResultSender,
    ) -> Result<Vec<CharacteristicReader>, Box<dyn Error>> {
        let rssi = device.rssi().await?.unwrap_or(999) as i32;
        let mut read_char_uuid_found = false;
        let mut msg_receivers: Vec<CharacteristicReader> = vec![];
        let stringified_addr = utils::mac_to_string(&device.address());
        let uuids = device.uuids().await?.unwrap_or_default();
        log::info!(
            "Discovered device {} with service UUIDs {:?}",
            &stringified_addr,
            &uuids
        );
        // let connection_status = ;
        let mut retries = 2;
        loop {
            match device.connect().await {
                Ok(()) => break,
                Err(err) => {
                    if retries > 0 {
                        println!("    Connect error: {}", &err);
                        retries -= 1;
                    } else {
                        return Err("Connection retries timeout.".into());
                    }
                }
            }
        }
        for service in device.services().await? {
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
                if flags.notify || flags.indicate {
                    msg_receivers.push(char.notify_io().await?);
                    log::info!(
                        "Setting up notification for characteristic {} of device {}",
                        char.uuid().await?,
                        &stringified_addr
                    );
                } else if flags.read && char.uuid().await? == read_char() {
                    utils::add_ignore_device(utils::BleScanDevice {
                        qaul_id: vec![],
                        rssi,
                        mac_address: device.address(),
                        device: device.clone(),
                        last_found_time: 0,
                        name: device.name().await.unwrap().expect("name not set."), // TODO: get device name
                        is_connected: false,
                    });
                    read_char_uuid_found = true;
                    let remote_qaul_id = char.read().await?;
                    self.address_lookup
                        .borrow_mut()
                        .insert(remote_qaul_id.clone(), device.address());

                    sender.send_device_found(remote_qaul_id, rssi)
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
        Ok(msg_receivers)
        // } else {
        //     Err("Device RSSI less than 0".into())
    }

    async fn spawn_msg_listener(
        &self,
        reader: CharacteristicReader,
        message_tx: Sender<BleMainLoopEvent>,
        mac_address: Address,
    ) {
        async_std::task::spawn(async move {
            while let Some(msg) = reader.recv().await.ok() {
                if message_tx.receiver_count() != 0 {
                    break;
                }
                let mut hex_msg = utils::bytes_to_hex(&msg);
                log::debug!("Received message: {:?} ", hex_msg);
                let stringified_addr = utils::mac_to_string(&mac_address);
                // let device = utils::find_device_by_mac(mac_address).unwrap();
                // let mut msg_map = MSG_MAP.lock().unwrap();
                match utils::find_msg_map_by_mac(stringified_addr.clone()) {
                    Some(old_value) => {
                        let mut old_value = old_value.clone();
                        if hex_msg.ends_with("2424")
                            || (old_value.ends_with("24") && hex_msg == "24")
                        {
                            old_value = old_value + &hex_msg;
                            let trim_old_value = &old_value[2..old_value.len() - 2];
                            if !trim_old_value.contains("$$") {
                                hex_msg = trim_old_value.to_string();
                                utils::remove_msg_map_by_mac(stringified_addr);
                                // msg_map.remove(&stringified_addr);
                            }
                        } else {
                            old_value += &hex_msg;
                            utils::add_msg_map(stringified_addr, old_value.clone());
                            // msg_map.insert(stringified_addr.clone(), old_value.clone());
                        }
                    }
                    None => {
                        if hex_msg.starts_with("2424") && hex_msg.ends_with("2424") {
                            // Nothing to do here for now
                        } else if hex_msg.starts_with("2424") {
                            utils::add_msg_map(stringified_addr, hex_msg.clone());
                            // msg_map.insert(stringified_addr.clone(), hex_msg.clone());
                        } else {
                            // Error handling
                        }
                    }
                }
                // let old_value_hex = utils::hex_to_bytes(&hex_msg.clone());
                // let mut msg_data = utils::bytes_to_str(&old_value_hex).unwrap();
                let mut data = if let Some(stripped) = hex_msg.strip_prefix("$$") {
                    stripped.to_string()
                } else {
                    hex_msg.to_string()
                };

                data = if let Some(stripped) = data.strip_suffix("$$") {
                    stripped.to_string()
                } else {
                    data.to_string()
                };
                let msg_object: Message = serde_json::from_str(&data).unwrap();
                let message = msg_object.message.clone().unwrap();
                let _ = message_tx
                    .send(BleMainLoopEvent::MessageReceived((
                        // hex_to_bytes(&message),
                        message,
                        mac_address,
                    )))
                    .await
                    .map_err(|err| log::error!("{:#?}", err));
            }
        });
    }

    async fn send_direct_message(
        &self,
        message_id: String,
        receiver_id: Vec<u8>,
        sender_id: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        println!(
            "=============Sending direct message again to {:?}",
            receiver_id
        );

        let addr_loopup = self.address_lookup.borrow();
        let mac_address = addr_loopup
            .get(&receiver_id)
            .ok_or("Could not find a device address for the given qaul ID!")?;
        let stringified_addr = utils::mac_to_string(mac_address);
        // let device = self.adapter.device(recipient.to_owned())?;

        // let device = utils::find_ignore_device_by_mac(*recipient);
        match utils::find_ignore_device_by_mac(*mac_address) {
            Some(ble_device) => {
                let device = ble_device.device.clone();

                // let mainQueue : HashMap<String, VecDeque<(String, Vec<u8>, Vec<u8>)>> = HashMap::new();
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

                let extracted_queue = hash_map.get(&stringified_addr).clone();
                let mut message_id: String = "".to_string();
                let mut send_queue: VecDeque<String> = VecDeque::new();
                if let Some(queue) = extracted_queue {
                    let mut queue = queue.clone();
                    if !queue.is_empty() {
                        let data = queue.pop_front().unwrap();
                        message_id = data.0;
                        let msg = Message {
                            qaulId: Some(data.1),
                            message: Some(data.2),
                        };
                        let json_str = serde_json::to_string(&msg).unwrap();
                        let bt_array = json_str.as_bytes();
                        let delimiter = vec![0x24, 0x24];
                        let temp = [delimiter.clone(), bt_array.to_vec(), delimiter].concat();
                        let mut final_data = utils::bytes_to_hex(&temp);

                        while final_data.len() > 40 {
                            send_queue.push_back(final_data[..40].to_string());
                            final_data = final_data[40..].to_string();
                        }
                        if !final_data.is_empty() {
                            send_queue.push_back(final_data);
                        }
                    }
                }
                if !device.is_connected().await? {
                    device.connect().await?;
                    log::info!("Connected to device {}", &stringified_addr);
                }
                while send_queue.len() > 0 {
                    let data = send_queue.pop_front().unwrap();
                    let data = utils::hex_to_bytes(&data);
                    for service in device.services().await? {
                        if service.uuid().await? == msg_service_uuid() {
                            for chara in service.characteristics().await? {
                                if chara.uuid().await? == msg_char() {
                                    chara.write(&data).await?;
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
            None => {
                return Err("Device not found".into());
            }
        }
    }
}

impl StartedBleService {
    pub async fn spawn_handles(self) {
        // let handle = async_std::task::spawn(async move {
        match self.join_handle {
            Some(join_handles) => {
                println!("Spawning handles");
                println!("id = {}", join_handles.task().id());
                join_handles.await;
            }
            None => {
                println!("No handle to spawn");
            }
        }
        // });
        // handle.await;
    }

    pub async fn direct_send(
        &mut self,
        direct_send_request: BleDirectSend,
    ) -> Result<(), Box<dyn Error>> {
        self.cmd_handle
            .send(BleMainLoopEvent::SendMessage((
                direct_send_request.message_id,
                direct_send_request.receiver_id,
                direct_send_request.sender_id,
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

        // let ble_service: IdleBleService;
        // match self.join_handle {
        //     Some(join_handles) => {
        //         println!("Spawning handles");
        //         ble_service = join_handles.await;
        //     }
        //     None => {
        //         println!("No handle to spawn");
        //     }
        // }
        sender.send_stop_successful();

        QaulBleService::Idle(self.join_handle.unwrap().await)
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
    qaul_uuids.insert(msg_service_uuid());
    qaul_uuids.insert(read_char());
    qaul_uuids.insert(msg_char());

    bluer::DiscoveryFilter {
        uuids: qaul_uuids,
        ..Default::default()
    }
}

// pub fn disconnect_device(mac_address: Vec<u8>) -> Result<(), Box<dyn Error>> {
//     let addr = Address::from_bytes(mac_address);
//     let session = bluer::Session::new()?;
//     let adapter = session.default_adapter()?;
//     let device = adapter.device(addr)?;
//     device.disconnect()?;
//     Ok(())
// }
