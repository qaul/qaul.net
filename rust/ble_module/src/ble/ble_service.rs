use super::super::BleRpc;
use super::utils::find_device_by_mac;
use crate::ble::ble_uuids::{main_service_uuid, msg_char, msg_service_uuid, read_char};
// use crate::rpc::SysRpcReceiver;
use crate::{
    ble::utils,
    rpc::{process_received_message, proto_sys::ble::Message::*, proto_sys::*, utils::*},
};
// use async_std::task;
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
// use serde_json::{self, de};
// use std::cmp::max_by_key;
// use std::string;
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
    // Stop,
    // MessageReceived((String, Address)),
    // MainCharEvent(CharacteristicControlEvent),
    // MsgCharEvent(CharacteristicControlEvent),
    DeviceDiscovered(Device),
    // SendMessage((Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)),
    // RpcEvent(Option<Ble>),
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

    ///Starts BLE advertisement, scan, and listen.
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
                // msg_service_uuid(),
                // msg_char(),
                // read_char(),
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
        let (cmd_tx, cmd_rx) = async_std::channel::bounded::<BleMainLoopEvent>(8);
        let (adp_send, adp_recv) = async_std::channel::unbounded::<Adapter>();
        match adp_send.try_send(self.adapter.clone()) {
            Ok(_) => {
                log::debug!("Sent adapter to channel");
            }
            Err(err) => {
                log::error!("{:#?}", err);
            }
        }               

        // ==================================================================================
        // ------------------------- SET UP APPLICATION -------------------------------------
        // ==================================================================================

        let (_, main_service_handle) = service_control();
        let (_, main_chara_handle) = characteristic_control();
        // let (_, msg_service_handle) = service_control();
        let (msg_chara_ctrl, msg_chara_handle) = characteristic_control();
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
                                                    device: device,
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
                                        // self.on_device_discovered( &device, &mut internal_sender);
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

        // let msg_service = Service {
        //     uuid: msg_service_uuid(),
        //     primary: true,
        //     characteristics: vec![Characteristic {
        //         uuid: msg_char(),
        //         write: Some(CharacteristicWrite {
        //             write: true,
        //             write_without_response: true,
        //             method: CharacteristicWriteMethod::Io,
        //             ..Default::default()
        //         }),
        //         control_handle: msg_chara_handle,
        //         ..Default::default()
        //     }],
        //     control_handle: msg_service_handle,
        //     ..Default::default()
        // };

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

         // let cmd_sender = cmd_tx.clone();
        let join_handle = async_std::task::Builder::new()
            .name("main-ble-loop".into())
            .local(async move {
                log::info!("Starting BLE main loop...");

                let adapter: Adapter;
                match adp_recv.recv().await {
                    Ok(adp) => {
                        adapter = adp;
                    }
                    Err(_) => {
                        adapter = self.adapter.clone();
                    }
                };

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

                        // match utils::find_device_by_mac(addr) {
                        //     Some(_) => {
                        //         log::warn!("Device mac_addres {:?}", addr);
                        //         utils::update_last_found(addr);
                        //         // None
                        //         let device = self.adapter.device(addr)?;
                        //         Some(BleMainLoopEvent::DeviceDiscovered(device))
                        //     }
                        //     None => {
                        //         log::warn!("Device mac_addres {:?}", addr);
                        //         if self.device_block_list.contains(&addr) {
                        //             return None;
                        //         }
                        //         match adapter.device(addr) {
                        //             Ok(device) => {
                        //                 Some(BleMainLoopEvent::DeviceDiscovered(device))
                        //             }
                        //             Err(_) => None,
                        //         }
                        //     }
                        // },
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


                

                // ==================================================================================
                // --------------------------------- MAIN BLE LOOP ----------------------------------
                // ==================================================================================
                // let (ble_msg_sender, ble_msg_reciever) = async_std::channel::unbounded::<(Address, Vec<u8>)>();
                // let (message_tx, message_rx) = async_std::channel::bounded::<BleMainLoopEvent>(32);
                // let main_evt_stream = main_chara_ctrl.map(BleMainLoopEvent::MainCharEvent);
                // let msg_evt_stream = msg_chara_ctrl.map(BleMainLoopEvent::MsgCharEvent);
                let rpc_reciever_stream = rpc_receiver.receiver.map(BleMainLoopEvent::RpcEvent);
                // .map(BleMainLoopEvent::RpcEvent);

                let mut merged_ble_streams = (
                    cmd_rx,
                    // main_evt_stream,
                    // msg_evt_stream,
                    device_stream,
                    // message_rx,
                    rpc_reciever_stream,
                )
                    .merge();


                let _ = self
                    .spawn_msg_listener(internal_sender.clone(), msg_chara_ctrl)
                    .await; 
                // utils::out_of_range_checker(adapter.clone(), internal_sender.clone());

                'outer: loop {
                    log::warn!("Waiting for event... ");
                    match merged_ble_streams.next().await {
                        Some(evt) => {
                            log::info!("Received event:");
                            match evt {
                                // BleMainLoopEvent::Stop => {
                                //     log::info!(
                                //     "Received stop signal, stopping advertising, scanning, and listening."
                                // );
                                //     break;      
                                // }
                                // BleMainLoopEvent::SendMessage((
                                //     message_id,
                                //     mut receiver_id,
                                //     sender_id,
                                //     data,
                                // // )) => match utils::bytes_to_str(&message_id) {
                                // )) => {
                                    // match std::str::from_utf8(&message_id) {
                                    // Ok(msg_id) => {



                                        // let msg_id = String::from_utf8_lossy(&message_id);
                                        // log::info!("Sending message with ID: {:?}", &msg_id);
                                        // match self
                                        //     .send_direct_message(
                                        //         msg_id.to_string(),
                                        //         &mut receiver_id,
                                        //         sender_id,
                                        //         data,
                                        //     )
                                        //     .await
                                        // {
                                        //     Ok(_) => {
                                        //         internal_sender.send_direct_send_success(receiver_id);
                                        //     }
                                        //     Err(err) => {
                                        //         internal_sender.send_direct_send_error(receiver_id, err.to_string());
                                        //         log::error!("Error sending direct BLE message: {:#?}", err)
                                        //     }
                                        // }


                                    // }
                                    // Err(err) => {
                                    //     log::error!("Message Id {:#?}", message_id);
                                    //     log::error!("Error converting message ID to string: {:#?}", err);
                                    // }
                                // }
                                // },
                                // BleMainLoopEvent::MessageReceived(_e) => {
                                    // let json_message = String::from_utf8_lossy(&e.0.as_bytes());
                                    // log::error!("Received messages: {:?} ", json_message);
                                    // let msg_object: Message = serde_json::from_str(&json_message).unwrap();
                                    // let message = msg_object.message.unwrap();
                                    // log::info!(
                                    //     "Received {} bytes of data from {}",
                                    //     message.len(),
                                    //     utils::mac_to_string(&e.1)
                                    // );
                                    // internal_sender.send_direct_received(e.1.0.to_vec(), message)
                                // }
                                // BleMainLoopEvent::MainCharEvent(_e) => {
                                    // TODO: should main character events be sent to the UI?
                                // }
                                // BleMainLoopEvent::MsgCharEvent(_e) => {
                                    // CharacteristicControlEvent::Write(write) => {
                                    //     log::info!("Accepting write event with MTU {} from {}", write.mtu(), write.device_address());
                                    //     let mac_address = write.device_address();
                                    //     let mut read_buf = vec![0; 20];
                                    //     if let Ok(mut reader) = write.accept() {   
                                    //         let k = reader.read(&mut read_buf).await;
                                    //         match k {
                                    //             Ok(0) => {
                                    //                 log::info!("Write stream from device {:?} has ended", &mac_address);
                                    //                 continue;
                                    //             }
                                    //             Ok(n) => {
                                    //                 log::error!("req {:?}  = {:?}", &read_buf, &n);                              
                                    //                 // let message_tx = message_tx.clone();
                                    //                 let _ = ble_msg_sender.send((mac_address, read_buf)).await; 
                                    //             },
                                    //             Err(err) => {
                                    //                 log::error!("Write stream error: {}", &err);
                                    //             }
                                    //         }     
                                    //         // let _ = self
                                    //         //     .spawn_msg_listener(Some(reader), message_tx, mac_address)
                                    //         //     .await;
                                    //     } else {
                                    //         log::error!("Error accepting write request");
                                    //     }
                                    // }
                                    // CharacteristicControlEvent::Notify(_) => (),
                                // },
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
                                        log::info!("Received rpc event: ");
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
                                                // QaulBleService::Started(ref mut svc) => {
                                                    log::info!("Received Direct Send Request: ");
                                                    // let receiver_id = req.receiver_id.clone();
                                                    // match svc.direct_send(req).await {
                                                    //     Ok(_) => local_sender_handle.send_direct_send_success(receiver_id),
                                                    //     Err(err) => local_sender_handle
                                                    //         .send_direct_send_error(receiver_i   d, err.to_string()),
                                                    // }

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
                                                    // match cmd_sender
                                                    // .send(BleMainLoopEvent::SendMessage((
                                                    //     req.message_id,
                                                    //     req.receiver_id.clone(),
                                                    //     req.sender_id,
                                                    //     req.data,
                                                    // )))
                                                    // .await {
                                                    //     Ok(()) => {
                                                    //         log::info!("Message sent to main loop");
                                                    //     }
                                                    //     Err(err) => {
                                                    //         log::error!("Failed to send message to main loop: {:#?}", &err);
                                                    //         internal_sender.send_direct_send_error(req.receiver_id, err.to_string());
                                                    //     }
                                                    // }
                                                // }
                                                // QaulBleService::Idle(_) => {
                                                //     log::info!("Received Direct Send Request, but bluetooth service is not running!");
                                                //     local_sender_handle.send_result_not_running()
                                                // }
                                            },
                                            InfoRequest(_) => {
                                                let mut sender_handle_clone = internal_sender.clone();
                                                // spawn(async move {
                                                    match get_device_info().await {
                                                        Ok(info) => {
                                                            sender_handle_clone.send_ble_sys_msg(InfoResponse(info))
                                                        }
                                                        Err(err) => {
                                                            log::error!("Error getting device info: {:#?}", &err)
                                                        }
                                                    }
                                                // });
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

        log::info!("BLE main loop start completed successfully!");
        QaulBleService::Started(StartedBleService {
            join_handle: Some(join_handle),
            cmd_handle: cmd_tx,
        })
    }

    /// Handles device discovery and validates the qaul uuids.
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
                    "Characteristic flags for device {} are: {:?} and char {}",
                    &stringified_addr,
                    &flags.read,
                    &remote_char_uuid,
                );
                if flags.notify || flags.indicate {
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
            log::info!("Spawned message listener for device.");             
            loop {
                if let Ok((mac_address, buffer)) = ble_msg_reciever.recv().await{
                    if buffer.len() == 0 {
                        log::info!("Write stream from device {:?} has ended", &mac_address);
                        continue;
                    }
                        let mut hex_msg = utils::bytes_to_hex(&buffer);
                        if hex_msg.contains("24") {
                            // Remove trailing zeros
                            let trimmed = hex_msg.trim_end_matches('0');
                            hex_msg = trimmed.to_string();
                        }
                        log::info!("Received message: {:?} from {:?}", &hex_msg, &mac_address);
                        let stringified_addr = utils::mac_to_string(&mac_address);

                        match utils::find_msg_map_by_mac(stringified_addr.clone()) {
                            Some(mut old_value) => {
                                if hex_msg.ends_with("2424")
                                    || (old_value.ends_with("24") && hex_msg == "24")
                                {
                                    old_value = old_value + &hex_msg;
                                    let trim_old_value = &old_value[4..old_value.len() - 4];
                                        log::error!("Received message: {:?} ", trim_old_value);
                                    if !trim_old_value.contains("2424") {
                                        hex_msg = trim_old_value.to_string();

                                        utils::message_received((hex_msg, mac_address), internal_sender.clone());
                                        utils::remove_msg_map_by_mac(stringified_addr);
                                    }
                                } else {
                                    old_value += &hex_msg;
                                    utils::add_msg_map(stringified_addr, old_value);
                                }
                            }
                            None => {
                                if hex_msg.starts_with("2424") && hex_msg.ends_with("2424") {
                                    let trim_hex_msg = &hex_msg[4..&hex_msg.len() - 4];
                                    if !trim_hex_msg.contains("2424") {
                                        hex_msg = trim_hex_msg.to_string();

                                        // log::error!("Received message: {:?} ", hex_msg);
                                        utils::message_received((hex_msg, mac_address), internal_sender.clone());
                                        // let _ = message_tx
                                        //     .send(BleMainLoopEvent::MessageReceived((hex_msg, mac_address)))
                                        //     .await
                                        //     .map_err(|err| log::error!("{:#?}", err));
                                    }
                                } else if hex_msg.starts_with("2424") {
                                    utils::add_msg_map(stringified_addr, hex_msg.clone());
                                } else {
                                    // Error handling
                                }
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
                    // log::info!("Accepting write event with MTU {} from {}", write.mtu(), write.device_address());
                    let mac_address = write.device_address();
                    match find_device_by_mac(mac_address) {
                        Some(_) => {
                            // utils::update_last_found(mac_address);
                        }
                        None => {
                            device_known = false;
                            log::warn!("Device not found in known devices");
                            // match self.adapter.device(mac_address) {
                            //     Ok(device) => {
                                    // self.
                                    // self.on_device_discovered(&device, &mut internal_sender.clone()).await.unwrap();
                                // },
                                // Err(_) => {
                                //     // log::error!("Error:");
                                // }
                            // };
                            
                        }
                    }
                    let mut read_buf = vec![0; 20               ];
                    if let Ok(mut reader) = write.accept() {   
                        let k = reader.read(&mut read_buf).await;
                        match k {
                            Ok(0) => {
                                log::debug!("Write stream from device {:?} has ended", &mac_address);
                                continue;
                            }
                            Ok(_) => {
                                if device_known { 
                                    let _ = ble_msg_sender.send((mac_address, read_buf)).await; 
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

        // match utils::find_ignore_device_by_mac(mac_address.clone()) {
        //     Some(_) => {
                // let device = ble_device.device.clone();
                let device =  match self.adapter.device(*mac_address) {
                    Ok(device) => device,
                    Err(err) => {
                        log::error!("Error:: {:#?}", err);
                        return Err("Error getting device".into());
                    }
                };
                // if(ble_device.device  device) {
                //     log::info!("Device found in ignore list");
                // }
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
                log::info!("Message added to queue -- > {:?}", &data);
                let extracted_queue = hash_map.get(&stringified_addr).clone();
                let mut message_id: String = "".to_string();
                let mut send_queue: VecDeque<String> = VecDeque::new();
                if let Some(queue) = extracted_queue {
                    let mut queue = queue.clone();
                    if !queue.is_empty() {
                        // log::error!("Here am I");
                        let data = queue.pop_front().unwrap();
                        message_id = data.0;
                        let i8_qaul_id: Vec<i8> = data.1.into_iter().map(|x| x as i8).collect();
                        let i8_message: Vec<i8> = data.2.into_iter().map(|x| x as i8).collect();
                        let msg = utils::Message {
                            qaul_id: Some(i8_qaul_id),
                            message: Some(i8_message),
                        };
                        let json_str = serde_json::to_string(&msg).unwrap();
                        // log::error!("json_str = {:?}", json_str);
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
                        // log::error!("queue length {}", send_queue.len());
                    }
                }
                log::error!("{:}", device.is_connected().await?);
                if !device.is_connected().await? {
                    // let mut retries = 1;
                    // loop {
                        match device.connect().await {
                            Ok(()) => {
                                log::error!("Device connected");
                                // break;
                            }
                            Err(err) => {
                                // if retries > 0 {
                                    log::error!("    Connect error: {}", &err);
                                    return Err("Connection error".into());
                                //     retries -= 1;
                                // } else {
                                //     self.adapter.remove_device(device.address()).await?;
                                    // return Err("Connection retries timeout.".into());
                                // }
                            }
                        // }
                    }
                }
                log::info!("Connected to device {}", &stringified_addr);
                let mut read_char_found = false;
                for service in device.services().await? {
                    if read_char_found {
                        break;
                    }
                    log::info!("Service UUID: {:?}", service.uuid().await?);
                    if service.uuid().await? == main_service_uuid() {
                        for chara in service.characteristics().await? {
                            log::info!("chara = {}", chara.uuid().await?);
                            if chara.uuid().await? == msg_char() {
                                utils::update_last_found(*mac_address);
                                // chara.write(&data).await?;
                                read_char_found = true;
                                log::error!("queue length =  {}", send_queue.len());
                                // let write_io = chara.write_io().await?;
                                while send_queue.len() > 0 {
                                    // let data = send_queue.pop_front().unwrap();
                                    let data: String;
                                    match send_queue.pop_front() {
                                        Some(queue_top) => data = queue_top,
                                        None => {
                                            log::error!("No data found in queue");
                                            break;
                                        }
                                    }
                                    log::info!(
                                        "Sending data to device {} : {:?}",
                                        &stringified_addr,
                                        &data
                                    );
                                    let data = utils::hex_to_bytes(&data);
                                    match chara.write(&data).await {
                                        Ok(()) => {
                                            log::info!(
                                                "    Data1 sent to device {}",
                                                &stringified_addr
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "    Error1 sending data to device {}: {:#?}",
                                                &stringified_addr,
                                                &err
                                            );
                                        }
                                    };
                                    // async_std::task::sleep(std::time::Duration::from_secs(2)).await;

                                    // match write_io.send(&data).await {
                                    //     Ok(()) => {
                                    //         log::info!(
                                    //             "    Data sent to device {}",
                                    //             &stringified_addr
                                    //         );
                                    //     }
                                    //     Err(err) => {
                                    //         log::error!(
                                    //             "    Error sending data to device {}: {:#?}",
                                    //             &stringified_addr,
                                    //             &err
                                    //         );
                                    //     }
                                    // };
                                    // log::info!("    {written} bytes written");
                                }
                            }
                        }
                    }
                }

                log::info!("Message sent to device {}", &stringified_addr);
                // device.disconnect().await?;
                if message_id != "" {
                    Ok(message_id)
                } else {
                    Err("Message ID not found".into())
                }
            // }
            // None => {
            //     return Err("Device not found".into());
            // }
        // }
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
    // qaul_uuids.insert(read_char());
    // qaul_uuids.insert(msg_char());

    bluer::DiscoveryFilter {
        uuids: qaul_uuids,
        transport: bluer::DiscoveryTransport::Le,
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
