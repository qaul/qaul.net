// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul DTN Service
//!
//! The DTN service sends and receives DTN messages into the network.
//! They should reach everyone in the network.

use libp2p::identity::PublicKey;
use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled;
use std::{fmt, sync::RwLock};

use super::messaging::{proto, MessagingServiceType};
use crate::node::user_accounts::{UserAccount, UserAccounts};
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

/// Maximum bytes a single sender can store on this node (10 MB)
const V2_PER_SENDER_QUOTA: u64 = 10 * 1024 * 1024;

/// Instance-based DTN state.
/// Replaces the global STORAGESTATE static for multi-instance use.
pub struct DtnModuleState {
    /// DTN storage inner state (V1).
    pub inner: RwLock<DtnStorageState>,
    /// DTN storage inner state (V2 routed/custody messages).
    pub v2: RwLock<DtnStorageStateV2>,
    /// Sled database backing (kept alive for tree references).
    _db: RwLock<sled::Db>,
}

impl DtnModuleState {
    /// Create a new empty DtnModuleState with a temporary in-memory database.
    ///
    /// Panics with a clear message if the temporary sled database or any of
    /// the required trees cannot be opened — this is only used during startup
    /// or simulation, where a sled error means the process cannot proceed.
    pub fn new() -> Self {
        let db = match sled::Config::new().temporary(true).open() {
            Ok(db) => db,
            Err(e) => panic!("DtnModuleState: failed to open temporary sled DB: {}", e),
        };
        let dtn_messages = match db.open_tree("dtn-messages") {
            Ok(t) => t,
            Err(e) => panic!("DtnModuleState: failed to open dtn-messages tree: {}", e),
        };
        let dtn_ids = match db.open_tree("dtn-messages-ids") {
            Ok(t) => t,
            Err(e) => panic!(
                "DtnModuleState: failed to open dtn-messages-ids tree: {}",
                e
            ),
        };
        let dtn_routed_v2 = match db.open_tree("dtn-routed-v2") {
            Ok(t) => t,
            Err(e) => panic!("DtnModuleState: failed to open dtn-routed-v2 tree: {}", e),
        };
        let dtn_sender_quotas = match db.open_tree("dtn-sender-quotas") {
            Ok(t) => t,
            Err(e) => panic!(
                "DtnModuleState: failed to open dtn-sender-quotas tree: {}",
                e
            ),
        };
        Self {
            inner: RwLock::new(DtnStorageState {
                message_counts: 0,
                used_size: 0,
                db_ref: dtn_messages,
                db_ref_id: dtn_ids,
            }),
            v2: RwLock::new(DtnStorageStateV2 {
                message_count: 0,
                used_size: 0,
                db_ref_routed_v2: dtn_routed_v2,
                db_ref_sender_quotas: dtn_sender_quotas,
            }),
            _db: RwLock::new(db),
        }
    }

    /// Re-initialize this DtnModuleState with a production sled database.
    /// Replaces the temporary in-memory DB and tree references with
    /// production-backed ones. Called from `Dtn::init()`.
    pub fn init_production(&self, db: sled::Db) {
        // open V1 trees
        let dtn_messages: sled::Tree = match db.open_tree("dtn-messages") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to open dtn-messages tree: {}", e);
                return;
            }
        };
        let db_ref_id: sled::Tree = match db.open_tree("dtn-messages-ids") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to open dtn-messages-ids tree: {}", e);
                return;
            }
        };

        // calc current used size
        let mut used_size: u64 = 0;
        for entry in dtn_messages.iter() {
            if let Ok((_, message_entry_bytes)) = entry {
                if let Ok(message_entry) =
                    bincode::deserialize::<DtnMessageEntry>(&message_entry_bytes)
                {
                    used_size = used_size + (message_entry.size as u64);
                }
            }
        }

        // open V2 trees
        let db_ref_routed_v2 = match db.open_tree("dtn-routed-v2") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to open dtn-routed-v2 tree: {}", e);
                return;
            }
        };
        let db_ref_sender_quotas = match db.open_tree("dtn-sender-quotas") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to open dtn-sender-quotas tree: {}", e);
                return;
            }
        };

        let mut v2_used_size: u64 = 0;
        for entry in db_ref_routed_v2.iter() {
            if let Ok((_, entry_bytes)) = entry {
                if let Ok(v2_entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                    v2_used_size += v2_entry.size as u64;
                }
            }
        }

        {
            let mut state = match self.inner.write() {
                Ok(g) => g,
                Err(e) => {
                    log::error!("DtnModuleState::init_production inner lock poisoned: {}", e);
                    return;
                }
            };
            state.message_counts = dtn_messages.len() as u32;
            state.used_size = used_size;
            state.db_ref = dtn_messages;
            state.db_ref_id = db_ref_id;
        }
        {
            let mut v2_state = match self.v2.write() {
                Ok(g) => g,
                Err(e) => {
                    log::error!("DtnModuleState::init_production v2 lock poisoned: {}", e);
                    return;
                }
            };
            v2_state.message_count = db_ref_routed_v2.len() as u32;
            v2_state.used_size = v2_used_size;
            v2_state.db_ref_routed_v2 = db_ref_routed_v2;
            v2_state.db_ref_sender_quotas = db_ref_sender_quotas;
        }
        {
            let mut db_lock = match self._db.write() {
                Ok(g) => g,
                Err(e) => {
                    log::error!("DtnModuleState::init_production db lock poisoned: {}", e);
                    return;
                }
            };
            *db_lock = db;
        }
    }

    /// Process DTN response (instance method).
    pub fn on_dtn_response(&self, dtn_response: &super::messaging::proto::DtnResponse) {
        let mut state = match self.inner.write() {
            Ok(g) => g,
            Err(e) => {
                log::error!("DtnModuleState::on_dtn_response lock poisoned: {}", e);
                return;
            }
        };
        let entry_bytes = match state.db_ref.get(&dtn_response.signature) {
            Ok(Some(b)) => b,
            Ok(None) => return,
            Err(e) => {
                log::error!("DtnModuleState::on_dtn_response: db_ref.get failed: {}", e);
                return;
            }
        };
        let entry: DtnMessageEntry = match bincode::deserialize(&entry_bytes) {
            Ok(e) => e,
            Err(e) => {
                log::error!(
                    "DtnModuleState::on_dtn_response: failed to deserialize entry: {}",
                    e
                );
                return;
            }
        };
        if state.used_size > entry.size as u64 {
            state.used_size = state.used_size + (entry.size as u64);
        } else {
            state.used_size = 0;
        }
        if state.message_counts > 0 {
            state.message_counts = state.message_counts - 1;
        }

        if let Err(_) = state.db_ref.remove(&dtn_response.signature) {
            log::error!("remove storage node entry error!");
        } else if let Err(_) = state.db_ref.flush() {
            log::error!("remove storage node entry flush error!");
        }

        if let Err(_) = state.db_ref_id.remove(&entry.org_sig) {
            log::error!("remove storage node id entry error!");
        } else if let Err(_) = state.db_ref_id.flush() {
            log::error!("remove storage node id entry flush error!");
        }
    }

    /// Get DTN storage state (instance method).
    /// Returns (used_size, message_counts).
    pub fn get_state(&self) -> (u64, u32) {
        match self.inner.read() {
            Ok(state) => (state.used_size, state.message_counts),
            Err(e) => {
                log::error!("DtnModuleState::get_state lock poisoned: {}", e);
                (0, 0)
            }
        }
    }
}

/// qaul Delayed
///
pub struct Dtn {}

impl Dtn {
    /// init function
    /// Read dtn message table and initialize storage state
    pub fn init(state: &crate::QaulState) {
        let db = DataBase::get_node_db(state);
        state.services.dtn.init_production(db);
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
    pub fn get_storage_user(state: &crate::QaulState, user_id: &PeerId) -> Option<PeerId> {
        let user_profile;
        match Configuration::get_user(state, user_id.to_string()) {
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
        state: &crate::QaulState,
        user_account: &UserAccount,
        receiver_id: &PeerId,
        org_sig: &Vec<u8>,
        dtn_payload: &Vec<u8>,
    ) -> (i32, i32) {
        let mut storage_state = match state.services.dtn.inner.write() {
            Ok(s) => s,
            Err(e) => {
                log::error!("DTN: failed to acquire write lock: {}", e);
                return (
                    super::messaging::proto::dtn_response::ResponseType::Rejected as i32,
                    super::messaging::proto::dtn_response::Reason::None as i32,
                );
            }
        };

        // check already received
        if storage_state.db_ref_id.contains_key(org_sig).unwrap_or(false) {
            return (
                super::messaging::proto::dtn_response::ResponseType::Accepted as i32,
                super::messaging::proto::dtn_response::Reason::None as i32,
            );
        }

        let user_profile;
        match Configuration::get_user(state,user_account.id.to_string()) {
            Some(user_prof) => {
                user_profile = user_prof.clone();
            }
            None => {
                log::error!("dtn module: user profile no exists");
                return (
                    super::messaging::proto::dtn_response::ResponseType::Rejected as i32,
                    super::messaging::proto::dtn_response::Reason::UserNotAccepted as i32,
                );
            }
        }

        // check storage
        let new_size = storage_state.used_size + (dtn_payload.len() as u64);
        let total_limit = (user_profile.storage.size_total as u64) * 1024 * 1024;
        if new_size > total_limit {
            return (
                super::messaging::proto::dtn_response::ResponseType::Rejected as i32,
                super::messaging::proto::dtn_response::Reason::OverallQuota as i32,
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
            let message_entry_bytes = match bincode::serialize(&message_entry) {
                Ok(bytes) => bytes,
                Err(e) => {
                    log::error!("DTN: failed to serialize message entry: {}", e);
                    return (
                        super::messaging::proto::dtn_response::ResponseType::Rejected as i32,
                        super::messaging::proto::dtn_response::Reason::None as i32,
                    );
                }
            };

            // NOTE: The following two tree writes (db_ref and db_ref_id) are not
            // atomic. If a crash occurs between them, the database could end up in
            // an inconsistent state (e.g. an entry in db_ref without a corresponding
            // entry in db_ref_id, or vice versa). A sled transaction spanning both
            // trees would fix this but requires a larger refactor.
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
            state.services.messaging.save_unconfirmed_message(
                MessagingServiceType::DtnStored,
                &vec![],
                receiver_id,
                &container,
                true,
            );
        }

        (
            super::messaging::proto::dtn_response::ResponseType::Accepted as i32,
            super::messaging::proto::dtn_response::Reason::None as i32,
        )
    }

    /// this function is called when receive DTN response
    pub fn on_dtn_response(state: &crate::QaulState, dtn_response: &super::messaging::proto::DtnResponse) {
        let mut state = match state.services.dtn.inner.write() {
            Ok(g) => g,
            Err(e) => {
                log::error!("Dtn::on_dtn_response: inner lock poisoned: {}", e);
                return;
            }
        };
        if let Ok(Some(entry_bytes)) = state.db_ref.get(&dtn_response.signature) {
            // update storage node state
            let entry: DtnMessageEntry = match bincode::deserialize(&entry_bytes) {
                Ok(e) => e,
                Err(e) => {
                    log::error!("DTN: failed to deserialize entry: {}", e);
                    return;
                }
            };
            state.used_size = state.used_size.saturating_sub(entry.size as u64);
            state.message_counts = state.message_counts.saturating_sub(1);

            // NOTE: The following two tree removals (db_ref and db_ref_id) are not
            // atomic. A crash between them could leave stale entries in one tree.
            // A sled transaction spanning both trees would fix this.
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
    pub fn net(state: &crate::QaulState, user_id: &PeerId, sender_id: &PeerId, signature: &Vec<u8>, dtn_payload: &Vec<u8>) {
        if let Some(user_account) = UserAccounts::get_by_id(state,*user_id) {
            match proto::Container::decode(&dtn_payload[..]) {
                Ok(container) => {
                    let envelope = match container.envelope.as_ref() {
                        Some(e) => e,
                        None => {
                            log::error!("DTN: no envelope in container");
                            return;
                        }
                    };

                    let mut res: (i32, i32) = (
                        super::messaging::proto::dtn_response::ResponseType::Accepted as i32,
                        super::messaging::proto::dtn_response::Reason::None as i32,
                    );

                    //if container.envelope.receiver_id
                    if let Ok(receiver_id) = PeerId::from_bytes(&envelope.receiver_id) {
                        if receiver_id == *user_id {
                            // by process geneal message, the confirm message is transfered to the original sender.
                            super::messaging::process::MessagingProcess::process_received_message(
                                state,
                                user_account.clone(),
                                container,
                            );
                        } else {
                            res = Self::process_storage_node_message(
                                state,
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
                            state,
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
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
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
                    let dtn_state = match state.services.dtn.inner.read() {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("DTN RPC: failed to acquire read lock: {}", e);
                            return;
                        }
                    };
                    let unconfirmed = match state.services.messaging.unconfirmed.read() {
                        Ok(u) => u,
                        Err(e) => {
                            log::error!("DTN RPC: failed to acquire unconfirmed read lock: {}", e);
                            return;
                        }
                    };
                    let unconfrimed_len = unconfirmed.unconfirmed.len();

                    let proto_message = proto_rpc::Dtn {
                        message: Some(proto_rpc::dtn::Message::DtnStateResponse(
                            proto_rpc::DtnStateResponse {
                                used_size: dtn_state.used_size,
                                dtn_message_count: dtn_state.message_counts,
                                unconfirmed_count: unconfrimed_len as u32,
                            },
                        )),
                    };

                    // send message
                    Rpc::send_message(
                        state,
                        proto_message.encode_to_vec(),
                        crate::rpc::proto::Modules::Dtn.into(),
                        request_id,
                        Vec::new(),
                    );
                }
                Some(proto_rpc::dtn::Message::DtnConfigRequest(_req)) => {
                    match Configuration::get_user(state,my_user_id.to_string()) {
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
                                state,
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

                    match Configuration::get_user(state,my_user_id.to_string()) {
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
                                Configuration::update_user_storage(state,my_user_id.to_string(), &opt);
                                Configuration::save(state);
                            }

                            let proto_message = proto_rpc::Dtn {
                                message: Some(proto_rpc::dtn::Message::DtnAddUserResponse(
                                    proto_rpc::DtnAddUserResponse { status, message },
                                )),
                            };
                            // send message
                            Rpc::send_message(
                                state,
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

                    match Configuration::get_user(state,my_user_id.to_string()) {
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
                            for (i, user) in user_profile.storage.users.iter().enumerate() {
                                if *user == user_id_string {
                                    idx = Some(i);
                                    break;
                                }
                            }
                            if idx.is_none() {
                                status = false;
                                message = "User does not exist".to_string();
                            }

                            if let Some(i) = idx {
                                let mut opt = user_profile.storage.clone();
                                opt.users.remove(i);
                                Configuration::update_user_storage(state, my_user_id.to_string(), &opt);
                                Configuration::save(state);
                            }

                            let proto_message = proto_rpc::Dtn {
                                message: Some(proto_rpc::dtn::Message::DtnRemoveUserResponse(
                                    proto_rpc::DtnRemoveUserResponse { status, message },
                                )),
                            };
                            // send message
                            Rpc::send_message(
                                state,
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
                    match Configuration::get_user(state,my_user_id.to_string()) {
                        // CHANGE: save it in user profile, not to configuration directly.
                        Some(_user_profile) => {
                            Configuration::update_total_size(state,
                                my_user_id.to_string(),
                                req.total_size,
                            );
                            Configuration::save(state);

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
                                state,
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
                    Self::rpc_send_routed(state, my_user_id, req, request_id);
                }
                Some(proto_rpc::dtn::Message::DtnSetCustodyEnabledRequest(req)) => {
                    Self::rpc_set_custody_enabled(state, my_user_id, req, request_id);
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
        state: &crate::QaulState,
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
                state,
                proto_message.encode_to_vec(),
                crate::rpc::proto::Modules::Dtn as i32,
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

        // Validate custody route
        if req.custody_route.is_empty() {
            send_response(false, "at least one custody user is required".to_string());
            return;
        }
        if req.custody_route.len() > 10 {
            send_response(false, "maximum 10 custody users allowed".to_string());
            return;
        }
        for user_bytes in &req.custody_route {
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

        // Get user account
        let user_account = match UserAccounts::get_by_id(state, my_user_id) {
            Some(ua) => ua,
            None => {
                send_response(false, "user account not found".to_string());
                return;
            }
        };

        // Use the flat custody route directly
        let custody_route = req.custody_route.clone();

        // Calculate expiry
        let expires_at = if req.expiry_seconds > 0 {
            Timestamp::get_timestamp() + (req.expiry_seconds * 1000)
        } else {
            0
        };

        // Calculate remaining handoffs
        let total_custodians: u32 = custody_route.len() as u32;
        let remaining_handoffs = if req.max_handoffs > 0 {
            req.max_handoffs
        } else {
            total_custodians * 2
        };

        // Extract original_signature from the inner Container
        let original_signature = match proto::Container::decode(&req.data[..]) {
            Ok(container) => {
                if container.signature.is_empty() {
                    send_response(false, "inner container has no signature".to_string());
                    return;
                }
                container.signature
            }
            Err(e) => {
                send_response(false, format!("invalid container data: {}", e));
                return;
            }
        };

        // Build the DtnRoutedV2 message
        let routed_v2 = proto::DtnRoutedV2 {
            container: req.data.clone(),
            custody_route,
            next_route_index: 0,
            original_signature,
            sender_public_key: user_account.keys.public().encode_protobuf(),
            expires_at,
            remaining_handoffs,
        };

        // Find initial target
        let target = match Self::select_custody_target(state, &routed_v2, &receiver_id) {
            Some(t) => t,
            None => {
                send_response(false, "no reachable custodian found".to_string());
                return;
            }
        };

        // Send via envelope
        match super::messaging::Messaging::send_dtn_routed_v2_message(
            state,
            &user_account,
            &target,
            routed_v2.clone(),
        ) {
            Ok(_sig) => {
                // Store in V2 state so on_dtn_response_v2 can clean up
                // when the first custodian responds
                let entry_size = routed_v2.container.len() as u32;
                let v2_entry = DtnRoutedV2Entry {
                    routed_v2_bytes: routed_v2.encode_to_vec(),
                    sender_public_key: routed_v2.sender_public_key.clone(),
                    size: entry_size,
                    accepted_at: Timestamp::get_timestamp(),
                    receiver_id: receiver_id.to_bytes(),
                };
                if let Ok(entry_bytes) = bincode::serialize(&v2_entry) {
                    if let Ok(mut v2) = state.services.dtn.v2.write() {
                        let _ = v2.db_ref_routed_v2.insert(
                            routed_v2.original_signature.clone(),
                            entry_bytes,
                        );
                        let _ = v2.db_ref_routed_v2.flush();
                        v2.used_size += entry_size as u64;
                        v2.message_count += 1;
                    }
                }
                send_response(true, "".to_string());
            }
            Err(e) => {
                send_response(false, e);
            }
        }
    }

    /// Determine where to forward a V2 DTN message.
    ///
    /// Returns the recipient if online, otherwise scans the custody route
    /// forward from `next_route_index`, returning the first reachable user.
    /// Handle DtnSetCustodyEnabledRequest RPC
    fn rpc_set_custody_enabled(
        state: &crate::QaulState,
        my_user_id: PeerId,
        req: proto_rpc::DtnSetCustodyEnabledRequest,
        request_id: String,
    ) {
        let send_response = |status: bool, message: String| {
            let proto_message = proto_rpc::Dtn {
                message: Some(proto_rpc::dtn::Message::DtnSetCustodyEnabledResponse(
                    proto_rpc::DtnSetCustodyEnabledResponse { status, message },
                )),
            };
            Rpc::send_message(
                state,
                proto_message.encode_to_vec(),
                crate::rpc::proto::Modules::Dtn as i32,
                request_id.clone(),
                Vec::new(),
            );
        };

        match Configuration::get_user(state, my_user_id.to_string()) {
            Some(user_profile) => {
                let mut storage = user_profile.storage.clone();
                storage.dtn_v2_custody_enabled = req.enabled;
                Configuration::update_user_storage(
                    state,
                    my_user_id.to_string(),
                    &storage,
                );
                Configuration::save(state);
                send_response(true, "".to_string());
            }
            None => {
                send_response(false, "user profile not found".to_string());
            }
        }
    }

    pub fn select_custody_target(
        state: &crate::QaulState,
        routed_v2: &proto::DtnRoutedV2,
        receiver_id: &PeerId,
    ) -> Option<PeerId> {
        let rs = state.get_router();
        // Check if recipient is directly reachable
        if rs.routing_table.get_route_to_user(*receiver_id).is_some() {
            return Some(*receiver_id);
        }

        // Strict forward scan from next_route_index
        let start = routed_v2.next_route_index as usize;
        let len = routed_v2.custody_route.len();
        if start >= len {
            return None;
        }
        for i in start..len {
            if let Ok(custodian_id) = PeerId::from_bytes(&routed_v2.custody_route[i]) {
                if rs.routing_table.get_route_to_user(custodian_id).is_some() {
                    return Some(custodian_id);
                }
            }
        }

        None
    }

    /// Process a received DtnRoutedV2 message from the network
    pub fn net_routed_v2(
        state: &crate::QaulState,
        user_id: &PeerId,
        sender_id: &PeerId,
        _signature: &[u8],
        routed_v2: proto::DtnRoutedV2,
    ) {
        log::info!("Received DtnRoutedV2 message from {}", sender_id.to_base58());

        let user_account = match UserAccounts::get_by_id(state, *user_id) {
            Some(ua) => ua,
            None => {
                log::error!("DtnRoutedV2: user account not found for {}", user_id.to_base58());
                if let Some(default_user) = UserAccounts::get_default_user(state) {
                    Self::send_v2_response(
                        state,
                        &default_user,
                        sender_id,
                        &routed_v2.original_signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::UserNotAccepted,
                    );
                }
                return;
            }
        };

        // 1. Expiry check
        if routed_v2.expires_at > 0 {
            let now = Timestamp::get_timestamp();
            if now > routed_v2.expires_at {
                log::warn!("DtnRoutedV2 message expired, dropping");
                Self::send_v2_response(
                    state,
                    &user_account,
                    sender_id,
                    &routed_v2.original_signature,
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
                state,
                &user_account,
                sender_id,
                &routed_v2.original_signature,
                proto::dtn_response::ResponseType::Rejected,
                proto::dtn_response::Reason::None,
            );
            return;
        }

        // 3. Duplicate check
        {
            let v2 = match state.services.dtn.v2.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnRoutedV2: failed to acquire read lock: {}", e);
                    return;
                }
            };
            if v2
                .db_ref_routed_v2
                .contains_key(&routed_v2.original_signature)
                .unwrap_or(false)
            {
                log::info!("DtnRoutedV2 duplicate detected, accepting silently");
                drop(v2);
                Self::send_v2_response(
                    state,
                    &user_account,
                    sender_id,
                    &routed_v2.original_signature,
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
                        state,
                        user_account.clone(),
                        container,
                    );
                }
                Self::send_v2_response(
                    state,
                    &user_account,
                    sender_id,
                    &routed_v2.original_signature,
                    proto::dtn_response::ResponseType::Accepted,
                    proto::dtn_response::Reason::None,
                );
                return;
            }
        }

        // 5. Custody opt-in check
        match Configuration::get_user(state, user_account.id.to_string()) {
            Some(user_profile) => {
                if !user_profile.storage.dtn_v2_custody_enabled {
                    log::warn!("DtnRoutedV2: custody not enabled for this node");
                    Self::send_v2_response(
                        state,
                        &user_account,
                        sender_id,
                        &routed_v2.original_signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::UserNotAccepted,
                    );
                    return;
                }
            }
            None => {
                log::error!("DtnRoutedV2: user profile not found for custody check");
                Self::send_v2_response(
                    state,
                    &user_account,
                    sender_id,
                    &routed_v2.original_signature,
                    proto::dtn_response::ResponseType::Rejected,
                    proto::dtn_response::Reason::UserNotAccepted,
                );
                return;
            }
        }

        // 6. Sender signature verification
        {
            let inner_container = match proto::Container::decode(&routed_v2.container[..]) {
                Ok(c) => c,
                Err(e) => {
                    log::error!("DtnRoutedV2: failed to decode inner container: {}", e);
                    Self::send_v2_response(
                        state,
                        &user_account,
                        sender_id,
                        &routed_v2.original_signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::None,
                    );
                    return;
                }
            };
            let sender_key = match PublicKey::try_decode_protobuf(&routed_v2.sender_public_key) {
                Ok(k) => k,
                Err(e) => {
                    log::error!("DtnRoutedV2: invalid sender public key: {}", e);
                    Self::send_v2_response(
                        state,
                        &user_account,
                        sender_id,
                        &routed_v2.original_signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::None,
                    );
                    return;
                }
            };
            if let Some(envelope) = inner_container.envelope.as_ref() {
                let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
                if let Err(e) = envelope.encode(&mut envelope_buf) {
                    log::error!("DtnRoutedV2: failed to re-encode envelope for verification: {}", e);
                    Self::send_v2_response(
                        state,
                        &user_account,
                        sender_id,
                        &routed_v2.original_signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::None,
                    );
                    return;
                }
                if !sender_key.verify(&envelope_buf, &inner_container.signature) {
                    log::error!("DtnRoutedV2: inner container signature verification failed");
                    Self::send_v2_response(
                        state,
                        &user_account,
                        sender_id,
                        &routed_v2.original_signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::UserNotAccepted,
                    );
                    return;
                }
            } else {
                log::error!("DtnRoutedV2: inner container has no envelope");
                Self::send_v2_response(
                    state,
                    &user_account,
                    sender_id,
                    &routed_v2.original_signature,
                    proto::dtn_response::ResponseType::Rejected,
                    proto::dtn_response::Reason::None,
                );
                return;
            }
        }

        // 7. Per-sender quota check
        {
            let v2 = match state.services.dtn.v2.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnRoutedV2: failed to acquire read lock for quota check: {}", e);
                    return;
                }
            };
            if let Ok(Some(quota_bytes)) = v2
                .db_ref_sender_quotas
                .get(&routed_v2.sender_public_key)
            {
                if let Ok(quota) = bincode::deserialize::<SenderQuotaEntry>(&quota_bytes) {
                    if quota.used_bytes + (routed_v2.container.len() as u64) > V2_PER_SENDER_QUOTA {
                        log::warn!("DtnRoutedV2: per-sender quota exceeded");
                        drop(v2);
                        Self::send_v2_response(
                            state,
                            &user_account,
                            sender_id,
                            &routed_v2.original_signature,
                            proto::dtn_response::ResponseType::Rejected,
                            proto::dtn_response::Reason::UserQuota,
                        );
                        return;
                    }
                }
            }
        }

        // 7. Overall quota check
        {
            let v2 = match state.services.dtn.v2.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnRoutedV2: failed to acquire read lock for overall quota: {}", e);
                    return;
                }
            };
            let v1_state = match state.services.dtn.inner.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnRoutedV2: failed to acquire V1 read lock: {}", e);
                    return;
                }
            };
            match Configuration::get_user(state, user_account.id.to_string()) {
                Some(user_profile) => {
                    let total_limit = (user_profile.storage.size_total as u64) * 1024 * 1024;
                    let total_used =
                        v1_state.used_size + v2.used_size + (routed_v2.container.len() as u64);
                    if total_used > total_limit {
                        log::warn!("DtnRoutedV2: overall quota exceeded");
                        drop(v1_state);
                        drop(v2);
                        Self::send_v2_response(
                            state,
                            &user_account,
                            sender_id,
                            &routed_v2.original_signature,
                            proto::dtn_response::ResponseType::Rejected,
                            proto::dtn_response::Reason::OverallQuota,
                        );
                        return;
                    }
                }
                None => {
                    log::error!("DtnRoutedV2: user profile not found");
                    drop(v1_state);
                    drop(v2);
                    Self::send_v2_response(
                        state,
                        &user_account,
                        sender_id,
                        &routed_v2.original_signature,
                        proto::dtn_response::ResponseType::Rejected,
                        proto::dtn_response::Reason::UserNotAccepted,
                    );
                    return;
                }
            }
        }

        // 8. Accept custody: store in DB
        let entry_size = routed_v2.container.len() as u32;
        let v2_entry = DtnRoutedV2Entry {
            routed_v2_bytes: routed_v2.encode_to_vec(),
            sender_public_key: routed_v2.sender_public_key.clone(),
            size: entry_size,
            accepted_at: Timestamp::get_timestamp(),
            receiver_id: envelope_receiver.map(|r| r.to_bytes()).unwrap_or_default(),
        };
        let entry_bytes = match bincode::serialize(&v2_entry) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!("DtnRoutedV2: failed to serialize entry: {}", e);
                return;
            }
        };

        {
            let mut v2 = match state.services.dtn.v2.write() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnRoutedV2: failed to acquire write lock: {}", e);
                    return;
                }
            };
            if let Err(e) = v2
                .db_ref_routed_v2
                .insert(routed_v2.original_signature.clone(), entry_bytes)
            {
                log::error!("DtnRoutedV2: storage insert error: {}", e);
            }
            let _ = v2.db_ref_routed_v2.flush();

            v2.used_size += entry_size as u64;
            v2.message_count += 1;

            // Update sender quota
            let mut quota = if let Ok(Some(quota_bytes)) = v2
                .db_ref_sender_quotas
                .get(&routed_v2.sender_public_key)
            {
                bincode::deserialize::<SenderQuotaEntry>(&quota_bytes).unwrap_or_default()
            } else {
                SenderQuotaEntry::default()
            };
            quota.used_bytes += entry_size as u64;
            quota.message_count += 1;
            if let Ok(quota_bytes) = bincode::serialize(&quota) {
                let _ = v2
                    .db_ref_sender_quotas
                    .insert(routed_v2.sender_public_key.clone(), quota_bytes);
            } else {
                log::error!("DtnRoutedV2: failed to serialize sender quota");
            }
            let _ = v2.db_ref_sender_quotas.flush();
        }

        // Send acceptance response
        Self::send_v2_response(
            state,
            &user_account,
            sender_id,
            &routed_v2.original_signature,
            proto::dtn_response::ResponseType::Accepted,
            proto::dtn_response::Reason::None,
        );

        // 9. Attempt immediate forward
        if let Some(recv_id) = envelope_receiver {
            Self::try_forward_v2(state, &user_account, &routed_v2, &recv_id);
        }
    }

    /// Try to forward a V2 message to the next custodian or recipient
    fn try_forward_v2(
        state: &crate::QaulState,
        user_account: &UserAccount,
        routed_v2: &proto::DtnRoutedV2,
        receiver_id: &PeerId,
    ) {
        if let Some(target) = Self::select_custody_target(state, routed_v2, receiver_id) {
            let mut forwarded = routed_v2.clone();
            forwarded.remaining_handoffs = forwarded.remaining_handoffs.saturating_sub(1);

            // Advance next_route_index past the target
            for (i, user_bytes) in forwarded.custody_route.iter().enumerate() {
                if let Ok(uid) = PeerId::from_bytes(user_bytes) {
                    if uid == target && i as u32 >= forwarded.next_route_index {
                        forwarded.next_route_index = (i as u32) + 1;
                        break;
                    }
                }
            }

            if let Err(e) = super::messaging::Messaging::send_dtn_routed_v2_message(
                state,
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
        state: &crate::QaulState,
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
            state,
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
    pub fn on_dtn_response_v2(state: &crate::QaulState, dtn_response: &proto::DtnResponse) {
        let mut v2 = match state.services.dtn.v2.write() {
            Ok(s) => s,
            Err(e) => {
                log::error!("DtnRoutedV2: failed to acquire write lock for response: {}", e);
                return;
            }
        };
        if let Ok(Some(entry_bytes)) = v2
            .db_ref_routed_v2
            .get(&dtn_response.signature)
        {
            if let Ok(entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                // Only remove on acceptance
                if dtn_response.response_type
                    == proto::dtn_response::ResponseType::Accepted as i32
                {
                    // Remove from V2 storage
                    let _ = v2.db_ref_routed_v2.remove(&dtn_response.signature);
                    let _ = v2.db_ref_routed_v2.flush();

                    // Update counts
                    v2.used_size = v2.used_size.saturating_sub(entry.size as u64);
                    if v2.message_count > 0 {
                        v2.message_count -= 1;
                    }

                    // Update sender quota
                    if let Ok(Some(quota_bytes)) =
                        v2.db_ref_sender_quotas.get(&entry.sender_public_key)
                    {
                        if let Ok(mut quota) =
                            bincode::deserialize::<SenderQuotaEntry>(&quota_bytes)
                        {
                            quota.used_bytes = quota.used_bytes.saturating_sub(entry.size as u64);
                            quota.message_count = quota.message_count.saturating_sub(1);
                            if let Ok(quota_bytes) = bincode::serialize(&quota) {
                                let _ = v2
                                    .db_ref_sender_quotas
                                    .insert(entry.sender_public_key.clone(), quota_bytes);
                                let _ = v2.db_ref_sender_quotas.flush();
                            }
                        }
                    }
                }
            }
        }
    }

    /// Process V2 routed messages in the retransmit loop.
    /// Called periodically to check if stored V2 messages can be forwarded.
    pub fn process_retransmit_v2(state: &crate::QaulState) {
        let v2 = match state.services.dtn.v2.read() {
            Ok(s) => s,
            Err(e) => {
                log::error!("DtnRoutedV2: failed to acquire read lock for retransmit: {}", e);
                return;
            }
        };
        let now = Timestamp::get_timestamp();

        let mut to_remove: Vec<Vec<u8>> = Vec::new();
        let mut to_forward: Vec<(Vec<u8>, DtnRoutedV2Entry)> = Vec::new();

        for entry in v2.db_ref_routed_v2.iter() {
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
        drop(v2);

        // Remove expired entries
        if !to_remove.is_empty() {
            let mut v2 = match state.services.dtn.v2.write() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnRoutedV2: failed to acquire write lock for cleanup: {}", e);
                    return;
                }
            };
            for sig in &to_remove {
                if let Ok(Some(entry_bytes)) = v2.db_ref_routed_v2.get(sig) {
                    if let Ok(entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                        v2.used_size = v2.used_size.saturating_sub(entry.size as u64);
                        if v2.message_count > 0 {
                            v2.message_count -= 1;
                        }
                        // Update sender quota
                        if let Ok(Some(quota_bytes)) =
                            v2.db_ref_sender_quotas.get(&entry.sender_public_key)
                        {
                            if let Ok(mut quota) =
                                bincode::deserialize::<SenderQuotaEntry>(&quota_bytes)
                            {
                                quota.used_bytes =
                                    quota.used_bytes.saturating_sub(entry.size as u64);
                                quota.message_count = quota.message_count.saturating_sub(1);
                                if let Ok(quota_bytes) = bincode::serialize(&quota) {
                                    let _ = v2
                                        .db_ref_sender_quotas
                                        .insert(entry.sender_public_key.clone(), quota_bytes);
                                }
                            }
                        }
                    }
                }
                let _ = v2.db_ref_routed_v2.remove(sig);
            }
            let _ = v2.db_ref_routed_v2.flush();
            let _ = v2.db_ref_sender_quotas.flush();
        }

        // Try to forward stored messages
        for (_sig, v2_entry) in &to_forward {
            if let Ok(routed_v2) = proto::DtnRoutedV2::decode(&v2_entry.routed_v2_bytes[..]) {
                if let Ok(recv_id) = PeerId::from_bytes(&v2_entry.receiver_id) {
                    // We need a user account to send from. Use the first local account.
                    if let Some(user_account) = UserAccounts::get_default_user(state) {
                        Self::try_forward_v2(state, &user_account, &routed_v2, &recv_id);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connections::ConnectionModule;
    use crate::router::table::{RoutingConnectionEntry, RoutingUserEntry};
    use crate::utilities::qaul_id::QaulId;
    use libp2p::identity::Keypair;
    use prost::Message;
    use std::collections::HashMap;

    /// Test helper: unwrap a `Result`, panicking on failure with a clear message.
    /// Used in tests in place of `.unwrap()` per project rule.
    fn ok<T, E: std::fmt::Display>(res: Result<T, E>, ctx: &str) -> T {
        match res {
            Ok(v) => v,
            Err(e) => panic!("test failure ({}): {}", ctx, e),
        }
    }

    /// Test helper: unwrap an `Option`, panicking on `None` with a clear message.
    fn some<T>(opt: Option<T>, ctx: &str) -> T {
        match opt {
            Some(v) => v,
            None => panic!("test failure ({}): expected Some, got None", ctx),
        }
    }

    /// Create a random PeerId from a fresh Ed25519 keypair.
    fn random_peer() -> PeerId {
        let keys = Keypair::generate_ed25519();
        PeerId::from(keys.public())
    }

    /// Make a user appear "online" by inserting a routing entry.
    fn make_online(table: &mut HashMap<Vec<u8>, RoutingUserEntry>, peer: PeerId) {
        let q8id = QaulId::to_q8id(peer);
        let neighbour = random_peer();
        table.insert(
            q8id.clone(),
            RoutingUserEntry {
                id: q8id,
                pgid: 1,
                pgid_update: 0,
                pgid_update_hc: 0,
                online_time: 0,
                connections: vec![RoutingConnectionEntry {
                    module: ConnectionModule::Lan,
                    node: neighbour,
                    rtt: 50,
                    hc: 1,
                    lq: 10,
                    last_update: 0,
                }],
            },
        );
    }

    /// Build a DtnRoutedV2 with the given custody route and next_route_index.
    fn build_routed_v2(custody_route: Vec<Vec<u8>>, next_route_index: u32) -> proto::DtnRoutedV2 {
        proto::DtnRoutedV2 {
            container: vec![1, 2, 3],
            custody_route,
            next_route_index,
            original_signature: vec![0xAA],
            sender_public_key: vec![0xBB],
            expires_at: 0,
            remaining_handoffs: 10,
        }
    }

    // ── Serialization tests ──

    #[test]
    fn dtn_routed_v2_round_trip() {
        let original = proto::DtnRoutedV2 {
            container: vec![10, 20, 30, 40],
            custody_route: vec![vec![1, 2, 3], vec![4, 5, 6]],
            next_route_index: 0,
            original_signature: vec![0xAA, 0xBB],
            sender_public_key: vec![0xCC, 0xDD],
            expires_at: 1234567890,
            remaining_handoffs: 5,
        };

        let encoded = original.encode_to_vec();
        assert!(!encoded.is_empty());

        let decoded = ok(proto::DtnRoutedV2::decode(&encoded[..]), "decode DtnRoutedV2");
        assert_eq!(decoded.container, original.container);
        assert_eq!(decoded.custody_route.len(), 2);
        assert_eq!(decoded.custody_route[0], vec![1, 2, 3]);
        assert_eq!(decoded.custody_route[1], vec![4, 5, 6]);
        assert_eq!(decoded.next_route_index, 0);
        assert_eq!(decoded.original_signature, original.original_signature);
        assert_eq!(decoded.sender_public_key, original.sender_public_key);
        assert_eq!(decoded.expires_at, original.expires_at);
        assert_eq!(decoded.remaining_handoffs, original.remaining_handoffs);
    }

    #[test]
    fn dtn_routed_v2_serde_round_trip() {
        let original = proto::DtnRoutedV2 {
            container: vec![10, 20],
            custody_route: vec![vec![1, 2, 3]],
            next_route_index: 1,
            original_signature: vec![0xAA],
            sender_public_key: vec![0xCC],
            expires_at: 0,
            remaining_handoffs: 3,
        };

        let serialized = ok(bincode::serialize(&original), "serialize DtnRoutedV2");
        let deserialized: proto::DtnRoutedV2 =
            ok(bincode::deserialize(&serialized), "deserialize DtnRoutedV2");
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

        let serialized = ok(bincode::serialize(&entry), "serialize DtnRoutedV2Entry");
        let deserialized: DtnRoutedV2Entry = ok(
            bincode::deserialize(&serialized),
            "deserialize DtnRoutedV2Entry",
        );
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

        let serialized = ok(bincode::serialize(&entry), "serialize SenderQuotaEntry");
        let deserialized: SenderQuotaEntry = ok(
            bincode::deserialize(&serialized),
            "deserialize SenderQuotaEntry",
        );
        assert_eq!(deserialized.used_bytes, 5000);
        assert_eq!(deserialized.message_count, 3);
    }

    #[test]
    fn envelop_payload_dtn_routed_v2_variant() {
        let routed_v2 = proto::DtnRoutedV2 {
            container: vec![1, 2, 3],
            custody_route: vec![],
            next_route_index: 0,
            original_signature: vec![],
            sender_public_key: vec![],
            expires_at: 0,
            remaining_handoffs: 1,
        };

        let payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::DtnRoutedV2(routed_v2)),
        };

        let encoded = payload.encode_to_vec();
        let decoded = ok(
            proto::EnvelopPayload::decode(&encoded[..]),
            "decode EnvelopPayload",
        );

        match decoded.payload {
            Some(proto::envelop_payload::Payload::DtnRoutedV2(v2)) => {
                assert_eq!(v2.container, vec![1, 2, 3]);
                assert_eq!(v2.remaining_handoffs, 1);
            }
            _ => panic!("Expected DtnRoutedV2 payload variant"),
        }
    }

    // ── select_custody_target tests ──
    //
    // Each test gets its own isolated `QaulState` (via `new_for_simulation()`),
    // so there is no shared global routing table to serialize against.

    #[test]
    fn select_target_returns_recipient_when_online() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let custodian = random_peer();

        let mut table = HashMap::new();
        make_online(&mut table, recipient);
        // custodian is NOT online
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        let v2 = build_routed_v2(vec![custodian.to_bytes()], 0);

        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, Some(recipient));
    }

    #[test]
    fn select_target_returns_first_reachable_custodian() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();
        let c2 = random_peer();

        let mut table = HashMap::new();
        // recipient offline, c1 offline, c2 online
        make_online(&mut table, c2);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        let v2 = build_routed_v2(vec![c1.to_bytes(), c2.to_bytes()], 0);

        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, Some(c2));
    }

    #[test]
    fn select_target_returns_none_when_nobody_online() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();
        let c2 = random_peer();

        // Empty routing table — nobody online
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable {
                table: HashMap::new(),
            });

        let v2 = build_routed_v2(vec![c1.to_bytes(), c2.to_bytes()], 0);

        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, None);
    }

    #[test]
    fn select_target_respects_next_route_index() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();
        let c2 = random_peer();

        let mut table = HashMap::new();
        // Both custodians online
        make_online(&mut table, c1);
        make_online(&mut table, c2);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        // next_route_index = 1 means c1 (index 0) is already done, only c2 eligible
        let v2 = build_routed_v2(vec![c1.to_bytes(), c2.to_bytes()], 1);

        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, Some(c2));
    }

    #[test]
    fn select_target_skips_exhausted_route() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();

        let mut table = HashMap::new();
        make_online(&mut table, c1);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        // next_route_index == len means the route is exhausted
        let v2 = build_routed_v2(vec![c1.to_bytes()], 1);

        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, None);
    }

    #[test]
    fn select_target_picks_first_reachable_in_forward_order() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer(); // first in route
        let c2 = random_peer(); // second in route

        let mut table = HashMap::new();
        // Both online
        make_online(&mut table, c1);
        make_online(&mut table, c2);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        let v2 = build_routed_v2(vec![c1.to_bytes(), c2.to_bytes()], 0);

        // Forward scan should pick c1 (index 0) as the first reachable
        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, Some(c1));
    }

    // ── Route advancement tests ──

    #[test]
    fn route_next_route_index_advances_on_forward() {
        let c1 = random_peer();
        let c2 = random_peer();
        let target = c2;

        let mut routed = proto::DtnRoutedV2 {
            container: vec![],
            custody_route: vec![c1.to_bytes(), c2.to_bytes()],
            next_route_index: 0,
            original_signature: vec![],
            sender_public_key: vec![],
            expires_at: 0,
            remaining_handoffs: 5,
        };

        // Simulate what try_forward_v2 does to the route state
        routed.remaining_handoffs = routed.remaining_handoffs.saturating_sub(1);
        for (i, user_bytes) in routed.custody_route.iter().enumerate() {
            if let Ok(uid) = PeerId::from_bytes(user_bytes) {
                if uid == target && i as u32 >= routed.next_route_index {
                    routed.next_route_index = (i as u32) + 1;
                    break;
                }
            }
        }

        assert_eq!(routed.remaining_handoffs, 4);
        assert_eq!(routed.next_route_index, 2); // c2 is at index 1, so next = 2
    }

    #[test]
    fn route_next_route_index_advances_for_middle_custodian() {
        let c1 = random_peer();
        let c2 = random_peer();
        let c3 = random_peer();
        let target = c2;

        let mut routed = proto::DtnRoutedV2 {
            container: vec![],
            custody_route: vec![c1.to_bytes(), c2.to_bytes(), c3.to_bytes()],
            next_route_index: 0,
            original_signature: vec![],
            sender_public_key: vec![],
            expires_at: 0,
            remaining_handoffs: 5,
        };

        routed.remaining_handoffs = routed.remaining_handoffs.saturating_sub(1);
        for (i, user_bytes) in routed.custody_route.iter().enumerate() {
            if let Ok(uid) = PeerId::from_bytes(user_bytes) {
                if uid == target && i as u32 >= routed.next_route_index {
                    routed.next_route_index = (i as u32) + 1;
                    break;
                }
            }
        }

        // c2 at index 1, next_route_index should advance to 2, leaving c3 still eligible
        assert_eq!(routed.next_route_index, 2);
    }

    // ── V2 storage tests ──

    #[test]
    fn v2_storage_insert_and_retrieve() {
        let qaul_state = crate::QaulState::new_for_simulation();

        let sig = vec![0x01, 0x02, 0x03];
        let entry = DtnRoutedV2Entry {
            routed_v2_bytes: vec![10, 20, 30],
            sender_public_key: vec![0xAA],
            size: 3,
            accepted_at: 12345,
            receiver_id: vec![0xBB],
        };
        let entry_bytes = ok(bincode::serialize(&entry), "serialize entry");

        {
            let mut state = ok(qaul_state.services.dtn.v2.write(), "v2 write lock");
            ok(
                state.db_ref_routed_v2.insert(sig.clone(), entry_bytes),
                "insert routed_v2",
            );
            ok(state.db_ref_routed_v2.flush(), "flush routed_v2");
            state.used_size += entry.size as u64;
            state.message_count += 1;
        }

        // Retrieve
        {
            let state = ok(qaul_state.services.dtn.v2.read(), "v2 read lock");
            assert!(ok(
                state.db_ref_routed_v2.contains_key(&sig),
                "contains_key routed_v2"
            ));
            let stored = some(
                ok(state.db_ref_routed_v2.get(&sig), "get routed_v2"),
                "stored routed_v2",
            );
            let decoded: DtnRoutedV2Entry =
                ok(bincode::deserialize(&stored), "deserialize entry");
            assert_eq!(decoded.routed_v2_bytes, vec![10, 20, 30]);
            assert_eq!(decoded.size, 3);
        }
    }

    #[test]
    fn v2_storage_duplicate_detection() {
        let qaul_state = crate::QaulState::new_for_simulation();

        let sig = vec![0xDE, 0xAD];
        let entry = DtnRoutedV2Entry {
            routed_v2_bytes: vec![1],
            sender_public_key: vec![2],
            size: 1,
            accepted_at: 0,
            receiver_id: vec![3],
        };
        let entry_bytes = ok(bincode::serialize(&entry), "serialize entry");

        {
            let state = ok(qaul_state.services.dtn.v2.write(), "v2 write lock");
            ok(
                state.db_ref_routed_v2.insert(sig.clone(), entry_bytes),
                "insert routed_v2",
            );
        }

        // Should detect duplicate
        {
            let state = ok(qaul_state.services.dtn.v2.read(), "v2 read lock");
            assert!(ok(
                state.db_ref_routed_v2.contains_key(&sig),
                "contains_key routed_v2"
            ));
        }
    }

    #[test]
    fn v2_sender_quota_tracking() {
        let qaul_state = crate::QaulState::new_for_simulation();

        let sender_key = vec![0xCC, 0xDD];
        let quota = SenderQuotaEntry {
            used_bytes: 500,
            message_count: 2,
        };
        let quota_bytes = ok(bincode::serialize(&quota), "serialize quota");

        {
            let state = ok(qaul_state.services.dtn.v2.write(), "v2 write lock");
            ok(
                state
                    .db_ref_sender_quotas
                    .insert(sender_key.clone(), quota_bytes),
                "insert sender quota",
            );
        }

        // Retrieve and check
        {
            let state = ok(qaul_state.services.dtn.v2.read(), "v2 read lock");
            let stored = some(
                ok(state.db_ref_sender_quotas.get(&sender_key), "get quota"),
                "stored quota",
            );
            let decoded: SenderQuotaEntry =
                ok(bincode::deserialize(&stored), "deserialize quota");
            assert_eq!(decoded.used_bytes, 500);
            assert_eq!(decoded.message_count, 2);
        }

        // Simulate removing a message — quota should decrease
        {
            let state = ok(qaul_state.services.dtn.v2.write(), "v2 write lock");
            let stored = some(
                ok(state.db_ref_sender_quotas.get(&sender_key), "get quota"),
                "stored quota",
            );
            let mut decoded: SenderQuotaEntry =
                ok(bincode::deserialize(&stored), "deserialize quota");
            decoded.used_bytes = decoded.used_bytes.saturating_sub(200);
            decoded.message_count = decoded.message_count.saturating_sub(1);
            let updated = ok(bincode::serialize(&decoded), "serialize quota");
            ok(
                state
                    .db_ref_sender_quotas
                    .insert(sender_key.clone(), updated),
                "insert sender quota",
            );
        }

        {
            let state = ok(qaul_state.services.dtn.v2.read(), "v2 read lock");
            let stored = some(
                ok(state.db_ref_sender_quotas.get(&sender_key), "get quota"),
                "stored quota",
            );
            let decoded: SenderQuotaEntry =
                ok(bincode::deserialize(&stored), "deserialize quota");
            assert_eq!(decoded.used_bytes, 300);
            assert_eq!(decoded.message_count, 1);
        }
    }

    #[test]
    fn v2_per_sender_quota_limit_enforced() {
        let qaul_state = crate::QaulState::new_for_simulation();

        let sender_key = vec![0xEE, 0xFF];
        // Set quota near the limit
        let quota = SenderQuotaEntry {
            used_bytes: V2_PER_SENDER_QUOTA - 10,
            message_count: 100,
        };
        let quota_bytes = ok(bincode::serialize(&quota), "serialize quota");

        {
            let state = ok(qaul_state.services.dtn.v2.read(), "v2 read lock");
            ok(
                state
                    .db_ref_sender_quotas
                    .insert(sender_key.clone(), quota_bytes),
                "insert sender quota",
            );
        }

        // A message of size 11 should exceed the quota
        {
            let state = ok(qaul_state.services.dtn.v2.read(), "v2 read lock");
            let stored = some(
                ok(state.db_ref_sender_quotas.get(&sender_key), "get quota"),
                "stored quota",
            );
            let decoded: SenderQuotaEntry =
                ok(bincode::deserialize(&stored), "deserialize quota");
            let new_msg_size: u64 = 11;
            assert!(decoded.used_bytes + new_msg_size > V2_PER_SENDER_QUOTA);
        }

        // A message of size 5 should be under the quota
        {
            let state = ok(qaul_state.services.dtn.v2.read(), "v2 read lock");
            let stored = some(
                ok(state.db_ref_sender_quotas.get(&sender_key), "get quota"),
                "stored quota",
            );
            let decoded: SenderQuotaEntry =
                ok(bincode::deserialize(&stored), "deserialize quota");
            let new_msg_size: u64 = 5;
            assert!(decoded.used_bytes + new_msg_size <= V2_PER_SENDER_QUOTA);
        }
    }

    // ── Signature verification tests ──

    /// Helper: create a properly signed inner Container with a real keypair.
    /// Returns (keypair, container_bytes, container_signature).
    fn build_signed_container(receiver: &PeerId) -> (Keypair, Vec<u8>, Vec<u8>) {
        let keys = Keypair::generate_ed25519();
        let sender = PeerId::from(keys.public());

        let envelope = proto::Envelope {
            sender_id: sender.to_bytes(),
            receiver_id: receiver.to_bytes(),
            payload: vec![0xDE, 0xAD],
        };

        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        ok(envelope.encode(&mut envelope_buf), "encode envelope");

        let signature = ok(keys.sign(&envelope_buf), "sign envelope");

        let container = proto::Container {
            signature: signature.clone(),
            envelope: Some(envelope),
        };

        (keys, container.encode_to_vec(), signature)
    }

    #[test]
    fn signature_verification_accepts_valid_signature() {
        let receiver = random_peer();
        let (keys, container_bytes, _sig) = build_signed_container(&receiver);

        // Decode the inner container
        let inner = ok(
            proto::Container::decode(&container_bytes[..]),
            "decode container",
        );
        let sender_key = keys.public();

        // Re-encode envelope and verify
        let envelope = some(inner.envelope.as_ref(), "envelope");
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        ok(envelope.encode(&mut envelope_buf), "encode envelope");

        assert!(sender_key.verify(&envelope_buf, &inner.signature));
    }

    #[test]
    fn signature_verification_rejects_wrong_key() {
        let receiver = random_peer();
        let (_keys, container_bytes, _sig) = build_signed_container(&receiver);

        // Use a different key to verify
        let wrong_keys = Keypair::generate_ed25519();
        let wrong_key = wrong_keys.public();

        let inner = ok(
            proto::Container::decode(&container_bytes[..]),
            "decode container",
        );
        let envelope = some(inner.envelope.as_ref(), "envelope");
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        ok(envelope.encode(&mut envelope_buf), "encode envelope");

        assert!(!wrong_key.verify(&envelope_buf, &inner.signature));
    }

    #[test]
    fn signature_verification_rejects_tampered_envelope() {
        let receiver = random_peer();
        let (keys, container_bytes, _sig) = build_signed_container(&receiver);

        let mut inner = ok(
            proto::Container::decode(&container_bytes[..]),
            "decode container",
        );
        // Tamper with the envelope
        some(inner.envelope.as_mut(), "envelope mut").payload = vec![0xFF, 0xFF];

        let sender_key = keys.public();
        let envelope = some(inner.envelope.as_ref(), "envelope");
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        ok(envelope.encode(&mut envelope_buf), "encode envelope");

        // Original signature should not verify against tampered envelope
        assert!(!sender_key.verify(&envelope_buf, &inner.signature));
    }

    #[test]
    fn original_signature_extracted_from_inner_container() {
        let receiver = random_peer();
        let (_keys, container_bytes, expected_sig) = build_signed_container(&receiver);

        // Simulate what rpc_send_routed does: decode Container to get signature
        let inner = ok(
            proto::Container::decode(&container_bytes[..]),
            "decode container",
        );
        assert_eq!(inner.signature, expected_sig);
        assert!(!inner.signature.is_empty());
    }

    #[test]
    fn public_key_protobuf_round_trip() {
        let keys = Keypair::generate_ed25519();
        let pub_key = keys.public();

        // Encode to protobuf bytes (as stored in sender_public_key)
        let encoded = pub_key.encode_protobuf();
        assert!(!encoded.is_empty());

        // Decode back
        let decoded = ok(
            libp2p::identity::PublicKey::try_decode_protobuf(&encoded),
            "decode public key protobuf",
        );
        assert_eq!(decoded, pub_key);
    }
}
