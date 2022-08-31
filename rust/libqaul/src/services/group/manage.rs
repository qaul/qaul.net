// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Management

use libp2p::PeerId;
use prost::Message;
use std::collections::BTreeMap;

use super::chat::{self, ChatStorage};
use super::conversation_id::ConversationId;
use super::{Group, GroupStorage};
use crate::utilities::timestamp;

/// Group Manage Structure
pub struct Manage {}
impl Manage {
    /// Create a new direct chat group
    ///
    /// The function expects two qaul user ID's:
    ///
    /// * `account_id` your user account ID
    /// * `user_id` the user ID of the other user
    pub fn create_new_direct_chat_group(account_id: &PeerId, user_id: &PeerId) -> Group {
        let group_id = ConversationId::from_peers(account_id, user_id).to_bytes();

        // check if group already exists
        if let Some(group) = GroupStorage::get_group(account_id.to_owned(), group_id.clone()) {
            return group;
        }

        let mut members = BTreeMap::new();
        members.insert(
            account_id.to_bytes(),
            super::GroupMember {
                user_id: account_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );
        members.insert(
            user_id.to_bytes(),
            super::GroupMember {
                user_id: user_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );

        let new_group = super::Group {
            id: group_id.clone(),
            name: "".to_string(),
            revision: 0,
            is_direct_chat: true,
            created_at: timestamp::Timestamp::get_timestamp(),
            members,
        };

        // save group to data base
        GroupStorage::save_group(account_id.to_owned(), new_group.clone());

        new_group
    }

    /// create new group from rpc command
    pub fn create_new_group(account_id: &PeerId, name: String) -> Vec<u8> {
        let id = uuid::Uuid::new_v4().as_bytes().to_vec();

        let mut members = BTreeMap::new();
        members.insert(
            account_id.to_bytes(),
            super::GroupMember {
                user_id: account_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::Admin.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Activated
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            },
        );

        let new_group = super::Group {
            id: id.clone(),
            name,
            revision: 0,
            is_direct_chat: false,
            created_at: timestamp::Timestamp::get_timestamp(),
            members,
        };

        // save group
        GroupStorage::save_group(account_id.to_owned(), new_group);

        return id;
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

            // save group
            GroupStorage::save_group(account_id.to_owned(), group);

            return Ok(());
        }

        Err("can not find group".to_string())
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
            revision: group.revision,
            is_direct_chat: group.is_direct_chat,
            members,
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
                Ok((_, group)) => {
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
                        revision: group.revision,
                        is_direct_chat: group.is_direct_chat,
                        members,
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
                Ok((_, invite)) => {
                    let mut members: Vec<super::proto_rpc::GroupMember> = Vec::new();
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
                            revision: invite.group.revision,
                            is_direct_chat: invite.group.is_direct_chat,
                            members,
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
    ///
    /// RISK: Someone could force us into a group without our invitation agreement.
    pub fn on_group_notify(
        sender_id: PeerId,
        account_id: PeerId,
        notify: &super::proto_net::GroupInfo,
    ) {
        // check for valid group ID
        let conversation_id;
        match ConversationId::from_bytes(&notify.group_id) {
            Ok(id) => conversation_id = id,
            Err(e) => {
                log::error!("invalid group id: {}", e);
                return;
            }
        }

        let mut first_join = false;
        let mut orign_members: BTreeMap<Vec<u8>, bool> = BTreeMap::new();
        let mut new_members: Vec<Vec<u8>> = vec![];

        // get group
        match GroupStorage::get_group(account_id, notify.group_id.clone()) {
            Some(group) => {
                let mut sender_is_admin = false;
                for (member_id, member) in &group.members {
                    orign_members.insert(member_id.clone(), true);

                    if member.user_id == sender_id.to_bytes() && member.role == 255 {
                        sender_is_admin = true;
                    }
                }

                // check if sender is administrator, otherwise return
                if !sender_is_admin {
                    log::error!(
                        "illegitimate update from user {} for group {}",
                        sender_id.to_base58(),
                        conversation_id.to_string(),
                    );
                    return;
                }
            }
            None => first_join = true,
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

        let group = super::Group {
            id: notify.group_id.clone(),
            name: notify.group_name.clone(),
            is_direct_chat: false,
            created_at: notify.created_at,
            revision: notify.revision,
            members,
        };

        // save group
        GroupStorage::save_group(account_id, group);

        // save events
        if first_join {
            let event = chat::rpc_proto::GroupEvent {
                event_type: chat::rpc_proto::GroupEventType::Joined.try_into().unwrap(),
                user_id: account_id.to_bytes(),
            };
            ChatStorage::save_event(
                &account_id,
                &sender_id,
                chat::rpc_proto::ChatContentType::Group.try_into().unwrap(),
                &event.encode_to_vec(),
                &conversation_id,
            );
        } else {
            for new_member in &new_members {
                let event = chat::rpc_proto::GroupEvent {
                    event_type: chat::rpc_proto::GroupEventType::Joined.try_into().unwrap(),
                    user_id: new_member.clone(),
                };
                ChatStorage::save_event(
                    &account_id,
                    &sender_id,
                    chat::rpc_proto::ChatContentType::Group.try_into().unwrap(),
                    &event.encode_to_vec(),
                    &conversation_id,
                );
            }

            for left_member in orign_members.keys() {
                let event = chat::rpc_proto::GroupEvent {
                    event_type: chat::rpc_proto::GroupEventType::Left.try_into().unwrap(),
                    user_id: left_member.clone(),
                };
                ChatStorage::save_event(
                    &account_id,
                    &sender_id,
                    chat::rpc_proto::ChatContentType::Group.try_into().unwrap(),
                    &event.encode_to_vec(),
                    &conversation_id,
                );
            }
        }
    }
}
