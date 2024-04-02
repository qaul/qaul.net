// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Management

use libp2p::PeerId;
use std::collections::BTreeMap;

use super::group_id::GroupId;
use super::{Group, GroupInvited, GroupStorage};
use crate::services::chat::{self, Chat, ChatStorage};
use crate::utilities::timestamp::Timestamp;

/// Group Manage Structure
pub struct GroupManage {}
impl GroupManage {
    /// Get a group from the data base
    ///
    /// If it is a direct chat group, and does not yet exist
    /// this function will create a new direct chat group and
    /// return it.
    pub fn get_group_create_direct(
        account_id: PeerId,
        group_id: GroupId,
        remote_id: &PeerId,
    ) -> Option<Group> {
        // try to get group from data base
        match GroupStorage::get_group(account_id.clone(), group_id.to_bytes()) {
            Some(group) => return Some(group),
            None => {
                // check if it is the direct chat group for the connection
                if group_id == GroupId::from_peers(&account_id, remote_id) {
                    // create a new direct chat group
                    let group = Self::create_new_direct_chat_group(&account_id, &remote_id);
                    return Some(group);
                }
            }
        }

        None
    }

    /// Create a new direct chat group
    ///
    /// The function expects two qaul user ID's:
    ///
    /// * `account_id` your user account ID
    /// * `user_id` the user ID of the other user
    pub fn create_new_direct_chat_group(account_id: &PeerId, user_id: &PeerId) -> Group {
        let group_id = GroupId::from_peers(account_id, user_id).to_bytes();

        // check if group already exists
        if let Some(group) = GroupStorage::get_group(account_id.to_owned(), group_id.clone()) {
            return group;
        }

        // create new group
        let mut group = Group::new();
        group.members.insert(
            account_id.to_bytes(),
            super::GroupMember {
                user_id: account_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );
        group.members.insert(
            user_id.to_bytes(),
            super::GroupMember {
                user_id: user_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );

        group.id = group_id.clone();
        group.is_direct_chat = true;

        // save group to data base
        GroupStorage::save_group(account_id.to_owned(), group.clone());

        group
    }

    /// create new group from rpc command
    pub fn create_new_group(account_id: &PeerId, name: String) -> Vec<u8> {
        let mut group = Group::new();

        group.id = uuid::Uuid::new_v4().as_bytes().to_vec();

        group.members.insert(
            account_id.to_bytes(),
            super::GroupMember {
                user_id: account_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated as i32,
                last_message_index: 0,
            },
        );

        group.name = name;

        // save group
        GroupStorage::save_group(account_id.to_owned(), group.clone());

        // save group created event
        let event = chat::rpc_proto::ChatContentMessage {
            message: Some(chat::rpc_proto::chat_content_message::Message::GroupEvent(
                chat::rpc_proto::GroupEvent {
                    event_type: chat::rpc_proto::GroupEventType::Created as i32,
                    user_id: account_id.to_bytes(),
                },
            )),
        };

        ChatStorage::save_message(
            account_id,
            &GroupId::from_bytes(&group.id).unwrap(),
            account_id,
            &Vec::new(),
            Timestamp::get_timestamp(),
            event,
            chat::rpc_proto::MessageStatus::Received,
        );

        return group.id;
    }

    /// rename group from RPC command
    ///
    /// `account_id` the user account ID
    pub fn rename_group(
        account_id: &PeerId,
        group_id: &Vec<u8>,
        name: String,
    ) -> Result<(), String> {
        if let Some(mut group) = GroupStorage::get_group(account_id.to_owned(), group_id.to_owned())
        {
            // check if administrator
            if let Some(member) = group.get_member(&account_id.to_bytes()) {
                // check permission
                if member.role != 255 {
                    return Err("you don't have the permissions to rename this group".to_string());
                }
            } else {
                return Err("you are not a member for this group".to_string());
            }

            // rename group
            group.name = name;

            // update revision
            group.revision = group.revision + 1;

            // save group
            GroupStorage::save_group(account_id.to_owned(), group);

            return Ok(());
        }

        Err("can not find group".to_string())
    }

    /// get a new message ID
    pub fn get_new_message_id(account_id: &PeerId, group_id: &Vec<u8>) -> Vec<u8> {
        if let Some(mut group) = GroupStorage::get_group(account_id.to_owned(), group_id.to_owned())
        {
            // get my member
            if let Some(member) = group.members.get(&account_id.to_bytes()) {
                let new_index = member.last_message_index + 1;

                // update & save last_index in group
                let mut member_updated = member.to_owned();
                member_updated.last_message_index = new_index;
                group.members.insert(account_id.to_bytes(), member_updated);
                GroupStorage::save_group(account_id.to_owned(), group);

                // create message id
                return Chat::generate_message_id(group_id, account_id, new_index);
            }
        }

        Vec::new()
    }

    /// get group information from rpc command
    ///
    /// `account_id` the user account ID
    pub fn group_info(
        account_id: &PeerId,
        group_id: &Vec<u8>,
    ) -> Result<super::proto_rpc::GroupInfo, String> {
        let group;
        match GroupStorage::get_group(account_id.to_owned(), group_id.to_owned()) {
            Some(group_result) => group = group_result,
            None => return Err("group not found".to_string()),
        }

        let mut members: Vec<super::proto_rpc::GroupMember> = vec![];
        for m in group.members.values() {
            let member = super::proto_rpc::GroupMember {
                user_id: m.user_id.clone(),
                role: m.role,
                joined_at: m.joined_at,
                state: m.state,
                last_message_index: m.last_message_index,
            };
            members.push(member);
        }

        let res = super::proto_rpc::GroupInfo {
            group_id: group.id,
            group_name: group.name,
            created_at: group.created_at,
            status: group.status,
            revision: group.revision,
            is_direct_chat: group.is_direct_chat,
            members,
            unread_messages: group.unread_messages,
            last_message_at: group.last_message_at,
            last_message: group.last_message_data,
            last_message_sender_id: group.last_message_sender_id,
        };
        Ok(res)
    }

    /// get group list from rpc command
    ///
    /// `account_id` the user account ID
    pub fn group_list(account_id: &PeerId) -> super::proto_rpc::GroupListResponse {
        let db_ref = GroupStorage::get_db_ref(account_id.to_owned());

        let mut res = super::proto_rpc::GroupListResponse { groups: vec![] };

        for entry in db_ref.groups.iter() {
            match entry {
                Ok((_, group_bytes)) => {
                    let group: Group = bincode::deserialize(&group_bytes).unwrap();
                    let mut members: Vec<super::proto_rpc::GroupMember> = vec![];
                    for m in group.members.values() {
                        let member = super::proto_rpc::GroupMember {
                            user_id: m.user_id.clone(),
                            role: m.role,
                            joined_at: m.joined_at,
                            state: m.state,
                            last_message_index: m.last_message_index,
                        };
                        members.push(member);
                    }

                    let grp = super::proto_rpc::GroupInfo {
                        group_id: group.id,
                        group_name: group.name,
                        created_at: group.created_at,
                        status: group.status,
                        revision: group.revision,
                        is_direct_chat: group.is_direct_chat,
                        members,
                        unread_messages: group.unread_messages,
                        last_message_at: group.last_message_at,
                        last_message: group.last_message_data,
                        last_message_sender_id: group.last_message_sender_id,
                    };
                    res.groups.push(grp);
                }
                _ => {}
            }
        }
        res
    }

    /// get invited list from rpc command
    pub fn invited_list(account_id: &PeerId) -> super::proto_rpc::GroupInvitedResponse {
        let db_ref = GroupStorage::get_db_ref(account_id.to_owned());

        let mut res = super::proto_rpc::GroupInvitedResponse { invited: vec![] };

        for entry in db_ref.invited.iter() {
            match entry {
                Ok((_, invite_bytes)) => {
                    let mut members: Vec<super::proto_rpc::GroupMember> = Vec::new();
                    let invite: GroupInvited = bincode::deserialize(&invite_bytes).unwrap();
                    for (_, member) in invite.group.members {
                        members.push(super::proto_rpc::GroupMember {
                            user_id: member.user_id,
                            role: member.role,
                            joined_at: member.joined_at,
                            state: member.state,
                            last_message_index: member.last_message_index,
                        });
                    }

                    let invited = super::proto_rpc::GroupInvited {
                        sender_id: invite.sender_id.clone(),
                        received_at: invite.received_at,
                        group: Some(super::proto_rpc::GroupInfo {
                            group_id: invite.group.id,
                            group_name: invite.group.name,
                            created_at: invite.group.created_at,
                            status: invite.group.status,
                            revision: invite.group.revision,
                            is_direct_chat: invite.group.is_direct_chat,
                            members,
                            unread_messages: 0,
                            last_message_at: 0,
                            last_message: Vec::new(),
                            last_message_sender_id: Vec::new(),
                        }),
                    };

                    res.invited.push(invited);
                }
                _ => {}
            }
        }
        res
    }

    /// process group notify message from network
    pub fn on_group_notify(
        sender_id: PeerId,
        account_id: PeerId,
        notify: &super::proto_net::GroupInfo,
    ) {
        // check for valid group ID
        let group_id;
        match GroupId::from_bytes(&notify.group_id) {
            Ok(id) => group_id = id,
            Err(e) => {
                log::error!("invalid group id: {}", e);
                return;
            }
        }

        let mut first_join = false;
        let mut orign_members: BTreeMap<Vec<u8>, bool> = BTreeMap::new();
        let mut new_members: Vec<Vec<u8>> = vec![];

        // get group
        let mut group: Group;
        match GroupStorage::get_group(account_id, notify.group_id.clone()) {
            Some(my_group) => {
                group = my_group;

                // check if the sent revision is higher then the one we already have
                // return otherwise
                if group.revision >= notify.revision {
                    log::warn!("group update: got a smaller revision");
                    return;
                }

                // check if sender is administrator, otherwise return
                let mut sender_is_admin = false;
                for (member_id, member) in &group.members {
                    orign_members.insert(member_id.clone(), true);

                    if member.user_id == sender_id.to_bytes() && member.role == 255 {
                        sender_is_admin = true;
                    }
                }

                if !sender_is_admin {
                    log::error!(
                        "illegitimate update from user {} for group {}",
                        sender_id.to_base58(),
                        group_id.to_string(),
                    );
                    return;
                }
            }
            None => {
                first_join = true;

                group = Group::new();
            }
        }

        // check for new members
        let mut members: BTreeMap<Vec<u8>, super::GroupMember> = BTreeMap::new();
        for m in &notify.members {
            if orign_members.contains_key(&m.user_id) {
                orign_members.remove(&m.user_id);
            } else {
                new_members.push(m.user_id.clone());
            }

            members.insert(
                m.user_id.clone(),
                super::GroupMember {
                    user_id: m.user_id.clone(),
                    role: m.role,
                    joined_at: m.joined_at,
                    state: m.state,
                    last_message_index: m.last_message_index,
                },
            );
        }

        // update group
        group.id = notify.group_id.clone();
        group.name = notify.group_name.clone();
        group.created_at = notify.created_at;
        group.revision = notify.revision;
        group.members = members;

        // activate group after invite accept
        if group.status == super::proto_rpc::GroupStatus::InviteAccepted as i32 {
            group.status = super::proto_rpc::GroupStatus::Active as i32;
        }

        // save group
        GroupStorage::save_group(account_id, group);

        // save events
        if first_join {
            let event = chat::rpc_proto::ChatContentMessage {
                message: Some(chat::rpc_proto::chat_content_message::Message::GroupEvent(
                    chat::rpc_proto::GroupEvent {
                        event_type: chat::rpc_proto::GroupEventType::Joined.try_into().unwrap(),
                        user_id: account_id.to_bytes(),
                    },
                )),
            };

            ChatStorage::save_message(
                &account_id,
                &group_id,
                &sender_id,
                &Vec::new(),
                Timestamp::get_timestamp(),
                event,
                chat::rpc_proto::MessageStatus::Received,
            );
        } else {
            for new_member in &new_members {
                let event = chat::rpc_proto::ChatContentMessage {
                    message: Some(chat::rpc_proto::chat_content_message::Message::GroupEvent(
                        chat::rpc_proto::GroupEvent {
                            event_type: chat::rpc_proto::GroupEventType::Joined.try_into().unwrap(),
                            user_id: new_member.clone(),
                        },
                    )),
                };

                ChatStorage::save_message(
                    &account_id,
                    &group_id,
                    &sender_id,
                    &Vec::new(),
                    Timestamp::get_timestamp(),
                    event,
                    chat::rpc_proto::MessageStatus::Received,
                );
            }

            for left_member in orign_members.keys() {
                let event = chat::rpc_proto::ChatContentMessage {
                    message: Some(chat::rpc_proto::chat_content_message::Message::GroupEvent(
                        chat::rpc_proto::GroupEvent {
                            event_type: chat::rpc_proto::GroupEventType::Left.try_into().unwrap(),
                            user_id: left_member.clone(),
                        },
                    )),
                };

                ChatStorage::save_message(
                    &account_id,
                    &group_id,
                    &sender_id,
                    &Vec::new(),
                    Timestamp::get_timestamp(),
                    event,
                    chat::rpc_proto::MessageStatus::Received,
                );
            }
        }
    }
}
