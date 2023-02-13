// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! BLE Connection Module
//!
//! **Connect to other nodes via Bluetooth Low Energy**
//!
//! This mode is currently implemented for the following systems:
//!
//! * Android
//! * iOS

use libp2p::{floodsub::Topic, PeerId};
use prost::Message;
use state::Storage;
use std::{collections::BTreeMap, fmt, sync::RwLock};
use uuid::Uuid;

use crate::connections::ConnectionModule;
use crate::node::Node;
use crate::router::neighbours::Neighbours;
use crate::rpc::{sys::Sys, Rpc};
use crate::services::{feed, messaging};
use crate::utilities::{qaul_id::QaulId, timestamp::Timestamp};

/// Protobuf BLE system communication with BLE module
///
/// Import protobuf message definition generated by
/// the rust module prost-build.
pub mod proto {
    include!("qaul.sys.ble.rs");
}
/// Protobuf BLE network communication
pub mod proto_net {
    include!("qaul.net.ble.rs");
}
/// Protobuf BLE RPC communication
pub mod proto_rpc {
    include!("qaul.rpc.ble.rs");
}

/// Module State
static BLE: Storage<RwLock<Ble>> = Storage::new();
/// List of detected BLE nodes needing ID confirmation
static TO_CONFIRM: Storage<RwLock<BTreeMap<Vec<u8>, ToConfirm>>> = Storage::new();
/// List of discovered and available BLE nodes
///
/// This structure contains a translation table from
/// the BLE ID to the BLE ID
static NODES: Storage<RwLock<BTreeMap<Vec<u8>, BleNode>>> = Storage::new();

/// Detected BLE node, which is not known yet
/// and therefore its ID needs to be confirmed.
#[allow(dead_code)]
pub struct ToConfirm {
    // small id
    small_id: Vec<u8>,
    // detected at
    detected_at: u64,
    // status of it's detection
    status: u8,
}

/// Currently available BLE Nodes
#[derive(Clone)]
pub struct BleNode {
    // node id
    id: Vec<u8>,
    // timestamp of last detection
    #[allow(dead_code)]
    timestamp: u64,
}

/// Ble Module Status
#[allow(dead_code)]
#[derive(Debug)]
pub enum ModuleStatus {
    Uninitalized,
    InfoRequestSent,
    InfoReceived,
    StartRequestSent,
    RightsMissing,
    StartSuccess,
    StopRequestSent,
    Stopped,
}

impl fmt::Display for ModuleStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// BLE - Bluetooth Low Energy - Connection Module
pub struct Ble {
    /// qaul BLE id
    ///
    /// 16 Byte short form of qaul node ID
    ///
    /// The BLE id is a smaller representation of the qaul
    /// node id to exchange it in a legacy BLE message with
    /// max 20 bytes
    pub ble_id: Vec<u8>,
    /// Module status
    pub status: ModuleStatus,
    /// BLE Devices
    pub devices: Vec<proto::BleDeviceInfo>,
}

impl Ble {
    /// initialize the BLE module
    pub fn init() {
        // get small BLE ID
        let ble_id = Node::get_small_id();

        // initialize local state
        {
            // create node states
            TO_CONFIRM.set(RwLock::new(BTreeMap::new()));
            NODES.set(RwLock::new(BTreeMap::new()));

            // set it to state
            let ble = Ble {
                ble_id,
                status: ModuleStatus::Uninitalized,
                devices: Vec::new(),
            };
            BLE.set(RwLock::new(ble));
        }

        //#[cfg(target_os = "android")]
        Self::info_send_request();
    }

    /// set module status
    fn status_set(status: ModuleStatus) {
        // get module state
        let mut ble = BLE.get().write().unwrap();

        // set status
        ble.status = status;
    }

    /// send info request
    fn info_send_request() {
        // create message
        let message = proto::Ble {
            message: Some(proto::ble::Message::InfoRequest(proto::BleInfoRequest {})),
        };

        // encode message
        let mut buf = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send the message
        Sys::send_message(buf);

        // update module status
        Self::status_set(ModuleStatus::InfoRequestSent);
    }

    /// info request received
    fn info_received(message: proto::BleInfoResponse) {
        //ble.devices.extend(message.device);
        if let Some(device) = message.device {
            // log received info
            log::info!("=========================");
            log::info!("BLE info received");
            log::info!("-------------------------");
            log::info!("This Devices ID");
            let node_id = Node::get_id();
            log::info!("- Node ID: {}", node_id.to_base58());
            log::info!("- Small Node ID: {:?}", QaulId::to_small(node_id));
            log::info!("-------------------------");
            log::info!("BLE Supported: {}", device.ble_support);
            log::info!("ID: {}", device.id);
            log::info!("Name: {}", device.name);
            log::info!("Bluetooth Enabled: {}", device.bluetooth_on);
            log::info!("Advertisement Extended: {}", device.adv_extended);
            if device.adv_extended {
                log::info!("    bytes: {}", device.adv_extended_bytes);
            }
            log::info!("2M supported: {}", device.le_2m);
            log::info!("LE coded supported: {}", device.le_coded);
            log::info!("LE audio supported: {}", device.le_audio);
            log::info!(
                "LE periodic advertisement supported: {}",
                device.le_periodic_adv_support
            );
            log::info!(
                "LE multiple advertisement supported: {}",
                device.le_multiple_adv_support
            );
            log::info!(
                "offload filter supported: {}",
                device.offload_filter_support
            );
            log::info!(
                "offload scan batching supported: {}",
                device.offload_scan_batching_support
            );
            log::info!("=========================");

            // save to state
            {
                let mut ble = BLE.get().write().unwrap();
                ble.devices.push(device);
            }

            // start module
            Self::module_start();
        } else {
            log::error!("No Bluetooth device available.");
        }
    }

    /// start module
    pub fn module_start() {
        log::info!("BLE send start request");

        let qaul_id;
        {
            let ble = BLE.get().write().unwrap();
            qaul_id = ble.ble_id.clone();
        }

        // create message
        let start_request = proto::BleStartRequest {
            qaul_id,
            power_setting: proto::BlePowerSetting::LowLatency.into(),
        };
        let message = proto::Ble {
            message: Some(proto::ble::Message::StartRequest(start_request)),
        };

        // encode message
        let mut buf = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send the message
        Sys::send_message(buf);

        // update module status
        Self::status_set(ModuleStatus::StartRequestSent);
    }

    /// check start module result
    fn module_start_result(message: proto::BleStartResult) {
        log::info!("BLE module start result received");
        if message.success {
            log::info!("BLE Module successfully started");
            Self::status_set(ModuleStatus::StartSuccess);
        } else {
            // TODO: manage rights, etc.
            log::warn!("BLE start error: {}", message.error_message);

            match message.error_reason {
                // proto::BleError::UnknownError
                0 => {
                    log::error!("BLE unknown start error");
                }
                // proto::BleError::RightsMissing
                1 => {
                    log::warn!("BLE rights missing");

                    // request rights from GUI
                    // create rights request message
                    log::info!("BLE rights requested");
                    let rights_request = proto_rpc::RightsRequest {};

                    // create BLE RPC message
                    let proto_message = proto_rpc::Ble {
                        message: Some(proto_rpc::ble::Message::RightsRequest(rights_request)),
                    };

                    // encode message
                    let mut buf = Vec::with_capacity(proto_message.encoded_len());
                    proto_message
                        .encode(&mut buf)
                        .expect("Vec<u8> provides capacity as needed");

                    // send message
                    Rpc::send_message(
                        buf,
                        crate::rpc::proto::Modules::Ble.into(),
                        "".to_string(),
                        Vec::new(),
                    );
                }
                // proto::BleError::Timeout
                2 => {
                    log::error!("BLE start timeout error");
                }
                _ => {
                    log::error!("BLE undefined start error");
                }
            }
        }
    }

    /// stop module
    pub fn module_stop() {
        log::info!("BLE send stop request");

        // create stop message
        let message = proto::BleStopRequest {};

        // encode message
        let mut buf = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send the message
        Sys::send_message(buf);

        // update module status
        Self::status_set(ModuleStatus::StopRequestSent);

        // TODO: empty all lists: Neighbours, Nodes, to Confirm
    }

    /// check start module result
    fn module_stop_result(message: proto::BleStopResult) {
        if message.success {
            log::info!("BLE module successfully stopped");
            // update module status
            Self::status_set(ModuleStatus::Stopped);
        } else {
            // TODO: how to handle that?
            log::error!("BLE stop request error: {}", message.error_message);
        }
    }

    /// Add Newly Discovered Node To all Lists
    ///
    /// This function adds a newly discovered and identified
    /// node to all necessary node lists:
    ///
    /// * BLE translation table
    /// * BLE Neighbours list
    fn node_discovered(small_id: Vec<u8>, node_id: Vec<u8>) {
        log::info!("BLE node discovered");

        // create node entry
        let node = BleNode {
            id: node_id,
            timestamp: Timestamp::get_timestamp(),
        };

        // add node to local state
        {
            // get state
            let mut nodes = NODES.get().write().unwrap();

            // add node
            nodes.insert(small_id, node.clone());
        }

        // add it to neighbours table
        match PeerId::from_bytes(&node.id) {
            Ok(node_id) => {
                log::info!("    Node ID: {}", node_id.to_base58());
                Neighbours::update_node(ConnectionModule::Ble, node_id, 50);
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }

    /// add a node to the available nodes list
    fn node_to_confirm(small_id: Vec<u8>) {
        // create node entry
        let confirm = ToConfirm {
            small_id: small_id.clone(),
            detected_at: Timestamp::get_timestamp(),
            status: 0,
        };

        // get state
        let mut nodes = TO_CONFIRM.get().write().unwrap();

        // add node
        nodes.insert(small_id.clone(), confirm);

        // send identification message
        Self::identification_send(small_id, true);
    }

    /// find node ID from small id
    fn find_node(small_id: Vec<u8>) -> Option<Vec<u8>> {
        // get nodes table
        let nodes = NODES.get().write().unwrap();

        // search for small id
        if let Some(node) = nodes.get(&small_id) {
            return Some(node.id.clone());
        }

        None
    }

    /// find node ID from small ID
    fn get_node_id(small_id: Vec<u8>) -> Option<PeerId> {
        if let Some(id) = Self::find_node(small_id) {
            match PeerId::from_bytes(&id) {
                Ok(peer) => return Some(peer),
                Err(e) => log::error!("{}", e),
            }
        }

        None
    }

    /// a new device got discovered via bluetooth
    fn device_discovered(message: proto::BleDeviceDiscovered) {
        log::info!("BLE device discovered: {:?}", message.qaul_id.clone());
        // check if node is known
        if let Some(node) = Neighbours::node_from_small_id(message.qaul_id.clone()) {
            log::info!(
                "BLE discovered Node ID: {}",
                QaulId::bytes_to_log_string(&node.id)
            );
            // add it to translation table
            Self::node_discovered(message.qaul_id, node.id);
        } else {
            log::info!("BLE discovered Node ID unknown");
            // confirm node
            Self::node_to_confirm(message.qaul_id);
        }
    }

    /// a formerly discovered device became unavailable
    fn device_unavailable(message: proto::BleDeviceUnavailable) {
        log::info!(
            "BLE device became unavailable: {:?}",
            message.qaul_id.clone()
        );
        // get state
        let mut nodes = NODES.get().write().unwrap();

        // remove device from list
        if let Some((_, ble_node)) = nodes.remove_entry(&message.qaul_id) {
            // remove it from neighbours list
            match PeerId::from_bytes(&ble_node.id) {
                Ok(node_id) => {
                    Neighbours::delete(ConnectionModule::Ble, node_id);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        } else {
            // remove it from TO_CONFIRM list
            let mut to_confirm = TO_CONFIRM.get().write().unwrap();
            if let None = to_confirm.remove_entry(&message.qaul_id) {
                // remove it from neighbours list
                log::error!("node to remove not found");
            }
        }
    }

    /// Identification Received
    ///
    /// Received identity information from another node
    fn identification_received(small_id: Vec<u8>, identification: proto_net::Identification) {
        log::info!("BLE identification received from {:?}", small_id.clone());

        // add node id
        if let Some(node) = identification.node {
            // remove node from to_confirm
            {
                let mut to_confirm = TO_CONFIRM.get().write().unwrap();
                to_confirm.remove_entry(&small_id);
            }

            // add node to discovered lists
            Self::node_discovered(small_id.clone(), node.id);

            // check if to send a response
            if identification.request {
                Self::identification_send(small_id, false);
            }
        }
    }

    /// Send Identification
    ///
    /// Send identity information to another node
    fn identification_send(receiver_small_id: Vec<u8>, request: bool) {
        log::info!("BLE send identity information");

        // get node ID
        let node_id = Node::get_id();

        // create identification message
        let identification = proto_net::Identification {
            request,
            node: Some(proto_net::NodeIdentification {
                id: node_id.to_bytes(),
            }),
        };

        // create unified message
        let message = proto_net::ble_message::Message::Identification(identification);

        Self::create_send_message(receiver_small_id, message);
    }

    /// send message
    ///
    /// * receiver_id: the small qaul id of the receiving node
    /// * sender_id: the small qaul id of the sending node (this node)
    /// * data: the binary data of the message to send
    pub fn message_send(receiver_id: Vec<u8>, sender_id: Vec<u8>, data: Vec<u8>) {
        log::info!("BLE send message to {:?}", receiver_id.clone());
        // create a random UUID as message id
        let message_id = Uuid::new_v4().as_bytes().to_vec();

        // create direct message
        let direct_message = proto::BleDirectSend {
            message_id,
            receiver_id,
            sender_id,
            data,
        };

        // create message
        let message = proto::Ble {
            message: Some(proto::ble::Message::DirectSend(direct_message)),
        };

        // encode message
        let mut buf = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send the message
        Sys::send_message(buf);
    }

    /// result of message sending
    fn message_send_result(result: proto::BleDirectSendResult) {
        if result.success {
            log::info!("BLE message successfully sent")
        } else {
            log::error!("error sending BLE message");
        }
    }

    /// send routing info message
    pub fn send_routing_info(node_id: PeerId, data: Vec<u8>) {
        log::info!("BLE send routing information");
        let message = proto_net::ble_message::Message::Info(data);

        Self::create_send_message(QaulId::to_small(node_id), message);
    }

    /// send messaging message
    pub fn send_messaging_message(node_id: PeerId, data: Vec<u8>) {
        log::info!("BLE send messaging message to {}", node_id.to_base58());

        let message = proto_net::ble_message::Message::Messaging(data);

        Self::create_send_message(QaulId::to_small(node_id), message);
    }

    /// send feed message
    pub fn send_feed_message(_topic: Topic, data: Vec<u8>) {
        log::info!("BLE send public message");

        // find all nodes, that are only connected through BLE
        let nodes = Neighbours::get_ble_only_nodes();

        // create BLE message
        let message = proto_net::ble_message::Message::Feed(data);

        // send it nodes
        for node_id in nodes {
            Self::create_send_message(QaulId::to_small(node_id), message.clone());
        }
    }

    /// create the message
    fn create_send_message(receiver_small_id: Vec<u8>, message: proto_net::ble_message::Message) {
        // get small qaul id of this node
        let sender_id = Node::get_small_id();

        // create message
        let proto_message = proto_net::BleMessage {
            message: Some(message),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send the message
        Self::message_send(receiver_small_id, sender_id, buf);
    }

    /// BLE message received
    fn message_received(message: proto::BleDirectReceived) {
        log::info!("BLE message received");

        // get node ID of sender
        let node_id: PeerId;
        if let Some(id) = Self::get_node_id(message.from.clone()) {
            node_id = id;
        } else {
            // TODO: find a better solution in the future
            //
            // if we don't know the ID of the peer yet,
            // put in our peer ID
            node_id = Node::get_id();
        }

        // decode and distribute messages
        match proto_net::BleMessage::decode(&message.data[..]) {
            Ok(ble_message) => match ble_message.message {
                Some(proto_net::ble_message::Message::Info(data)) => {
                    log::info!("BLE routing info received");
                    let received = qaul_info::QaulInfoReceived {
                        received_from: node_id,
                        data,
                    };
                    crate::router::info::RouterInfo::received(received);
                }
                Some(proto_net::ble_message::Message::Feed(data)) => {
                    log::info!("BLE public message received");
                    match feed::proto_net::FeedContainer::decode(&data[..]) {
                        Ok(feed_container) => {
                            feed::Feed::received(ConnectionModule::Ble, node_id, feed_container);
                        }
                        Err(e) => {
                            log::error!("BleMessage feed decoding error: {}", e);
                        }
                    }
                }
                Some(proto_net::ble_message::Message::Messaging(data)) => {
                    log::info!("BLE messaging message received");
                    let received = qaul_messaging::QaulMessagingReceived {
                        received_from: node_id,
                        data,
                    };
                    messaging::Messaging::received(received);
                }
                Some(proto_net::ble_message::Message::Identification(identification)) => {
                    log::info!("BLE identification received");
                    Self::identification_received(message.from, identification);
                }
                _ => {
                    log::error!("unprocessable BleMessage");
                }
            },
            Err(e) => {
                log::error!("Protobuf BleMessage decoding error: {}", e);
            }
        }
    }

    /// receive sys messages from BLE module
    pub fn sys_received(data: Vec<u8>) {
        match proto::Ble::decode(&data[..]) {
            Ok(ble) => match ble.message {
                Some(proto::ble::Message::InfoResponse(info_response)) => {
                    Self::info_received(info_response);
                }
                Some(proto::ble::Message::StartResult(start_result)) => {
                    Self::module_start_result(start_result);
                }
                Some(proto::ble::Message::StopResult(stop_result)) => {
                    Self::module_stop_result(stop_result);
                }
                Some(proto::ble::Message::DeviceDiscovered(device)) => {
                    Self::device_discovered(device);
                }
                Some(proto::ble::Message::DeviceUnavailable(device)) => {
                    Self::device_unavailable(device);
                }
                Some(proto::ble::Message::DirectReceived(direct_received)) => {
                    Self::message_received(direct_received);
                }
                Some(proto::ble::Message::DirectSendResult(direct_send_result)) => {
                    Self::message_send_result(direct_send_result);
                }
                _ => {
                    log::error!("Unprocessable BLE Sys message received");
                }
            },
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    /// Process incoming RPC request messages for BLE module
    pub fn rpc(data: Vec<u8>) {
        log::trace!("BLE rpc message received");

        match proto_rpc::Ble::decode(&data[..]) {
            Ok(ble) => {
                match ble.message {
                    Some(proto_rpc::ble::Message::InfoRequest(_)) => {
                        // get module state
                        let ble = BLE.get().read().unwrap();

                        // create binary device info message
                        let mut device_info: Vec<u8> = Vec::new();
                        if let Some(device) = ble.devices.first() {
                            device_info = Vec::with_capacity(device.encoded_len());
                            device
                                .encode(&mut device_info)
                                .expect("Vec<u8> provides capacity as needed");
                        }

                        // create discovered response message
                        let info = proto_rpc::InfoResponse {
                            small_id: ble.ble_id.clone(),
                            status: ble.status.to_string(),
                            device_info,
                        };

                        // create BLE RPC message
                        let proto_message = proto_rpc::Ble {
                            message: Some(proto_rpc::ble::Message::InfoResponse(info)),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            buf,
                            crate::rpc::proto::Modules::Ble.into(),
                            "".to_string(),
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::ble::Message::StartRequest(_)) => {
                        // start BLE module
                        Self::module_start();
                    }
                    Some(proto_rpc::ble::Message::StopRequest(_)) => {
                        // stop BLE module
                        Self::module_stop();
                    }
                    Some(proto_rpc::ble::Message::DiscoveredRequest(_)) => {
                        // get nodes state
                        let nodes = NODES.get().read().unwrap();
                        // get to confirm state
                        let to_confirm = TO_CONFIRM.get().read().unwrap();

                        // create discovered response message
                        let discovered = proto_rpc::DiscoveredResponse {
                            nodes_count: nodes.len() as u32,
                            to_confirm_count: to_confirm.len() as u32,
                        };

                        // create BLE RPC message
                        let proto_message = proto_rpc::Ble {
                            message: Some(proto_rpc::ble::Message::DiscoveredResponse(discovered)),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            buf,
                            crate::rpc::proto::Modules::Ble.into(),
                            "".to_string(),
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::ble::Message::RightsResult(rights_result)) => {
                        if rights_result.rights_granted {
                            log::info!("BLE rights granted");
                            Self::module_start();
                        } else {
                            log::error!("BLE rights not granted");
                        }
                    }
                    _ => {
                        log::error!("BLE rpc message undefined");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
