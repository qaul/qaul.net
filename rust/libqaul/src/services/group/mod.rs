// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Group Service
//!
//! The Group service sends and receives messages and files into group members.
//! The Group messages carry on the Messaging service
//! Messaging(Group(GroupContainer(...)))

use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use self::proto_net::GroupMemberRole;

use super::chat::{self, Chat};
use super::messaging::{proto, Messaging, MessagingServiceType};
use crate::node::user_accounts::{UserAccount, UserAccounts};
use crate::rpc::Rpc;
use crate::utilities::timestamp::Timestamp;

pub mod crdt;
pub mod crdt_store;
pub mod crdt_wire;
pub mod group_id;
mod manage;
mod member;
mod message;
pub mod search;
pub mod storage;

pub use group_id::GroupId;
pub use manage::GroupManage;
use member::Member;
pub use message::GroupMessage;
pub use search::{GroupSearch, GroupSearchable};
pub use storage::{GroupSaveReason, GroupStorage};

pub use qaul_proto::qaul_net_group as proto_net;
/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_group as proto_rpc;

/// Structure of group member
#[derive(Serialize, Deserialize, Clone)]
pub struct GroupMember {
    // user id
    pub user_id: Vec<u8>,
    // role = 0 => member, 255 => admin
    pub role: i32,
    // joined at
    pub joined_at: u64,
    // state for future using
    pub state: i32,
    // last message index
    pub last_message_index: u32,
}

/// Structure of Group
#[derive(Serialize, Deserialize, Clone)]
pub struct GroupInvited {
    /// sender id
    pub sender_id: Vec<u8>,
    /// received at
    pub received_at: u64,
    /// group info
    pub group: Group,
}

/// Structure of Group
#[derive(Serialize, Deserialize, Clone)]
pub struct Group {
    /// group id
    pub id: Vec<u8>,
    /// group name
    pub name: String,
    /// is direct chat group
    pub is_direct_chat: bool,
    /// created at
    pub created_at: u64,
    /// group status
    ///
    pub status: i32,
    /// group revision number
    ///
    /// this number increases with every revision
    pub revision: u32,
    /// group founder (creator) — bootstrap admin for the membership
    /// CRDT, un-removable, and the only actor who may remove an admin.
    /// `#[serde(default)]` (empty) for groups stored before this field
    /// existed; set to the creator's id for groups created since.
    #[serde(default)]
    pub founder: Vec<u8>,
    /// members
    pub members: BTreeMap<Vec<u8>, GroupMember>,
    /// how many unread messages are there
    pub unread_messages: u32,
    /// last message update
    pub last_message_at: u64,
    /// last message
    ///
    /// This field contains the data of the
    /// qaul.rpc.chat message `ChatContentMessage`
    pub last_message_data: Vec<u8>,
    /// last message sender id
    pub last_message_sender_id: Vec<u8>,
}

/// Group module to process transfer, receive and RPC commands
impl Group {
    /// initialize group chat module
    pub fn init() {
        // initialize group storage
        GroupStorage::init();
    }

    /// creates a new empty group
    ///
    /// to be filled with content
    pub fn new() -> Group {
        Group {
            id: Vec::new(),
            name: "".to_string(),
            is_direct_chat: false,
            created_at: Timestamp::get_timestamp(),
            status: 0,
            revision: 0,
            founder: Vec::new(),
            members: BTreeMap::new(),
            unread_messages: 0,
            last_message_at: 0,
            last_message_data: Vec::new(),
            last_message_sender_id: Vec::new(),
        }
    }

    /// get a group member
    pub fn get_member(&self, user_id: &[u8]) -> Option<&GroupMember> {
        self.members.get(user_id)
    }

    /// Verify if a user is the administrator of the group
    #[allow(dead_code)]
    pub fn is_administrator(&self, user_id: &[u8]) -> bool {
        self.members
            .get(user_id)
            .is_some_and(|member| member.role == GroupMemberRole::Admin as i32)
    }

    /// Verify if a user is a member of the group
    pub fn is_member(&self, user_id: &[u8]) -> bool {
        self.members.contains_key(user_id)
    }

    /// Verify if both user_id's are members of the group
    ///
    /// This is a convenient function to verify incoming messages.
    pub fn are_members(&self, user_id_1: &[u8], user_id_2: &[u8]) -> bool {
        self.is_member(user_id_1) && self.is_member(user_id_2)
    }

    /// update group member
    pub fn update_group_member(state: &crate::QaulState, account_id: &PeerId, group_id: &[u8], member: &GroupMember) {
        GroupStorage::with_group_mut(state, account_id, group_id, GroupSaveReason::MembershipChanged, |group| {
            group.members.insert(member.user_id.clone(), member.clone());
        });
    }

    /// Send an already encoded messaging payload to all remote group members.
    pub fn send_to_remote_members(
        state: &crate::QaulState,
        user_account: &UserAccount,
        group: &Group,
        data: &[u8],
        service_type: MessagingServiceType,
        message_id: &[u8],
        error_context: &str,
    ) {
        for member in group.members.values() {
            let receiver = match PeerId::from_bytes(&member.user_id) {
                Ok(id) => id,
                Err(e) => {
                    log::error!("failed to parse member peer id: {}", e);
                    continue;
                }
            };
            if receiver == user_account.id {
                continue;
            }

            if let Err(error) = Messaging::pack_and_send_message(
                state,
                user_account,
                &receiver,
                data.to_vec(),
                service_type.clone(),
                message_id,
                true,
            ) {
                log::error!("{} {}", error_context, error);
            }
        }
    }

    /// Send packed notify message directly
    pub fn send_notify_message(state: &crate::QaulState, user_account: &UserAccount, receiver: &PeerId, data: Vec<u8>) {
        // pack group container into messaging message
        let proto_message = proto::Messaging {
            message: Some(proto::messaging::Message::GroupInviteMessage(
                proto::GroupInviteMessage { content: data },
            )),
        };

        // send message via messaging
        match Messaging::pack_and_send_message(
            state,
            user_account,
            receiver,
            proto_message.encode_to_vec(),
            MessagingServiceType::Group,
            &[],
            true,
        ) {
            Ok(_) => {}
            Err(err) => {
                log::error!("group notify message sending failed {}", err);
            }
        }
    }

    // ------------------------------------------------------------------
    //                     Membership / metadata CRDT
    // ------------------------------------------------------------------

    /// Emit a signed CRDT op for `group_id`: stamp it with the next
    /// lamport, sign it with the local identity key, persist it to the
    /// op set, and broadcast it to the group's remote members.
    ///
    /// Additive to the legacy invite/reply path — failures here never
    /// block the legacy operation. Returns the op_id on success.
    pub fn emit_crdt_op(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &[u8],
        kind: crdt::OpKind,
    ) -> Option<crdt::OpId> {
        let user_account = UserAccounts::get_by_id(state, *account_id)?;
        let group = GroupStorage::get_group(state, *account_id, group_id)?;
        let db = GroupStorage::get_db_ref(state, *account_id);

        // next lamport from the current op set
        let founder = if group.founder.is_empty() {
            account_id.to_bytes()
        } else {
            group.founder.clone()
        };
        let lamport = crdt_store::load_crdt(&db, group_id, founder).next_lamport();

        // fresh random op_id
        let mut op_id = [0u8; 16];
        {
            use rand::Rng;
            rand::rng().fill(&mut op_id[..]);
        }
        let created_at = Timestamp::get_timestamp();

        let wire = crdt_wire::sign_op(
            &user_account.keys,
            group_id,
            op_id,
            lamport,
            created_at,
            &kind,
        )?;

        // persist locally + reconcile the materialized membership
        crdt_store::save_op(&db, group_id, &wire);
        Self::reconcile_group_from_crdt(state, account_id, group_id);

        // recipients: every current remote member, plus — for an Add —
        // the member being added (who is not yet in the member list, so
        // would otherwise miss the op that adds them).
        let mut recipients: Vec<Vec<u8>> = group
            .members
            .keys()
            .filter(|id| id.as_slice() != account_id.to_bytes().as_slice())
            .cloned()
            .collect();
        if let crdt::OpKind::Add { member_id, .. } = &kind {
            if !recipients.iter().any(|r| r == member_id) {
                recipients.push(member_id.clone());
            }
        }

        // send the op (wrapped so it routes to Group::net on receive)
        let container = proto_net::GroupContainer {
            message: Some(proto_net::group_container::Message::GroupOp(wire)),
        };
        let bytes = container.encode_to_vec();
        for rid in recipients {
            if let Ok(peer) = PeerId::from_bytes(&rid) {
                Self::send_notify_message(state, &user_account, &peer, bytes.clone());
            }
        }
        log::trace!(
            "emitted group CRDT op for {} (lamport {})",
            bs58::encode(group_id).into_string(),
            lamport
        );
        Some(op_id)
    }

    /// Handle an incoming CRDT `GroupOp`: resolve the actor's public key
    /// (a PeerId is a hash of the key), verify the signature, and
    /// persist the op to the group's op set. Merge is unconditional —
    /// authorization is enforced when the view is derived.
    fn on_crdt_op(
        state: &crate::QaulState,
        account_id: &PeerId,
        sender_id: &PeerId,
        group_op: proto_net::GroupOp,
    ) {
        let actor_peer = match PeerId::from_bytes(&group_op.actor_id) {
            Ok(p) => p,
            Err(_) => {
                log::warn!("group CRDT op from {}: invalid actor_id", sender_id.to_base58());
                return;
            }
        };
        let pubkey = {
            let router = state.get_router();
            crate::router::users::Users::get_pub_key(&router, &actor_peer)
        };
        let pubkey = match pubkey {
            Some(k) => k,
            None => {
                log::warn!(
                    "group CRDT op: unknown public key for actor {}",
                    actor_peer.to_base58()
                );
                return;
            }
        };
        if crdt_wire::verify_and_decode(&group_op, &pubkey).is_none() {
            log::warn!(
                "group CRDT op from {}: signature verification failed",
                sender_id.to_base58()
            );
            return;
        }
        let group_id = group_op.group_id.clone();
        let db = GroupStorage::get_db_ref(state, *account_id);
        crdt_store::save_op(&db, &group_id, &group_op);
        // CRDT is the source of truth: reflect the op into the
        // materialized Group membership/metadata.
        Self::reconcile_group_from_crdt(state, account_id, &group_id);
        log::trace!(
            "stored verified group CRDT op for {} from {}",
            bs58::encode(&group_id).into_string(),
            sender_id.to_base58()
        );
    }

    /// Derive the converged CRDT view for a group (for inspection /
    /// the read RPC). `None` if the group is unknown or predates CRDT
    /// (no founder recorded).
    pub fn crdt_view(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &[u8],
    ) -> Option<(crdt::GroupView, usize)> {
        let group = GroupStorage::get_group(state, *account_id, group_id)?;
        if group.founder.is_empty() {
            return None;
        }
        let db = GroupStorage::get_db_ref(state, *account_id);
        let crdt = crdt_store::load_crdt(&db, group_id, group.founder.clone());
        let view = crdt.view();
        let count = crdt.op_count();
        Some((view, count))
    }

    /// Reconcile the materialized `Group.members` / name from the CRDT,
    /// making the CRDT the source of truth for membership.
    ///
    /// The CRDT wins for every member it has an opinion about:
    /// - members present in the derived view are inserted/updated with
    ///   the view's role (other per-member fields preserved if already
    ///   present, else defaulted to an activated member joining now);
    /// - a member the CRDT has at least one `Add` op for but that is
    ///   *not* in the view was tombstoned by a `Remove`, so it is
    ///   dropped from `Group.members`;
    /// - a member the CRDT is entirely silent about (no add op) is left
    ///   untouched — this preserves members of a mixed/legacy group
    ///   during the transition and avoids dropping a freshly-invited
    ///   peer before its add op has arrived.
    ///
    /// No-op for groups with no founder recorded (pre-CRDT groups).
    pub fn reconcile_group_from_crdt(
        state: &crate::QaulState,
        account_id: &PeerId,
        group_id: &[u8],
    ) {
        let group = match GroupStorage::get_group(state, *account_id, group_id) {
            Some(g) if !g.founder.is_empty() => g,
            _ => return,
        };
        let db = GroupStorage::get_db_ref(state, *account_id);
        let crdt = crdt_store::load_crdt(&db, group_id, group.founder.clone());
        let view = crdt.view();

        GroupStorage::with_group_mut(state, account_id, group_id, |group| {
            // 1. drop members the CRDT knows about (has an add for) but
            //    that the view no longer contains (tombstoned).
            let to_drop: Vec<Vec<u8>> = group
                .members
                .keys()
                .filter(|id| {
                    !view.members.contains_key(*id)
                        && !crdt.add_op_ids_for(id).is_empty()
                })
                .cloned()
                .collect();
            for id in to_drop {
                group.members.remove(&id);
            }

            // 2. insert / update every member the view contains.
            for (id, m) in &view.members {
                match group.members.get_mut(id) {
                    Some(existing) => existing.role = m.role,
                    None => {
                        group.members.insert(
                            id.clone(),
                            GroupMember {
                                user_id: id.clone(),
                                role: m.role,
                                joined_at: Timestamp::get_timestamp(),
                                state: proto_rpc::GroupMemberState::Activated as i32,
                                last_message_index: 0,
                            },
                        );
                    }
                }
            }

            // 3. metadata name (LWW) wins when set.
            if let Some(name) = &view.name {
                group.name = name.clone();
            }
        });
    }

    /// Send capsuled group message through messaging service
    #[allow(dead_code)]
    pub fn send_group_message(
        state: &crate::QaulState,
        user_account: &UserAccount,
        receiver: &PeerId,
        group_id: Vec<u8>,
        data: &[u8],
    ) {
        // get last index
        let group;
        match GroupStorage::get_group(state, user_account.id, &group_id) {
            Some(v) => group = v,
            None => return,
        }

        let mut my_member;
        match group.get_member(&user_account.id.to_bytes()) {
            Some(v) => {
                my_member = v.clone();
            }
            _ => {
                return;
            }
        }

        let last_index = my_member.last_message_index + 1;
        let message_id = Chat::generate_message_id(&group.id, &user_account.id, last_index);
        let common_message = proto::CommonMessage {
            message_id: message_id.clone(),
            group_id: group.id.clone(),
            sent_at: Timestamp::get_timestamp(),
            payload: Some(proto::common_message::Payload::GroupMessage(
                proto::GroupMessage {
                    content: data.to_vec(),
                },
            )),
        };

        let send_message = proto::Messaging {
            message: Some(proto::messaging::Message::CommonMessage(common_message)),
        };

        // send message via messaging
        match Messaging::pack_and_send_message(
            state,
            user_account,
            &receiver,
            send_message.encode_to_vec(),
            MessagingServiceType::Group,
            &message_id,
            true,
        ) {
            Ok(_) => {
                // update member state
                my_member.last_message_index = last_index;
                Self::update_group_member(state, &user_account.id, &group_id, &my_member);
            }
            Err(err) => {
                log::error!("group message sending failed {}", err);
            }
        }
    }

    /// Send group updated to all members
    fn post_group_update(state: &crate::QaulState, account_id: &PeerId, group_id: &[u8]) {
        let group;
        match GroupStorage::get_group(state, account_id.to_owned(), group_id) {
            Some(my_group) => group = my_group,
            None => return,
        }

        // create group notify message and post to all members
        let mut members = Vec::with_capacity(group.members.len());
        for m in group.members.values() {
            if m.state > 0 {
                members.push(proto_net::GroupMember {
                    user_id: m.user_id.clone(),
                    role: m.role,
                    state: m.state,
                    joined_at: m.joined_at,
                    last_message_index: m.last_message_index,
                });
            }
        }

        let notify = proto_net::GroupInfo {
            group_id: group_id.to_vec(),
            group_name: group.name.clone(),
            created_at: group.created_at,
            revision: group.revision,
            members,
            founder: group.founder.clone(),
        };

        let container = proto_net::GroupContainer {
            message: Some(proto_net::group_container::Message::GroupInfo(notify)),
        };

        let send_message = proto::Messaging {
            message: Some(proto::messaging::Message::GroupInviteMessage(
                proto::GroupInviteMessage {
                    content: container.encode_to_vec(),
                },
            )),
        };
        let send_message_bytes = send_message.encode_to_vec();

        // send to all group members
        if let Some(user_account) = UserAccounts::get_by_id(state, *account_id) {
            Self::send_to_remote_members(
                state,
                &user_account,
                &group,
                &send_message_bytes,
                MessagingServiceType::Group,
                &[],
                "send group notify error",
            );
        }
    }

    /// Process incoming NET messages for group chat module
    pub fn net(state: &crate::QaulState, sender_id: &PeerId, receiver_id: &PeerId, data: &[u8]) {
        // check receiver id is in users list
        let user;
        match UserAccounts::get_by_id(state, *receiver_id) {
            Some(usr) => {
                user = usr;
            }
            None => {
                log::error!("no user id={}", receiver_id);
                return;
            }
        }

        match proto_net::GroupContainer::decode(data) {
            Ok(messaging) => match messaging.message {
                Some(proto_net::group_container::Message::InviteMember(invite_member)) => {
                    log::trace!("group::on_receive_invite");
                    Member::on_be_invited(state, &sender_id, &receiver_id, &invite_member);
                }
                Some(proto_net::group_container::Message::Removed(removed)) => {
                    log::trace!("group::on_removed");
                    // remove user from group and deactivate group
                    if let Err(error) = Member::on_removed(state, &sender_id, &receiver_id, &removed) {
                        log::error!("group on_removed error {}", error);
                    }
                }
                Some(proto_net::group_container::Message::ReplyInvite(reply_invite)) => {
                    log::trace!("group::on_answered for invite");
                    if let Err(error) =
                        Member::on_reply_invite(state, sender_id, receiver_id, &reply_invite)
                    {
                        log::error!("group on_reply_invite error {}", error);
                    } else {
                        if reply_invite.accept {
                            Self::post_group_update(state, &user.id, &reply_invite.group_id);
                        }
                    }
                }
                Some(proto_net::group_container::Message::GroupInfo(group_info)) => {
                    log::trace!("group info arrived");
                    manage::GroupManage::on_group_notify(state, *sender_id, *receiver_id, &group_info);
                }
                Some(proto_net::group_container::Message::GroupOp(group_op)) => {
                    Self::on_crdt_op(state, &user.id, sender_id, group_op);
                }
                None => {
                    log::error!("group message from {} was empty", sender_id.to_base58())
                }
            },
            Err(e) => {
                log::error!(
                    "Error decoding Group Message from {} to {}: {}",
                    sender_id.to_base58(),
                    receiver_id.to_base58(),
                    e
                );
            }
        }
    }

    /// Process incoming RPC request messages for group chat module
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        let my_user_id = match PeerId::from_bytes(&user_id) {
            Ok(id) => id,
            Err(e) => {
                log::error!("failed to parse user id in group rpc: {}", e);
                return;
            }
        };

        match proto_rpc::Group::decode(&data[..]) {
            Ok(group) => {
                match group.message {
                    Some(proto_rpc::group::Message::GroupCreateRequest(group_create_req)) => {
                        let id = GroupManage::create_new_group(
                            state,
                            &my_user_id,
                            group_create_req.group_name.clone(),
                        );
                        // create response
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupCreateResponse(
                                proto_rpc::GroupCreateResponse {
                                    group_id: id,
                                    result: Some(proto_rpc::GroupResult {
                                        status: true,
                                        message: "".to_string(),
                                    }),
                                },
                            )),
                        };

                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupRenameRequest(group_rename_req)) => {
                        let mut status = true;
                        let mut message: String = "".to_string();
                        if let Err(err) = GroupManage::rename_group(
                            state,
                            &my_user_id,
                            &group_rename_req.group_id,
                            group_rename_req.group_name.clone(),
                        ) {
                            status = false;
                            message = err.clone();
                        }

                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupRenameResponse(
                                proto_rpc::GroupRenameResponse {
                                    group_id: group_rename_req.group_id.clone(),
                                    group_name: group_rename_req.group_name.clone(),
                                    result: Some(proto_rpc::GroupResult { status, message }),
                                },
                            )),
                        };

                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );

                        // post updates
                        if status {
                            Self::post_group_update(state, &my_user_id, &group_rename_req.group_id);

                            // shadow the rename as a CRDT metadata op
                            Self::emit_crdt_op(
                                state,
                                &my_user_id,
                                &group_rename_req.group_id,
                                crdt::OpKind::UpdateMetadata {
                                    name: Some(group_rename_req.group_name.clone()),
                                    avatar: None,
                                },
                            );
                        }
                    }
                    Some(proto_rpc::group::Message::GroupInfoRequest(group_info_req)) => {
                        match GroupManage::group_info(state, &my_user_id, &group_info_req.group_id) {
                            Ok(res) => {
                                let proto_message = proto_rpc::Group {
                                    message: Some(proto_rpc::group::Message::GroupInfoResponse(
                                        res,
                                    )),
                                };

                                // send message
                                Rpc::send_message(
                                    state,
                                    proto_message.encode_to_vec(),
                                    crate::rpc::proto::Modules::Group.into(),
                                    request_id,
                                    Vec::new(),
                                );
                            }
                            Err(err) => {
                                log::error!("Get group info error, {}", err);
                            }
                        }
                    }
                    Some(proto_rpc::group::Message::GroupListRequest(group_list_req)) => {
                        let list = GroupManage::group_list(
                            state,
                            &my_user_id,
                            group_list_req.offset,
                            group_list_req.limit,
                        );
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupListResponse(list)),
                        };
                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupSearchRequest(group_search_req)) => {
                        // Empty query: fall back to the recency-sorted full list
                        let list = if group_search_req.query.trim().is_empty() {
                            GroupManage::group_list(
                                state,
                                &my_user_id,
                                group_search_req.offset,
                                group_search_req.limit,
                            )
                        } else {
                            GroupManage::group_search(
                                state,
                                &my_user_id,
                                &group_search_req.query,
                                group_search_req.offset,
                                group_search_req.limit,
                            )
                        };
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupListResponse(list)),
                        };
                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupInvitedRequest(group_invited_req)) => {
                        let invited = GroupManage::invited_list(
                            state,
                            &my_user_id,
                            group_invited_req.offset,
                            group_invited_req.limit,
                        );
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupInvitedResponse(invited)),
                        };
                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupInviteMemberRequest(invite_req)) => {
                        let mut status = true;
                        let mut message: String = "".to_string();
                        let invite_user_id = match PeerId::from_bytes(&invite_req.user_id) {
                            Ok(id) => id,
                            Err(e) => {
                                log::error!("failed to parse invite user id: {}", e);
                                return;
                            }
                        };
                        if let Err(err) = Member::invite(
                            state,
                            &my_user_id,
                            &invite_req.group_id,
                            &invite_user_id,
                        ) {
                            status = false;
                            message = err.clone();
                            log::error!("Get group info error, {}", err);
                        }
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupInviteMemberResponse(
                                proto_rpc::GroupInviteMemberResponse {
                                    group_id: invite_req.group_id.clone(),
                                    user_id: invite_req.user_id.clone(),
                                    result: Some(proto_rpc::GroupResult { status, message }),
                                },
                            )),
                        };

                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );

                        // additively shadow the invite as a CRDT op so
                        // members converge on membership without the
                        // rev-counter merge. Best-effort.
                        if status {
                            Self::emit_crdt_op(
                                state,
                                &my_user_id,
                                &invite_req.group_id,
                                crdt::OpKind::Add {
                                    member_id: invite_req.user_id.clone(),
                                    role: crdt::ROLE_MEMBER,
                                },
                            );
                        }
                    }
                    Some(proto_rpc::group::Message::GroupReplyInviteRequest(reply_req)) => {
                        let mut status = true;
                        let mut message: String = "".to_string();

                        if let Err(err) =
                            Member::reply_invite(state, &my_user_id, &reply_req.group_id, reply_req.accept)
                        {
                            status = false;
                            message = err.clone();
                            log::error!("Get group info error, {}", err);
                        }
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupReplyInviteResponse(
                                proto_rpc::GroupReplyInviteResponse {
                                    group_id: reply_req.group_id.clone(),
                                    result: Some(proto_rpc::GroupResult { status, message }),
                                },
                            )),
                        };

                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupRemoveMemberRequest(remove_req)) => {
                        let mut status = true;
                        let mut message: String = "".to_string();

                        let remove_user_id = match PeerId::from_bytes(&remove_req.user_id) {
                            Ok(id) => id,
                            Err(e) => {
                                log::error!("failed to parse remove user id: {}", e);
                                return;
                            }
                        };
                        if let Err(err) = Member::remove(
                            state,
                            &my_user_id,
                            &remove_req.group_id,
                            &remove_user_id,
                        ) {
                            status = false;
                            message = err.clone();
                            log::error!("Get group info error, {}", err);
                        }

                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupRemoveMemberResponse(
                                proto_rpc::GroupRemoveMemberResponse {
                                    group_id: remove_req.group_id.clone(),
                                    user_id: remove_req.user_id.clone(),
                                    result: Some(proto_rpc::GroupResult { status, message }),
                                },
                            )),
                        };

                        // send message
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );

                        if status {
                            Self::post_group_update(state, &my_user_id, &remove_req.group_id);

                            // shadow the removal as a CRDT op tombstoning
                            // every add we have seen for that member.
                            let db = GroupStorage::get_db_ref(state, my_user_id);
                            let founder = GroupStorage::get_group(state, my_user_id, &remove_req.group_id)
                                .map(|g| g.founder)
                                .unwrap_or_default();
                            let founder = if founder.is_empty() { my_user_id.to_bytes() } else { founder };
                            let observed = crdt_store::load_crdt(&db, &remove_req.group_id, founder)
                                .add_op_ids_for(&remove_req.user_id);
                            Self::emit_crdt_op(
                                state,
                                &my_user_id,
                                &remove_req.group_id,
                                crdt::OpKind::Remove {
                                    member_id: remove_req.user_id.clone(),
                                    observed_adds: observed,
                                },
                            );
                        }
                    }
                    Some(proto_rpc::group::Message::GroupCrdtViewRequest(view_req)) => {
                        let resp = match Self::crdt_view(state, &my_user_id, &view_req.group_id) {
                            Some((view, op_count)) => proto_rpc::GroupCrdtViewResponse {
                                group_id: view_req.group_id.clone(),
                                found: true,
                                founder: GroupStorage::get_group(state, my_user_id, &view_req.group_id)
                                    .map(|g| g.founder)
                                    .unwrap_or_default(),
                                name: view.name.clone().unwrap_or_default(),
                                op_count: op_count as u32,
                                members: view
                                    .members
                                    .iter()
                                    .map(|(id, m)| proto_rpc::GroupCrdtMember {
                                        user_id: id.clone(),
                                        role: m.role,
                                    })
                                    .collect(),
                            },
                            None => proto_rpc::GroupCrdtViewResponse {
                                group_id: view_req.group_id.clone(),
                                found: false,
                                ..Default::default()
                            },
                        };
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupCrdtViewResponse(resp)),
                        };
                        Rpc::send_message(
                            state,
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    _ => {
                        log::error!("Unhandled Protobuf Group chat message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
