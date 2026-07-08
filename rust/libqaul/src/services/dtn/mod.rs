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
use std::collections::HashMap;
use std::{fmt, sync::RwLock};

use super::messaging::{proto, MessagingServiceType};
use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::router::users::Users;
use crate::rpc::Rpc;
use crate::storage::configuration::Configuration;
use crate::storage::database::DataBase;
use crate::utilities::qaul_id::QaulId;
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

/// Maximum time a V2 custody entry is retained when the sender specified
/// no expiry (expires_at == 0), counted from local acceptance: 7 days in
/// milliseconds. Without this cap such entries would be stored forever
/// if the recipient never becomes reachable.
const V2_MAX_RETENTION_MS: u64 = 7 * 24 * 60 * 60 * 1000;

/// Outcome of the stateless checks run on every incoming DtnRoutedV2
/// before the custody acceptance pipeline.
#[derive(Debug, PartialEq, Eq)]
enum V2Precheck {
    /// message expired — reject
    Expired,
    /// the local user is the final recipient — deliver
    Deliver,
    /// no handoffs left and we are not the recipient — reject
    Exhausted,
    /// continue with the custody acceptance pipeline
    Continue,
}

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
    pub fn new() -> Self {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let dtn_messages = db.open_tree("dtn-messages").unwrap();
        let dtn_ids = db.open_tree("dtn-messages-ids").unwrap();
        let dtn_routed_v2 = db.open_tree("dtn-routed-v2").unwrap();
        let dtn_sender_quotas = db.open_tree("dtn-sender-quotas").unwrap();
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
        let mut sender_quotas: HashMap<Vec<u8>, SenderQuotaEntry> = HashMap::new();
        for entry in db_ref_routed_v2.iter() {
            if let Ok((_, entry_bytes)) = entry {
                if let Ok(v2_entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                    v2_used_size += v2_entry.size as u64;
                    let quota = sender_quotas
                        .entry(v2_entry.sender_public_key.clone())
                        .or_default();
                    quota.used_bytes += v2_entry.size as u64;
                    quota.message_count += 1;
                }
            }
        }

        // Rebuild the sender-quota tree from the entry scan. The entry
        // insert and the quota update are two non-atomic tree writes, so a
        // crash between them leaves the quotas drifted; recomputing them
        // here heals that on every restart.
        match db_ref_sender_quotas.clear() {
            Ok(()) => {
                for (sender, quota) in &sender_quotas {
                    match bincode::serialize(quota) {
                        Ok(quota_bytes) => {
                            if let Err(e) =
                                db_ref_sender_quotas.insert(sender.clone(), quota_bytes)
                            {
                                log::error!("Failed to rebuild sender quota: {}", e);
                            }
                        }
                        Err(e) => log::error!("Failed to serialize sender quota: {}", e),
                    }
                }
                if let Err(e) = db_ref_sender_quotas.flush() {
                    log::error!("Failed to flush rebuilt sender quotas: {}", e);
                }
            }
            Err(e) => log::error!("Failed to clear dtn-sender-quotas tree: {}", e),
        }

        {
            let mut state = self.inner.write().unwrap();
            state.message_counts = dtn_messages.len() as u32;
            state.used_size = used_size;
            state.db_ref = dtn_messages;
            state.db_ref_id = db_ref_id;
        }
        {
            let mut v2_state = self.v2.write().unwrap();
            v2_state.message_count = db_ref_routed_v2.len() as u32;
            v2_state.used_size = v2_used_size;
            v2_state.db_ref_routed_v2 = db_ref_routed_v2;
            v2_state.db_ref_sender_quotas = db_ref_sender_quotas;
        }
        {
            let mut db_lock = self._db.write().unwrap();
            *db_lock = db;
        }
    }

    /// Process DTN response (instance method).
    ///
    /// A custodian receives confirmation that a stored message was
    /// delivered, so it frees the storage that message occupied and
    /// drops it from both index trees.
    pub fn on_dtn_response(&self, dtn_response: &super::messaging::proto::DtnResponse) {
        let mut state = self.inner.write().unwrap();

        // look up the stored entry; not finding it (or a bad row) is a
        // no-op, never a panic
        let entry_bytes = match state.db_ref.get(&dtn_response.signature) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => return,
            Err(e) => {
                log::error!("dtn on_dtn_response db_ref get: {}", e);
                return;
            }
        };
        let entry: DtnMessageEntry = match bincode::deserialize(&entry_bytes) {
            Ok(e) => e,
            Err(e) => {
                log::error!("dtn on_dtn_response entry deserialize: {}", e);
                return;
            }
        };

        // The message was delivered and is being removed, so FREE its
        // storage (subtract). Previously this incorrectly *added* the
        // size, so used_size grew without bound until the node started
        // rejecting every new message as over-quota.
        state.used_size = state.used_size.saturating_sub(entry.size as u64);
        state.message_counts = state.message_counts.saturating_sub(1);

        // remove from both index trees atomically: a crash between two
        // separate removals would otherwise desync db_ref / db_ref_id
        // and corrupt dedup state.
        use sled::Transactional;
        let sig = dtn_response.signature.clone();
        let org_sig = entry.org_sig.clone();
        let res: sled::transaction::TransactionResult<(), ()> =
            (&state.db_ref, &state.db_ref_id).transaction(|(db_ref, db_ref_id)| {
                db_ref.remove(sig.as_slice())?;
                db_ref_id.remove(org_sig.as_slice())?;
                Ok(())
            });
        if let Err(e) = res {
            log::error!("dtn on_dtn_response tree removal transaction failed: {:?}", e);
            return;
        }
        let _ = state.db_ref.flush();
        let _ = state.db_ref_id.flush();
    }

    /// Get DTN storage state (instance method).
    /// Returns (used_size, message_counts).
    pub fn get_state(&self) -> (u64, u32) {
        let state = self.inner.read().unwrap();
        (state.used_size, state.message_counts)
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
            // (storage accounting is updated only after the entry is
            // committed to the index trees, below, so a failed write
            // can't inflate used_size / message_counts.)
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

            // Write both index trees (db_ref and db_ref_id) atomically:
            // a crash between two separate inserts would leave them
            // desynced (an entry in one tree without its counterpart),
            // corrupting dedup/cleanup. A sled transaction commits both
            // or neither.
            use sled::Transactional;
            let sig = signature.clone();
            let org = org_sig.clone();
            let res: sled::transaction::TransactionResult<(), ()> = (
                &storage_state.db_ref,
                &storage_state.db_ref_id,
            )
                .transaction(|(db_ref, db_ref_id)| {
                    db_ref.insert(sig.as_slice(), message_entry_bytes.as_slice())?;
                    db_ref_id.insert(org.as_slice(), sig.as_slice())?;
                    Ok(())
                });
            match res {
                Ok(()) => {
                    let _ = storage_state.db_ref.flush();
                    let _ = storage_state.db_ref_id.flush();
                    // entry is committed — now account for its storage
                    storage_state.message_counts = storage_state.message_counts + 1;
                    storage_state.used_size = new_size;
                }
                Err(e) => {
                    log::error!("dtn entry store transaction failed: {:?}", e);
                    return (
                        super::messaging::proto::dtn_response::ResponseType::Rejected as i32,
                        super::messaging::proto::dtn_response::Reason::None as i32,
                    );
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
                    let (used_size_v2, dtn_message_count_v2) =
                        match state.services.dtn.v2.read() {
                            Ok(v2) => (v2.used_size, v2.message_count),
                            Err(e) => {
                                log::error!(
                                    "DTN RPC: failed to acquire V2 read lock: {}",
                                    e
                                );
                                return;
                            }
                        };

                    let proto_message = proto_rpc::Dtn {
                        message: Some(proto_rpc::dtn::Message::DtnStateResponse(
                            proto_rpc::DtnStateResponse {
                                used_size: dtn_state.used_size,
                                dtn_message_count: dtn_state.message_counts,
                                unconfirmed_count: unconfrimed_len as u32,
                                used_size_v2,
                                dtn_message_count_v2,
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

        // Convert the RPC hops into wire hops and validate.
        let custody_route: Vec<proto::RouteHop> = req
            .custody_route
            .iter()
            .map(|h| proto::RouteHop {
                hop: h.hop,
                ids: h.ids.clone(),
            })
            .collect();
        let (custody_route, total_custodians) =
            match Self::validate_custody_route(&custody_route, &my_user_id, &receiver_id) {
                Ok(v) => v,
                Err(e) => {
                    send_response(false, e);
                    return;
                }
            };

        // Get user account
        let user_account = match UserAccounts::get_by_id(state, my_user_id) {
            Some(ua) => ua,
            None => {
                send_response(false, "user account not found".to_string());
                return;
            }
        };

        // Calculate expiry
        let expires_at = if req.expiry_seconds > 0 {
            Timestamp::get_timestamp() + (req.expiry_seconds * 1000)
        } else {
            0
        };

        // Calculate remaining handoffs
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
            current_hop: 0,
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

    /// Maximum number of custodian IDs across a whole custody route.
    const MAX_CUSTODIANS: usize = 10;

    /// Validate and canonicalize a hop-numbered custody route.
    ///
    /// Rules: at least one custodian; hop numbers are >= 1 and unique
    /// (each hop appears once with its list of alternatives); each hop
    /// names at least one ID; every ID is a valid PeerId that is
    /// neither the sender nor the receiver and appears only once; at
    /// most `MAX_CUSTODIANS` IDs total.
    ///
    /// Returns the route sorted by hop ascending, plus the total ID
    /// count.
    fn validate_custody_route(
        route: &[proto::RouteHop],
        sender: &PeerId,
        receiver: &PeerId,
    ) -> Result<(Vec<proto::RouteHop>, u32), String> {
        if route.is_empty() {
            return Err("at least one custody hop is required".to_string());
        }
        let mut seen_hops: std::collections::HashSet<u32> = std::collections::HashSet::new();
        let mut seen_ids: std::collections::HashSet<Vec<u8>> = std::collections::HashSet::new();
        let mut total: usize = 0;
        for hop in route {
            if hop.hop < 1 {
                return Err("hop numbers must be >= 1".to_string());
            }
            if !seen_hops.insert(hop.hop) {
                return Err(format!("duplicate hop number {}", hop.hop));
            }
            if hop.ids.is_empty() {
                return Err(format!("hop {} has no custodians", hop.hop));
            }
            for id_bytes in &hop.ids {
                match PeerId::from_bytes(id_bytes) {
                    Ok(uid) => {
                        if uid == *sender || uid == *receiver {
                            return Err(
                                "custodians must not include sender or receiver".to_string()
                            );
                        }
                    }
                    Err(_) => return Err("invalid custodian user ID".to_string()),
                }
                if !seen_ids.insert(id_bytes.clone()) {
                    return Err("a custodian appears more than once in the route".to_string());
                }
                total += 1;
            }
        }
        if total > Self::MAX_CUSTODIANS {
            return Err(format!(
                "maximum {} custodians allowed",
                Self::MAX_CUSTODIANS
            ));
        }
        let mut sorted = route.to_vec();
        sorted.sort_by_key(|h| h.hop);
        Ok((sorted, total as u32))
    }

    /// Where to forward a V2 message, and the hop number to stamp on it.
    ///
    /// - If the ultimate recipient is directly reachable, deliver to it
    ///   (the returned hop is left at `current_hop` — delivery ends the
    ///   route, so it is never re-forwarded).
    /// - Otherwise walk the hops strictly greater than `current_hop` in
    ///   ascending order. All IDs sharing a hop are interchangeable
    ///   alternatives: the first reachable one at the lowest such hop is
    ///   the target. A hop whose custodians are all unreachable is
    ///   skipped in favour of the next hop (routes may be sparse).
    ///
    /// Returns `(target, new_current_hop)`, or `None` when nothing is
    /// reachable.
    pub fn select_custody_target_hop(
        state: &crate::QaulState,
        routed_v2: &proto::DtnRoutedV2,
        receiver_id: &PeerId,
    ) -> Option<(PeerId, u32)> {
        let rs = state.get_router();
        // Recipient directly reachable → deliver, route terminates.
        if rs.routing_table.get_route_to_user(*receiver_id).is_some() {
            return Some((*receiver_id, routed_v2.current_hop));
        }

        // Hops strictly after the one we currently hold, ascending.
        let mut hops: Vec<&proto::RouteHop> = routed_v2
            .custody_route
            .iter()
            .filter(|h| h.hop > routed_v2.current_hop)
            .collect();
        hops.sort_by_key(|h| h.hop);

        for hop in hops {
            for id_bytes in &hop.ids {
                if let Ok(custodian_id) = PeerId::from_bytes(id_bytes) {
                    if rs.routing_table.get_route_to_user(custodian_id).is_some() {
                        return Some((custodian_id, hop.hop));
                    }
                }
            }
        }

        None
    }

    /// Back-compat wrapper returning just the target (used where the
    /// new hop cursor is not needed).
    pub fn select_custody_target(
        state: &crate::QaulState,
        routed_v2: &proto::DtnRoutedV2,
        receiver_id: &PeerId,
    ) -> Option<PeerId> {
        Self::select_custody_target_hop(state, routed_v2, receiver_id).map(|(id, _)| id)
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

        // 1. Stateless prechecks: expiry, final delivery, handoff budget
        let envelope_receiver = Self::get_receiver_from_container(&routed_v2.container);
        match Self::precheck_routed_v2(
            &routed_v2,
            envelope_receiver.as_ref(),
            user_id,
            Timestamp::get_timestamp(),
        ) {
            V2Precheck::Expired => {
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
            V2Precheck::Deliver => {
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
            V2Precheck::Exhausted => {
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
            V2Precheck::Continue => {}
        }

        // 2. Duplicate check
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

        // 3. Custody opt-in check
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

        // 4. Sender signature verification
        let sender_pub_key = {
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
                envelope
                    .encode(&mut envelope_buf)
                    .expect("Vec<u8> provides capacity as needed");
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
            sender_key
        };

        // 5. Blocked-sender check: a sender this node's user has blocked
        // must not consume custody storage
        {
            let sender_peer = PeerId::from_public_key(&sender_pub_key);
            let rs = state.get_router();
            if let Some(user) = Users::get_user_snapshot(&rs, &QaulId::to_q8id(sender_peer)) {
                if user.blocked {
                    log::warn!(
                        "DtnRoutedV2: rejecting message from blocked sender {}",
                        sender_peer.to_base58()
                    );
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

        // 6. Per-sender quota check
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
        if let Some((target, new_hop)) =
            Self::select_custody_target_hop(state, routed_v2, receiver_id)
        {
            let mut forwarded = routed_v2.clone();
            forwarded.remaining_handoffs = forwarded.remaining_handoffs.saturating_sub(1);
            // Advance the cursor to the hop the target sits at, so the
            // next custodian only considers later hops.
            forwarded.current_hop = new_hop;

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

    /// Stateless checks for an incoming DtnRoutedV2 message.
    ///
    /// Order matters: the recipient check runs before the handoff check —
    /// a message that has used up all its custody handoffs must still be
    /// delivered when it arrives at its final recipient, because delivery
    /// is not a handoff.
    fn precheck_routed_v2(
        routed_v2: &proto::DtnRoutedV2,
        envelope_receiver: Option<&PeerId>,
        my_id: &PeerId,
        now: u64,
    ) -> V2Precheck {
        if routed_v2.expires_at > 0 && now > routed_v2.expires_at {
            return V2Precheck::Expired;
        }
        if envelope_receiver == Some(my_id) {
            return V2Precheck::Deliver;
        }
        if routed_v2.remaining_handoffs == 0 {
            return V2Precheck::Exhausted;
        }
        V2Precheck::Continue
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
                        // Check expiry. Entries whose sender specified no
                        // expiry (expires_at == 0) are still bounded by the
                        // local maximum retention, counted from acceptance.
                        let effective_expires_at = if routed_v2.expires_at > 0 {
                            routed_v2.expires_at
                        } else {
                            v2_entry.accepted_at.saturating_add(V2_MAX_RETENTION_MS)
                        };
                        if now > effective_expires_at {
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

    // A delivered DTN message must FREE the storage it occupied (the
    // accounting previously *added* the size, so used_size grew without
    // bound until the custodian rejected everything as over-quota), and
    // both index trees must be cleared together (atomic removal).
    #[test]
    fn dtn_response_frees_storage_and_clears_both_trees() {
        let dtn = DtnModuleState::new();
        let sig = vec![1u8; 16];
        let org_sig = vec![2u8; 16];
        let entry = DtnMessageEntry {
            org_sig: org_sig.clone(),
            size: 500,
        };

        // simulate one stored message occupying 500 bytes
        {
            let st = dtn.inner.write().unwrap();
            st.db_ref
                .insert(sig.as_slice(), bincode::serialize(&entry).unwrap())
                .unwrap();
            st.db_ref_id
                .insert(org_sig.as_slice(), sig.as_slice())
                .unwrap();
        }
        {
            let mut st = dtn.inner.write().unwrap();
            st.used_size = 500;
            st.message_counts = 1;
        }

        // delivery confirmation for that message
        let resp = proto::DtnResponse {
            signature: sig.clone(),
            ..Default::default()
        };
        dtn.on_dtn_response(&resp);

        let st = dtn.inner.read().unwrap();
        assert_eq!(st.used_size, 0, "delivered message must free its storage");
        assert_eq!(st.message_counts, 0);
        assert!(
            !st.db_ref.contains_key(sig.as_slice()).unwrap(),
            "entry removed from db_ref"
        );
        assert!(
            !st.db_ref_id.contains_key(org_sig.as_slice()).unwrap(),
            "entry removed from db_ref_id (atomic with db_ref)"
        );
    }

    // An unknown / already-delivered signature is a harmless no-op
    // (never a panic), even on a corrupted row.
    #[test]
    fn dtn_response_unknown_signature_is_noop() {
        let dtn = DtnModuleState::new();
        let resp = proto::DtnResponse {
            signature: vec![9u8; 16],
            ..Default::default()
        };
        dtn.on_dtn_response(&resp); // must not panic
        assert_eq!(dtn.inner.read().unwrap().used_size, 0);
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

    /// One hop entry.
    fn hop(n: u32, ids: Vec<Vec<u8>>) -> proto::RouteHop {
        proto::RouteHop { hop: n, ids }
    }

    /// Build a DtnRoutedV2 with the given hop-numbered custody route
    /// and current_hop cursor.
    fn build_routed_v2(custody_route: Vec<proto::RouteHop>, current_hop: u32) -> proto::DtnRoutedV2 {
        proto::DtnRoutedV2 {
            container: vec![1, 2, 3],
            custody_route,
            current_hop,
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
            custody_route: vec![
                proto::RouteHop { hop: 1, ids: vec![vec![1, 2, 3]] },
                proto::RouteHop { hop: 2, ids: vec![vec![4, 5, 6]] },
            ],
            current_hop: 0,
            original_signature: vec![0xAA, 0xBB],
            sender_public_key: vec![0xCC, 0xDD],
            expires_at: 1234567890,
            remaining_handoffs: 5,
        };

        let encoded = original.encode_to_vec();
        assert!(!encoded.is_empty());

        let decoded = proto::DtnRoutedV2::decode(&encoded[..]).unwrap();
        assert_eq!(decoded.container, original.container);
        assert_eq!(decoded.custody_route.len(), 2);
        assert_eq!(decoded.custody_route[0].hop, 1);
        assert_eq!(decoded.custody_route[0].ids, vec![vec![1, 2, 3]]);
        assert_eq!(decoded.custody_route[1].hop, 2);
        assert_eq!(decoded.custody_route[1].ids, vec![vec![4, 5, 6]]);
        assert_eq!(decoded.current_hop, 0);
        assert_eq!(decoded.original_signature, original.original_signature);
        assert_eq!(decoded.sender_public_key, original.sender_public_key);
        assert_eq!(decoded.expires_at, original.expires_at);
        assert_eq!(decoded.remaining_handoffs, original.remaining_handoffs);
    }

    #[test]
    fn dtn_routed_v2_serde_round_trip() {
        let original = proto::DtnRoutedV2 {
            container: vec![10, 20],
            custody_route: vec![proto::RouteHop { hop: 1, ids: vec![vec![1, 2, 3]] }],
            current_hop: 1,
            original_signature: vec![0xAA],
            sender_public_key: vec![0xCC],
            expires_at: 0,
            remaining_handoffs: 3,
        };

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
            custody_route: vec![],
            current_hop: 0,
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

        let v2 = build_routed_v2(vec![hop(1, vec![custodian.to_bytes()])], 0);

        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, Some(recipient));
    }

    #[test]
    fn select_target_advances_to_next_hop_when_earlier_unreachable() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();
        let c2 = random_peer();

        let mut table = HashMap::new();
        // recipient offline, hop-1 (c1) offline, hop-2 (c2) online
        make_online(&mut table, c2);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        let v2 = build_routed_v2(
            vec![hop(1, vec![c1.to_bytes()]), hop(2, vec![c2.to_bytes()])],
            0,
        );

        // hop 1 unreachable -> advance to hop 2, target c2 at hop 2
        assert_eq!(
            Dtn::select_custody_target_hop(&qaul_state, &v2, &recipient),
            Some((c2, 2))
        );
    }

    #[test]
    fn select_target_uses_same_hop_alternative() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let a = random_peer();
        let b = random_peer();

        let mut table = HashMap::new();
        // a is offline, b online — both share hop 1 as alternatives
        make_online(&mut table, b);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        let v2 = build_routed_v2(vec![hop(1, vec![a.to_bytes(), b.to_bytes()])], 0);

        // first reachable alternative at the same hop is chosen, hop stays 1
        assert_eq!(
            Dtn::select_custody_target_hop(&qaul_state, &v2, &recipient),
            Some((b, 1))
        );
    }

    #[test]
    fn select_target_skips_fully_unreachable_hop_across_gap() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();
        let c3 = random_peer();
        let c5 = random_peer();

        let mut table = HashMap::new();
        // hop 3's only custodian offline; hop 5 online. Route is sparse.
        make_online(&mut table, c5);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        let v2 = build_routed_v2(
            vec![
                hop(1, vec![c1.to_bytes()]),
                hop(3, vec![random_peer().to_bytes()]),
                hop(5, vec![c5.to_bytes()]),
            ],
            1, // already took hop 1
        );

        // hop 3 unreachable -> skip the gap to hop 5
        let _ = c3;
        assert_eq!(
            Dtn::select_custody_target_hop(&qaul_state, &v2, &recipient),
            Some((c5, 5))
        );
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

        let v2 = build_routed_v2(
            vec![hop(1, vec![c1.to_bytes()]), hop(2, vec![c2.to_bytes()])],
            0,
        );

        let target = Dtn::select_custody_target(&qaul_state, &v2, &recipient);
        assert_eq!(target, None);
    }

    #[test]
    fn select_target_respects_current_hop() {
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

        // current_hop = 1 means hop 1 (c1) is already taken, only hop 2 eligible
        let v2 = build_routed_v2(
            vec![hop(1, vec![c1.to_bytes()]), hop(2, vec![c2.to_bytes()])],
            1,
        );

        assert_eq!(
            Dtn::select_custody_target_hop(&qaul_state, &v2, &recipient),
            Some((c2, 2))
        );
    }

    #[test]
    fn select_target_none_when_route_exhausted() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();

        let mut table = HashMap::new();
        make_online(&mut table, c1);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        // current_hop == last hop means the route is exhausted
        let v2 = build_routed_v2(vec![hop(1, vec![c1.to_bytes()])], 1);

        assert_eq!(
            Dtn::select_custody_target(&qaul_state, &v2, &recipient),
            None
        );
    }

    #[test]
    fn select_target_picks_lowest_reachable_hop() {
        let qaul_state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let c1 = random_peer();
        let c2 = random_peer();

        let mut table = HashMap::new();
        // Both online
        make_online(&mut table, c1);
        make_online(&mut table, c2);
        qaul_state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });

        let v2 = build_routed_v2(
            vec![hop(1, vec![c1.to_bytes()]), hop(2, vec![c2.to_bytes()])],
            0,
        );

        // lowest hop is tried first -> c1 at hop 1
        assert_eq!(
            Dtn::select_custody_target_hop(&qaul_state, &v2, &recipient),
            Some((c1, 1))
        );
    }

    // ── validate_custody_route tests ──

    #[test]
    fn validate_route_rejects_empty() {
        let (s, r) = (random_peer(), random_peer());
        assert!(Dtn::validate_custody_route(&[], &s, &r).is_err());
    }

    #[test]
    fn validate_route_rejects_hop_zero() {
        let (s, r) = (random_peer(), random_peer());
        let route = vec![hop(0, vec![random_peer().to_bytes()])];
        assert!(Dtn::validate_custody_route(&route, &s, &r).is_err());
    }

    #[test]
    fn validate_route_rejects_duplicate_hop_number() {
        let (s, r) = (random_peer(), random_peer());
        let route = vec![
            hop(2, vec![random_peer().to_bytes()]),
            hop(2, vec![random_peer().to_bytes()]),
        ];
        assert!(Dtn::validate_custody_route(&route, &s, &r).is_err());
    }

    #[test]
    fn validate_route_rejects_sender_or_receiver() {
        let (s, r) = (random_peer(), random_peer());
        assert!(Dtn::validate_custody_route(&[hop(1, vec![s.to_bytes()])], &s, &r).is_err());
        assert!(Dtn::validate_custody_route(&[hop(1, vec![r.to_bytes()])], &s, &r).is_err());
    }

    #[test]
    fn validate_route_rejects_duplicate_custodian() {
        let (s, r) = (random_peer(), random_peer());
        let dup = random_peer().to_bytes();
        let route = vec![hop(1, vec![dup.clone()]), hop(2, vec![dup])];
        assert!(Dtn::validate_custody_route(&route, &s, &r).is_err());
    }

    #[test]
    fn validate_route_sorts_by_hop_and_counts() {
        let (s, r) = (random_peer(), random_peer());
        let route = vec![
            hop(5, vec![random_peer().to_bytes(), random_peer().to_bytes()]),
            hop(1, vec![random_peer().to_bytes()]),
        ];
        let (sorted, total) =
            Dtn::validate_custody_route(&route, &s, &r).expect("valid route");
        assert_eq!(total, 3);
        assert_eq!(sorted[0].hop, 1);
        assert_eq!(sorted[1].hop, 5);
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
        let entry_bytes = bincode::serialize(&entry).unwrap();

        {
            let mut state = qaul_state.services.dtn.v2.write().unwrap();
            state
                .db_ref_routed_v2
                .insert(sig.clone(), entry_bytes)
                .unwrap();
            state.db_ref_routed_v2.flush().unwrap();
            state.used_size += entry.size as u64;
            state.message_count += 1;
        }

        // Retrieve
        {
            let state = qaul_state.services.dtn.v2.read().unwrap();
            assert!(state.db_ref_routed_v2.contains_key(&sig).unwrap());
            let stored = state.db_ref_routed_v2.get(&sig).unwrap().unwrap();
            let decoded: DtnRoutedV2Entry = bincode::deserialize(&stored).unwrap();
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
        let entry_bytes = bincode::serialize(&entry).unwrap();

        {
            let state = qaul_state.services.dtn.v2.write().unwrap();
            state
                .db_ref_routed_v2
                .insert(sig.clone(), entry_bytes)
                .unwrap();
        }

        // Should detect duplicate
        {
            let state = qaul_state.services.dtn.v2.read().unwrap();
            assert!(state.db_ref_routed_v2.contains_key(&sig).unwrap());
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
        let quota_bytes = bincode::serialize(&quota).unwrap();

        {
            let state = qaul_state.services.dtn.v2.write().unwrap();
            state
                .db_ref_sender_quotas
                .insert(sender_key.clone(), quota_bytes)
                .unwrap();
        }

        // Retrieve and check
        {
            let state = qaul_state.services.dtn.v2.read().unwrap();
            let stored = state
                .db_ref_sender_quotas
                .get(&sender_key)
                .unwrap()
                .unwrap();
            let decoded: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
            assert_eq!(decoded.used_bytes, 500);
            assert_eq!(decoded.message_count, 2);
        }

        // Simulate removing a message — quota should decrease
        {
            let state = qaul_state.services.dtn.v2.write().unwrap();
            let stored = state
                .db_ref_sender_quotas
                .get(&sender_key)
                .unwrap()
                .unwrap();
            let mut decoded: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
            decoded.used_bytes = decoded.used_bytes.saturating_sub(200);
            decoded.message_count = decoded.message_count.saturating_sub(1);
            let updated = bincode::serialize(&decoded).unwrap();
            state
                .db_ref_sender_quotas
                .insert(sender_key.clone(), updated)
                .unwrap();
        }

        {
            let state = qaul_state.services.dtn.v2.read().unwrap();
            let stored = state
                .db_ref_sender_quotas
                .get(&sender_key)
                .unwrap()
                .unwrap();
            let decoded: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
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
        let quota_bytes = bincode::serialize(&quota).unwrap();

        {
            let state = qaul_state.services.dtn.v2.read().unwrap();
            state
                .db_ref_sender_quotas
                .insert(sender_key.clone(), quota_bytes)
                .unwrap();
        }

        // A message of size 11 should exceed the quota
        {
            let state = qaul_state.services.dtn.v2.read().unwrap();
            let stored = state
                .db_ref_sender_quotas
                .get(&sender_key)
                .unwrap()
                .unwrap();
            let decoded: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
            let new_msg_size: u64 = 11;
            assert!(decoded.used_bytes + new_msg_size > V2_PER_SENDER_QUOTA);
        }

        // A message of size 5 should be under the quota
        {
            let state = qaul_state.services.dtn.v2.read().unwrap();
            let stored = state
                .db_ref_sender_quotas
                .get(&sender_key)
                .unwrap()
                .unwrap();
            let decoded: SenderQuotaEntry = bincode::deserialize(&stored).unwrap();
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
        envelope.encode(&mut envelope_buf).unwrap();

        let signature = keys.sign(&envelope_buf).unwrap();

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
        let inner = proto::Container::decode(&container_bytes[..]).unwrap();
        let sender_key = keys.public();

        // Re-encode envelope and verify
        let envelope = inner.envelope.as_ref().unwrap();
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        envelope.encode(&mut envelope_buf).unwrap();

        assert!(sender_key.verify(&envelope_buf, &inner.signature));
    }

    #[test]
    fn signature_verification_rejects_wrong_key() {
        let receiver = random_peer();
        let (_keys, container_bytes, _sig) = build_signed_container(&receiver);

        // Use a different key to verify
        let wrong_keys = Keypair::generate_ed25519();
        let wrong_key = wrong_keys.public();

        let inner = proto::Container::decode(&container_bytes[..]).unwrap();
        let envelope = inner.envelope.as_ref().unwrap();
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        envelope.encode(&mut envelope_buf).unwrap();

        assert!(!wrong_key.verify(&envelope_buf, &inner.signature));
    }

    #[test]
    fn signature_verification_rejects_tampered_envelope() {
        let receiver = random_peer();
        let (keys, container_bytes, _sig) = build_signed_container(&receiver);

        let mut inner = proto::Container::decode(&container_bytes[..]).unwrap();
        // Tamper with the envelope
        inner.envelope.as_mut().unwrap().payload = vec![0xFF, 0xFF];

        let sender_key = keys.public();
        let envelope = inner.envelope.as_ref().unwrap();
        let mut envelope_buf = Vec::with_capacity(envelope.encoded_len());
        envelope.encode(&mut envelope_buf).unwrap();

        // Original signature should not verify against tampered envelope
        assert!(!sender_key.verify(&envelope_buf, &inner.signature));
    }

    #[test]
    fn original_signature_extracted_from_inner_container() {
        let receiver = random_peer();
        let (_keys, container_bytes, expected_sig) = build_signed_container(&receiver);

        // Simulate what rpc_send_routed does: decode Container to get signature
        let inner = proto::Container::decode(&container_bytes[..]).unwrap();
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
        let decoded = libp2p::identity::PublicKey::try_decode_protobuf(&encoded).unwrap();
        assert_eq!(decoded, pub_key);
    }

    // ── Release hardening: retention, quota rebuild, blocked senders ──

    /// Unwrap a `Result` in a test with a panic message that names the step.
    fn ok<T, E: std::fmt::Debug>(res: Result<T, E>, ctx: &str) -> T {
        match res {
            Ok(v) => v,
            Err(e) => panic!("{}: {:?}", ctx, e),
        }
    }

    /// Build a simulation state with one local account whose profile has
    /// DTN V2 custody enabled. Bypasses `UserAccounts::create` because that
    /// calls `Configuration::save`, which would write a config.yaml into
    /// the test runner's working directory.
    fn make_custody_state() -> (crate::QaulState, UserAccount) {
        let state = crate::QaulState::new_for_simulation();
        let keys = Keypair::generate_ed25519();
        let id = PeerId::from(keys.public());
        let account = UserAccount {
            id,
            keys,
            name: "custodian".to_string(),
            password_hash: None,
            password_salt: None,
        };
        match state.user_accounts.inner.write() {
            Ok(mut users) => users.users.push(account.clone()),
            Err(e) => panic!("user accounts lock poisoned: {}", e),
        }
        match state.config.inner.write() {
            Ok(mut cfg) => {
                cfg.user_accounts
                    .push(crate::storage::configuration::UserAccount {
                        id: id.to_string(),
                        storage: crate::storage::configuration::StorageOptions {
                            dtn_v2_custody_enabled: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            }
            Err(e) => panic!("config lock poisoned: {}", e),
        }
        (state, account)
    }

    /// Build a fully valid DtnRoutedV2 around a signed container.
    fn build_valid_routed_v2(
        keys: &Keypair,
        container_bytes: Vec<u8>,
        signature: Vec<u8>,
        expires_at: u64,
    ) -> proto::DtnRoutedV2 {
        proto::DtnRoutedV2 {
            container: container_bytes,
            custody_route: vec![proto::RouteHop { hop: 1, ids: vec![random_peer().to_bytes()] }],
            current_hop: 0,
            original_signature: signature,
            sender_public_key: keys.public().encode_protobuf(),
            expires_at,
            remaining_handoffs: 5,
        }
    }

    /// Insert a V2 entry with its quota record, as acceptance does.
    fn store_v2_entry(
        state: &crate::QaulState,
        sig: &[u8],
        entry: &DtnRoutedV2Entry,
    ) {
        let mut v2 = ok(state.services.dtn.v2.write(), "v2 write lock");
        let entry_bytes = ok(bincode::serialize(entry), "serialize entry");
        ok(
            v2.db_ref_routed_v2.insert(sig.to_vec(), entry_bytes),
            "insert entry",
        );
        v2.used_size += entry.size as u64;
        v2.message_count += 1;
        let quota = SenderQuotaEntry {
            used_bytes: entry.size as u64,
            message_count: 1,
        };
        let quota_bytes = ok(bincode::serialize(&quota), "serialize quota");
        ok(
            v2.db_ref_sender_quotas
                .insert(entry.sender_public_key.clone(), quota_bytes),
            "insert quota",
        );
    }

    /// Entries whose sender specified no expiry (expires_at == 0) must
    /// still be swept once they exceed the local maximum retention.
    /// Without the retention cap, a custodian that never reaches the
    /// recipient stores such messages forever.
    #[test]
    fn retransmit_sweeps_no_expiry_entries_after_max_retention() {
        let (state, _account) = make_custody_state();
        let now = Timestamp::get_timestamp();

        // Stale entry: no expiry, accepted longer than max retention ago
        let receiver = random_peer();
        let (stale_keys, stale_container, stale_sig) = build_signed_container(&receiver);
        let stale_v2 = build_valid_routed_v2(&stale_keys, stale_container, stale_sig.clone(), 0);
        let stale_entry = DtnRoutedV2Entry {
            routed_v2_bytes: stale_v2.encode_to_vec(),
            sender_public_key: stale_v2.sender_public_key.clone(),
            size: 100,
            accepted_at: now.saturating_sub(V2_MAX_RETENTION_MS + 1_000),
            receiver_id: receiver.to_bytes(),
        };
        store_v2_entry(&state, &stale_sig, &stale_entry);

        // Fresh entry: no expiry, accepted just now — must survive
        let (fresh_keys, fresh_container, fresh_sig) = build_signed_container(&receiver);
        let fresh_v2 = build_valid_routed_v2(&fresh_keys, fresh_container, fresh_sig.clone(), 0);
        let fresh_entry = DtnRoutedV2Entry {
            routed_v2_bytes: fresh_v2.encode_to_vec(),
            sender_public_key: fresh_v2.sender_public_key.clone(),
            size: 40,
            accepted_at: now,
            receiver_id: receiver.to_bytes(),
        };
        store_v2_entry(&state, &fresh_sig, &fresh_entry);

        Dtn::process_retransmit_v2(&state);

        let v2 = ok(state.services.dtn.v2.read(), "v2 read lock");
        assert!(
            !ok(
                v2.db_ref_routed_v2.contains_key(&stale_sig),
                "contains stale"
            ),
            "no-expiry entry past max retention must be swept"
        );
        assert!(
            ok(
                v2.db_ref_routed_v2.contains_key(&fresh_sig),
                "contains fresh"
            ),
            "fresh no-expiry entry must survive the sweep"
        );
        assert_eq!(v2.message_count, 1);
        assert_eq!(v2.used_size, 40);

        // The swept entry's sender quota must be released
        let quota_bytes = ok(
            v2.db_ref_sender_quotas.get(&stale_entry.sender_public_key),
            "get stale quota",
        );
        if let Some(bytes) = quota_bytes {
            let quota: SenderQuotaEntry =
                ok(bincode::deserialize(&bytes), "decode stale quota");
            assert_eq!(quota.used_bytes, 0);
            assert_eq!(quota.message_count, 0);
        }
    }

    /// A crash between the entry insert and the quota update leaves
    /// dtn-sender-quotas out of sync with dtn-routed-v2. Init must
    /// rebuild the quotas from the entries so drift heals on restart.
    #[test]
    fn init_production_rebuilds_sender_quotas_from_entries() {
        let db = ok(
            sled::Config::new().temporary(true).open(),
            "open temp sled db",
        );
        let entries_tree = ok(db.open_tree("dtn-routed-v2"), "open entries tree");
        let quotas_tree = ok(db.open_tree("dtn-sender-quotas"), "open quotas tree");

        let sender_a = vec![0xAA];
        let sender_b = vec![0xBB];
        let sender_c = vec![0xCC];

        for (sig, sender, size) in [
            (vec![0x01], &sender_a, 100u32),
            (vec![0x02], &sender_a, 50),
            (vec![0x03], &sender_b, 70),
        ] {
            let entry = DtnRoutedV2Entry {
                routed_v2_bytes: vec![0; size as usize],
                sender_public_key: sender.clone(),
                size,
                accepted_at: 1_000,
                receiver_id: vec![],
            };
            let bytes = ok(bincode::serialize(&entry), "serialize entry");
            ok(entries_tree.insert(sig, bytes), "insert entry");
        }

        // Drifted quota records: A wildly wrong, B missing, C stale
        let wrong_a = SenderQuotaEntry {
            used_bytes: 999_999,
            message_count: 42,
        };
        ok(
            quotas_tree.insert(
                sender_a.clone(),
                ok(bincode::serialize(&wrong_a), "serialize wrong quota"),
            ),
            "insert wrong quota",
        );
        let stale_c = SenderQuotaEntry {
            used_bytes: 10,
            message_count: 1,
        };
        ok(
            quotas_tree.insert(
                sender_c.clone(),
                ok(bincode::serialize(&stale_c), "serialize stale quota"),
            ),
            "insert stale quota",
        );

        let module_state = DtnModuleState::new();
        module_state.init_production(db);

        let v2 = ok(module_state.v2.read(), "v2 read lock");
        assert_eq!(v2.used_size, 220);
        assert_eq!(v2.message_count, 3);

        let quota_a: SenderQuotaEntry = match ok(
            v2.db_ref_sender_quotas.get(&sender_a),
            "get quota A",
        ) {
            Some(bytes) => ok(bincode::deserialize(&bytes), "decode quota A"),
            None => panic!("quota for sender A missing after rebuild"),
        };
        assert_eq!(quota_a.used_bytes, 150);
        assert_eq!(quota_a.message_count, 2);

        let quota_b: SenderQuotaEntry = match ok(
            v2.db_ref_sender_quotas.get(&sender_b),
            "get quota B",
        ) {
            Some(bytes) => ok(bincode::deserialize(&bytes), "decode quota B"),
            None => panic!("quota for sender B missing after rebuild"),
        };
        assert_eq!(quota_b.used_bytes, 70);
        assert_eq!(quota_b.message_count, 1);

        assert!(
            ok(v2.db_ref_sender_quotas.get(&sender_c), "get quota C").is_none(),
            "stale quota record for sender with no entries must be dropped"
        );
    }

    /// Register a remote user in the router users table.
    fn register_user(state: &crate::QaulState, keys: &Keypair, blocked: bool) {
        let rs = state.get_router();
        crate::router::users::Users::add(
            state,
            &rs,
            crate::router::users::User {
                id: PeerId::from(keys.public()),
                key: keys.public(),
                name: "remote".to_string(),
                verified: false,
                blocked,
                capabilities: 0,
                bio: String::new(),
                avatar: vec![],
                version: 0,
                updated_at: 0,
                signed_profile_bytes: vec![],
                signed_profile_signature: vec![],
                preferred_custody_route: vec![],
            },
        );
    }

    /// A message that has consumed all custody handoffs must still be
    /// delivered when it reaches its final recipient — delivery is not a
    /// handoff. (Regression: the handoff check used to run before the
    /// recipient check and rejected such messages even at the recipient.)
    #[test]
    fn precheck_delivers_to_recipient_with_zero_handoffs() {
        let me = random_peer();
        let mut v2 = build_routed_v2(vec![hop(1, vec![random_peer().to_bytes()])], 0);
        v2.remaining_handoffs = 0;
        assert_eq!(
            Dtn::precheck_routed_v2(&v2, Some(&me), &me, 1_000),
            V2Precheck::Deliver
        );
    }

    /// At a custodian (not the recipient), an exhausted handoff budget
    /// still rejects the message.
    #[test]
    fn precheck_rejects_exhausted_handoffs_at_custodian() {
        let me = random_peer();
        let receiver = random_peer();
        let mut v2 = build_routed_v2(vec![hop(1, vec![random_peer().to_bytes()])], 0);
        v2.remaining_handoffs = 0;
        assert_eq!(
            Dtn::precheck_routed_v2(&v2, Some(&receiver), &me, 1_000),
            V2Precheck::Exhausted
        );
    }

    /// An expired message is rejected even at its final recipient.
    #[test]
    fn precheck_expiry_wins_over_delivery() {
        let me = random_peer();
        let mut v2 = build_routed_v2(vec![hop(1, vec![random_peer().to_bytes()])], 0);
        v2.expires_at = 500;
        assert_eq!(
            Dtn::precheck_routed_v2(&v2, Some(&me), &me, 1_000),
            V2Precheck::Expired
        );
    }

    /// A custodian with handoff budget continues into the acceptance
    /// pipeline.
    #[test]
    fn precheck_continues_for_custodian_with_budget() {
        let me = random_peer();
        let receiver = random_peer();
        let v2 = build_routed_v2(vec![hop(1, vec![random_peer().to_bytes()])], 0);
        assert_eq!(
            Dtn::precheck_routed_v2(&v2, Some(&receiver), &me, 1_000),
            V2Precheck::Continue
        );
    }

    /// A blocked sender must not be able to consume custody storage.
    #[test]
    fn net_routed_v2_rejects_blocked_sender() {
        let (state, account) = make_custody_state();

        let receiver = random_peer();
        let (keys, container_bytes, signature) = build_signed_container(&receiver);
        register_user(&state, &keys, true);

        let routed_v2 = build_valid_routed_v2(&keys, container_bytes, signature.clone(), 0);
        let sender_peer = PeerId::from(keys.public());
        Dtn::net_routed_v2(&state, &account.id, &sender_peer, &[], routed_v2);

        let v2 = ok(state.services.dtn.v2.read(), "v2 read lock");
        assert!(
            !ok(
                v2.db_ref_routed_v2.contains_key(&signature),
                "contains key"
            ),
            "blocked sender's message must not be stored"
        );
        assert_eq!(v2.message_count, 0);
        assert_eq!(v2.used_size, 0);
    }

    /// Control for the blocked test: the identical message from a
    /// non-blocked sender is accepted — proving the rejection above is
    /// caused by the blocked flag, not by some other pipeline step.
    #[test]
    fn net_routed_v2_accepts_unblocked_sender() {
        let (state, account) = make_custody_state();

        let receiver = random_peer();
        let (keys, container_bytes, signature) = build_signed_container(&receiver);
        register_user(&state, &keys, false);

        let routed_v2 = build_valid_routed_v2(&keys, container_bytes, signature.clone(), 0);
        let sender_peer = PeerId::from(keys.public());
        Dtn::net_routed_v2(&state, &account.id, &sender_peer, &[], routed_v2);

        let v2 = ok(state.services.dtn.v2.read(), "v2 read lock");
        assert!(
            ok(
                v2.db_ref_routed_v2.contains_key(&signature),
                "contains key"
            ),
            "non-blocked sender's message must be stored"
        );
        assert_eq!(v2.message_count, 1);
    }
}
