// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul DTN Service
//!
//! The DTN service sends and receives DTN messages into the network.
//! They should reach everyone in the network.

use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled;
use state::InitCell;
use std::{convert::TryInto, fmt, sync::RwLock};

use super::messaging::{proto, MessagingServiceType};
use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::router::table::RoutingTable;
use crate::rpc::Rpc;
use crate::storage::configuration::Configuration;
use crate::storage::database::DataBase;
use crate::utilities::timestamp::Timestamp;

/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_dtn as proto_rpc;
/// DTN message entry new_sig => {org_sig, size}
/// This structure is used to update storage node state(used size and message count)
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DtnMessageEntry {
    /// original DTN message signature
    pub org_sig: Vec<u8>,
    /// DTN payload size (bytes)
    pub size: u32,
}

/// dtn storage state
#[derive(Clone)]
pub struct DtnStorageState {
    /// Repacked and pending DTN message count
    pub message_counts: u32,
    /// Current used size
    pub used_size: u64,
    /// DTN message table ref
    ///
    /// value: bincode of `DtnMessageEntry`
    pub db_ref: sled::Tree,
    /// DTN message id table ref (org_dtn_sig => new_dtn_sig)
    /// This is used to prevent dup DTN message incoming
    /// saved as `Vec<u8>`
    pub db_ref_id: sled::Tree,
}

/// mutable state of storge
pub static STORAGESTATE: InitCell<RwLock<DtnStorageState>> = InitCell::new();

/// DTN V2 routed message entry stored in sled
#[derive(Serialize, Deserialize, Clone)]
pub struct DtnRoutedV2Entry {
    /// serialized DtnRoutedV2 protobuf message
    pub routed_v2_bytes: Vec<u8>,
    /// public key of the original sender
    pub sender_public_key: Vec<u8>,
    /// size of the entry in bytes
    pub size: u32,
    /// timestamp when this entry was accepted
    pub accepted_at: u64,
    /// the ultimate receiver's user ID
    pub receiver_id: Vec<u8>,
}

/// Per-sender quota tracking for V2 DTN messages
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SenderQuotaEntry {
    /// total bytes used by this sender
    pub used_bytes: u64,
    /// number of messages stored for this sender
    pub message_count: u32,
}

/// V2 DTN storage state
#[derive(Clone)]
pub struct DtnStorageStateV2 {
    /// V2 routed messages: original_signature => DtnRoutedV2Entry
    pub db_ref_routed_v2: sled::Tree,
    /// Per-sender quota tracking: sender_public_key => SenderQuotaEntry
    pub db_ref_sender_quotas: sled::Tree,
    /// Total used size for V2 messages
    pub used_size: u64,
    /// Total V2 message count
    pub message_count: u32,
}

/// mutable state of V2 DTN storage
pub static STORAGESTATE_V2: InitCell<RwLock<DtnStorageStateV2>> = InitCell::new();

/// Maximum bytes a single sender can store on this node (10 MB)
const V2_PER_SENDER_QUOTA: u64 = 10 * 1024 * 1024;

/// qaul Delayed
///
pub struct Dtn {}

impl Dtn {
    /// init function
    /// Read dtn message table and initialize storage state
    pub fn init() {
        let db = DataBase::get_node_db();

        // open trees
        let dtn_messages: sled::Tree = db.open_tree("dtn-messages").unwrap();
        let db_ref_id: sled::Tree = db.open_tree("dtn-messages-ids").unwrap();

        // calc current used size
        let mut used_size: u64 = 0;
        for entry in dtn_messages.iter() {
            if let Ok((_, message_entry_bytes)) = entry {
                let message_entry: DtnMessageEntry =
                    bincode::deserialize(&message_entry_bytes).unwrap();
                used_size = used_size + (message_entry.size as u64);
            }
        }
        let storage_state = DtnStorageState {
            message_counts: dtn_messages.len() as u32,
            used_size,
            db_ref: dtn_messages,
            db_ref_id: db_ref_id,
        };

        STORAGESTATE.set(RwLock::new(storage_state));

        // Initialize V2 storage
        let db_v2 = DataBase::get_node_db();
        let db_ref_routed_v2 = db_v2.open_tree("dtn-routed-v2").unwrap();
        let db_ref_sender_quotas = db_v2.open_tree("dtn-sender-quotas").unwrap();

        let mut v2_used_size: u64 = 0;
        for entry in db_ref_routed_v2.iter() {
            if let Ok((_, entry_bytes)) = entry {
                if let Ok(v2_entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                    v2_used_size += v2_entry.size as u64;
                }
            }
        }

        let v2_state = DtnStorageStateV2 {
            message_count: db_ref_routed_v2.len() as u32,
            used_size: v2_used_size,
            db_ref_routed_v2,
            db_ref_sender_quotas,
        };

        STORAGESTATE_V2.set(RwLock::new(v2_state));
    }

    /// Convert Group ID from String to Binary
    fn id_string_to_bin(id: String) -> Result<Vec<u8>, String> {
        // check length
        if id.len() < 52 {
            return Err("Group ID not long enough".to_string());
        }

        // convert input
        match bs58::decode(id).into_vec() {
            Ok(id_bin) => Ok(id_bin),
            Err(e) => {
                let err = fmt::format(format_args!("{}", e));
                Err(err)
            }
        }
    }

    /// Get storage node user id
    pub fn get_storage_user(user_id: &PeerId) -> Option<PeerId> {
        let user_profile;
        match Configuration::get_user(user_id.to_string()) {
            Some(user_prof) => {
                user_profile = user_prof.clone();
            }
            None => {
                log::error!("dtn module: user profile no exists");
                return None;
            }
        }

        for user in &user_profile.storage.users {
            match Self::id_string_to_bin(user.clone()) {
                Ok(v) => match PeerId::from_bytes(&v) {
                    Ok(id) => {
                        return Some(id.clone());
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        None
    }

    /// process DTN message by role as stroage node
    fn process_storage_node_message(
        user_account: &UserAccount,
        receiver_id: &PeerId,
        org_sig: &Vec<u8>,
        dtn_payload: &Vec<u8>,
    ) -> (i32, i32) {
        let mut storage_state = STORAGESTATE.get().write().unwrap();

        // check already received
        if storage_state.db_ref_id.contains_key(org_sig).unwrap() {
            return (
                super::messaging::proto::dtn_response::ResponseType::Accepted
                    .try_into()
                    .unwrap(),
                super::messaging::proto::dtn_response::Reason::None
                    .try_into()
                    .unwrap(),
            );
        }

        let user_profile;
        match Configuration::get_user(user_account.id.to_string()) {
            Some(user_prof) => {
                user_profile = user_prof.clone();
            }
            None => {
                log::error!("dtn module: user profile no exists");
                return (
                    super::messaging::proto::dtn_response::ResponseType::Rejected
                        .try_into()
                        .unwrap(),
                    super::messaging::proto::dtn_response::Reason::UserNotAccepted
                        .try_into()
                        .unwrap(),
                );
            }
        }

        // check storage
        let new_size = storage_state.used_size + (dtn_payload.len() as u64);
        let total_limit = (user_profile.storage.size_total as u64) * 1024 * 1024;
        if new_size > total_limit {
            return (
                super::messaging::proto::dtn_response::ResponseType::Rejected
                    .try_into()
                    .unwrap(),
                super::messaging::proto::dtn_response::Reason::OverallQuota
                    .try_into()
                    .unwrap(),
            );
        }

        // repack message and send
        let payload = super::messaging::proto::EnvelopPayload {
            payload: Some(super::messaging::proto::envelop_payload::Payload::Dtn(
                dtn_payload.clone(),
            )),
        };

        let envelop = super::messaging::proto::Envelope {
            sender_id: user_account.id.to_bytes(),
            receiver_id: receiver_id.to_bytes(),
            payload: payload.encode_to_vec(),
        };

        if let Ok(signature) = user_account.keys.sign(&envelop.encode_to_vec()) {
            // save dtn message entry
            storage_state.message_counts = storage_state.message_counts + 1;
            storage_state.used_size = new_size;

            let message_entry = DtnMessageEntry {
                org_sig: org_sig.clone(),
                size: dtn_payload.len() as u32,
            };
            let message_entry_bytes = bincode::serialize(&message_entry).unwrap();

            if let Err(_e) = storage_state
                .db_ref
                .insert(signature.clone(), message_entry_bytes)
            {
                log::error!("dnt entry storing error!");
            } else {
                if let Err(_e) = storage_state.db_ref.flush() {
                    log::error!("dnt entry flushing error!");
                }
            }

            // save message id
            if let Err(_e) = storage_state
                .db_ref_id
                .insert(org_sig.clone(), signature.clone())
            {
                log::error!("dtn id db storing error!");
            } else {
                if let Err(_e) = storage_state.db_ref_id.flush() {
                    log::error!("dtn id db flushing error!");
                }
            }

            let container = super::messaging::proto::Container {
                signature: signature.clone(),
                envelope: Some(envelop),
            };
            super::messaging::Messaging::save_unconfirmed_message(
                MessagingServiceType::DtnStored,
                &vec![],
                receiver_id,
                &container,
                true,
            );
        }

        (
            super::messaging::proto::dtn_response::ResponseType::Accepted
                .try_into()
                .unwrap(),
            super::messaging::proto::dtn_response::Reason::None
                .try_into()
                .unwrap(),
        )
        // update storage state
    }

    /// this function is called when receive DTN response
    pub fn on_dtn_response(dtn_response: &super::messaging::proto::DtnResponse) {
        // check if storage node case
        let mut state = STORAGESTATE.get().write().unwrap();
        if state.db_ref.contains_key(&dtn_response.signature).unwrap() {
            // update storage node state
            let entry_bytes = state.db_ref.get(&dtn_response.signature).unwrap().unwrap();
            let entry: DtnMessageEntry = bincode::deserialize(&entry_bytes).unwrap();
            if state.used_size > entry.size as u64 {
                state.used_size = state.used_size + (entry.size as u64);
            } else {
                state.used_size = 0;
            }
            if state.message_counts > 0 {
                state.message_counts = state.message_counts - 1;
            }

            // remove entry
            if let Err(_) = state.db_ref.remove(&dtn_response.signature) {
                log::error!("remove storage node entry error!");
            } else {
                if let Err(_) = state.db_ref.flush() {
                    log::error!("remove storage node entry flush error!");
                }
            }

            if let Err(_) = state.db_ref_id.remove(&entry.org_sig) {
                log::error!("remove storage node id entry error!");
            } else {
                if let Err(_) = state.db_ref_id.flush() {
                    log::error!("remove storage node id entry flush error!");
                }
            }
        }
    }

    /// process DTN messages from network
    pub fn net(user_id: &PeerId, sender_id: &PeerId, signature: &Vec<u8>, dtn_payload: &Vec<u8>) {
        if let Some(user_account) = UserAccounts::get_by_id(*user_id) {
            match proto::Container::decode(&dtn_payload[..]) {
                Ok(container) => {
                    let envelope = container.envelope.as_ref().unwrap();

                    let mut res: (i32, i32) = (
                        super::messaging::proto::dtn_response::ResponseType::Accepted
                            .try_into()
                            .unwrap(),
                        super::messaging::proto::dtn_response::Reason::None
                            .try_into()
                            .unwrap(),
                    );

                    //if container.envelope.receiver_id
                    if let Ok(receiver_id) = PeerId::from_bytes(&envelope.receiver_id) {
                        if receiver_id == *user_id {
                            // by process geneal message, the confirm message is transfered to the original sender.
                            super::messaging::process::MessagingProcess::process_received_message(
                                user_account.clone(),
                                container,
                            );
                        } else {
                            res = Self::process_storage_node_message(
                                &user_account,
                                &receiver_id,
                                signature,
                                dtn_payload,
                            );
                        }

                        // we send DTN response message
                        let dnt_response = super::messaging::proto::DtnResponse {
                            response_type: res.0,
                            reason: res.1,
                            signature: signature.clone(),
                        };
                        let send_message = proto::Messaging {
                            message: Some(proto::messaging::Message::DtnResponse(dnt_response)),
                        };
                        if let Err(_) = super::messaging::Messaging::pack_and_send_message(
                            &user_account,
                            sender_id,
                            send_message.encode_to_vec(),
                            MessagingServiceType::DtnStored,
                            &Vec::new(),
                            false,
                        ) {
                            log::error!("send dtn message error!");
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// process commands from RPC
    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        // create peer ID from bytes
        let my_user_id;
        match PeerId::from_bytes(&user_id) {
            Ok(peer_id) => my_user_id = peer_id,
            Err(e) => {
                log::error!("invalid user id: {}", e);
                return;
            }
        }

        match proto_rpc::Dtn::decode(&data[..]) {
            Ok(dtn) => match dtn.message {
                Some(proto_rpc::dtn::Message::DtnStateRequest(_req)) => {
                    let state = STORAGESTATE.get().read().unwrap();
                    let unconfirmed = super::messaging::UNCONFIRMED.get().read().unwrap();
                    let unconfrimed_len = unconfirmed.unconfirmed.len();

                    let proto_message = proto_rpc::Dtn {
                        message: Some(proto_rpc::dtn::Message::DtnStateResponse(
                            proto_rpc::DtnStateResponse {
                                used_size: state.used_size,
                                dtn_message_count: state.message_counts,
                                unconfirmed_count: unconfrimed_len as u32,
                            },
                        )),
                    };

                    // send message
                    Rpc::send_message(
                        proto_message.encode_to_vec(),
                        crate::rpc::proto::Modules::Dtn.into(),
                        request_id,
                        Vec::new(),
                    );
                }
                Some(proto_rpc::dtn::Message::DtnConfigRequest(_req)) => {
                    match Configuration::get_user(my_user_id.to_string()) {
                        Some(user_profile) => {
                            let mut users: Vec<Vec<u8>> = Vec::new();
                            // create users list
                            for user in user_profile.storage.users {
                                // convert string to bytes id
                                match bs58::decode(user).into_vec() {
                                    Ok(user_id) => users.push(user_id),
                                    Err(e) => log::error!(
                                        "invalid bs58 DTN storage user configuration: {}",
                                        e
                                    ),
                                }
                            }

                            // create message
                            let proto_message = proto_rpc::Dtn {
                                message: Some(proto_rpc::dtn::Message::DtnConfigResponse(
                                    proto_rpc::DtnConfigResponse {
                                        total_size: user_profile.storage.size_total,
                                        users: users,
                                    },
                                )),
                            };

                            // send message
                            Rpc::send_message(
                                proto_message.encode_to_vec(),
                                crate::rpc::proto::Modules::Dtn.into(),
                                request_id,
                                Vec::new(),
                            );
                        }
                        None => {
                            log::error!("user profile does not exists");
                        }
                    }
                }
                Some(proto_rpc::dtn::Message::DtnAddUserRequest(req)) => {
                    let mut status = true;
                    let mut message: String = "".to_string();

                    match Configuration::get_user(my_user_id.to_string()) {
                        Some(user_profile) => {
                            // CHANGE: save it to user account and not to configuration directly

                            // convert binary data to user string
                            let user_id_string;
                            match PeerId::from_bytes(&req.user_id) {
                                Ok(user_id) => user_id_string = user_id.to_base58(),
                                Err(e) => {
                                    log::error!("configuration error reading account it: {}", e);
                                    return;
                                }
                            }

                            // check if already exist
                            for user in &user_profile.storage.users {
                                if *user == user_id_string {
                                    status = false;
                                    message = "User already exist".to_string();
                                    break;
                                }
                            }
                            if status {
                                let mut opt = user_profile.storage.clone();
                                opt.users.push(user_id_string);
                                Configuration::update_user_storage(my_user_id.to_string(), &opt);
                                Configuration::save();
                            }

                            let proto_message = proto_rpc::Dtn {
                                message: Some(proto_rpc::dtn::Message::DtnAddUserResponse(
                                    proto_rpc::DtnAddUserResponse { status, message },
                                )),
                            };
                            // send message
                            Rpc::send_message(
                                proto_message.encode_to_vec(),
                                crate::rpc::proto::Modules::Dtn.into(),
                                request_id,
                                Vec::new(),
                            );
                        }
                        None => {
                            log::error!("user profile does not exists");
                        }
                    }
                }
                Some(proto_rpc::dtn::Message::DtnRemoveUserRequest(req)) => {
                    let mut status = true;
                    let mut message: String = "".to_string();

                    match Configuration::get_user(my_user_id.to_string()) {
                        Some(user_profile) => {
                            // CHANGE: save it to user_account and not to configuration directly

                            // convert binary data to user string
                            let user_id_string;
                            match PeerId::from_bytes(&req.user_id) {
                                Ok(user_id) => user_id_string = user_id.to_base58(),
                                Err(e) => {
                                    log::error!("configuration error reading account it: {}", e);
                                    return;
                                }
                            }

                            // check if user storage exists
                            let mut idx: Option<usize> = None;
                            for i in 0..user_profile.storage.users.len() {
                                if *user_profile.storage.users.get(i).unwrap() == user_id_string {
                                    idx = Some(i);
                                    break;
                                }
                            }
                            match idx {
                                None => {
                                    status = false;
                                    message = "User does not exist".to_string();
                                }
                                _ => {}
                            }

                            if status {
                                let mut opt = user_profile.storage.clone();
                                opt.users.remove(idx.unwrap());
                                Configuration::update_user_storage(my_user_id.to_string(), &opt);
                                Configuration::save();
                            }

                            let proto_message = proto_rpc::Dtn {
                                message: Some(proto_rpc::dtn::Message::DtnRemoveUserResponse(
                                    proto_rpc::DtnRemoveUserResponse { status, message },
                                )),
                            };
                            // send message
                            Rpc::send_message(
                                proto_message.encode_to_vec(),
                                crate::rpc::proto::Modules::Dtn.into(),
                                request_id,
                                Vec::new(),
                            );
                        }
                        None => {
                            log::error!("user profile does not exists");
                        }
                    }
                }
                Some(proto_rpc::dtn::Message::DtnSetTotalSizeRequest(req)) => {
                    match Configuration::get_user(my_user_id.to_string()) {
                        // CHANGE: save it in user profile, not to configuration directly.
                        Some(_user_profile) => {
                            Configuration::update_total_size(
                                my_user_id.to_string(),
                                req.total_size,
                            );
                            Configuration::save();

                            let proto_message = proto_rpc::Dtn {
                                message: Some(proto_rpc::dtn::Message::DtnSetTotalSizeResponse(
                                    proto_rpc::DtnSetTotalSizeResponse {
                                        status: true,
                                        message: "".to_string(),
                                    },
                                )),
                            };
                            // send message
                            Rpc::send_message(
                                proto_message.encode_to_vec(),
                                crate::rpc::proto::Modules::Dtn.into(),
                                request_id,
                                Vec::new(),
                            );
                        }
                        None => {
                            log::error!("user profile does not exists");
                        }
                    }
                }
                Some(proto_rpc::dtn::Message::DtnSendRoutedRequest(req)) => {
                    Self::rpc_send_routed(my_user_id, req, request_id);
                }
                _ => {
                    log::error!("Unhandled Protobuf DTN RPC message");
                }
            },
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    /// Handle DtnSendRoutedRequest RPC
    fn rpc_send_routed(
        my_user_id: PeerId,
        req: proto_rpc::DtnSendRoutedRequest,
        request_id: String,
    ) {
        let send_response = |status: bool, message: String| {
            let proto_message = proto_rpc::Dtn {
                message: Some(proto_rpc::dtn::Message::DtnSendRoutedResponse(
                    proto_rpc::DtnSendRoutedResponse { status, message },
                )),
            };
            Rpc::send_message(
                proto_message.encode_to_vec(),
                crate::rpc::proto::Modules::Dtn.into(),
                request_id.clone(),
                Vec::new(),
            );
        };

        // Validate receiver
        let receiver_id = match PeerId::from_bytes(&req.receiver_id) {
            Ok(id) => id,
            Err(_) => {
                send_response(false, "invalid receiver_id".to_string());
                return;
            }
        };

        // Validate routes
        if req.routes.is_empty() {
            send_response(false, "at least one route is required".to_string());
            return;
        }
        if req.routes.len() > 5 {
            send_response(false, "maximum 5 routes allowed".to_string());
            return;
        }
        for route in &req.routes {
            if route.custody_users.is_empty() {
                send_response(false, "routes must not be empty".to_string());
                return;
            }
            if route.custody_users.len() > 10 {
                send_response(false, "maximum 10 custodians per route".to_string());
                return;
            }
            for user_bytes in &route.custody_users {
                if let Ok(uid) = PeerId::from_bytes(user_bytes) {
                    if uid == my_user_id || uid == receiver_id {
                        send_response(false, "custodians must not include sender or receiver".to_string());
                        return;
                    }
                } else {
                    send_response(false, "invalid custodian user ID".to_string());
                    return;
                }
            }
        }

        // Get user account
        let user_account = match UserAccounts::get_by_id(my_user_id) {
            Some(ua) => ua,
            None => {
                send_response(false, "user account not found".to_string());
                return;
            }
        };

        // Build custody routes
        let routes: Vec<proto::CustodyRoute> = req
            .routes
            .iter()
            .map(|r| proto::CustodyRoute {
                custody_users: r.custody_users.clone(),
                next_index: 0,
            })
            .collect();

        // Calculate expiry
        let expires_at = if req.expiry_seconds > 0 {
            Timestamp::get_timestamp() + (req.expiry_seconds * 1000)
        } else {
            0
        };

        // Calculate remaining handoffs
        let total_custodians: u32 = routes.iter().map(|r| r.custody_users.len() as u32).sum();
        let remaining_handoffs = if req.max_handoffs > 0 {
            req.max_handoffs
        } else {
            total_custodians * 2
        };

        // Build the DtnRoutedV2 message
        let routed_v2 = proto::DtnRoutedV2 {
            container: req.data.clone(),
            routes,
            original_signature: Vec::new(), // will be set after signing
            sender_public_key: user_account.keys.public().encode_protobuf(),
            expires_at,
            remaining_handoffs,
        };

        // Find initial target
        let target = match Self::select_custody_target(&routed_v2, &receiver_id) {
            Some(t) => t,
            None => {
                send_response(false, "no reachable custodian found".to_string());
                return;
            }
        };

        // Send via envelope
        match super::messaging::Messaging::send_dtn_routed_v2_message(
            &user_account,
            &target,
            routed_v2,
        ) {
            Ok(_sig) => {
                send_response(true, "".to_string());
            }
            Err(e) => {
                send_response(false, e);
            }
        }
    }

    /// Determine where to forward a V2 DTN message.
    ///
    /// Returns the recipient if online, otherwise scans custody routes
    /// in priority order, within each route scanning from the end
    /// (closest to recipient) backward to next_index.
    pub fn select_custody_target(
        routed_v2: &proto::DtnRoutedV2,
        receiver_id: &PeerId,
    ) -> Option<PeerId> {
        // Check if recipient is directly reachable
        if RoutingTable::get_route_to_user(*receiver_id).is_some() {
            return Some(*receiver_id);
        }

        // Try each route in priority order
        for route in &routed_v2.routes {
            let len = route.custody_users.len();
            let start = route.next_index as usize;
            if start >= len {
                continue;
            }
            // Scan from the end (closest to recipient) backward to next_index
            for i in (start..len).rev() {
                if let Ok(custodian_id) = PeerId::from_bytes(&route.custody_users[i]) {
                    if RoutingTable::get_route_to_user(custodian_id).is_some() {
                        return Some(custodian_id);
                    }
                }
            }
        }

        None
    }

    /// Process a received DtnRoutedV2 message from the network
    pub fn net_routed_v2(
        user_id: &PeerId,
        sender_id: &PeerId,
        signature: &[u8],
        routed_v2: proto::DtnRoutedV2,
    ) {
        log::info!("Received DtnRoutedV2 message from {}", sender_id.to_base58());

        let user_account = match UserAccounts::get_by_id(*user_id) {
            Some(ua) => ua,
            None => return,
        };

        // 1. Expiry check
        if routed_v2.expires_at > 0 {
            let now = Timestamp::get_timestamp();
            if now > routed_v2.expires_at {
                log::warn!("DtnRoutedV2 message expired, dropping");
                Self::send_v2_response(
                    &user_account,
                    sender_id,
                    signature,
                    proto::dtn_response::ResponseType::Rejected,
                    proto::dtn_response::Reason::None,
                );
                return;
            }
        }

        // 2. Handoff check
        if routed_v2.remaining_handoffs == 0 {
            log::warn!("DtnRoutedV2 message has no remaining handoffs, dropping");
            Self::send_v2_response(
                &user_account,
                sender_id,
                signature,
                proto::dtn_response::ResponseType::Rejected,
                proto::dtn_response::Reason::None,
            );
            return;
        }

        // 3. Duplicate check
        {
            let state = STORAGESTATE_V2.get().read().unwrap();
            if state
                .db_ref_routed_v2
                .contains_key(&routed_v2.original_signature)
                .unwrap_or(false)
            {
                log::info!("DtnRoutedV2 duplicate detected, accepting silently");
                Self::send_v2_response(
                    &user_account,
                    sender_id,
                    signature,
                    proto::dtn_response::ResponseType::Accepted,
                    proto::dtn_response::Reason::None,
                );
                return;
            }
        }

        // 4. Am I the recipient?
        let envelope_receiver = Self::get_receiver_from_container(&routed_v2.container);
        if let Some(recv_id) = envelope_receiver {
            if recv_id == *user_id {
                log::info!("DtnRoutedV2: I am the recipient, processing inner container");
                if let Ok(container) = proto::Container::decode(&routed_v2.container[..]) {
                    super::messaging::process::MessagingProcess::process_received_message(
                        user_account.clone(),
                        container,
                    );
                }
                Self::send_v2_response(
                    &user_account,
                    sender_id,
                    signature,
                    proto::dtn_response::ResponseType::Accepted,
                    proto::dtn_response::Reason::None,
                );
                return;
            }
        }

        // 5. Per-sender quota check
        {
            let state = STORAGESTATE_V2.get().read().unwrap();
            if let Ok(Some(quota_bytes)) = state
                .db_ref_sender_quotas
                .get(&routed_v2.sender_public_key)
            {
                if let Ok(quota) = bincode::deserialize::<SenderQuotaEntry>(&quota_bytes) {
                    if quota.used_bytes + (routed_v2.container.len() as u64) > V2_PER_SENDER_QUOTA {
                        log::warn!("DtnRoutedV2: per-sender quota exceeded");
                        Self::send_v2_response(
                            &user_account,
                            sender_id,
                            signature,
                            proto::dtn_response::ResponseType::Rejected,
                            proto::dtn_response::Reason::UserQuota,
                        );
                        return;
                    }
                }
            }
        }

        // 6. Overall quota check
        {
            let state = STORAGESTATE_V2.get().read().unwrap();
            let v1_state = STORAGESTATE.get().read().unwrap();
            match Configuration::get_user(user_account.id.to_string()) {
                Some(user_profile) => {
                    let total_limit = (user_profile.storage.size_total as u64) * 1024 * 1024;
                    let total_used =
                        v1_state.used_size + state.used_size + (routed_v2.container.len() as u64);
                    if total_used > total_limit {
                        log::warn!("DtnRoutedV2: overall quota exceeded");
                        Self::send_v2_response(
                            &user_account,
                            sender_id,
                            signature,
                            proto::dtn_response::ResponseType::Rejected,
                            proto::dtn_response::Reason::OverallQuota,
                        );
                        return;
                    }
                }
                None => {
                    log::error!("DtnRoutedV2: user profile not found");
                    Self::send_v2_response(
                        &user_account,
                        sender_id,
                        signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::UserNotAccepted,
                    );
                    return;
                }
            }
        }

        // 7. Accept custody: store in DB
        let entry_size = routed_v2.container.len() as u32;
        let v2_entry = DtnRoutedV2Entry {
            routed_v2_bytes: routed_v2.encode_to_vec(),
            sender_public_key: routed_v2.sender_public_key.clone(),
            size: entry_size,
            accepted_at: Timestamp::get_timestamp(),
            receiver_id: envelope_receiver.map(|r| r.to_bytes()).unwrap_or_default(),
        };
        let entry_bytes = bincode::serialize(&v2_entry).unwrap();

        {
            let mut state = STORAGESTATE_V2.get().write().unwrap();
            if let Err(e) = state
                .db_ref_routed_v2
                .insert(routed_v2.original_signature.clone(), entry_bytes)
            {
                log::error!("DtnRoutedV2: storage insert error: {}", e);
            }
            let _ = state.db_ref_routed_v2.flush();

            state.used_size += entry_size as u64;
            state.message_count += 1;

            // Update sender quota
            let mut quota = if let Ok(Some(quota_bytes)) = state
                .db_ref_sender_quotas
                .get(&routed_v2.sender_public_key)
            {
                bincode::deserialize::<SenderQuotaEntry>(&quota_bytes).unwrap_or_default()
            } else {
                SenderQuotaEntry::default()
            };
            quota.used_bytes += entry_size as u64;
            quota.message_count += 1;
            let quota_bytes = bincode::serialize(&quota).unwrap();
            let _ = state
                .db_ref_sender_quotas
                .insert(routed_v2.sender_public_key.clone(), quota_bytes);
            let _ = state.db_ref_sender_quotas.flush();
        }

        // Send acceptance response
        Self::send_v2_response(
            &user_account,
            sender_id,
            signature,
            proto::dtn_response::ResponseType::Accepted,
            proto::dtn_response::Reason::None,
        );

        // 8. Attempt immediate forward
        if let Some(recv_id) = envelope_receiver {
            Self::try_forward_v2(&user_account, &routed_v2, &recv_id);
        }
    }

    /// Try to forward a V2 message to the next custodian or recipient
    fn try_forward_v2(
        user_account: &UserAccount,
        routed_v2: &proto::DtnRoutedV2,
        receiver_id: &PeerId,
    ) {
        if let Some(target) = Self::select_custody_target(routed_v2, receiver_id) {
            // Advance the route state: if target is in a route, advance next_index
            let mut forwarded = routed_v2.clone();
            forwarded.remaining_handoffs = forwarded.remaining_handoffs.saturating_sub(1);

            // Advance next_index for the route containing the target
            for route in &mut forwarded.routes {
                for (i, user_bytes) in route.custody_users.iter().enumerate() {
                    if let Ok(uid) = PeerId::from_bytes(user_bytes) {
                        if uid == target && i as u32 >= route.next_index {
                            route.next_index = (i as u32) + 1;
                            break;
                        }
                    }
                }
            }

            if let Err(e) = super::messaging::Messaging::send_dtn_routed_v2_message(
                user_account,
                &target,
                forwarded,
            ) {
                log::error!("DtnRoutedV2: forward error: {}", e);
            }
        }
    }

    /// Send a DTN response message for V2 handling
    fn send_v2_response(
        user_account: &UserAccount,
        sender_id: &PeerId,
        signature: &[u8],
        response_type: proto::dtn_response::ResponseType,
        reason: proto::dtn_response::Reason,
    ) {
        let dtn_response = proto::DtnResponse {
            response_type: response_type as i32,
            reason: reason as i32,
            signature: signature.to_vec(),
        };
        let send_message = proto::Messaging {
            message: Some(proto::messaging::Message::DtnResponse(dtn_response)),
        };
        if let Err(e) = super::messaging::Messaging::pack_and_send_message(
            user_account,
            sender_id,
            send_message.encode_to_vec(),
            MessagingServiceType::DtnStored,
            &Vec::new(),
            false,
        ) {
            log::error!("DtnRoutedV2: send response error: {}", e);
        }
    }

    /// Extract the receiver PeerId from a serialized Container
    fn get_receiver_from_container(container_bytes: &[u8]) -> Option<PeerId> {
        if let Ok(container) = proto::Container::decode(container_bytes) {
            if let Some(envelope) = container.envelope {
                return PeerId::from_bytes(&envelope.receiver_id).ok();
            }
        }
        None
    }

    /// Handle DTN response for V2 messages
    pub fn on_dtn_response_v2(dtn_response: &proto::DtnResponse) {
        let mut state = STORAGESTATE_V2.get().write().unwrap();
        if let Ok(Some(entry_bytes)) = state
            .db_ref_routed_v2
            .get(&dtn_response.signature)
        {
            if let Ok(entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                // Only remove on acceptance
                if dtn_response.response_type
                    == proto::dtn_response::ResponseType::Accepted as i32
                {
                    // Remove from V2 storage
                    let _ = state.db_ref_routed_v2.remove(&dtn_response.signature);
                    let _ = state.db_ref_routed_v2.flush();

                    // Update counts
                    state.used_size = state.used_size.saturating_sub(entry.size as u64);
                    if state.message_count > 0 {
                        state.message_count -= 1;
                    }

                    // Update sender quota
                    if let Ok(Some(quota_bytes)) =
                        state.db_ref_sender_quotas.get(&entry.sender_public_key)
                    {
                        if let Ok(mut quota) =
                            bincode::deserialize::<SenderQuotaEntry>(&quota_bytes)
                        {
                            quota.used_bytes = quota.used_bytes.saturating_sub(entry.size as u64);
                            quota.message_count = quota.message_count.saturating_sub(1);
                            let quota_bytes = bincode::serialize(&quota).unwrap();
                            let _ = state
                                .db_ref_sender_quotas
                                .insert(entry.sender_public_key.clone(), quota_bytes);
                            let _ = state.db_ref_sender_quotas.flush();
                        }
                    }
                }
            }
        }
    }

    /// Process V2 routed messages in the retransmit loop.
    /// Called periodically to check if stored V2 messages can be forwarded.
    pub fn process_retransmit_v2() {
        let state = STORAGESTATE_V2.get().read().unwrap();
        let now = Timestamp::get_timestamp();

        let mut to_remove: Vec<Vec<u8>> = Vec::new();
        let mut to_forward: Vec<(Vec<u8>, DtnRoutedV2Entry)> = Vec::new();

        for entry in state.db_ref_routed_v2.iter() {
            if let Ok((sig, entry_bytes)) = entry {
                if let Ok(v2_entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                    if let Ok(routed_v2) =
                        proto::DtnRoutedV2::decode(&v2_entry.routed_v2_bytes[..])
                    {
                        // Check expiry
                        if routed_v2.expires_at > 0 && now > routed_v2.expires_at {
                            to_remove.push(sig.to_vec());
                            continue;
                        }

                        to_forward.push((sig.to_vec(), v2_entry));
                    }
                }
            }
        }
        drop(state);

        // Remove expired entries
        if !to_remove.is_empty() {
            let mut state = STORAGESTATE_V2.get().write().unwrap();
            for sig in &to_remove {
                if let Ok(Some(entry_bytes)) = state.db_ref_routed_v2.get(sig) {
                    if let Ok(entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                        state.used_size = state.used_size.saturating_sub(entry.size as u64);
                        if state.message_count > 0 {
                            state.message_count -= 1;
                        }
                        // Update sender quota
                        if let Ok(Some(quota_bytes)) =
                            state.db_ref_sender_quotas.get(&entry.sender_public_key)
                        {
                            if let Ok(mut quota) =
                                bincode::deserialize::<SenderQuotaEntry>(&quota_bytes)
                            {
                                quota.used_bytes =
                                    quota.used_bytes.saturating_sub(entry.size as u64);
                                quota.message_count = quota.message_count.saturating_sub(1);
                                let quota_bytes = bincode::serialize(&quota).unwrap();
                                let _ = state
                                    .db_ref_sender_quotas
                                    .insert(entry.sender_public_key.clone(), quota_bytes);
                            }
                        }
                    }
                }
                let _ = state.db_ref_routed_v2.remove(sig);
            }
            let _ = state.db_ref_routed_v2.flush();
            let _ = state.db_ref_sender_quotas.flush();
        }

        // Try to forward stored messages
        for (_sig, v2_entry) in &to_forward {
            if let Ok(routed_v2) = proto::DtnRoutedV2::decode(&v2_entry.routed_v2_bytes[..]) {
                if let Ok(recv_id) = PeerId::from_bytes(&v2_entry.receiver_id) {
                    // We need a user account to send from. Use the first local account.
                    if let Some(user_account) = UserAccounts::get_default_user() {
                        Self::try_forward_v2(&user_account, &routed_v2, &recv_id);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn dtn_routed_v2_round_trip() {
        let route = proto::CustodyRoute {
            custody_users: vec![vec![1, 2, 3], vec![4, 5, 6]],
            next_index: 0,
        };

        let original = proto::DtnRoutedV2 {
            container: vec![10, 20, 30, 40],
            routes: vec![route],
            original_signature: vec![0xAA, 0xBB],
            sender_public_key: vec![0xCC, 0xDD],
            expires_at: 1234567890,
            remaining_handoffs: 5,
        };

        // Encode
        let encoded = original.encode_to_vec();
        assert!(!encoded.is_empty());

        // Decode
        let decoded = proto::DtnRoutedV2::decode(&encoded[..]).unwrap();
        assert_eq!(decoded.container, original.container);
        assert_eq!(decoded.routes.len(), 1);
        assert_eq!(decoded.routes[0].custody_users.len(), 2);
        assert_eq!(decoded.routes[0].next_index, 0);
        assert_eq!(decoded.original_signature, original.original_signature);
        assert_eq!(decoded.sender_public_key, original.sender_public_key);
        assert_eq!(decoded.expires_at, original.expires_at);
        assert_eq!(decoded.remaining_handoffs, original.remaining_handoffs);
    }

    #[test]
    fn dtn_routed_v2_serde_round_trip() {
        let route = proto::CustodyRoute {
            custody_users: vec![vec![1, 2, 3]],
            next_index: 1,
        };

        let original = proto::DtnRoutedV2 {
            container: vec![10, 20],
            routes: vec![route],
            original_signature: vec![0xAA],
            sender_public_key: vec![0xCC],
            expires_at: 0,
            remaining_handoffs: 3,
        };

        // Serde round-trip (for sled storage)
        let serialized = bincode::serialize(&original).unwrap();
        let deserialized: proto::DtnRoutedV2 = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.container, original.container);
        assert_eq!(deserialized.remaining_handoffs, 3);
    }

    #[test]
    fn dtn_routed_v2_entry_serde_round_trip() {
        let entry = DtnRoutedV2Entry {
            routed_v2_bytes: vec![1, 2, 3, 4],
            sender_public_key: vec![5, 6],
            size: 100,
            accepted_at: 999,
            receiver_id: vec![7, 8, 9],
        };

        let serialized = bincode::serialize(&entry).unwrap();
        let deserialized: DtnRoutedV2Entry = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.routed_v2_bytes, entry.routed_v2_bytes);
        assert_eq!(deserialized.size, 100);
        assert_eq!(deserialized.accepted_at, 999);
    }

    #[test]
    fn sender_quota_entry_serde_round_trip() {
        let entry = SenderQuotaEntry {
            used_bytes: 5000,
            message_count: 3,
        };

        let serialized = bincode::serialize(&entry).unwrap();
        let deserialized: SenderQuotaEntry = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.used_bytes, 5000);
        assert_eq!(deserialized.message_count, 3);
    }

    #[test]
    fn envelop_payload_dtn_routed_v2_variant() {
        let routed_v2 = proto::DtnRoutedV2 {
            container: vec![1, 2, 3],
            routes: vec![],
            original_signature: vec![],
            sender_public_key: vec![],
            expires_at: 0,
            remaining_handoffs: 1,
        };

        let payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::DtnRoutedV2(routed_v2)),
        };

        let encoded = payload.encode_to_vec();
        let decoded = proto::EnvelopPayload::decode(&encoded[..]).unwrap();

        match decoded.payload {
            Some(proto::envelop_payload::Payload::DtnRoutedV2(v2)) => {
                assert_eq!(v2.container, vec![1, 2, 3]);
                assert_eq!(v2.remaining_handoffs, 1);
            }
            _ => panic!("Expected DtnRoutedV2 payload variant"),
        }
    }
}
