// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Discovered user table
//!
//! This table contains all users known to this node.

use libp2p::{identity::PublicKey, PeerId};
use prost::Message;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use state::InitCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::router_net_proto;
use super::table::RoutingTable;
use crate::node::user_accounts::UserAccounts;
use crate::rpc::Rpc;
use crate::services::group::group_id::GroupId;
use crate::storage::database::DbUsers;
use crate::utilities::qaul_id::QaulId;

/// Import protobuf users RPC message definition
pub use qaul_proto::qaul_rpc_users as proto;

/// mutable state of users table
static USERS: InitCell<RwLock<Users>> = InitCell::new();

/// implementation of all known users for routing references
pub struct Users {
    /// the BTreeMap key is the 8 byte qaul ID (q8id)
    pub users: BTreeMap<Vec<u8>, User>,
}

impl Users {
    /// Initialize the router::users::Users module
    /// this module is automatically initialized
    /// when the router module is initialized
    pub fn init() {
        {
            // create users table and save it to state
            let users = Users {
                users: BTreeMap::new(),
            };
            USERS.set(RwLock::new(users));
        }

        // fill user table with users from data base
        let tree = DbUsers::get_tree();
        let mut users = USERS.get().write().unwrap();
        // iterate over all values in db
        for res in tree.iter() {
            if let Ok((_vec, user_bytes)) = res {
                // decode user bytes
                let user: UserData = bincode::deserialize(&user_bytes).unwrap();
                // encode values from bytes
                let q8id = QaulId::bytes_to_q8id(user.id.clone());
                let id = PeerId::from_bytes(&user.id).unwrap();
                let key = PublicKey::try_decode_protobuf(&user.key).unwrap();
                // fill result into user table
                users.users.insert(
                    q8id,
                    User {
                        id,
                        key,
                        name: user.name,
                        verified: user.verified,
                        blocked: user.blocked,
                    },
                );
            }
        }
    }

    /// add a new user
    ///
    /// This user will be added to the users list in memory and to the data base
    pub fn add(id: PeerId, key: PublicKey, name: String, verified: bool, blocked: bool) {
        // save user to the data base
        DbUsers::add_user(UserData {
            id: id.to_bytes(),
            key: key.clone().encode_protobuf(),
            name: name.clone(),
            verified,
            blocked,
        });

        // add user to the users table
        let q8id = QaulId::to_q8id(id.clone());
        let mut users = USERS.get().write().unwrap();
        users.users.insert(
            q8id,
            User {
                id,
                key,
                name,
                verified,
                blocked,
            },
        );
    }

    /// add a new user to the users list, and check whether the
    /// User ID matches the public key
    /// and save it to the data base
    pub fn add_with_check(id: PeerId, key: PublicKey, name: String) {
        // check if user is valid
        if id != key.clone().to_peer_id() {
            log::error!("user id & key do not match {}", id.to_base58());
            return;
        }

        // check if user already exists
        {
            let q8id = QaulId::to_q8id(id.clone());
            let users = USERS.get().read().unwrap();

            // check if user already exists
            if users.users.contains_key(&q8id) {
                return;
            }
        }
        // add user
        Self::add(id, key, name, false, false);
    }

    /// check missed users from ids
    pub fn get_missed_ids(ids: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut res: Vec<Vec<u8>> = vec![];
        let users = USERS.get().read().unwrap();
        for id in ids {
            if !users.users.contains_key(id) {
                res.push(id.clone());
            }
        }
        res
    }

    /// get the public key of a known user
    pub fn get_pub_key(user_id: &PeerId) -> Option<PublicKey> {
        // get q8id
        let q8id = QaulId::to_q8id(user_id.to_owned());

        // get public key
        Self::get_pub_key_by_q8id(&q8id)
    }

    /// get the public key of a known user by it's q8id
    pub fn get_pub_key_by_q8id(q8id: &Vec<u8>) -> Option<PublicKey> {
        let store = USERS.get().read().unwrap();
        let result = store.users.get(q8id);
        match result {
            Some(user) => Some(user.key.clone()),
            None => None,
        }
    }

    /// get user by q8id
    pub fn get_user_id_by_q8id(q8id: Vec<u8>) -> Option<PeerId> {
        let store = USERS.get().read().unwrap();

        if let Some(user) = store.users.get(&q8id) {
            return Some(user.id);
        }

        None
    }

    /// create and send the user info table for the
    /// RouterInfo message which is sent regularly to neighbours
    ///
    /// This is a wrapper function for the PeerIds for the function
    /// `get_user_info_table_by_q8ids(q8ids)`
    pub fn _get_user_info_table_by_ids(ids: &Vec<PeerId>) -> router_net_proto::UserInfoTable {
        let mut q8ids: Vec<Vec<u8>> = Vec::new();
        for id in ids {
            // convert qaul ID to q8id
            let q8id = QaulId::to_q8id(id.to_owned());
            q8ids.push(q8id);
        }

        Self::get_user_info_table_by_q8ids(&q8ids)
    }

    /// create and send the user info table for the
    /// RouterInfo message which is sent regularly to neighbours
    pub fn get_user_info_table_by_q8ids(q8ids: &Vec<Vec<u8>>) -> router_net_proto::UserInfoTable {
        let store = USERS.get().read().unwrap();
        let mut users = router_net_proto::UserInfoTable { info: Vec::new() };

        for q8id in q8ids {
            if let Some(value) = store.users.get(q8id) {
                let user_info = router_net_proto::UserInfo {
                    id: value.id.to_bytes(),
                    key: value.key.clone().encode_protobuf(),
                    name: value.name.clone(),
                };
                users.info.push(user_info);
            }
        }
        users
    }

    /// add new users from the received bytes of a UserInfoTable
    pub fn add_user_info_table(users: &Vec<router_net_proto::UserInfo>) {
        // loop through it and add it to the users list
        for value in users {
            let id_result = PeerId::from_bytes(&value.id);
            let key_result = PublicKey::try_decode_protobuf(&value.key);

            if let (Ok(id), Ok(key)) = (id_result, key_result) {
                Self::add_with_check(id, key, value.name.clone());
            }
        }
    }

    fn compare(a: &[u8], b: &[u8]) -> Ordering {
        for (ai, bi) in a.iter().zip(b.iter()) {
            match ai.cmp(&bi) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }

        // if every single element was equal, compare length
        a.len().cmp(&b.len())
    }

    /// get security number
    fn get_security_number(my_user: &PeerId, user_id: &Vec<u8>) -> Result<Vec<u8>, String> {
        let q8id = QaulId::bytes_to_q8id(user_id.clone());
        let q8id_my = QaulId::to_q8id(my_user.clone());

        // find user from users
        let users = USERS.get().read().unwrap();
        if !users.users.contains_key(&q8id) {
            return Err("user no exists".to_string());
        }

        if !users.users.contains_key(&q8id_my) {
            return Err("my user is not existed".to_string());
        }
        let mut key1 = users.users.get(&q8id_my).unwrap().key.encode_protobuf();
        let mut key2 = users.users.get(&q8id).unwrap().key.encode_protobuf();

        // merge two keys
        let mut data: Vec<u8> = vec![];
        match Self::compare(&key1, &key2) {
            Ordering::Less => {
                data.append(&mut key1);
                data.append(&mut key2);
            }
            _ => {
                data.append(&mut key2);
                data.append(&mut key1);
            }
        }

        let mut key_data = data.clone();
        data.clear();

        for _ in 0..5200 {
            data.append(&mut key_data);
            let hash = Sha512::digest(&data);
            let mut hash_vec = hash[..64].to_vec();
            data.clear();
            data.append(&mut hash_vec);
        }
        Ok(data[..16].to_vec())
    }

    /// Process incoming RPC request messages
    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>) {
        let account_id = PeerId::from_bytes(&user_id).unwrap();

        match proto::Users::decode(&data[..]) {
            Ok(users) => {
                match users.message {
                    Some(proto::users::Message::UserRequest(user_request)) => {
                        Self::build_user_list(
                            UserFilter::All,
                            user_request.offset,
                            user_request.limit,
                        );
                    }
                    Some(proto::users::Message::UserOnlineRequest(user_online_request)) => {
                        Self::build_user_list(
                            UserFilter::OnlineOnly,
                            user_online_request.offset,
                            user_online_request.limit,
                        );
                    }
                    Some(proto::users::Message::UserUpdate(updated_user)) => {
                        log::trace!("UserUpdate protobuf RPC message");
                        // attempt to find the user with the associated id
                        Self::with_resolved_user(
                            &updated_user.id,
                            |user_id, _q8id, user_result| {
                                // update user entity
                                let user = User {
                                    id: user_id,
                                    key: user_result.key.clone(),
                                    name: user_result.name.clone(),
                                    verified: updated_user.verified,
                                    blocked: updated_user.blocked,
                                };

                                *user_result = user;

                                // persist the updated entity
                                DbUsers::add_user(UserData {
                                    id: user_id.to_bytes(),
                                    key: user_result.key.clone().encode_protobuf(),
                                    name: user_result.name.clone(),
                                    verified: updated_user.verified,
                                    blocked: updated_user.blocked,
                                });
                            },
                        );
                    }
                    Some(proto::users::Message::SecurityNumberRequest(secure_req)) => {
                        match Self::get_security_number(&account_id, &secure_req.user_id) {
                            Ok(x) => {
                                let mut security_number_blocks: Vec<u32> = vec![];
                                for i in 0..x.len() / 2 {
                                    let number = x[i * 2] as u32 + (x[i * 2 + 1] as u32 * 256);
                                    security_number_blocks.push(number);
                                }

                                // create message
                                let proto_message = proto::Users {
                                    message: Some(proto::users::Message::SecurityNumberResponse(
                                        proto::SecurityNumberResponse {
                                            user_id: secure_req.user_id.clone(),
                                            security_hash: x.clone(),
                                            security_number_blocks,
                                        },
                                    )),
                                };

                                // encode message
                                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                                proto_message
                                    .encode(&mut buf)
                                    .expect("Vec<u8> provides capacity as needed");

                                // send message
                                Rpc::send_message(
                                    buf,
                                    crate::rpc::proto::Modules::Users.into(),
                                    "".to_string(),
                                    Vec::new(),
                                );
                            }
                            Err(error) => {
                                log::error!("security number error: {}", error);
                            }
                        }
                    }
                    Some(proto::users::Message::GetUserByIdRequest(req)) => {
                        log::trace!("GetByIdRequest protobuf RPC message");
                        // attempt to find the user with the associated id
                        Self::with_resolved_user(&req.user_id, |_, q8id, user| {
                            let online_users = RoutingTable::get_online_users_info();
                            let entry = build_user_entry(user, &online_users, &account_id, q8id);

                            let proto_message = proto::Users {
                                message: Some(proto::users::Message::GetUserByIdResponse(
                                    proto::GetUserByIdResponse { user: Some(entry) },
                                )),
                            };

                            let mut buf = Vec::with_capacity(proto_message.encoded_len());
                            proto_message
                                .encode(&mut buf)
                                .expect("Vec<u8> provides capacity as needed");

                            // send encoded rpc message containing found user entity
                            Rpc::send_message(
                                buf,
                                crate::rpc::proto::Modules::Users.into(),
                                "".to_string(),
                                Vec::new(),
                            );
                        });
                    }
                    _ => {}
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    /// create the qaul RPC definitions of a public key
    ///
    /// Returns a tuple with the key type & the base58 encoded
    /// (key_type: String, key_base58: String)
    pub fn get_protobuf_public_key(key: PublicKey) -> (String, String) {
        // extract values
        let key_type: String;
        let key_base58: String;

        #[allow(unreachable_patterns)]
        match key.try_into_ed25519() {
            Ok(ed_key) => {
                key_type = "Ed25519".to_owned();
                key_base58 = bs58::encode(ed_key.to_bytes()).into_string();
            }
            _ => {
                key_type = "UNDEFINED".to_owned();
                key_base58 = "UNDEFINED".to_owned();
            }
        }

        (key_type, key_base58)
    }

    /// helper function that, given a set of user id bytes, will attempt to find that user entity and pass it to a closure
    fn with_resolved_user<F>(user_id_bytes: &[u8], f: F)
    where
        F: FnOnce(PeerId, &Vec<u8>, &mut User),
    {
        // resolve a user id from raw bytes
        let (user_id, q8id) = match PeerId::from_bytes(user_id_bytes) {
            Ok(id) => {
                let q8id = QaulId::to_q8id(id);
                (id, q8id)
            }
            Err(_) => {
                log::error!("invalid PeerId");
                return;
            }
        };

        // acquire a lock to lookup the user entry
        let mut store = USERS.get().write().unwrap();
        match store.users.get_mut(&q8id) {
            // pass found user to f
            Some(user) => f(user_id, &q8id, user),
            None => log::error!("user not found: {}", user_id.to_base58()),
        }
    }

    /// Build users list from those found in the users store.
    ///
    /// Only completes successfully if there is a default user account, otherwise it always returns
    /// an empty list.
    fn build_user_list(filter: UserFilter, offset: u32, limit: u32) {
        let users = USERS.get().read().unwrap();

        let user_list = if let Some(account) = UserAccounts::get_default_user() {
            let online_users = RoutingTable::get_online_users_info();
            build_user_list_from(
                &users.users,
                &online_users,
                &account.id,
                filter,
                offset,
                limit,
            )
        } else {
            proto::UserList {
                user: Vec::new(),
                pagination: None,
            }
        };

        let proto_message = proto::Users {
            message: Some(proto::users::Message::UserList(user_list)),
        };

        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        Rpc::send_message(
            buf,
            crate::rpc::proto::Modules::Users.into(),
            "".to_string(),
            Vec::new(),
        );
    }
}

/// Describes what users to include when building the users list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UserFilter {
    /// Include all known users.
    All,
    /// Include only users present in the routing table (online).
    OnlineOnly,
}

/// user structure
pub struct User {
    pub id: PeerId,
    pub key: PublicKey,
    pub name: String,
    pub verified: bool,
    pub blocked: bool,
}

/// user structure for storing it in the data base
#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: Vec<u8>,
    pub key: Vec<u8>,
    pub name: String,
    pub verified: bool,
    pub blocked: bool,
}

/// Build a single `proto::UserEntry` for the given user.
fn build_user_entry(
    user: &User,
    online_users: &BTreeMap<Vec<u8>, Vec<super::table::RoutingConnectionEntry>>,
    account_id: &PeerId,
    q8id: &Vec<u8>,
) -> proto::UserEntry {
    let mut connectivity: i32 = 0;
    let mut connections: Vec<proto::RoutingTableConnection> = Vec::new();

    if let Some(entries) = online_users.get(q8id) {
        for entry in entries {
            connections.push(proto::RoutingTableConnection {
                module: entry.module.as_int(),
                hop_count: entry.hc as u32,
                rtt: entry.rtt,
                via: entry.node.to_bytes(),
            });
        }
        connectivity = 1;
    }

    let (_key_type, key_base58) = Users::get_protobuf_public_key(user.key.clone());
    let group_id = GroupId::from_peers(account_id, &user.id).to_bytes();

    proto::UserEntry {
        name: user.name.clone(),
        id: user.id.to_bytes(),
        group_id,
        key_base58,
        connectivity,
        verified: user.verified,
        blocked: user.blocked,
        connections,
    }
}

/// Build a paginated user list from a set of users and online users,
/// optionally filtering out offline users.
fn build_user_list_from(
    users: &BTreeMap<Vec<u8>, User>,
    online_users: &BTreeMap<Vec<u8>, Vec<super::table::RoutingConnectionEntry>>,
    account_id: &PeerId,
    filter: UserFilter,
    offset: u32,
    limit: u32,
) -> proto::UserList {
    let mut user_list = proto::UserList {
        user: Vec::new(),
        pagination: None,
    };

    let online_only = filter == UserFilter::OnlineOnly;

    let mut total = if online_only {
        online_users.len() as u32
    } else {
        users.len() as u32
    };

    let mut skipped: u32 = 0;

    for (id, user) in users {
        if online_only && !online_users.contains_key(id) {
            continue;
        }

        if skipped < offset {
            skipped += 1;
            continue;
        }

        if limit > 0 && user_list.user.len() >= limit as usize {
            break;
        }

        let entry = build_user_entry(user, online_users, account_id, id);
        user_list.user.push(entry);
    }

    // When online_only is true, total was estimated from the routing table which
    // may contain peers absent from the users store. If we didn't break early
    // (i.e. we exhausted all matching entries), we know the exact count.
    let exhausted = limit == 0 || (user_list.user.len() as u32) < limit;
    if online_only && exhausted {
        total = skipped + user_list.user.len() as u32;
    }

    let has_more = limit > 0 && offset.saturating_add(limit) < total;

    user_list.pagination = Some(proto::PaginationMetadata {
        has_more,
        total,
        offset,
        limit,
    });

    user_list
}

#[cfg(test)]
mod tests {
    use super::super::table::RoutingConnectionEntry;
    use super::*;
    use crate::connections::ConnectionModule;
    use libp2p::identity::Keypair;

    /// Helper: generate a (PeerId, PublicKey) pair from a fresh Ed25519 keypair.
    fn gen_peer() -> (PeerId, PublicKey) {
        let kp = Keypair::generate_ed25519();
        let pk = kp.public();
        let id = pk.to_peer_id();
        (id, pk)
    }

    /// Helper: build a `BTreeMap<Vec<u8>, User>` with `n` users and return
    /// the generated PeerIds (in insertion order) for later use.
    fn make_users(n: usize) -> (BTreeMap<Vec<u8>, User>, Vec<PeerId>) {
        let mut map = BTreeMap::new();
        let mut ids = Vec::new();
        for i in 0..n {
            let (id, key) = gen_peer();
            let q8id = QaulId::to_q8id(id);
            map.insert(
                q8id,
                User {
                    id,
                    key,
                    name: format!("user_{}", i),
                    verified: false,
                    blocked: false,
                },
            );
            ids.push(id);
        }
        (map, ids)
    }

    /// Helper: build online_users map for specified q8ids.
    fn make_online(
        users: &BTreeMap<Vec<u8>, User>,
        count: usize,
    ) -> BTreeMap<Vec<u8>, Vec<RoutingConnectionEntry>> {
        let mut online = BTreeMap::new();
        let via_peer = gen_peer().0;
        for (i, (q8id, _user)) in users.iter().enumerate() {
            if i >= count {
                break;
            }
            online.insert(
                q8id.clone(),
                vec![RoutingConnectionEntry {
                    module: ConnectionModule::Lan,
                    node: via_peer,
                    rtt: 10,
                    hc: 1,
                    lq: 100,
                    last_update: 0,
                }],
            );
        }
        online
    }

    /// Helper: a dummy account_id for the "self" user in the list builder.
    fn account_id() -> PeerId {
        gen_peer().0
    }

    // ---------------------------------------------------------------
    // Test cases
    // ---------------------------------------------------------------

    #[test]
    fn no_pagination_backwards_compat() {
        let (users, _ids) = make_users(5);
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 0, 0);

        assert_eq!(list.user.len(), 5);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn empty_users() {
        let users = BTreeMap::new();
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 0, 10);

        assert_eq!(list.user.len(), 0);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 0);
    }

    #[test]
    fn pagination_echoes_offset_and_limit() {
        let (users, _ids) = make_users(5);
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 3, 7);

        let p = list.pagination.unwrap();
        assert_eq!(p.offset, 3);
        assert_eq!(p.limit, 7);
    }

    #[test]
    fn first_page() {
        let (users, _ids) = make_users(5);
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 0, 2);

        assert_eq!(list.user.len(), 2);
        let p = list.pagination.unwrap();
        assert!(p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn middle_page() {
        let (users, _ids) = make_users(5);
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 2, 2);

        assert_eq!(list.user.len(), 2);
        let p = list.pagination.unwrap();
        assert!(p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn last_page_partial() {
        let (users, _ids) = make_users(5);
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 4, 2);

        assert_eq!(list.user.len(), 1);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn offset_beyond_total() {
        let (users, _ids) = make_users(5);
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 10, 2);

        assert_eq!(list.user.len(), 0);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn limit_larger_than_total() {
        let (users, _ids) = make_users(5);
        let online = BTreeMap::new();
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::All, 0, 100);

        assert_eq!(list.user.len(), 5);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn online_only_filtering() {
        let (users, _ids) = make_users(5);
        let online = make_online(&users, 2);
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::OnlineOnly, 0, 0);

        assert_eq!(list.user.len(), 2);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 2);
    }

    #[test]
    fn online_only_with_pagination() {
        let (users, _ids) = make_users(5);
        let online = make_online(&users, 3);
        let acc = account_id();

        let list = build_user_list_from(&users, &online, &acc, UserFilter::OnlineOnly, 0, 1);

        assert_eq!(list.user.len(), 1);
        let p = list.pagination.unwrap();
        assert!(p.has_more);
        assert_eq!(p.total, 3);
    }

    /// When the routing table contains peers that are absent from the users
    /// store, the initial `total` estimate (from `online_users.len()`) is
    /// higher than the real count of renderable entries.
    ///
    /// - On the **last** page (or when limit=0) the code corrects `total` to
    ///   the exact count because it exhausted all matching entries.
    /// - On a **non-final** page (broke out early due to limit) the correction
    ///   cannot kick in, so `total` may overestimate.
    ///
    /// In practice, this should never happen, but this test documents the known gap.
    #[test]
    fn online_only_with_phantom_routing_peers() {
        let (users, _ids) = make_users(3);
        // 2 of the 3 users are online
        let mut online = make_online(&users, 2);

        // add 2 phantom peers that exist in the routing table but NOT in
        // the users store to simulate stale/unknown entries.
        let via_peer = gen_peer().0;
        for _ in 0..2 {
            let phantom_id = gen_peer().0;
            let phantom_q8id = QaulId::to_q8id(phantom_id);
            online.insert(
                phantom_q8id,
                vec![RoutingConnectionEntry {
                    module: ConnectionModule::Lan,
                    node: via_peer,
                    rtt: 5,
                    hc: 1,
                    lq: 100,
                    last_update: 0,
                }],
            );
        }
        // Now, online_users.len() == 4, but only 2 have matching User entries.

        let acc = account_id();

        // Case 1: no limit — exhausts all entries, total is corrected to 2.
        let list = build_user_list_from(&users, &online, &acc, UserFilter::OnlineOnly, 0, 0);
        assert_eq!(list.user.len(), 2);
        let p = list.pagination.unwrap();
        assert_eq!(
            p.total, 2,
            "total should be corrected when all entries are exhausted"
        );
        assert!(!p.has_more);

        // Case 2: limit = 1 — breaks early, total stays at the routing-table
        // estimate (4) because we can't know better without scanning further.
        let list = build_user_list_from(&users, &online, &acc, UserFilter::OnlineOnly, 0, 1);
        assert_eq!(list.user.len(), 1);
        let p = list.pagination.unwrap();
        assert_eq!(p.total, 4, "total is an overestimate on non-final pages");
        assert!(p.has_more);
    }

    #[test]
    fn build_user_entry_offline() {
        let (users, _ids) = make_users(1);
        let online: BTreeMap<Vec<u8>, Vec<RoutingConnectionEntry>> = BTreeMap::new();
        let acc = account_id();

        let (q8id, user) = users.iter().next().unwrap();
        let entry = build_user_entry(user, &online, &acc, q8id);

        assert_eq!(entry.name, "user_0");
        assert_eq!(entry.id, user.id.to_bytes());
        assert_eq!(entry.connectivity, 0);
        assert!(entry.connections.is_empty());
        assert!(!entry.key_base58.is_empty());
        assert!(!entry.group_id.is_empty());
        assert_eq!(entry.verified, user.verified);
        assert_eq!(entry.blocked, user.blocked);
    }

    #[test]
    fn build_user_entry_online() {
        let (users, _ids) = make_users(1);
        let online = make_online(&users, 1);
        let acc = account_id();

        let (q8id, user) = users.iter().next().unwrap();
        let entry = build_user_entry(user, &online, &acc, q8id);

        assert_eq!(entry.connectivity, 1);
        assert_eq!(entry.connections.len(), 1);

        let conn = &entry.connections[0];
        assert_eq!(conn.hop_count, 1);
        assert_eq!(conn.rtt, 10);
    }
}
