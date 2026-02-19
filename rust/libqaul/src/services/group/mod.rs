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

pub mod group_id;
mod manage;
mod member;
mod message;
pub mod storage;

pub use group_id::GroupId;
pub use manage::GroupManage;
use member::Member;
pub use message::GroupMessage;
pub use storage::GroupStorage;

/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_group as proto_rpc;
pub use qaul_proto::qaul_net_group as proto_net;

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
            members: BTreeMap::new(),
            unread_messages: 0,
            last_message_at: 0,
            last_message_data: Vec::new(),
            last_message_sender_id: Vec::new(),
        }
    }

    /// get a group member
    pub fn get_member(&self, user_id: &Vec<u8>) -> Option<&GroupMember> {
        if self.members.contains_key(user_id) {
            return self.members.get(user_id);
        }
        None
    }

    /// Verify if a user is the administrator of the group
    #[allow(dead_code)]
    pub fn is_administrator(&self, user_id: &Vec<u8>) -> bool {
        if let Some(member) = self.members.get(user_id) {
            if member.role == GroupMemberRole::Admin as i32 {
                return true;
            }
        }
        false
    }

    /// Verify if a user is a member of the group
    pub fn is_member(&self, user_id: &Vec<u8>) -> bool {
        self.members.contains_key(user_id)
    }

    /// Verify if both user_id's are members of the group
    ///
    /// This is a convenient function to verify incoming messages.
    pub fn are_members(&self, user_id_1: &Vec<u8>, user_id_2: &Vec<u8>) -> bool {
        if self.is_member(user_id_1) {
            return self.is_member(user_id_2);
        }

        false
    }

    /// update group member
    pub fn update_group_member(account_id: &PeerId, group_id: &Vec<u8>, member: &GroupMember) {
        if let Some(mut group) = GroupStorage::get_group(account_id.to_owned(), group_id.to_owned())
        {
            // insert member
            group.members.insert(member.user_id.clone(), member.clone());

            // update DB
            GroupStorage::save_group(account_id.to_owned(), group);
        }
    }

    /// Send packed notify message directly
    pub fn send_notify_message(user_account: &UserAccount, receiver: &PeerId, data: Vec<u8>) {
        // pack group container into messaging message
        let proto_message = proto::Messaging {
            message: Some(proto::messaging::Message::GroupInviteMessage(
                proto::GroupInviteMessage { content: data },
            )),
        };

        // send message via messaging
        let message_id: Vec<u8> = Vec::new();
        match Messaging::pack_and_send_message(
            user_account,
            &receiver,
            proto_message.encode_to_vec(),
            MessagingServiceType::Group,
            &message_id,
            true,
        ) {
            Ok(_) => {}
            Err(err) => {
                log::error!("group notify message sending failed {}", err);
            }
        }
    }

    /// Send capsuled group message through messaging service
    #[allow(dead_code)]
    pub fn send_group_message(
        user_account: &UserAccount,
        receiver: &PeerId,
        group_id: Vec<u8>,
        data: &Vec<u8>,
    ) {
        // get last index
        let group;
        match GroupStorage::get_group(user_account.id, group_id.clone()) {
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
                    content: data.clone(),
                },
            )),
        };

        let send_message = proto::Messaging {
            message: Some(proto::messaging::Message::CommonMessage(
                common_message.clone(),
            )),
        };

        // send message via messaging
        match Messaging::pack_and_send_message(
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
                Self::update_group_member(&user_account.id, &group_id, &my_member);
            }
            Err(err) => {
                log::error!("group message sending failed {}", err);
            }
        }
    }

    /// Send group updated to all members
    fn post_group_update(account_id: &PeerId, group_id: &Vec<u8>) {
        let group;
        match GroupStorage::get_group(account_id.to_owned(), group_id.to_owned()) {
            Some(my_group) => group = my_group,
            None => return,
        }

        // create group notify message and post to all members
        let mut members: Vec<proto_net::GroupMember> = vec![];
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
            group_id: group_id.clone(),
            group_name: group.name.clone(),
            created_at: group.created_at,
            revision: group.revision,
            members,
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

        // send to all group members
        if let Some(user_account) = UserAccounts::get_by_id(*account_id) {
            for user_id in group.members.keys() {
                let receiver = PeerId::from_bytes(&user_id.clone()).unwrap();
                if receiver != *account_id {
                    let message_id: Vec<u8> = Vec::new();
                    if let Err(error) = Messaging::pack_and_send_message(
                        &user_account,
                        &receiver,
                        send_message.encode_to_vec(),
                        MessagingServiceType::Group,
                        &message_id,
                        true,
                    ) {
                        log::error!("send group notify error {}", error);
                    }
                }
            }
        }
    }

    /// Process incoming NET messages for group chat module
    pub fn net(sender_id: &PeerId, receiver_id: &PeerId, data: &Vec<u8>) {
        // check receiver id is in users list
        let user;
        match UserAccounts::get_by_id(receiver_id.clone()) {
            Some(usr) => {
                user = usr;
            }
            None => {
                log::error!("no user id={}", receiver_id);
                return;
            }
        }

        match proto_net::GroupContainer::decode(&data[..]) {
            Ok(messaging) => match messaging.message {
                Some(proto_net::group_container::Message::InviteMember(invite_member)) => {
                    log::trace!("group::on_receive_invite");
                    Member::on_be_invited(&sender_id, &receiver_id, &invite_member);
                }
                Some(proto_net::group_container::Message::Removed(removed)) => {
                    log::trace!("group::on_removed");
                    // remove user from group and deactivate group
                    if let Err(error) = Member::on_removed(&sender_id, &receiver_id, &removed) {
                        log::error!("group on_removed error {}", error);
                    }
                }
                Some(proto_net::group_container::Message::ReplyInvite(reply_invite)) => {
                    log::trace!("group::on_answered for invite");
                    if let Err(error) =
                        Member::on_reply_invite(sender_id, receiver_id, &reply_invite)
                    {
                        log::error!("group on_reply_invite error {}", error);
                    } else {
                        if reply_invite.accept {
                            Self::post_group_update(&user.id, &reply_invite.group_id);
                        }
                    }
                }
                Some(proto_net::group_container::Message::GroupInfo(group_info)) => {
                    log::trace!("group info arrived");
                    manage::GroupManage::on_group_notify(
                        sender_id.to_owned(),
                        receiver_id.to_owned(),
                        &group_info,
                    );
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
    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        let my_user_id = PeerId::from_bytes(&user_id).unwrap();

        match proto_rpc::Group::decode(&data[..]) {
            Ok(group) => {
                match group.message {
                    Some(proto_rpc::group::Message::GroupCreateRequest(group_create_req)) => {
                        let id = GroupManage::create_new_group(
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
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );

                        // post updates
                        if status {
                            Self::post_group_update(&my_user_id, &group_rename_req.group_id);
                        }
                    }
                    Some(proto_rpc::group::Message::GroupInfoRequest(group_info_req)) => {
                        match GroupManage::group_info(&my_user_id, &group_info_req.group_id) {
                            Ok(res) => {
                                let proto_message = proto_rpc::Group {
                                    message: Some(proto_rpc::group::Message::GroupInfoResponse(
                                        res,
                                    )),
                                };

                                // send message
                                Rpc::send_message(
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
                    Some(proto_rpc::group::Message::GroupListRequest(_group_list_req)) => {
                        let list = GroupManage::group_list(&my_user_id);
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupListResponse(list)),
                        };
                        // send message
                        Rpc::send_message(
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupInvitedRequest(_group_invited_req)) => {
                        let invited = GroupManage::invited_list(&my_user_id);
                        let proto_message = proto_rpc::Group {
                            message: Some(proto_rpc::group::Message::GroupInvitedResponse(invited)),
                        };
                        // send message
                        Rpc::send_message(
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupInviteMemberRequest(invite_req)) => {
                        let mut status = true;
                        let mut message: String = "".to_string();
                        if let Err(err) = Member::invite(
                            &my_user_id,
                            &invite_req.group_id,
                            &PeerId::from_bytes(&invite_req.user_id).unwrap(),
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
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupReplyInviteRequest(reply_req)) => {
                        let mut status = true;
                        let mut message: String = "".to_string();

                        if let Err(err) =
                            Member::reply_invite(&my_user_id, &reply_req.group_id, reply_req.accept)
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
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto_rpc::group::Message::GroupRemoveMemberRequest(remove_req)) => {
                        let mut status = true;
                        let mut message: String = "".to_string();

                        if let Err(err) = Member::remove(
                            &my_user_id,
                            &remove_req.group_id,
                            &PeerId::from_bytes(&remove_req.user_id).unwrap(),
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
                            proto_message.encode_to_vec(),
                            crate::rpc::proto::Modules::Group.into(),
                            request_id,
                            Vec::new(),
                        );

                        if status {
                            Self::post_group_update(&my_user_id, &remove_req.group_id);
                        }
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
