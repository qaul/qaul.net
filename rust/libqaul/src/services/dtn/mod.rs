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
use sha2::{Digest, Sha256};
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


/// DTN V2 custody entry stored in sled
#[derive(Serialize, Deserialize, Clone)]
pub struct DtnRoutedV2Entry {
    /// serialized DtnV2Container protobuf message
    pub container_v2_bytes: Vec<u8>,
    /// public key of the original sender
    pub sender_public_key: Vec<u8>,
    /// size of the entry in bytes (the stored envelope payload)
    pub size: u32,
    /// timestamp when this entry was accepted (custodian-local receive time)
    pub accepted_at: u64,
    /// the ultimate receiver's user ID
    pub receiver_id: Vec<u8>,
    /// admission tier: see `tier` constants below. Governs retention and the
    /// order in which entries are shed under storage pressure.
    pub tier: u8,
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
    /// V2 custody messages: original_signature => DtnRoutedV2Entry
    pub db_ref_routed_v2: sled::Tree,
    /// Per-sender quota tracking: sender_public_key => SenderQuotaEntry
    pub db_ref_sender_quotas: sled::Tree,
    /// Custody grants this node has been given by recipients, so the originate
    /// path can attach them: recipient_id => encoded CustodyGrant bytes
    pub db_ref_grants: sled::Tree,
    /// Total used size for V2 messages
    pub used_size: u64,
    /// Total V2 message count
    pub message_count: u32,
    /// Bytes currently held for grant-less untrusted-tier senders. The
    /// aggregate cap on this (V2_UNTRUSTED_POOL_QUOTA) is what actually bounds
    /// a Sybil flood — per-sender quota alone can't, since identities are free.
    pub untrusted_used: u64,
}

/// Admission tier for a stored custody entry (`DtnRoutedV2Entry.tier`).
/// Untrusted entries are the first to be shed and have the shortest retention.
const TIER_UNTRUSTED: u8 = 0;
const TIER_TRUSTED: u8 = 1;
const TIER_GRANT: u8 = 2;
/// A signed DTN response (e.g. a DELIVERY ack) being routed back to an offline
/// sender over the reverse route. Control traffic: exempt from the grant/PoW
/// admission gates (it is tied to an already-accepted custody flow and carries
/// a verifiable signed response), and shed on the short untrusted retention.
const TIER_CONTROL: u8 = 3;

/// Per-sender ceiling for a grant-less stranger admitted via proof-of-work.
const V2_UNTRUSTED_PER_SENDER_QUOTA: u64 = 5 * 1024 * 1024;
/// Per-sender ceiling for a locally-trusted contact (verified).
const V2_TRUSTED_PER_SENDER_QUOTA: u64 = 20 * 1024 * 1024;
/// Aggregate ceiling for the whole grant-less untrusted pool. Bounds a Sybil
/// flood even when each fake sender stays under its own per-sender quota.
const V2_UNTRUSTED_POOL_QUOTA: u64 = 50 * 1024 * 1024;

/// Maximum time an untrusted custody entry is retained, counted from local
/// acceptance: 7 days. Without this cap such entries would be stored forever
/// if the recipient never becomes reachable.
const V2_MAX_RETENTION_MS: u64 = 7 * 24 * 60 * 60 * 1000;
/// Retention for trusted / grant-backed entries: 180 days.
const V2_TRUSTED_RETENTION_MS: u64 = 180 * 24 * 60 * 60 * 1000;

/// Minimum acceptable proof-of-work difficulty (leading zero bits) for a
/// grant-less deposit. ~2^20 hashes to solve (sub-second) but trivial to
/// verify; a one-machine Sybil flood pays this per (identity, custodian, day).
const V2_MIN_POW_DIFFICULTY: u32 = 20;

/// Milliseconds in a day, used to bind a PoW stamp to the current day.
const MS_PER_DAY: u64 = 24 * 60 * 60 * 1000;

/// Outcome of the stateless checks run on every incoming DtnV2Container
/// before the custody acceptance pipeline.
#[derive(Debug, PartialEq, Eq)]
enum V2Precheck {
    /// message expired — reject
    Expired,
    /// the local user is the final recipient — deliver
    Deliver,
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
        let dtn_grants = db.open_tree("dtn-grants").unwrap();
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
                untrusted_used: 0,
                db_ref_routed_v2: dtn_routed_v2,
                db_ref_sender_quotas: dtn_sender_quotas,
                db_ref_grants: dtn_grants,
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
        let db_ref_grants = match db.open_tree("dtn-grants") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to open dtn-grants tree: {}", e);
                return;
            }
        };

        let mut v2_used_size: u64 = 0;
        let mut v2_untrusted_used: u64 = 0;
        let mut sender_quotas: HashMap<Vec<u8>, SenderQuotaEntry> = HashMap::new();
        for entry in db_ref_routed_v2.iter() {
            if let Ok((_, entry_bytes)) = entry {
                if let Ok(v2_entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                    v2_used_size += v2_entry.size as u64;
                    if v2_entry.tier == TIER_UNTRUSTED {
                        v2_untrusted_used += v2_entry.size as u64;
                    }
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
            v2_state.untrusted_used = v2_untrusted_used;
            v2_state.db_ref_routed_v2 = db_ref_routed_v2;
            v2_state.db_ref_sender_quotas = db_ref_sender_quotas;
            v2_state.db_ref_grants = db_ref_grants;
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
                Some(proto_rpc::dtn::Message::DtnIssueGrantRequest(req)) => {
                    Self::rpc_issue_grant(state, my_user_id, req, request_id);
                }
                Some(proto_rpc::dtn::Message::DtnImportGrantRequest(req)) => {
                    Self::rpc_import_grant(state, req, request_id);
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

    // ===================================================================
    // DTN v2 redesign: crypto, proof-of-work, grant & route helpers
    // ===================================================================

    /// Derive the wire id (PeerId bytes) for a protobuf-encoded public key.
    fn peer_id_bytes_from_pubkey(pubkey_bytes: &[u8]) -> Option<Vec<u8>> {
        PublicKey::try_decode_protobuf(pubkey_bytes)
            .ok()
            .map(|pk| PeerId::from_public_key(&pk).to_bytes())
    }

    /// Build an (unsigned) DtnRoute from a flat custody-user list: one RouteHop
    /// per custodian (single interchangeable entry), ordered by hop ascending.
    fn build_route(
        original_signature: Vec<u8>,
        custody_route: &[Vec<u8>],
        sender_public_key: Vec<u8>,
        expires_at: Option<u64>,
    ) -> proto::DtnRoute {
        let route_hop = custody_route
            .iter()
            .map(|id| proto::RouteHop {
                route_entry: vec![proto::RouteEntry { id: id.clone() }],
            })
            .collect();
        proto::DtnRoute {
            original_signature,
            route_hop,
            sender_public_key,
            expires_at,
        }
    }

    /// Verify the sender's signature over the encoded route bytes.
    fn verify_route_sig(route: &proto::DtnRoute, dtn_route_bytes: &[u8], sig: &[u8]) -> bool {
        match PublicKey::try_decode_protobuf(&route.sender_public_key) {
            Ok(key) => key.verify(dtn_route_bytes, sig),
            Err(_) => false,
        }
    }

    /// Sign a custody grant with the local user's key (canonical: sig empty).
    fn sign_grant(
        user_account: &UserAccount,
        mut grant: proto::CustodyGrant,
    ) -> Option<proto::CustodyGrant> {
        grant.signature = Vec::new();
        match user_account.keys.sign(&grant.encode_to_vec()) {
            Ok(sig) => {
                grant.signature = sig;
                Some(grant)
            }
            Err(e) => {
                log::error!("DTN v2: grant signing failed: {}", e);
                None
            }
        }
    }

    /// Verify a recipient-signed custody grant in isolation: the embedded
    /// public key must be the named recipient, the grant must be unexpired, and
    /// the signature must verify. The caller additionally checks that the grant
    /// names *this* sender and receiver and that the deposit fits `quota_bytes`.
    fn verify_grant(grant: &proto::CustodyGrant, now_ms: u64) -> bool {
        match Self::peer_id_bytes_from_pubkey(&grant.recipient_public_key) {
            Some(id) if id == grant.recipient => {}
            _ => return false,
        }
        if grant.not_after != 0 && now_ms > grant.not_after {
            return false;
        }
        let key = match PublicKey::try_decode_protobuf(&grant.recipient_public_key) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let mut unsigned = grant.clone();
        unsigned.signature = Vec::new();
        key.verify(&unsigned.encode_to_vec(), &grant.signature)
    }

    /// Proof-of-work bind hash over (original_signature, custodian_id, day, nonce).
    fn pow_hash(original_signature: &[u8], custodian_id: &[u8], day: u64, nonce: u64) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(original_signature);
        hasher.update(custodian_id);
        hasher.update(day.to_le_bytes());
        hasher.update(nonce.to_le_bytes());
        hasher.finalize().into()
    }

    /// Count leading zero bits across a 32-byte digest.
    fn leading_zero_bits(hash: &[u8; 32]) -> u32 {
        let mut bits = 0u32;
        for byte in hash {
            if *byte == 0 {
                bits += 8;
            } else {
                bits += byte.leading_zeros();
                break;
            }
        }
        bits
    }

    /// Verify a PoW stamp, bound to this custodian and the current or previous
    /// day (tolerating a clock at the day boundary). A stamp solved for one
    /// custodian/day cannot be replayed against another.
    fn verify_pow(
        stamp: &proto::PowStamp,
        original_signature: &[u8],
        custodian_id: &[u8],
        now_ms: u64,
    ) -> bool {
        if stamp.difficulty < V2_MIN_POW_DIFFICULTY {
            return false;
        }
        let today = now_ms / MS_PER_DAY;
        for day in [today, today.saturating_sub(1)] {
            let hash = Self::pow_hash(original_signature, custodian_id, day, stamp.nonce);
            if Self::leading_zero_bits(&hash) >= stamp.difficulty {
                return true;
            }
        }
        false
    }

    /// Solve a PoW stamp (originate side / tests).
    fn solve_pow(
        original_signature: &[u8],
        custodian_id: &[u8],
        day: u64,
        difficulty: u32,
    ) -> proto::PowStamp {
        let mut nonce = 0u64;
        loop {
            let hash = Self::pow_hash(original_signature, custodian_id, day, nonce);
            if Self::leading_zero_bits(&hash) >= difficulty {
                return proto::PowStamp { nonce, difficulty };
            }
            nonce = nonce.wrapping_add(1);
        }
    }

    /// Build and sign a DtnResponseV2 (canonical: signature field empty while
    /// signing, then filled).
    fn build_signed_response(
        user_account: &UserAccount,
        kind: proto::dtn_response_v2::Kind,
        response_type: proto::dtn_response_v2::ResponseType,
        reason: proto::dtn_response_v2::Reason,
        original_signature: Vec<u8>,
    ) -> Option<proto::DtnResponseV2> {
        let mut resp = proto::DtnResponseV2 {
            kind: kind as i32,
            response_type: response_type as i32,
            reason: reason as i32,
            original_signature,
            responder_public_key: user_account.keys.public().encode_protobuf(),
            signature: Vec::new(),
        };
        match user_account.keys.sign(&resp.encode_to_vec()) {
            Ok(sig) => {
                resp.signature = sig;
                Some(resp)
            }
            Err(e) => {
                log::error!("DTN v2: response signing failed: {}", e);
                None
            }
        }
    }

    /// Verify a DtnResponseV2 signature against its embedded responder key.
    /// Returns the responder's PeerId bytes on success — this is the check that
    /// makes only a genuinely-signed response able to mutate custody state.
    fn verify_response_v2(resp: &proto::DtnResponseV2) -> Option<Vec<u8>> {
        let key = PublicKey::try_decode_protobuf(&resp.responder_public_key).ok()?;
        let mut unsigned = resp.clone();
        unsigned.signature = Vec::new();
        if key.verify(&unsigned.encode_to_vec(), &resp.signature) {
            Some(PeerId::from_public_key(&key).to_bytes())
        } else {
            None
        }
    }

    /// If an inner custody container carries a signed DTN response (an ack
    /// being routed back to the sender), return it. Used to distinguish control
    /// traffic from ordinary custody messages.
    fn extract_ack(inner_container_bytes: &[u8]) -> Option<proto::DtnResponseV2> {
        let container = proto::Container::decode(inner_container_bytes).ok()?;
        let envelope = container.envelope?;
        let payload = proto::EnvelopPayload::decode(&envelope.payload[..]).ok()?;
        match payload.payload {
            Some(proto::envelop_payload::Payload::DtnResponseV2(r)) => Some(r),
            _ => None,
        }
    }

    /// Route a signed DELIVERY ack back to the (possibly offline) original
    /// sender over the reverse of the forward route.
    ///
    /// The ack is wrapped as its own custody message — a `DtnV2Container` whose
    /// inner container carries the signed `DtnResponseV2` and whose route is the
    /// forward route reversed. It is stored locally (TIER_CONTROL) so the
    /// retransmit loop keeps retrying until a reverse path exists, and forwarded
    /// immediately if a next hop is reachable. Custodians treat it as control
    /// traffic (see the admission bypass in `net_routed_v2`), so it is not
    /// subject to grant/PoW gates.
    fn route_delivery_ack(
        state: &crate::QaulState,
        user_account: &UserAccount,
        forward_route: &proto::DtnRoute,
        original_signature: &[u8],
    ) {
        // The original sender is the author of the forward route.
        let origin = match PublicKey::try_decode_protobuf(&forward_route.sender_public_key) {
            Ok(k) => PeerId::from_public_key(&k),
            Err(e) => {
                log::error!("DtnV2: cannot derive sender for delivery ack: {}", e);
                return;
            }
        };

        let ack = match Self::build_signed_response(
            user_account,
            proto::dtn_response_v2::Kind::Delivery,
            proto::dtn_response_v2::ResponseType::Accepted,
            proto::dtn_response_v2::Reason::None,
            original_signature.to_vec(),
        ) {
            Some(a) => a,
            None => return,
        };
        // Dedup key for the ack's own custody entry.
        let ack_dedup = ack.signature.clone();

        // Inner container addressed to the original sender, carrying the ack.
        let payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::DtnResponseV2(ack)),
        };
        let inner_envelope = proto::Envelope {
            sender_id: user_account.id.to_bytes(),
            receiver_id: origin.to_bytes(),
            payload: payload.encode_to_vec(),
        };
        let inner_sig = match user_account.keys.sign(&inner_envelope.encode_to_vec()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("DtnV2: delivery ack inner signing failed: {}", e);
                return;
            }
        };
        let inner_container = proto::Container {
            signature: inner_sig,
            envelope: Some(inner_envelope),
        };
        let inner_bytes = inner_container.encode_to_vec();

        // Reverse route: same hops, reversed order, authored by us (the acker).
        let mut reversed_hops = forward_route.route_hop.clone();
        reversed_hops.reverse();
        let reverse_route = proto::DtnRoute {
            original_signature: ack_dedup.clone(),
            route_hop: reversed_hops,
            sender_public_key: user_account.keys.public().encode_protobuf(),
            expires_at: None,
        };
        let reverse_route_bytes = reverse_route.encode_to_vec();
        let reverse_route_sig = match user_account.keys.sign(&reverse_route_bytes) {
            Ok(s) => s,
            Err(e) => {
                log::error!("DtnV2: delivery ack route signing failed: {}", e);
                return;
            }
        };
        let ack_container = proto::DtnV2Container {
            dtn_route: reverse_route_bytes,
            dtn_route_sig: reverse_route_sig,
            envelope: inner_bytes,
            custody_grant: None,
            pow: None,
        };

        // Store our own copy so the retransmit loop keeps retrying the reverse
        // delivery until a path to the sender exists.
        let entry_size = ack_container.envelope.len() as u32;
        let entry = DtnRoutedV2Entry {
            container_v2_bytes: ack_container.encode_to_vec(),
            sender_public_key: reverse_route.sender_public_key.clone(),
            size: entry_size,
            accepted_at: Timestamp::get_timestamp(),
            receiver_id: origin.to_bytes(),
            tier: TIER_CONTROL,
        };
        if let Ok(bytes) = bincode::serialize(&entry) {
            if let Ok(mut v2) = state.services.dtn.v2.write() {
                if v2.db_ref_routed_v2.insert(ack_dedup, bytes).is_ok() {
                    let _ = v2.db_ref_routed_v2.flush();
                    v2.used_size += entry_size as u64;
                    v2.message_count += 1;
                }
            }
        }

        // Forward now if a next hop (or the sender) is reachable.
        Self::try_forward_v2(state, user_account, &ack_container, &reverse_route, &origin);
    }

    /// Store a custody grant this node has been given, keyed by the issuing
    /// recipient id, so the originate path can attach it to outgoing messages.
    fn store_held_grant(state: &crate::QaulState, grant: &proto::CustodyGrant) -> bool {
        match state.services.dtn.v2.write() {
            Ok(v2) => {
                if let Err(e) = v2
                    .db_ref_grants
                    .insert(grant.recipient.clone(), grant.encode_to_vec())
                {
                    log::error!("DTN v2: failed to store held grant: {}", e);
                    return false;
                }
                let _ = v2.db_ref_grants.flush();
                true
            }
            Err(e) => {
                log::error!("DTN v2: failed to acquire write lock for grant store: {}", e);
                false
            }
        }
    }

    /// Look up a held grant issued by `recipient_id`, if any.
    fn held_grant_for(state: &crate::QaulState, recipient_id: &[u8]) -> Option<proto::CustodyGrant> {
        let v2 = state.services.dtn.v2.read().ok()?;
        match v2.db_ref_grants.get(recipient_id) {
            Ok(Some(bytes)) => proto::CustodyGrant::decode(&bytes[..]).ok(),
            _ => None,
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

        // Optional expiry (retention is otherwise custodian-local)
        let expires_at = if req.expiry_seconds > 0 {
            Some(Timestamp::get_timestamp() + (req.expiry_seconds * 1000))
        } else {
            None
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

        // Build and sign the immutable route
        let route = Self::build_route(
            original_signature.clone(),
            &req.custody_route,
            user_account.keys.public().encode_protobuf(),
            expires_at,
        );
        let dtn_route_bytes = route.encode_to_vec();
        let dtn_route_sig = match user_account.keys.sign(&dtn_route_bytes) {
            Ok(sig) => sig,
            Err(e) => {
                send_response(false, format!("route signing failed: {}", e));
                return;
            }
        };

        // Find the first hop / recipient (my_user_id is the sender, not in the
        // route, so traversal treats it as being before the first hop).
        let target = match Self::select_custody_target(state, &route, &my_user_id, &receiver_id) {
            Some(t) => t,
            None => {
                send_response(false, "no reachable custodian found".to_string());
                return;
            }
        };

        // Admission credential: attach a recipient-issued grant if we hold one,
        // otherwise a proof-of-work stamp bound to the first custodian so a
        // grant-less deposit can still be admitted into the untrusted pool.
        let custody_grant = Self::held_grant_for(state, &receiver_id.to_bytes());
        let pow = if custody_grant.is_none() {
            let day = Timestamp::get_timestamp() / MS_PER_DAY;
            Some(Self::solve_pow(
                &original_signature,
                &target.to_bytes(),
                day,
                V2_MIN_POW_DIFFICULTY,
            ))
        } else {
            None
        };
        let tier = if custody_grant.is_some() {
            TIER_GRANT
        } else {
            TIER_TRUSTED
        };

        let container_v2 = proto::DtnV2Container {
            dtn_route: dtn_route_bytes,
            dtn_route_sig,
            envelope: req.data.clone(),
            custody_grant,
            pow,
        };

        // Send via envelope
        match super::messaging::Messaging::send_dtn_v2_message(
            state,
            &user_account,
            &target,
            container_v2.clone(),
        ) {
            Ok(_sig) => {
                // Store our own copy so the retransmit loop can re-forward and
                // on_dtn_response_v2 can clean up once the receiver confirms
                // delivery. Size accounts the stored payload (envelope).
                let entry_size = container_v2.envelope.len() as u32;
                let v2_entry = DtnRoutedV2Entry {
                    container_v2_bytes: container_v2.encode_to_vec(),
                    sender_public_key: route.sender_public_key.clone(),
                    size: entry_size,
                    accepted_at: Timestamp::get_timestamp(),
                    receiver_id: receiver_id.to_bytes(),
                    tier,
                };
                if let Ok(entry_bytes) = bincode::serialize(&v2_entry) {
                    if let Ok(mut v2) = state.services.dtn.v2.write() {
                        let _ = v2
                            .db_ref_routed_v2
                            .insert(original_signature.clone(), entry_bytes);
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

    /// Handle DtnIssueGrantRequest RPC: the local user (recipient) issues a
    /// signed custody grant authorizing `grantee` to have messages held for it.
    fn rpc_issue_grant(
        state: &crate::QaulState,
        my_user_id: PeerId,
        req: proto_rpc::DtnIssueGrantRequest,
        request_id: String,
    ) {
        let respond = |status: bool, grant: Vec<u8>, message: String| {
            let proto_message = proto_rpc::Dtn {
                message: Some(proto_rpc::dtn::Message::DtnIssueGrantResponse(
                    proto_rpc::DtnIssueGrantResponse {
                        status,
                        grant,
                        message,
                    },
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

        if PeerId::from_bytes(&req.grantee).is_err() {
            respond(false, Vec::new(), "invalid grantee id".to_string());
            return;
        }
        let user_account = match UserAccounts::get_by_id(state, my_user_id) {
            Some(ua) => ua,
            None => {
                respond(false, Vec::new(), "user account not found".to_string());
                return;
            }
        };
        let quota_bytes = if req.quota_bytes > 0 {
            req.quota_bytes
        } else {
            V2_TRUSTED_PER_SENDER_QUOTA
        };
        let grant = proto::CustodyGrant {
            grantee: req.grantee,
            recipient: my_user_id.to_bytes(),
            recipient_public_key: user_account.keys.public().encode_protobuf(),
            quota_bytes,
            epoch: req.epoch,
            not_after: req.not_after,
            signature: Vec::new(),
        };
        match Self::sign_grant(&user_account, grant) {
            Some(signed) => respond(true, signed.encode_to_vec(), "".to_string()),
            None => respond(false, Vec::new(), "grant signing failed".to_string()),
        }
    }

    /// Handle DtnImportGrantRequest RPC: store a grant this node has been given
    /// so the originate path can attach it to outgoing custody messages.
    fn rpc_import_grant(
        state: &crate::QaulState,
        req: proto_rpc::DtnImportGrantRequest,
        request_id: String,
    ) {
        let respond = |status: bool, message: String| {
            let proto_message = proto_rpc::Dtn {
                message: Some(proto_rpc::dtn::Message::DtnImportGrantResponse(
                    proto_rpc::DtnImportGrantResponse { status, message },
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

        let grant = match proto::CustodyGrant::decode(&req.grant[..]) {
            Ok(g) => g,
            Err(e) => {
                respond(false, format!("invalid grant: {}", e));
                return;
            }
        };
        if !Self::verify_grant(&grant, Timestamp::get_timestamp()) {
            respond(false, "grant signature verification failed".to_string());
            return;
        }
        if Self::store_held_grant(state, &grant) {
            respond(true, "".to_string());
        } else {
            respond(false, "failed to store grant".to_string());
        }
    }

    /// Determine the next target for a V2 custody message via stateless
    /// traversal of the immutable signed route.
    ///
    /// Deliver directly to the receiver if reachable; otherwise locate our own
    /// id in the route and forward to the *furthest* reachable hop strictly
    /// after it (skipping dead intermediate hops). If our id is not in the
    /// route (we were reached opportunistically) treat our position as being
    /// before the first hop. The route carries no cursor — position is derived
    /// each time, so the message is never rewritten in transit.
    pub fn select_custody_target(
        state: &crate::QaulState,
        route: &proto::DtnRoute,
        my_id: &PeerId,
        receiver_id: &PeerId,
    ) -> Option<PeerId> {
        let rs = state.get_router();
        // Deliver straight to the recipient when reachable.
        if rs.routing_table.get_route_to_user(*receiver_id).is_some() {
            return Some(*receiver_id);
        }

        let my_bytes = my_id.to_bytes();
        // Our hop index = first hop that lists our id.
        let self_hop = route
            .route_hop
            .iter()
            .position(|hop| hop.route_entry.iter().any(|e| e.id == my_bytes));
        let start_after = match self_hop {
            Some(i) => i + 1,
            None => 0,
        };
        // Hops strictly after ours, furthest first — first reachable wins.
        for hop in route.route_hop[start_after..].iter().rev() {
            for entry in &hop.route_entry {
                if let Ok(custodian_id) = PeerId::from_bytes(&entry.id) {
                    if rs.routing_table.get_route_to_user(custodian_id).is_some() {
                        return Some(custodian_id);
                    }
                }
            }
        }
        None
    }

    /// Free the accounting for a custody entry being removed (does NOT remove
    /// the tree key — the caller does that). Decrements node totals, the
    /// untrusted-pool counter, and the per-sender quota.
    fn free_entry(v2: &mut DtnStorageStateV2, entry: &DtnRoutedV2Entry) {
        v2.used_size = v2.used_size.saturating_sub(entry.size as u64);
        v2.message_count = v2.message_count.saturating_sub(1);
        if entry.tier == TIER_UNTRUSTED {
            v2.untrusted_used = v2.untrusted_used.saturating_sub(entry.size as u64);
        }
        if let Ok(Some(quota_bytes)) = v2.db_ref_sender_quotas.get(&entry.sender_public_key) {
            if let Ok(mut quota) = bincode::deserialize::<SenderQuotaEntry>(&quota_bytes) {
                quota.used_bytes = quota.used_bytes.saturating_sub(entry.size as u64);
                quota.message_count = quota.message_count.saturating_sub(1);
                if let Ok(bytes) = bincode::serialize(&quota) {
                    let _ = v2
                        .db_ref_sender_quotas
                        .insert(entry.sender_public_key.clone(), bytes);
                }
            }
        }
    }

    /// Build and send a signed DtnResponseV2 to `target_id`.
    fn send_response_v2(
        state: &crate::QaulState,
        user_account: &UserAccount,
        target_id: &PeerId,
        kind: proto::dtn_response_v2::Kind,
        response_type: proto::dtn_response_v2::ResponseType,
        reason: proto::dtn_response_v2::Reason,
        original_signature: &[u8],
    ) {
        let resp = match Self::build_signed_response(
            user_account,
            kind,
            response_type,
            reason,
            original_signature.to_vec(),
        ) {
            Some(r) => r,
            None => return,
        };
        let send_message = proto::Messaging {
            message: Some(proto::messaging::Message::DtnResponseV2(resp)),
        };
        if let Err(e) = super::messaging::Messaging::pack_and_send_message(
            state,
            user_account,
            target_id,
            send_message.encode_to_vec(),
            MessagingServiceType::DtnStored,
            &Vec::new(),
            false,
        ) {
            log::error!("DtnV2: send response error: {}", e);
        }
    }

    /// Process a received DtnV2Container from the network.
    ///
    /// The route is verified as a signed, immutable blob and the admission
    /// decision is capability-first: a recipient-signed grant, else a trusted
    /// local contact, else a valid proof-of-work stamp for the grant-less pool.
    pub fn net_routed_v2(
        state: &crate::QaulState,
        user_id: &PeerId,
        sender_id: &PeerId,
        _signature: &[u8],
        container: proto::DtnV2Container,
    ) {
        log::info!("Received DtnV2Container from {}", sender_id.to_base58());
        let now = Timestamp::get_timestamp();

        let user_account = match UserAccounts::get_by_id(state, *user_id) {
            Some(ua) => ua,
            None => {
                log::error!(
                    "DtnV2: user account not found for {}",
                    user_id.to_base58()
                );
                return;
            }
        };

        // 1. Decode and verify the signed immutable route. A route that does
        //    not verify is unauthenticated garbage — drop it silently, do not
        //    even acknowledge (an attacker must not learn anything).
        let route = match proto::DtnRoute::decode(&container.dtn_route[..]) {
            Ok(r) => r,
            Err(e) => {
                log::error!("DtnV2: failed to decode route: {}", e);
                return;
            }
        };
        if !Self::verify_route_sig(&route, &container.dtn_route, &container.dtn_route_sig) {
            log::error!("DtnV2: route signature verification failed, dropping");
            return;
        }
        let original_signature = route.original_signature.clone();

        // 2. Stateless prechecks: expiry, or am-I-the-final-recipient.
        let envelope_receiver = Self::get_receiver_from_container(&container.envelope);
        match Self::precheck_routed_v2(&route, envelope_receiver.as_ref(), user_id, now) {
            V2Precheck::Expired => {
                log::warn!("DtnV2 message expired, dropping");
                Self::send_response_v2(
                    state,
                    &user_account,
                    sender_id,
                    proto::dtn_response_v2::Kind::DropReport,
                    proto::dtn_response_v2::ResponseType::Rejected,
                    proto::dtn_response_v2::Reason::None,
                    &original_signature,
                );
                return;
            }
            V2Precheck::Deliver => {
                // A delivered container that carries a signed DTN response is a
                // DELIVERY ack routed back to us (the original sender) — dispatch
                // it to free our outgoing custody copy, do NOT re-ack it.
                if let Some(ack) = Self::extract_ack(&container.envelope) {
                    log::info!("DtnV2: received routed DELIVERY ack");
                    Self::on_dtn_response_v2(state, &ack);
                    Self::send_response_v2(
                        state,
                        &user_account,
                        sender_id,
                        proto::dtn_response_v2::Kind::CustodyRelease,
                        proto::dtn_response_v2::ResponseType::Accepted,
                        proto::dtn_response_v2::Reason::None,
                        &original_signature,
                    );
                    return;
                }

                log::info!("DtnV2: I am the recipient, processing inner container");
                if let Ok(inner) = proto::Container::decode(&container.envelope[..]) {
                    super::messaging::process::MessagingProcess::process_received_message(
                        state,
                        user_account.clone(),
                        inner,
                    );
                }
                // Authoritative signed DELIVERY ack, routed back to the (possibly
                // offline) original sender over the reverse of the forward route
                // so it survives the sender being offline at delivery time.
                Self::route_delivery_ack(state, &user_account, &route, &original_signature);
                // Custody release to the immediate previous holder so it can
                // free its storage now.
                Self::send_response_v2(
                    state,
                    &user_account,
                    sender_id,
                    proto::dtn_response_v2::Kind::CustodyRelease,
                    proto::dtn_response_v2::ResponseType::Accepted,
                    proto::dtn_response_v2::Reason::None,
                    &original_signature,
                );
                return;
            }
            V2Precheck::Continue => {}
        }

        // 3. Duplicate check — already in custody. Send a receipt and stop.
        {
            let v2 = match state.services.dtn.v2.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnV2: read lock failed: {}", e);
                    return;
                }
            };
            if v2
                .db_ref_routed_v2
                .contains_key(&original_signature)
                .unwrap_or(false)
            {
                drop(v2);
                log::info!("DtnV2 duplicate, accepting silently");
                Self::send_response_v2(
                    state,
                    &user_account,
                    sender_id,
                    proto::dtn_response_v2::Kind::Receipt,
                    proto::dtn_response_v2::ResponseType::Accepted,
                    proto::dtn_response_v2::Reason::None,
                    &original_signature,
                );
                return;
            }
        }

        // 4. Custody opt-in.
        match Configuration::get_user(state, user_account.id.to_string()) {
            Some(profile) if profile.storage.dtn_v2_custody_enabled => {}
            _ => {
                log::warn!("DtnV2: custody not enabled on this node");
                Self::send_response_v2(
                    state,
                    &user_account,
                    sender_id,
                    proto::dtn_response_v2::Kind::DropReport,
                    proto::dtn_response_v2::ResponseType::Rejected,
                    proto::dtn_response_v2::Reason::UserNotAccepted,
                    &original_signature,
                );
                return;
            }
        }

        // 4b. Control traffic (a DELIVERY ack being routed back to the sender)
        //     bypasses the grant/PoW admission gates: it is tied to an
        //     already-accepted custody flow and carries a verifiable signed
        //     response. The route signature (verified above) authenticates it.
        //     We store it on the short control retention and forward it on.
        if Self::extract_ack(&container.envelope).is_some() {
            let entry_size = container.envelope.len() as u32;
            let entry = DtnRoutedV2Entry {
                container_v2_bytes: container.encode_to_vec(),
                sender_public_key: route.sender_public_key.clone(),
                size: entry_size,
                accepted_at: now,
                receiver_id: envelope_receiver
                    .as_ref()
                    .map(|r| r.to_bytes())
                    .unwrap_or_default(),
                tier: TIER_CONTROL,
            };
            if let Ok(bytes) = bincode::serialize(&entry) {
                if let Ok(mut v2) = state.services.dtn.v2.write() {
                    if v2
                        .db_ref_routed_v2
                        .insert(original_signature.clone(), bytes)
                        .is_ok()
                    {
                        let _ = v2.db_ref_routed_v2.flush();
                        v2.used_size += entry_size as u64;
                        v2.message_count += 1;
                    }
                }
            }
            Self::send_response_v2(
                state,
                &user_account,
                sender_id,
                proto::dtn_response_v2::Kind::Receipt,
                proto::dtn_response_v2::ResponseType::Accepted,
                proto::dtn_response_v2::Reason::None,
                &original_signature,
            );
            if let Some(recv) = envelope_receiver {
                Self::try_forward_v2(state, &user_account, &container, &route, &recv);
            }
            return;
        }

        // 5. Inner-container sender signature verification. The signed route
        //    vouches for `sender_public_key`; the inner message must be signed
        //    by that same key.
        let inner = match proto::Container::decode(&container.envelope[..]) {
            Ok(c) => c,
            Err(e) => {
                log::error!("DtnV2: failed to decode inner container: {}", e);
                Self::send_response_v2(
                    state,
                    &user_account,
                    sender_id,
                    proto::dtn_response_v2::Kind::DropReport,
                    proto::dtn_response_v2::ResponseType::Rejected,
                    proto::dtn_response_v2::Reason::None,
                    &original_signature,
                );
                return;
            }
        };
        let sender_pub_key = match PublicKey::try_decode_protobuf(&route.sender_public_key) {
            Ok(k) => k,
            Err(e) => {
                log::error!("DtnV2: invalid sender public key: {}", e);
                return;
            }
        };
        match inner.envelope.as_ref() {
            Some(envelope) => {
                let mut buf = Vec::with_capacity(envelope.encoded_len());
                envelope
                    .encode(&mut buf)
                    .expect("Vec<u8> provides capacity as needed");
                if !sender_pub_key.verify(&buf, &inner.signature) {
                    log::error!("DtnV2: inner container signature verification failed");
                    Self::send_response_v2(
                        state,
                        &user_account,
                        sender_id,
                        proto::dtn_response_v2::Kind::DropReport,
                        proto::dtn_response_v2::ResponseType::Rejected,
                        proto::dtn_response_v2::Reason::UserNotAccepted,
                        &original_signature,
                    );
                    return;
                }
            }
            None => {
                log::error!("DtnV2: inner container has no envelope");
                return;
            }
        }
        let inner_sender = PeerId::from_public_key(&sender_pub_key);
        let inner_sender_bytes = inner_sender.to_bytes();

        // 6. Blocked-sender check.
        {
            let rs = state.get_router();
            if let Some(user) = Users::get_user_snapshot(&rs, &QaulId::to_q8id(inner_sender)) {
                if user.blocked {
                    log::warn!(
                        "DtnV2: rejecting message from blocked sender {}",
                        inner_sender.to_base58()
                    );
                    Self::send_response_v2(
                        state,
                        &user_account,
                        sender_id,
                        proto::dtn_response_v2::Kind::DropReport,
                        proto::dtn_response_v2::ResponseType::Rejected,
                        proto::dtn_response_v2::Reason::Blocked,
                        &original_signature,
                    );
                    return;
                }
            }
        }

        let payload_size = container.envelope.len() as u64;
        let receiver_bytes = envelope_receiver
            .as_ref()
            .map(|r| r.to_bytes())
            .unwrap_or_default();

        // 7. Admission tier: grant → trusted contact → proof-of-work pool.
        let is_trusted = {
            let rs = state.get_router();
            Users::get_user_snapshot(&rs, &QaulId::to_q8id(inner_sender))
                .map(|u| u.verified)
                .unwrap_or(false)
        };
        let grant_ok = match container.custody_grant.as_ref() {
            Some(grant) => {
                Self::verify_grant(grant, now)
                    && grant.grantee == inner_sender_bytes
                    && grant.recipient == receiver_bytes
                    && payload_size <= grant.quota_bytes
            }
            None => false,
        };

        let (tier, per_sender_ceiling) = if grant_ok {
            let ceiling = container
                .custody_grant
                .as_ref()
                .map(|g| g.quota_bytes)
                .unwrap_or(V2_TRUSTED_PER_SENDER_QUOTA);
            (TIER_GRANT, ceiling)
        } else if is_trusted {
            (TIER_TRUSTED, V2_TRUSTED_PER_SENDER_QUOTA)
        } else {
            // Grant-less stranger: a proof-of-work stamp bound to THIS custodian
            // is required. This gates the grant-less untrusted pool; without it
            // a Sybil flood is free.
            match container.pow.as_ref() {
                Some(pow) if Self::verify_pow(pow, &original_signature, &user_id.to_bytes(), now) => {
                    // Aggregate untrusted-pool cap — the real anti-flood bound.
                    let pool_used = state
                        .services
                        .dtn
                        .v2
                        .read()
                        .map(|v2| v2.untrusted_used)
                        .unwrap_or(u64::MAX);
                    if pool_used + payload_size > V2_UNTRUSTED_POOL_QUOTA {
                        log::warn!("DtnV2: untrusted pool full, rejecting");
                        Self::send_response_v2(
                            state,
                            &user_account,
                            sender_id,
                            proto::dtn_response_v2::Kind::DropReport,
                            proto::dtn_response_v2::ResponseType::Rejected,
                            proto::dtn_response_v2::Reason::OverallQuota,
                            &original_signature,
                        );
                        return;
                    }
                    (TIER_UNTRUSTED, V2_UNTRUSTED_PER_SENDER_QUOTA)
                }
                Some(_) => {
                    log::warn!("DtnV2: invalid proof-of-work, rejecting");
                    Self::send_response_v2(
                        state,
                        &user_account,
                        sender_id,
                        proto::dtn_response_v2::Kind::DropReport,
                        proto::dtn_response_v2::ResponseType::Rejected,
                        proto::dtn_response_v2::Reason::NoGrant,
                        &original_signature,
                    );
                    return;
                }
                None => {
                    log::warn!("DtnV2: grant-less deposit without proof-of-work, rejecting");
                    Self::send_response_v2(
                        state,
                        &user_account,
                        sender_id,
                        proto::dtn_response_v2::Kind::DropReport,
                        proto::dtn_response_v2::ResponseType::Rejected,
                        proto::dtn_response_v2::Reason::PowRequired,
                        &original_signature,
                    );
                    return;
                }
            }
        };

        // 8. Per-sender quota against the tier ceiling.
        {
            let v2 = match state.services.dtn.v2.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnV2: read lock failed for quota: {}", e);
                    return;
                }
            };
            if let Ok(Some(quota_bytes)) = v2.db_ref_sender_quotas.get(&route.sender_public_key) {
                if let Ok(quota) = bincode::deserialize::<SenderQuotaEntry>(&quota_bytes) {
                    if quota.used_bytes + payload_size > per_sender_ceiling {
                        drop(v2);
                        log::warn!("DtnV2: per-sender quota exceeded");
                        Self::send_response_v2(
                            state,
                            &user_account,
                            sender_id,
                            proto::dtn_response_v2::Kind::DropReport,
                            proto::dtn_response_v2::ResponseType::Rejected,
                            proto::dtn_response_v2::Reason::UserQuota,
                            &original_signature,
                        );
                        return;
                    }
                }
            }
        }

        // 9. Overall node quota (V1 + V2).
        {
            let v2 = match state.services.dtn.v2.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnV2: read lock failed for overall quota: {}", e);
                    return;
                }
            };
            let v1 = match state.services.dtn.inner.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnV2: V1 read lock failed: {}", e);
                    return;
                }
            };
            if let Some(profile) = Configuration::get_user(state, user_account.id.to_string()) {
                let total_limit = (profile.storage.size_total as u64) * 1024 * 1024;
                if v1.used_size + v2.used_size + payload_size > total_limit {
                    drop(v1);
                    drop(v2);
                    log::warn!("DtnV2: overall quota exceeded");
                    Self::send_response_v2(
                        state,
                        &user_account,
                        sender_id,
                        proto::dtn_response_v2::Kind::DropReport,
                        proto::dtn_response_v2::ResponseType::Rejected,
                        proto::dtn_response_v2::Reason::OverallQuota,
                        &original_signature,
                    );
                    return;
                }
            }
        }

        // 10. Accept custody: store the entry and update accounting.
        let entry_size = payload_size as u32;
        let v2_entry = DtnRoutedV2Entry {
            container_v2_bytes: container.encode_to_vec(),
            sender_public_key: route.sender_public_key.clone(),
            size: entry_size,
            accepted_at: now,
            receiver_id: receiver_bytes,
            tier,
        };
        let entry_bytes = match bincode::serialize(&v2_entry) {
            Ok(b) => b,
            Err(e) => {
                log::error!("DtnV2: failed to serialize entry: {}", e);
                return;
            }
        };
        {
            let mut v2 = match state.services.dtn.v2.write() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnV2: write lock failed: {}", e);
                    return;
                }
            };
            if let Err(e) = v2
                .db_ref_routed_v2
                .insert(original_signature.clone(), entry_bytes)
            {
                log::error!("DtnV2: storage insert error: {}", e);
                return;
            }
            let _ = v2.db_ref_routed_v2.flush();
            v2.used_size += payload_size;
            v2.message_count += 1;
            if tier == TIER_UNTRUSTED {
                v2.untrusted_used += payload_size;
            }
            let mut quota = if let Ok(Some(quota_bytes)) =
                v2.db_ref_sender_quotas.get(&route.sender_public_key)
            {
                bincode::deserialize::<SenderQuotaEntry>(&quota_bytes).unwrap_or_default()
            } else {
                SenderQuotaEntry::default()
            };
            quota.used_bytes += payload_size;
            quota.message_count += 1;
            if let Ok(bytes) = bincode::serialize(&quota) {
                let _ = v2
                    .db_ref_sender_quotas
                    .insert(route.sender_public_key.clone(), bytes);
            }
            let _ = v2.db_ref_sender_quotas.flush();
        }

        // 11. Signed receipt to the previous holder, then immediate forward.
        Self::send_response_v2(
            state,
            &user_account,
            sender_id,
            proto::dtn_response_v2::Kind::Receipt,
            proto::dtn_response_v2::ResponseType::Accepted,
            proto::dtn_response_v2::Reason::None,
            &original_signature,
        );
        if let Some(recv) = envelope_receiver {
            Self::try_forward_v2(state, &user_account, &container, &route, &recv);
        }
    }

    /// Try to forward a stored V2 message to the next reachable hop or the
    /// recipient. The route is immutable, so forwarding re-sends the same
    /// container unchanged.
    fn try_forward_v2(
        state: &crate::QaulState,
        user_account: &UserAccount,
        container: &proto::DtnV2Container,
        route: &proto::DtnRoute,
        receiver_id: &PeerId,
    ) {
        if let Some(target) =
            Self::select_custody_target(state, route, &user_account.id, receiver_id)
        {
            if let Err(e) = super::messaging::Messaging::send_dtn_v2_message(
                state,
                user_account,
                &target,
                container.clone(),
            ) {
                log::error!("DtnV2: forward error: {}", e);
            }
        }
    }

    /// Stateless checks for an incoming DtnV2Container.
    ///
    /// The recipient check runs before anything else that could drop the
    /// message: a message that reaches its final recipient must be delivered
    /// even if it would otherwise be past policy.
    fn precheck_routed_v2(
        route: &proto::DtnRoute,
        envelope_receiver: Option<&PeerId>,
        my_id: &PeerId,
        now: u64,
    ) -> V2Precheck {
        if envelope_receiver == Some(my_id) {
            return V2Precheck::Deliver;
        }
        if let Some(expires_at) = route.expires_at {
            if expires_at > 0 && now > expires_at {
                return V2Precheck::Expired;
            }
        }
        V2Precheck::Continue
    }

    /// Extract the receiver PeerId from a serialized inner Container.
    fn get_receiver_from_container(container_bytes: &[u8]) -> Option<PeerId> {
        if let Ok(container) = proto::Container::decode(container_bytes) {
            if let Some(envelope) = container.envelope {
                return PeerId::from_bytes(&envelope.receiver_id).ok();
            }
        }
        None
    }

    /// Handle a signed DtnResponseV2.
    ///
    /// The signature is verified against the responder's embedded key BEFORE
    /// any state change — this is the fix for the old unauthenticated ACK,
    /// where any peer could forge a response and free (or falsely confirm)
    /// custody storage. Only a validly-signed response mutates the store.
    pub fn on_dtn_response_v2(state: &crate::QaulState, resp: &proto::DtnResponseV2) {
        let responder_id = match Self::verify_response_v2(resp) {
            Some(id) => id,
            None => {
                log::warn!("DtnV2: dropping response with invalid signature");
                return;
            }
        };

        let kind = proto::dtn_response_v2::Kind::try_from(resp.kind)
            .unwrap_or(proto::dtn_response_v2::Kind::Receipt);
        let accepted =
            resp.response_type == proto::dtn_response_v2::ResponseType::Accepted as i32;

        match kind {
            proto::dtn_response_v2::Kind::Delivery
            | proto::dtn_response_v2::Kind::CustodyRelease
                if accepted =>
            {
                let mut v2 = match state.services.dtn.v2.write() {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("DtnV2: write lock failed for response: {}", e);
                        return;
                    }
                };
                let entry_bytes = match v2.db_ref_routed_v2.get(&resp.original_signature) {
                    Ok(Some(b)) => b,
                    _ => return,
                };
                let entry = match bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                    Ok(e) => e,
                    Err(e) => {
                        log::error!("DtnV2: response entry deserialize: {}", e);
                        return;
                    }
                };
                // A DELIVERY ack is authoritative only from the real receiver.
                if kind == proto::dtn_response_v2::Kind::Delivery
                    && !entry.receiver_id.is_empty()
                    && responder_id != entry.receiver_id
                {
                    log::warn!("DtnV2: DELIVERY ack responder is not the receiver, ignoring");
                    return;
                }
                Self::free_entry(&mut v2, &entry);
                let _ = v2.db_ref_routed_v2.remove(&resp.original_signature);
                let _ = v2.db_ref_routed_v2.flush();
                let _ = v2.db_ref_sender_quotas.flush();
            }
            proto::dtn_response_v2::Kind::DropReport => {
                // A downstream custodian dropped the message. This is the
                // origin sender's cue to re-route; we do not delete our own
                // custody copy on someone else's drop.
                log::info!("DtnV2: received DROP_REPORT (reason {})", resp.reason);
            }
            _ => {
                // RECEIPT (or a non-accepted DELIVERY/RELEASE): custody stays
                // held until an authoritative delivery/release arrives.
                log::debug!("DtnV2: response kind {} noted", resp.kind);
            }
        }
    }

    /// Periodic retention sweep + re-forward for stored V2 custody messages.
    pub fn process_retransmit_v2(state: &crate::QaulState) {
        let now = Timestamp::get_timestamp();
        let mut to_remove: Vec<Vec<u8>> = Vec::new();
        let mut to_forward: Vec<DtnRoutedV2Entry> = Vec::new();

        {
            let v2 = match state.services.dtn.v2.read() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnV2: read lock failed for retransmit: {}", e);
                    return;
                }
            };
            for item in v2.db_ref_routed_v2.iter() {
                let (sig, entry_bytes) = match item {
                    Ok(kv) => kv,
                    Err(_) => continue,
                };
                let v2_entry = match bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                // Retention: an explicit sender expiry wins; otherwise a
                // tier-dependent local cap counted from acceptance (untrusted
                // entries are shed first / soonest).
                let route_expiry = proto::DtnV2Container::decode(&v2_entry.container_v2_bytes[..])
                    .ok()
                    .and_then(|c| proto::DtnRoute::decode(&c.dtn_route[..]).ok())
                    .and_then(|r| r.expires_at)
                    .filter(|e| *e > 0);
                let retention_ms = match v2_entry.tier {
                    TIER_UNTRUSTED | TIER_CONTROL => V2_MAX_RETENTION_MS,
                    _ => V2_TRUSTED_RETENTION_MS,
                };
                let effective_expires_at =
                    route_expiry.unwrap_or_else(|| v2_entry.accepted_at.saturating_add(retention_ms));
                if now > effective_expires_at {
                    to_remove.push(sig.to_vec());
                    continue;
                }
                to_forward.push(v2_entry);
            }
        }

        if !to_remove.is_empty() {
            let mut v2 = match state.services.dtn.v2.write() {
                Ok(s) => s,
                Err(e) => {
                    log::error!("DtnV2: write lock failed for cleanup: {}", e);
                    return;
                }
            };
            for sig in &to_remove {
                if let Ok(Some(entry_bytes)) = v2.db_ref_routed_v2.get(sig) {
                    if let Ok(entry) = bincode::deserialize::<DtnRoutedV2Entry>(&entry_bytes) {
                        Self::free_entry(&mut v2, &entry);
                    }
                }
                let _ = v2.db_ref_routed_v2.remove(sig);
            }
            let _ = v2.db_ref_routed_v2.flush();
            let _ = v2.db_ref_sender_quotas.flush();
        }

        for v2_entry in &to_forward {
            let container = match proto::DtnV2Container::decode(&v2_entry.container_v2_bytes[..]) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let route = match proto::DtnRoute::decode(&container.dtn_route[..]) {
                Ok(r) => r,
                Err(_) => continue,
            };
            if let Ok(recv_id) = PeerId::from_bytes(&v2_entry.receiver_id) {
                if let Some(user_account) = UserAccounts::get_default_user(state) {
                    Self::try_forward_v2(state, &user_account, &container, &route, &recv_id);
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

    // ── V1 response accounting (unchanged behaviour, must keep passing) ──

    // A delivered DTN message must FREE the storage it occupied and clear both
    // index trees together (atomic removal).
    #[test]
    fn dtn_response_frees_storage_and_clears_both_trees() {
        let dtn = DtnModuleState::new();
        let sig = vec![1u8; 16];
        let org_sig = vec![2u8; 16];
        let entry = DtnMessageEntry {
            org_sig: org_sig.clone(),
            size: 500,
        };
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
        let resp = proto::DtnResponse {
            signature: sig.clone(),
            ..Default::default()
        };
        dtn.on_dtn_response(&resp);

        let st = dtn.inner.read().unwrap();
        assert_eq!(st.used_size, 0, "delivered message must free its storage");
        assert_eq!(st.message_counts, 0);
        assert!(!st.db_ref.contains_key(sig.as_slice()).unwrap());
        assert!(!st.db_ref_id.contains_key(org_sig.as_slice()).unwrap());
    }

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

    // ── Helpers ──

    fn random_peer() -> PeerId {
        PeerId::from(Keypair::generate_ed25519().public())
    }

    fn account_from(keys: Keypair) -> UserAccount {
        let id = PeerId::from(keys.public());
        UserAccount {
            id,
            keys,
            name: "u".to_string(),
            password_hash: None,
            password_salt: None,
        }
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

    /// Sign an inner Container for `receiver_keys`. Returns
    /// (sender_keys, container_bytes, original_signature).
    fn signed_inner(receiver_keys: &Keypair) -> (Keypair, Vec<u8>, Vec<u8>) {
        let sender_keys = Keypair::generate_ed25519();
        let sender = PeerId::from(sender_keys.public());
        let receiver = PeerId::from(receiver_keys.public());
        let envelope = proto::Envelope {
            sender_id: sender.to_bytes(),
            receiver_id: receiver.to_bytes(),
            payload: vec![1, 2, 3],
        };
        let mut buf = Vec::with_capacity(envelope.encoded_len());
        envelope.encode(&mut buf).unwrap();
        let signature = sender_keys.sign(&buf).unwrap();
        let container = proto::Container {
            signature: signature.clone(),
            envelope: Some(envelope),
        };
        (sender_keys, container.encode_to_vec(), signature)
    }

    /// Build a signed DtnV2Container around a signed inner container.
    fn build_container_v2(
        sender_keys: &Keypair,
        container_bytes: Vec<u8>,
        original_signature: Vec<u8>,
        custodians: Vec<Vec<u8>>,
        grant: Option<proto::CustodyGrant>,
        pow: Option<proto::PowStamp>,
        expires_at: Option<u64>,
    ) -> proto::DtnV2Container {
        let route = Dtn::build_route(
            original_signature,
            &custodians,
            sender_keys.public().encode_protobuf(),
            expires_at,
        );
        let route_bytes = route.encode_to_vec();
        let sig = sender_keys.sign(&route_bytes).unwrap();
        proto::DtnV2Container {
            dtn_route: route_bytes,
            dtn_route_sig: sig,
            envelope: container_bytes,
            custody_grant: grant,
            pow,
        }
    }

    /// A recipient-signed custody grant for `grantee`.
    fn make_grant(receiver_keys: &Keypair, grantee: Vec<u8>, quota: u64, not_after: u64) -> proto::CustodyGrant {
        let recipient = PeerId::from(receiver_keys.public());
        let mut grant = proto::CustodyGrant {
            grantee,
            recipient: recipient.to_bytes(),
            recipient_public_key: receiver_keys.public().encode_protobuf(),
            quota_bytes: quota,
            epoch: 1,
            not_after,
            signature: Vec::new(),
        };
        grant.signature = receiver_keys.sign(&grant.encode_to_vec()).unwrap();
        grant
    }

    /// Simulation state with one local account whose profile has V2 custody
    /// enabled. Returns (state, custodian_account).
    fn make_custody_state() -> (crate::QaulState, UserAccount) {
        let state = crate::QaulState::new_for_simulation();
        let account = account_from(Keypair::generate_ed25519());
        match state.user_accounts.inner.write() {
            Ok(mut users) => users.users.push(account.clone()),
            Err(e) => panic!("user accounts lock poisoned: {}", e),
        }
        match state.config.inner.write() {
            Ok(mut cfg) => {
                cfg.user_accounts
                    .push(crate::storage::configuration::UserAccount {
                        id: account.id.to_string(),
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

    // ── Signed immutable route ──

    // The route is the integrity anchor: a valid sender signature must verify,
    // and any tampering with the route bytes or a wrong key must be rejected —
    // otherwise a custodian could rewrite the route undetected.
    #[test]
    fn route_signature_verifies_and_rejects_tampering() {
        let keys = Keypair::generate_ed25519();
        let route = Dtn::build_route(
            vec![0xAA],
            &[random_peer().to_bytes()],
            keys.public().encode_protobuf(),
            None,
        );
        let bytes = route.encode_to_vec();
        let sig = keys.sign(&bytes).unwrap();
        assert!(Dtn::verify_route_sig(&route, &bytes, &sig));

        // tampered bytes
        let mut tampered = bytes.clone();
        tampered[0] ^= 0xFF;
        assert!(!Dtn::verify_route_sig(&route, &tampered, &sig));

        // wrong signing key
        let other = Keypair::generate_ed25519();
        let wrong_sig = other.sign(&bytes).unwrap();
        assert!(!Dtn::verify_route_sig(&route, &bytes, &wrong_sig));
    }

    // ── Custody grants ──

    #[test]
    fn grant_valid_verifies() {
        let receiver_keys = Keypair::generate_ed25519();
        let grantee = random_peer().to_bytes();
        let grant = make_grant(&receiver_keys, grantee, 1024, 0);
        assert!(Dtn::verify_grant(&grant, 1000));
    }

    // A grant whose embedded key is not the named recipient is a forgery.
    #[test]
    fn grant_rejects_mismatched_recipient_key() {
        let receiver_keys = Keypair::generate_ed25519();
        let mut grant = make_grant(&receiver_keys, random_peer().to_bytes(), 1024, 0);
        // Replace the recipient id with someone else while keeping the key.
        grant.recipient = random_peer().to_bytes();
        assert!(!Dtn::verify_grant(&grant, 1000));
    }

    #[test]
    fn grant_rejects_expired() {
        let receiver_keys = Keypair::generate_ed25519();
        let grant = make_grant(&receiver_keys, random_peer().to_bytes(), 1024, 500);
        assert!(!Dtn::verify_grant(&grant, 1000), "grant past not_after must fail");
        assert!(Dtn::verify_grant(&grant, 400), "grant before not_after must pass");
    }

    #[test]
    fn grant_rejects_tampered_signature() {
        let receiver_keys = Keypair::generate_ed25519();
        let mut grant = make_grant(&receiver_keys, random_peer().to_bytes(), 1024, 0);
        grant.quota_bytes = 999_999; // change a signed field after signing
        assert!(!Dtn::verify_grant(&grant, 1000));
    }

    // ── Proof of work ──

    #[test]
    fn pow_solve_then_verify() {
        let sig = vec![7u8; 32];
        let custodian = random_peer().to_bytes();
        let day = 20_000u64;
        let stamp = Dtn::solve_pow(&sig, &custodian, day, V2_MIN_POW_DIFFICULTY);
        let now = day * MS_PER_DAY + 5;
        assert!(Dtn::verify_pow(&stamp, &sig, &custodian, now));
    }

    // A stamp below the minimum difficulty is rejected regardless of its hash.
    #[test]
    fn pow_rejects_below_min_difficulty() {
        let sig = vec![7u8; 32];
        let custodian = random_peer().to_bytes();
        let day = 20_000u64;
        let mut stamp = Dtn::solve_pow(&sig, &custodian, day, V2_MIN_POW_DIFFICULTY);
        stamp.difficulty = V2_MIN_POW_DIFFICULTY - 1;
        assert!(!Dtn::verify_pow(&stamp, &sig, &custodian, day * MS_PER_DAY));
    }

    // A stamp solved for one custodian cannot be replayed against another —
    // this is what stops a single solved stamp fanning out across the mesh.
    #[test]
    fn pow_bound_to_custodian_and_day() {
        let sig = vec![7u8; 32];
        let custodian = random_peer().to_bytes();
        let day = 20_000u64;
        let stamp = Dtn::solve_pow(&sig, &custodian, day, V2_MIN_POW_DIFFICULTY);

        let other_custodian = random_peer().to_bytes();
        assert!(!Dtn::verify_pow(&stamp, &sig, &other_custodian, day * MS_PER_DAY));

        // Two days later (outside the today/yesterday window) it is stale.
        let much_later = (day + 2) * MS_PER_DAY;
        assert!(!Dtn::verify_pow(&stamp, &sig, &custodian, much_later));
    }

    #[test]
    fn leading_zero_bits_counts_msb_first() {
        let mut h = [0u8; 32];
        assert_eq!(Dtn::leading_zero_bits(&h), 256);
        h[0] = 0b0001_0000;
        assert_eq!(Dtn::leading_zero_bits(&h), 3);
    }

    // ── Signed responses ──

    #[test]
    fn signed_response_verifies_and_returns_responder() {
        let account = account_from(Keypair::generate_ed25519());
        let resp = Dtn::build_signed_response(
            &account,
            proto::dtn_response_v2::Kind::Delivery,
            proto::dtn_response_v2::ResponseType::Accepted,
            proto::dtn_response_v2::Reason::None,
            vec![0xAB],
        )
        .expect("sign");
        assert_eq!(Dtn::verify_response_v2(&resp), Some(account.id.to_bytes()));
    }

    // A forged response signature must not verify — the guard that stops a
    // malicious custodian from forging "delivered" and silencing the sender.
    #[test]
    fn forged_response_signature_is_rejected() {
        let account = account_from(Keypair::generate_ed25519());
        let mut resp = Dtn::build_signed_response(
            &account,
            proto::dtn_response_v2::Kind::Delivery,
            proto::dtn_response_v2::ResponseType::Accepted,
            proto::dtn_response_v2::Reason::None,
            vec![0xAB],
        )
        .expect("sign");
        resp.signature[0] ^= 0xFF;
        assert_eq!(Dtn::verify_response_v2(&resp), None);
    }

    // ── Prechecks ──

    #[test]
    fn precheck_delivers_to_recipient() {
        let me = random_peer();
        let route = Dtn::build_route(vec![], &[], vec![], None);
        assert_eq!(
            Dtn::precheck_routed_v2(&route, Some(&me), &me, 1000),
            V2Precheck::Deliver
        );
    }

    #[test]
    fn precheck_expired_when_past_expiry() {
        let me = random_peer();
        let other = random_peer();
        let route = Dtn::build_route(vec![], &[], vec![], Some(500));
        assert_eq!(
            Dtn::precheck_routed_v2(&route, Some(&other), &me, 1000),
            V2Precheck::Expired
        );
    }

    #[test]
    fn precheck_continues_for_custodian() {
        let me = random_peer();
        let other = random_peer();
        let route = Dtn::build_route(vec![], &[], vec![], None);
        assert_eq!(
            Dtn::precheck_routed_v2(&route, Some(&other), &me, 1000),
            V2Precheck::Continue
        );
    }

    // ── Stateless traversal ──

    #[test]
    fn select_target_returns_recipient_when_online() {
        let state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let sender = random_peer();
        let mut table = HashMap::new();
        make_online(&mut table, recipient);
        state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });
        let route = Dtn::build_route(vec![0xAA], &[random_peer().to_bytes()], vec![], None);
        assert_eq!(
            Dtn::select_custody_target(&state, &route, &sender, &recipient),
            Some(recipient)
        );
    }

    // Forward to the FURTHEST reachable hop, skipping a dead intermediate hop.
    #[test]
    fn select_target_picks_furthest_reachable_hop() {
        let state = crate::QaulState::new_for_simulation();
        let recipient = random_peer();
        let sender = random_peer();
        let c1 = random_peer();
        let c2 = random_peer();
        let c3 = random_peer();
        let mut table = HashMap::new();
        make_online(&mut table, c1);
        make_online(&mut table, c3); // c2 is offline
        state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable { table });
        let route = Dtn::build_route(
            vec![0xAA],
            &[c1.to_bytes(), c2.to_bytes(), c3.to_bytes()],
            vec![],
            None,
        );
        assert_eq!(
            Dtn::select_custody_target(&state, &route, &sender, &recipient),
            Some(c3)
        );
    }

    #[test]
    fn select_target_none_when_nobody_online() {
        let state = crate::QaulState::new_for_simulation();
        state
            .get_router()
            .routing_table
            .set(crate::router::table::RoutingTable {
                table: HashMap::new(),
            });
        let route = Dtn::build_route(vec![0xAA], &[random_peer().to_bytes()], vec![], None);
        assert_eq!(
            Dtn::select_custody_target(&state, &route, &random_peer(), &random_peer()),
            None
        );
    }

    // ── Admission pipeline (end to end) ──

    #[test]
    fn admission_accepts_grant_backed_message() {
        let (state, account) = make_custody_state();
        let receiver_keys = Keypair::generate_ed25519();
        let (sender_keys, container_bytes, sig) = signed_inner(&receiver_keys);
        let sender_id = PeerId::from(sender_keys.public());
        let grant = make_grant(
            &receiver_keys,
            sender_id.to_bytes(),
            10 * 1024 * 1024,
            0,
        );
        let container = build_container_v2(
            &sender_keys,
            container_bytes,
            sig.clone(),
            vec![random_peer().to_bytes()],
            Some(grant),
            None,
            None,
        );
        Dtn::net_routed_v2(&state, &account.id, &sender_id, &[], container);

        let v2 = state.services.dtn.v2.read().unwrap();
        let stored = v2.db_ref_routed_v2.get(&sig).unwrap();
        assert!(stored.is_some(), "grant-backed message must be admitted");
        let entry: DtnRoutedV2Entry = bincode::deserialize(&stored.unwrap()).unwrap();
        assert_eq!(entry.tier, TIER_GRANT);
    }

    #[test]
    fn admission_accepts_pow_stranger_into_untrusted_pool() {
        let (state, account) = make_custody_state();
        let receiver_keys = Keypair::generate_ed25519();
        let (sender_keys, container_bytes, sig) = signed_inner(&receiver_keys);
        let sender_id = PeerId::from(sender_keys.public());
        let day = Timestamp::get_timestamp() / MS_PER_DAY;
        let pow = Dtn::solve_pow(&sig, &account.id.to_bytes(), day, V2_MIN_POW_DIFFICULTY);
        let container = build_container_v2(
            &sender_keys,
            container_bytes,
            sig.clone(),
            vec![random_peer().to_bytes()],
            None,
            Some(pow),
            None,
        );
        Dtn::net_routed_v2(&state, &account.id, &sender_id, &[], container);

        let v2 = state.services.dtn.v2.read().unwrap();
        let stored = v2.db_ref_routed_v2.get(&sig).unwrap();
        assert!(stored.is_some(), "valid PoW stranger must be admitted");
        let entry: DtnRoutedV2Entry = bincode::deserialize(&stored.unwrap()).unwrap();
        assert_eq!(entry.tier, TIER_UNTRUSTED);
        assert!(v2.untrusted_used > 0, "untrusted pool accounting must update");
    }

    // A grant-less stranger with no proof of work is the Sybil-flood vector —
    // it must be rejected at admission, before consuming any storage.
    #[test]
    fn admission_rejects_grantless_stranger_without_pow() {
        let (state, account) = make_custody_state();
        let receiver_keys = Keypair::generate_ed25519();
        let (sender_keys, container_bytes, sig) = signed_inner(&receiver_keys);
        let sender_id = PeerId::from(sender_keys.public());
        let container = build_container_v2(
            &sender_keys,
            container_bytes,
            sig.clone(),
            vec![random_peer().to_bytes()],
            None,
            None,
            None,
        );
        Dtn::net_routed_v2(&state, &account.id, &sender_id, &[], container);

        let v2 = state.services.dtn.v2.read().unwrap();
        assert!(
            v2.db_ref_routed_v2.get(&sig).unwrap().is_none(),
            "grant-less no-PoW stranger must not consume storage"
        );
    }

    // ── Signed response handling (security) ──

    #[test]
    fn valid_delivery_ack_frees_custody() {
        let (state, _account) = make_custody_state();
        let receiver_keys = Keypair::generate_ed25519();
        let receiver_account = account_from(receiver_keys);
        let sig = vec![0x42; 16];
        // seed a stored entry addressed to receiver
        {
            let mut v2 = state.services.dtn.v2.write().unwrap();
            let entry = DtnRoutedV2Entry {
                container_v2_bytes: vec![],
                sender_public_key: vec![0x01],
                size: 100,
                accepted_at: 0,
                receiver_id: receiver_account.id.to_bytes(),
                tier: TIER_GRANT,
            };
            v2.db_ref_routed_v2
                .insert(sig.clone(), bincode::serialize(&entry).unwrap())
                .unwrap();
            v2.used_size += 100;
            v2.message_count += 1;
        }
        let resp = Dtn::build_signed_response(
            &receiver_account,
            proto::dtn_response_v2::Kind::Delivery,
            proto::dtn_response_v2::ResponseType::Accepted,
            proto::dtn_response_v2::Reason::None,
            sig.clone(),
        )
        .expect("sign");
        Dtn::on_dtn_response_v2(&state, &resp);

        let v2 = state.services.dtn.v2.read().unwrap();
        assert!(v2.db_ref_routed_v2.get(&sig).unwrap().is_none());
        assert_eq!(v2.used_size, 0);
    }

    // A forged (badly-signed) response must NOT mutate custody storage. This is
    // the core fix over the old unauthenticated ACK, where any peer could free
    // or falsely confirm a stored message.
    #[test]
    fn forged_response_does_not_touch_custody() {
        let (state, _account) = make_custody_state();
        let receiver_account = account_from(Keypair::generate_ed25519());
        let sig = vec![0x42; 16];
        {
            let mut v2 = state.services.dtn.v2.write().unwrap();
            let entry = DtnRoutedV2Entry {
                container_v2_bytes: vec![],
                sender_public_key: vec![0x01],
                size: 100,
                accepted_at: 0,
                receiver_id: receiver_account.id.to_bytes(),
                tier: TIER_GRANT,
            };
            v2.db_ref_routed_v2
                .insert(sig.clone(), bincode::serialize(&entry).unwrap())
                .unwrap();
            v2.used_size += 100;
            v2.message_count += 1;
        }
        let mut resp = Dtn::build_signed_response(
            &receiver_account,
            proto::dtn_response_v2::Kind::Delivery,
            proto::dtn_response_v2::ResponseType::Accepted,
            proto::dtn_response_v2::Reason::None,
            sig.clone(),
        )
        .expect("sign");
        resp.signature[0] ^= 0xFF; // forge

        Dtn::on_dtn_response_v2(&state, &resp);

        let v2 = state.services.dtn.v2.read().unwrap();
        assert!(
            v2.db_ref_routed_v2.get(&sig).unwrap().is_some(),
            "forged response must leave custody untouched"
        );
        assert_eq!(v2.used_size, 100);
    }

    // ── Reverse-routed DELIVERY ack (survives an offline sender) ──

    /// Build a control-ack DtnV2Container: an inner container carrying a signed
    /// DtnResponseV2(DELIVERY) addressed to `origin`, over `custodians`.
    fn build_ack_container(
        acker: &Keypair,
        origin: &PeerId,
        custodians: Vec<Vec<u8>>,
    ) -> proto::DtnV2Container {
        let mut ack = proto::DtnResponseV2 {
            kind: proto::dtn_response_v2::Kind::Delivery as i32,
            response_type: proto::dtn_response_v2::ResponseType::Accepted as i32,
            reason: proto::dtn_response_v2::Reason::None as i32,
            original_signature: vec![0x99],
            responder_public_key: acker.public().encode_protobuf(),
            signature: Vec::new(),
        };
        ack.signature = acker.sign(&ack.encode_to_vec()).unwrap();
        let payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::DtnResponseV2(ack)),
        };
        let env = proto::Envelope {
            sender_id: PeerId::from(acker.public()).to_bytes(),
            receiver_id: origin.to_bytes(),
            payload: payload.encode_to_vec(),
        };
        let inner = proto::Container {
            signature: acker.sign(&env.encode_to_vec()).unwrap(),
            envelope: Some(env),
        };
        let route = Dtn::build_route(
            vec![0xAC],
            &custodians,
            acker.public().encode_protobuf(),
            None,
        );
        let rb = route.encode_to_vec();
        let rs = acker.sign(&rb).unwrap();
        proto::DtnV2Container {
            dtn_route: rb,
            dtn_route_sig: rs,
            envelope: inner.encode_to_vec(),
            custody_grant: None,
            pow: None,
        }
    }

    // On delivery, the receiver must route a DELIVERY ack back toward the
    // sender and keep a copy (TIER_CONTROL) so the retransmit loop retries it —
    // this is what lets the ack survive an offline sender.
    #[test]
    fn delivery_routes_ack_back_to_sender() {
        let (state, account) = make_custody_state();
        // message addressed to the local account, so it is delivered here
        let (sender_keys, container_bytes, sig) = signed_inner(&account.keys);
        let sender_id = PeerId::from(sender_keys.public());
        let container = build_container_v2(
            &sender_keys,
            container_bytes,
            sig,
            vec![random_peer().to_bytes()],
            None,
            None,
            None,
        );
        Dtn::net_routed_v2(&state, &account.id, &sender_id, &[], container);

        let v2 = state.services.dtn.v2.read().unwrap();
        let has_control_ack = v2.db_ref_routed_v2.iter().filter_map(|kv| kv.ok()).any(|(_, b)| {
            bincode::deserialize::<DtnRoutedV2Entry>(&b)
                .map(|e| e.tier == TIER_CONTROL)
                .unwrap_or(false)
        });
        assert!(has_control_ack, "delivery must store a reverse-routed ack copy");
    }

    // A custodian forwards a routed DELIVERY ack without a grant or PoW — a
    // grant-less ordinary message would be rejected, but control traffic is
    // exempt because it is tied to an already-accepted custody flow.
    #[test]
    fn custodian_accepts_control_ack_without_admission() {
        let (state, account) = make_custody_state();
        let acker = Keypair::generate_ed25519();
        let origin = random_peer(); // ultimate sender, not this custodian
        let container = build_ack_container(&acker, &origin, vec![random_peer().to_bytes()]);
        Dtn::net_routed_v2(
            &state,
            &account.id,
            &PeerId::from(acker.public()),
            &[],
            container,
        );

        let v2 = state.services.dtn.v2.read().unwrap();
        let stored = v2.db_ref_routed_v2.get(&vec![0xAC]).unwrap();
        assert!(stored.is_some(), "control ack must be admitted without grant/PoW");
        let entry: DtnRoutedV2Entry = bincode::deserialize(&stored.unwrap()).unwrap();
        assert_eq!(entry.tier, TIER_CONTROL);
    }

    // ── Retention ──

    // An untrusted entry with no explicit expiry must be swept once it exceeds
    // the untrusted retention window, so a never-reachable recipient can't pin
    // grant-less storage forever.
    #[test]
    fn retransmit_sweeps_untrusted_after_max_retention() {
        let (state, _account) = make_custody_state();
        let now = Timestamp::get_timestamp();
        let sig = vec![0x55; 16];
        {
            let mut v2 = state.services.dtn.v2.write().unwrap();
            let entry = DtnRoutedV2Entry {
                container_v2_bytes: proto::DtnV2Container {
                    dtn_route: Dtn::build_route(sig.clone(), &[], vec![], None).encode_to_vec(),
                    dtn_route_sig: vec![],
                    envelope: vec![],
                    custody_grant: None,
                    pow: None,
                }
                .encode_to_vec(),
                sender_public_key: vec![0x01],
                size: 10,
                accepted_at: now.saturating_sub(V2_MAX_RETENTION_MS + 1),
                receiver_id: random_peer().to_bytes(),
                tier: TIER_UNTRUSTED,
            };
            v2.db_ref_routed_v2
                .insert(sig.clone(), bincode::serialize(&entry).unwrap())
                .unwrap();
            v2.used_size += 10;
            v2.untrusted_used += 10;
            v2.message_count += 1;
        }
        Dtn::process_retransmit_v2(&state);

        let v2 = state.services.dtn.v2.read().unwrap();
        assert!(v2.db_ref_routed_v2.get(&sig).unwrap().is_none(), "stale untrusted entry swept");
        assert_eq!(v2.used_size, 0);
        assert_eq!(v2.untrusted_used, 0);
    }

    // ── Wire round-trips ──

    #[test]
    fn dtn_v2_container_survives_envelope_chain() {
        let receiver_keys = Keypair::generate_ed25519();
        let (sender_keys, container_bytes, sig) = signed_inner(&receiver_keys);
        let custodian = random_peer();
        let container =
            build_container_v2(&sender_keys, container_bytes, sig, vec![custodian.to_bytes()], None, None, None);
        let payload = proto::EnvelopPayload {
            payload: Some(proto::envelop_payload::Payload::DtnV2(container)),
        };
        let decoded = proto::EnvelopPayload::decode(&payload.encode_to_vec()[..]).unwrap();
        match decoded.payload {
            Some(proto::envelop_payload::Payload::DtnV2(c)) => {
                let route = proto::DtnRoute::decode(&c.dtn_route[..]).unwrap();
                assert_eq!(route.route_hop.len(), 1);
                assert_eq!(route.route_hop[0].route_entry[0].id, custodian.to_bytes());
            }
            _ => panic!("expected DtnV2 payload"),
        }
    }

    #[test]
    fn dtn_response_v2_in_dtn_oneof() {
        let account = account_from(Keypair::generate_ed25519());
        let resp = Dtn::build_signed_response(
            &account,
            proto::dtn_response_v2::Kind::Receipt,
            proto::dtn_response_v2::ResponseType::Accepted,
            proto::dtn_response_v2::Reason::None,
            vec![0x01],
        )
        .expect("sign");
        let dtn = proto::Dtn {
            message: Some(proto::dtn::Message::ResponseV2(resp)),
        };
        let decoded = proto::Dtn::decode(&dtn.encode_to_vec()[..]).unwrap();
        match decoded.message {
            Some(proto::dtn::Message::ResponseV2(r)) => {
                assert_eq!(Dtn::verify_response_v2(&r), Some(account.id.to_bytes()));
            }
            _ => panic!("expected ResponseV2 variant"),
        }
    }

    // ── Storage entry serde ──

    #[test]
    fn dtn_routed_v2_entry_serde_round_trip() {
        let entry = DtnRoutedV2Entry {
            container_v2_bytes: vec![1, 2, 3, 4],
            sender_public_key: vec![5, 6],
            size: 100,
            accepted_at: 999,
            receiver_id: vec![7, 8, 9],
            tier: TIER_TRUSTED,
        };
        let bytes = bincode::serialize(&entry).unwrap();
        let back: DtnRoutedV2Entry = bincode::deserialize(&bytes).unwrap();
        assert_eq!(back.container_v2_bytes, entry.container_v2_bytes);
        assert_eq!(back.size, 100);
        assert_eq!(back.tier, TIER_TRUSTED);
    }

    #[test]
    fn sender_quota_entry_serde_round_trip() {
        let entry = SenderQuotaEntry {
            used_bytes: 5000,
            message_count: 3,
        };
        let back: SenderQuotaEntry =
            bincode::deserialize(&bincode::serialize(&entry).unwrap()).unwrap();
        assert_eq!(back.used_bytes, 5000);
        assert_eq!(back.message_count, 3);
    }
}
