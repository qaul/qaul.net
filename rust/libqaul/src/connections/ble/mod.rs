// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
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
use std::{collections::BTreeMap, fmt, sync::RwLock, sync::Mutex};
use uuid::Uuid;

use crate::connections::ConnectionModule;
use crate::node::Node;
use crate::router::neighbours::Neighbours;
use crate::rpc::{sys::Sys, Rpc};
use crate::services::{feed, messaging};
use crate::utilities::{qaul_id::QaulId, timestamp::Timestamp};

#[cfg(feature = "ble-encryption")]
pub mod crypto;
#[cfg(feature = "ble-encryption")]
use crypto::BleCrypto;

/// Protobuf BLE network communication
pub use qaul_proto::qaul_net_ble as proto_net;
/// Protobuf BLE RPC communication
pub use qaul_proto::qaul_rpc_ble as proto_rpc;
/// Protobuf BLE system communication with BLE module
pub use qaul_proto::qaul_sys_ble as proto;

/// Instance-based BLE module state.
/// Replaces the global BLE, TO_CONFIRM, and NODES statics for multi-instance use.
pub struct BleModuleState {
    /// BLE module inner state.
    pub inner: RwLock<Ble>,
    /// Detected BLE nodes pending ID confirmation.
    pub to_confirm: RwLock<BTreeMap<Vec<u8>, ToConfirm>>,
    /// Discovered and available BLE nodes.
    pub nodes: RwLock<BTreeMap<Vec<u8>, BleNode>>,
    /// Sender for libqaul → ble_module direction (tokio mpsc).
    /// Set during BLE init on Linux; None on other platforms.
    pub ble_sender: Mutex<Option<tokio::sync::mpsc::Sender<Vec<u8>>>>,
    /// BLE crypto module state (transport encryption sessions).
    #[cfg(feature = "ble-encryption")]
    pub crypto: RwLock<crypto::BleCryptoModule>,
}

impl BleModuleState {
    /// Create a new empty BleModuleState.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Ble {
                ble_id: Vec::new(),
                status: ModuleStatus::Uninitalized,
                devices: Vec::new(),
            }),
            to_confirm: RwLock::new(BTreeMap::new()),
            nodes: RwLock::new(BTreeMap::new()),
            ble_sender: Mutex::new(None),
            #[cfg(feature = "ble-encryption")]
            crypto: RwLock::new(crypto::BleCryptoModule::new()),
        }
    }
}

/// Detected BLE node, which is not known yet
/// and therefore its ID needs to be confirmed.
#[allow(dead_code)]
pub struct ToConfirm {
    // small id
    q8id: Vec<u8>,
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
    /// 8 Bytes short form of qaul node ID
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
    pub async fn init(state: &crate::QaulState) {
        // get q8id
        let ble_id = Node::get_q8id(state);
        #[cfg(all(target_os = "linux", feature = "ble"))]
        {
            // Create the channel from libqaul → ble_module.
            // Store the sender in BleModuleState; pass the receiver to ble_module.
            let (ble_tx, ble_rx) = tokio::sync::mpsc::channel(32);
            *state.connections.ble.ble_sender.lock().unwrap() = Some(ble_tx);
            let rpc_receiver = ble_module::rpc::BleRpc { receiver: ble_rx };

            // Capture the SYS channel sender for the 'static tokio::spawn closure.
            let sys_sender = state.sys.extern_send.clone();
            tokio::spawn(async move {
                while !ble_module::is_ble_enabled().await {
                    log::error!("BLE not enabled, Please power on bluetooth on your device");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
                let sender = sys_sender;
                ble_module::init(
                    Box::new(move |sys_msg| {
                        if let Err(err) = sender.send(sys_msg) {
                            log::error!("{:?}", err);
                        }
                    }),
                    rpc_receiver,
                );
            });
        }

        // initialize local state via QaulState
        {
            let ble_state = &state.connections.ble;
            // Set BLE ID in the inner state
            let mut ble = ble_state.inner.write().unwrap();
            ble.ble_id = ble_id;
            ble.status = ModuleStatus::Uninitalized;
            ble.devices = Vec::new();
        }

        #[cfg(not(feature = "ble-encryption"))]
        log::info!("BLE transport encryption disabled (feature `ble-encryption`)");

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        //#[cfg(target_os = "android")]
        Self::info_send_request(state);
    }

    /// set module status
    fn status_set(state: &crate::QaulState, status: ModuleStatus) {
        // get module state
        let mut ble = state.connections.ble.inner.write().unwrap();

        // set status
        ble.status = status;
    }

    /// send info request
    fn info_send_request(state: &crate::QaulState) {
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
        Sys::send_message(state, buf);

        // update module status
        Self::status_set(state, ModuleStatus::InfoRequestSent);
    }

    /// info request received
    fn info_received(state: &crate::QaulState, message: proto::BleInfoResponse) {
        //ble.devices.extend(message.device);
        if let Some(device) = message.device {
            // log received info
            log::info!("=========================");
            log::info!("BLE info received");
            log::info!("-------------------------");
            log::info!("This Devices ID");
            let node_id = Node::get_id(state);
            log::info!("- Node ID: {}", node_id.to_base58());
            log::info!("- Small Node ID: {:?}", QaulId::to_q8id(node_id));
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
                let mut ble = state.connections.ble.inner.write().unwrap();
                ble.devices.push(device);
            }

            // start module
            Self::module_start(state);
        } else {
            log::error!("No Bluetooth device available.");
        }
    }

    /// start module
    pub fn module_start(state: &crate::QaulState) {
        log::info!("BLE send start request");

        let qaul_id;
        {
            let ble = state.connections.ble.inner.read().unwrap();
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
        Sys::send_message(state, buf);

        // update module status
        Self::status_set(state, ModuleStatus::StartRequestSent);
    }

    /// check start module result
    fn module_start_result(state: &crate::QaulState, message: proto::BleStartResult) {
        log::info!("BLE module start result received");
        if message.success {
            log::info!("BLE Module successfully started");
            Self::status_set(state, ModuleStatus::StartSuccess);
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
                        state,
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
    pub fn module_stop(state: &crate::QaulState) {
        log::info!("BLE send stop request");

        // create stop message
        let message = proto::BleStopRequest {};

        // encode message
        let mut buf = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send the message
        Sys::send_message(state, buf);

        // update module status
        Self::status_set(state, ModuleStatus::StopRequestSent);

        // TODO: empty all lists: Neighbours, Nodes, to Confirm
    }

    /// check start module result
    fn module_stop_result(state: &crate::QaulState, message: proto::BleStopResult) {
        if message.success {
            log::info!("BLE module successfully stopped");
            // update module status
            Self::status_set(state, ModuleStatus::Stopped);
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
    fn node_discovered(state: &crate::QaulState, q8id: Vec<u8>, node_id: Vec<u8>) {
        log::info!("BLE node discovered");

        // create node entry
        let node = BleNode {
            id: node_id,
            timestamp: Timestamp::get_timestamp(),
        };

        // add node to local state
        {
            // get state
            let mut nodes = state.connections.ble.nodes.write().unwrap();

            // add node
            nodes.insert(q8id, node.clone());
        }

        // add it to neighbours table
        match PeerId::from_bytes(&node.id) {
            Ok(node_id) => {
                log::info!("    Node ID: {}", node_id.to_base58());
                let rs = state.get_router();
                Neighbours::update_node(&rs, ConnectionModule::Ble, node_id, 50);
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }

    /// add a node to the available nodes list
    fn node_to_confirm(state: &crate::QaulState, q8id: Vec<u8>) {
        // check if node confirmation request has already been sent
        if let true = Self::node_confirmation_in_progress(state, &q8id) {
            log::info!("node id confirmation in progress");
            return;
        }

        // create node entry
        let confirm = ToConfirm {
            q8id: q8id.clone(),
            detected_at: Timestamp::get_timestamp(),
            status: 0,
        };

        // get state
        let mut nodes = state.connections.ble.to_confirm.write().unwrap();

        // add node
        nodes.insert(q8id.clone(), confirm);

        // send identification message
        Self::identification_send(state, q8id, true);
    }

    /// Check if node is already scheduled for confirmation
    fn node_confirmation_in_progress(state: &crate::QaulState, q8id: &Vec<u8>) -> bool {
        // get state
        let nodes = state.connections.ble.to_confirm.read().unwrap();

        // search node
        if let Some(to_confirm) = nodes.get(q8id) {
            if to_confirm.detected_at > Timestamp::get_timestamp() - (30 * 1000) {
                return true;
            }
        }

        false
    }

    /// a new device got discovered via bluetooth
    fn device_discovered(state: &crate::QaulState, message: proto::BleDeviceDiscovered) {
        log::info!("BLE device discovered: {:x?}", message.qaul_id.clone());
        // check if node is known
        let rs = state.get_router();
        if let Some(node) = Neighbours::node_from_q8id(&rs, message.qaul_id.clone()) {
            log::info!(
                "BLE discovered Node ID: {}",
                QaulId::bytes_to_log_string(&node.id)
            );
            // add it to translation table
            Self::node_discovered(state, message.qaul_id, node.id);
        } else {
            log::info!("BLE discovered Node ID unknown");
            // confirm node
            Self::node_to_confirm(state, message.qaul_id);
        }
    }

    /// a formerly discovered device became unavailable
    fn device_unavailable(state: &crate::QaulState, message: proto::BleDeviceUnavailable) {
        log::info!(
            "BLE device became unavailable: {:?}",
            message.qaul_id.clone()
        );

        // Clean up crypto session
        #[cfg(feature = "ble-encryption")]
        BleCrypto::on_node_unavailable(state, &message.qaul_id);

        // get state
        let mut nodes = state.connections.ble.nodes.write().unwrap();

        // remove device from list
        if let Some((_, ble_node)) = nodes.remove_entry(&message.qaul_id) {
            // remove it from neighbours list
            match PeerId::from_bytes(&ble_node.id) {
                Ok(node_id) => {
                    let rs = state.get_router();
                    rs.neighbours.delete(ConnectionModule::Ble, node_id);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        } else {
            // remove it from TO_CONFIRM list
            let mut to_confirm = state.connections.ble.to_confirm.write().unwrap();
            if let None = to_confirm.remove_entry(&message.qaul_id) {
                // remove it from neighbours list
                log::error!("node to remove not found");
            }
        }
    }

    /// Identification Received
    ///
    /// Received identity information from another node
    fn identification_received(state: &crate::QaulState, q8id: Vec<u8>, identification: proto_net::Identification) {
        log::info!("BLE identification received from {:?}", q8id.clone());

        // add node id
        if let Some(node) = identification.node {
            #[cfg(feature = "ble-encryption")]
            let remote_node_bytes = node.id.clone();

            // remove node from to_confirm
            {
                let mut to_confirm = state.connections.ble.to_confirm.write().unwrap();
                to_confirm.remove_entry(&q8id);
            }

            #[cfg(feature = "ble-encryption")]
            let remote_id = match PeerId::from_bytes(&remote_node_bytes) {
                Ok(id) => id,
                Err(e) => {
                    log::error!("BLE identification: invalid node ID: {}", e);
                    return;
                }
            };

            // add node to discovered lists
            Self::node_discovered(state, q8id.clone(), node.id);

            // check if to send a response
            if identification.request {
                Self::identification_send(state, q8id.clone(), false);
            }

            // Initiate encrypted handshake after identification
            #[cfg(feature = "ble-encryption")]
            if let Some(handshake_msg) = BleCrypto::initiate_handshake(state, &q8id, remote_id) {
                Self::send_handshake_message(state, q8id, handshake_msg);
            }
        }
    }

    /// Send a handshake message to a peer
    #[cfg(feature = "ble-encryption")]
    fn send_handshake_message(state: &crate::QaulState, receiver_small_id: Vec<u8>, handshake: proto_net::NoiseHandshake) {
        log::info!(
            "BLE sending handshake message {} to {:?}",
            handshake.message_number,
            receiver_small_id
        );
        let message = proto_net::ble_message::Message::Handshake(handshake);
        Self::create_send_message_raw(state, receiver_small_id, message);
    }

    /// Send Identification
    ///
    /// Send identity information to another node
    fn identification_send(state: &crate::QaulState, receiver_q8id: Vec<u8>, request: bool) {
        log::info!("BLE send identity information");

        // get node ID
        let node_id = Node::get_id(state);

        // create identification message
        let identification = proto_net::Identification {
            request,
            node: Some(proto_net::NodeIdentification {
                id: node_id.to_bytes(),
            }),
        };

        // create unified message
        let message = proto_net::ble_message::Message::Identification(identification);

        Self::create_send_message(state, receiver_q8id, message);
    }

    /// send message
    ///
    /// * receiver_id: the small qaul id of the receiving node
    /// * sender_id: the small qaul id of the sending node (this node)
    /// * data: the binary data of the message to send
    pub fn message_send(state: &crate::QaulState, receiver_id: Vec<u8>, sender_id: Vec<u8>, data: Vec<u8>) {
        log::info!("BLE send message to {:x?}", receiver_id.clone());
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
        Sys::send_message(state, buf);
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
    pub fn send_routing_info(state: &crate::QaulState, node_id: PeerId, data: Vec<u8>) {
        log::info!("BLE send routing information");
        let message = proto_net::ble_message::Message::Info(data);

        Self::create_send_message(state, QaulId::to_q8id(node_id), message);
    }

    /// send messaging message
    pub fn send_messaging_message(state: &crate::QaulState, node_id: PeerId, data: Vec<u8>) {
        log::info!("BLE send messaging message to {}", node_id.to_base58());

        let message = proto_net::ble_message::Message::Messaging(data);

        Self::create_send_message(state, QaulId::to_q8id(node_id), message);
    }

    /// send feed message
    pub fn send_feed_message(state: &crate::QaulState, _topic: Topic, data: Vec<u8>) {
        log::info!("BLE send public message");

        // find all nodes, that are only connected through BLE
        let rs = state.get_router();
        let nodes = rs.neighbours.get_ble_only_nodes();

        // create BLE message
        let message = proto_net::ble_message::Message::Feed(data);

        // send it nodes
        for node_id in nodes {
            Self::create_send_message(state, QaulId::to_q8id(node_id), message.clone());
        }
    }

    /// Create and send an encrypted message if session is established
    ///
    /// If no encrypted session is established, the message is sent unencrypted.
    #[cfg(feature = "ble-encryption")]
    fn create_send_message(state: &crate::QaulState, receiver_q8id: Vec<u8>, message: proto_net::ble_message::Message) {
        // Check if encrypted session is established
        if BleCrypto::is_session_established(state, &receiver_q8id) {
            // Encode inner message
            let proto_message = proto_net::BleMessage {
                message: Some(message.clone()),
            };
            let mut plaintext = Vec::with_capacity(proto_message.encoded_len());
            proto_message
                .encode(&mut plaintext)
                .expect("encoding failed");

            // Encrypt the message
            match BleCrypto::encrypt(state, &receiver_q8id, plaintext) {
                Ok(encrypted) => {
                    log::trace!("BLE: sending encrypted message");
                    let encrypted_msg = proto_net::ble_message::Message::Encrypted(encrypted);
                    Self::create_send_message_raw(state, receiver_q8id, encrypted_msg);
                    return;
                }
                Err(e) => {
                    log::error!("BLE encryption failed: {:?}", e);
                    // Fall through to send unencrypted
                }
            }
        }

        // Fallback: send unencrypted (during handshake or if encryption fails)
        Self::create_send_message_raw(state, receiver_q8id, message);
    }

    /// Send a message without transport encryption.
    #[cfg(not(feature = "ble-encryption"))]
    fn create_send_message(state: &crate::QaulState, receiver_q8id: Vec<u8>, message: proto_net::ble_message::Message) {
        Self::create_send_message_raw(state, receiver_q8id, message);
    }

    /// Send a message without encryption
    ///
    /// This is used for handshake messages and as fallback when encryption fails.
    fn create_send_message_raw(state: &crate::QaulState, receiver_q8id: Vec<u8>, message: proto_net::ble_message::Message) {
        // get small qaul id of this node
        let sender_id = Node::get_q8id(state);

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
        Self::message_send(state, receiver_q8id, sender_id, buf);
    }

    /// BLE message received
    fn message_received(state: &crate::QaulState, message: proto::BleDirectReceived) {
        log::info!("BLE message received");
        // get node ID of sender
        let node_id: PeerId;
        let rs = state.get_router();
        if let Some(node) = Neighbours::node_from_q8id(&rs, message.from.clone()) {
            match PeerId::from_bytes(&node.id) {
                Ok(id) => node_id = id,
                Err(e) => {
                    log::error!("Neighbour ID Vec error: {}", e);
                    return;
                }
            }
        } else {
            log::warn!("BLE node ID not found");
            // TODO: find a better solution in the future
            //
            // Idea: there could be a specific BLE unknown node ID
            //       that only allows for node ID identification messages.
            //
            // if we don't know the ID of the peer yet,
            // put in our peer ID
            node_id = Node::get_id(state);
        }

        // decode and distribute messages
        match proto_net::BleMessage::decode(&message.data[..]) {
            Ok(ble_message) => match ble_message.message {
                // Handle encrypted messages
                #[cfg(feature = "ble-encryption")]
                Some(proto_net::ble_message::Message::Encrypted(encrypted)) => {
                    log::info!("BLE encrypted message received");
                    match BleCrypto::decrypt(state, &message.from, encrypted) {
                        Ok(plaintext) => {
                            // Decode and process inner message
                            match proto_net::BleMessage::decode(&plaintext[..]) {
                                Ok(inner) => {
                                    Self::process_decrypted_message(state, message.from, node_id, inner);
                                }
                                Err(e) => {
                                    log::error!("BLE inner message decoding error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("BLE decryption failed: {:?}", e);
                        }
                    }
                }
                #[cfg(not(feature = "ble-encryption"))]
                Some(proto_net::ble_message::Message::Encrypted(_)) => {
                    log::warn!(
                        "BLE encrypted message dropped: feature `ble-encryption` is disabled"
                    );
                }
                // Handle handshake messages
                #[cfg(feature = "ble-encryption")]
                Some(proto_net::ble_message::Message::Handshake(handshake)) => {
                    log::info!("BLE handshake message received");
                    Self::handle_handshake(state, message.from, node_id, handshake);
                }
                #[cfg(not(feature = "ble-encryption"))]
                Some(proto_net::ble_message::Message::Handshake(_)) => {
                    log::warn!("BLE handshake message dropped: feature `ble-encryption` is disabled");
                }
                Some(proto_net::ble_message::Message::Info(data)) => {
                    log::info!("BLE routing info received");
                    let received = qaul_info::QaulInfoReceived {
                        received_from: node_id,
                        data,
                    };
                    let rs = state.get_router();
                    crate::router::info::RouterInfo::received(state, &rs, received);
                }
                Some(proto_net::ble_message::Message::Feed(data)) => {
                    log::info!("BLE public message received");
                    match feed::proto_net::FeedContainer::decode(&data[..]) {
                        Ok(feed_container) => {
                            feed::Feed::received(state, ConnectionModule::Ble, node_id, feed_container);
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
                    messaging::Messaging::received(state, received);
                }
                Some(proto_net::ble_message::Message::Identification(identification)) => {
                    log::info!("BLE identification received");
                    Self::identification_received(state, message.from, identification);
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

    /// Process a decrypted BLE message
    #[cfg(feature = "ble-encryption")]
    fn process_decrypted_message(
        state: &crate::QaulState,
        from: Vec<u8>,
        node_id: PeerId,
        ble_message: proto_net::BleMessage,
    ) {
        match ble_message.message {
            Some(proto_net::ble_message::Message::Info(data)) => {
                log::info!("BLE routing info received (decrypted)");
                let received = qaul_info::QaulInfoReceived {
                    received_from: node_id,
                    data,
                };
                let rs = state.get_router();
                crate::router::info::RouterInfo::received(state, &rs, received);
            }
            Some(proto_net::ble_message::Message::Feed(data)) => {
                log::info!("BLE public message received (decrypted)");
                match feed::proto_net::FeedContainer::decode(&data[..]) {
                    Ok(feed_container) => {
                        feed::Feed::received(state, ConnectionModule::Ble, node_id, feed_container);
                    }
                    Err(e) => {
                        log::error!("BleMessage feed decoding error: {}", e);
                    }
                }
            }
            Some(proto_net::ble_message::Message::Messaging(data)) => {
                log::info!("BLE messaging message received (decrypted)");
                let received = qaul_messaging::QaulMessagingReceived {
                    received_from: node_id,
                    data,
                };
                messaging::Messaging::received(state, received);
            }
            Some(proto_net::ble_message::Message::Identification(identification)) => {
                log::info!("BLE identification received (decrypted)");
                Self::identification_received(state, from, identification);
            }
            _ => {
                log::error!("unprocessable decrypted BleMessage");
            }
        }
    }

    /// Handle incoming handshake messages
    #[cfg(feature = "ble-encryption")]
    fn handle_handshake(state: &crate::QaulState, from: Vec<u8>, remote_id: PeerId, handshake: proto_net::NoiseHandshake) {
        match handshake.message_number {
            1 => {
                // Check for simultaneous handshake initiation (race condition).
                // If we also have a pending handshake to this peer, use a
                // deterministic tiebreaker: the peer with the lexicographically
                // lower small_id becomes the initiator.
                if BleCrypto::has_pending_session(state, &from) {
                    let local_id = Node::get_q8id(state);
                    if local_id < from.to_vec() {
                        // We have the lower ID, so we stay as initiator.
                        // Ignore the remote's handshake 1; they will process
                        // our handshake 1 and respond with handshake 2.
                        log::info!(
                            "BLE: simultaneous handshake detected with {:?}, we win tiebreaker (initiator)",
                            from
                        );
                        return;
                    }
                    // Remote has the lower ID, so they are the initiator.
                    // Drop our pending session and become the responder.
                    log::info!(
                        "BLE: simultaneous handshake detected with {:?}, they win tiebreaker (we become responder)",
                        from
                    );
                }

                // Received first handshake, respond with second
                log::info!("BLE: processing handshake 1 from {:?}", from);
                match BleCrypto::process_handshake_1(state, &from, handshake, remote_id) {
                    Ok(response) => {
                        Self::send_handshake_message(state, from, response);
                    }
                    Err(e) => {
                        log::error!("BLE handshake 1 failed: {:?}", e);
                    }
                }
            }
            2 => {
                // Received second handshake, complete session
                log::info!("BLE: processing handshake 2 from {:?}", from);
                match BleCrypto::process_handshake_2(state, &from, handshake) {
                    Ok(()) => {
                        log::info!("BLE encryption session established with {:?}", from);
                    }
                    Err(e) => {
                        log::error!("BLE handshake 2 failed: {:?}", e);
                    }
                }
            }
            _ => {
                log::error!(
                    "BLE invalid handshake message number: {}",
                    handshake.message_number
                );
            }
        }
    }

    /// receive sys messages from BLE module
    pub fn sys_received(state: &crate::QaulState, data: Vec<u8>) {
        match proto::Ble::decode(&data[..]) {
            Ok(ble) => match ble.message {
                Some(proto::ble::Message::InfoResponse(info_response)) => {
                    Self::info_received(state, info_response);
                }
                Some(proto::ble::Message::StartResult(start_result)) => {
                    Self::module_start_result(state, start_result);
                }
                Some(proto::ble::Message::StopResult(stop_result)) => {
                    Self::module_stop_result(state, stop_result);
                }
                Some(proto::ble::Message::DeviceDiscovered(device)) => {
                    Self::device_discovered(state, device);
                }
                Some(proto::ble::Message::DeviceUnavailable(device)) => {
                    Self::device_unavailable(state, device);
                }
                Some(proto::ble::Message::DirectReceived(direct_received)) => {
                    Self::message_received(state, direct_received);
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
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, request_id: String) {
        log::trace!("BLE rpc message received");

        match proto_rpc::Ble::decode(&data[..]) {
            Ok(ble) => {
                match ble.message {
                    Some(proto_rpc::ble::Message::InfoRequest(_)) => {
                        // get module state
                        let ble = state.connections.ble.inner.read().unwrap();

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
                            q8id: ble.ble_id.clone(),
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
                            state,
                            buf,
                            crate::rpc::proto::Modules::Ble.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::ble::Message::StartRequest(_)) => {
                        // start BLE module
                        Self::module_start(state);
                    }
                    Some(proto_rpc::ble::Message::StopRequest(_)) => {
                        // stop BLE module
                        Self::module_stop(state);
                    }
                    Some(proto_rpc::ble::Message::DiscoveredRequest(_)) => {
                        // get nodes state
                        let nodes = state.connections.ble.nodes.read().unwrap();
                        // get to confirm state
                        let to_confirm = state.connections.ble.to_confirm.read().unwrap();

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
                            state,
                            buf,
                            crate::rpc::proto::Modules::Ble.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::ble::Message::RightsResult(rights_result)) => {
                        if rights_result.rights_granted {
                            log::info!("BLE rights granted");
                            Self::module_start(state);
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
