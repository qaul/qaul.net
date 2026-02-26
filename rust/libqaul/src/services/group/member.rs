// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Members Management
//!
//! Invite new group members.
//! Accept or reject invitations.

use libp2p::PeerId;
use prost::Message;

use super::chat::{self, ChatStorage};
use super::group_id::GroupId;
use super::{Group, GroupMember, GroupStorage};
use crate::utilities::timestamp::Timestamp;
use crate::{node::user_accounts::UserAccounts, utilities::timestamp};

/// Group Member Structure
pub struct Member {}

impl Member {
    fn to_proto_net_member(member: &GroupMember) -> super::proto_net::GroupMember {
        super::proto_net::GroupMember {
            user_id: member.user_id.clone(),
            role: member.role,
            joined_at: member.joined_at,
            state: member.state,
            last_message_index: member.last_message_index,
        }
    }

    fn from_proto_net_member(member: &super::proto_net::GroupMember) -> GroupMember {
        GroupMember {
            user_id: member.user_id.clone(),
            role: member.role,
            joined_at: member.joined_at,
            state: member.state,
            last_message_index: member.last_message_index,
        }
    }

    fn group_event_message(
        event_type: chat::rpc_proto::GroupEventType,
        user_id: Vec<u8>,
    ) -> chat::rpc_proto::ChatContentMessage {
        chat::rpc_proto::ChatContentMessage {
            message: Some(chat::rpc_proto::chat_content_message::Message::GroupEvent(
                chat::rpc_proto::GroupEvent {
                    event_type: event_type as i32,
                    user_id,
                },
            )),
        }
    }

    fn save_group_event_message(
        account_id: &PeerId,
        group_id: &GroupId,
        sender_id: &PeerId,
        event_type: chat::rpc_proto::GroupEventType,
        user_id: Vec<u8>,
    ) {
        ChatStorage::save_message(
            account_id,
            group_id,
            sender_id,
            &[],
            Timestamp::get_timestamp(),
            Self::group_event_message(event_type, user_id),
            chat::rpc_proto::MessageStatus::Received,
        );
    }

    /// invite member from rpc command
    pub fn invite(account_id: &PeerId, group_id: &[u8], user_id: &PeerId) -> Result<bool, String> {
        // get group
        let mut group;
        match GroupStorage::get_group(account_id.to_owned(), group_id) {
            Some(my_group) => group = my_group,
            None => return Err("group not found".to_string()),
        }

        let account_id_bytes = account_id.to_bytes();
        let user_id_bytes = user_id.to_bytes();

        // check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow this action".to_string());
        }

        // check admin permission
        if let Some(member) = group.get_member(&account_id_bytes) {
            if member.role != 255 {
                return Err("you haven't permission for remove member".to_string());
            }
        } else {
            return Err("you are not member in this group".to_string());
        }

        // check user
        if let Some(member) = group.get_member(&user_id_bytes) {
            if member.state > 0 {
                return Err("user is already member in this group".to_string());
            } else {
                return Err("already sent invite to member".to_string());
            }
        }

        // send invite.
        let mut members = Vec::with_capacity(group.members.len());
        for member in group.members.values() {
            members.push(Self::to_proto_net_member(member));
        }

        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::InviteMember(
                super::proto_net::InviteMember {
                    group: Some(super::proto_net::GroupInfo {
                        group_id: group.id.clone(),
                        group_name: group.name.clone(),
                        created_at: group.created_at,
                        revision: group.revision,
                        members,
                    }),
                },
            )),
        };

        if let Some(user_account) = UserAccounts::get_by_id(*account_id) {
            Group::send_notify_message(&user_account, user_id, proto_message.encode_to_vec());

            // save new user
            let member = super::GroupMember {
                user_id: user_id_bytes.clone(),
                role: super::proto_rpc::GroupMemberRole::User.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Invited
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            };

            group.members.insert(user_id_bytes, member);

            GroupStorage::save_group(user_account.id, group);
        } else {
            return Err("user account problem".to_string());
        }
        Ok(true)
    }

    /// reply to invited message from rpc command
    pub fn reply_invite(
        account_id: &PeerId,
        group_id: &[u8],
        accept: bool,
    ) -> Result<bool, String> {
        // check if there is a group invite
        let invite;
        match GroupStorage::get_invite(account_id.to_owned(), group_id) {
            Some(my_invite) => invite = my_invite,
            None => return Err("there is no group invite".to_string()),
        }

        // create receiver id
        let receiver;
        match PeerId::from_bytes(&invite.sender_id) {
            Ok(sender) => receiver = sender,
            Err(_e) => return Err("invite sender_id not valid".to_string()),
        }

        // send reply.
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::ReplyInvite(
                super::proto_net::ReplyInvite {
                    group_id: group_id.to_vec(),
                    accept,
                },
            )),
        };

        if let Some(user_account) = UserAccounts::get_by_id(*account_id) {
            Group::send_notify_message(&user_account, &receiver, proto_message.encode_to_vec());

            // remove invited
            GroupStorage::remove_invite_deferred(account_id.to_owned(), group_id);
        } else {
            return Err("user account problem".to_string());
        }

        // save group into data base if invite was accepted
        if accept {
            // save group to data base
            GroupStorage::save_group_deferred(account_id.to_owned(), invite.group);
        }
        GroupStorage::flush_account(account_id);

        if accept {
            Self::save_group_event_message(
                account_id,
                &GroupId::from_bytes(group_id).unwrap(),
                account_id,
                chat::rpc_proto::GroupEventType::InviteAccepted,
                account_id.to_bytes(),
            );
        }

        Ok(true)
    }

    /// remove member from rpc command
    pub fn remove(account_id: &PeerId, group_id: &[u8], user_id: &PeerId) -> Result<bool, String> {
        // get user account from node
        let user_account;
        match UserAccounts::get_by_id(*account_id) {
            Some(my_account) => user_account = my_account,
            None => return Err("user account has problem".to_string()),
        }

        let account_id_bytes = account_id.to_bytes();
        let user_id_bytes = user_id.to_bytes();

        // get group from data base
        let mut group;
        match GroupStorage::get_group(account_id.to_owned(), group_id) {
            Some(my_group) => group = my_group,
            None => return Err("group not found".to_string()),
        }

        //check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow this action".to_string());
        }

        // check permissions
        if let Some(member) = group.get_member(&account_id_bytes) {
            if member.role != 255 {
                return Err("you haven't permission for remove member".to_string());
            }
        } else {
            return Err("you are not member in this group".to_string());
        }

        // update group
        if let Some(_member) = group.get_member(&user_id_bytes) {
            // remove member
            group.members.remove(&user_id_bytes);

            // set new revision
            group.revision += 1;

            // save to data base
            GroupStorage::save_group(account_id.to_owned(), group);
        } else {
            return Err("this user is not member of this group".to_string());
        }

        // send direct message to removed user
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::Removed(
                super::proto_net::RemovedMember {
                    group_id: group_id.to_vec(),
                },
            )),
        };
        Group::send_notify_message(&user_account, user_id, proto_message.encode_to_vec());

        // save group event
        Self::save_group_event_message(
            account_id,
            &GroupId::from_bytes(group_id).unwrap(),
            user_id,
            chat::rpc_proto::GroupEventType::Left,
            user_id.to_bytes(),
        );

        Ok(true)
    }

    /// process group invite message from network
    pub fn on_be_invited(
        sender_id: &PeerId,
        account_id: &PeerId,
        invite_message: &super::proto_net::InviteMember,
    ) {
        let Some(group_info) = invite_message.group.as_ref() else {
            log::error!("invite message contains no group");
            return;
        };

        // create new group
        let mut group = Group::new();

        for member in &group_info.members {
            group
                .members
                .insert(member.user_id.clone(), Self::from_proto_net_member(member));
        }

        group.id = group_info.group_id.clone();
        group.name = group_info.group_name.clone();
        group.created_at = group_info.created_at;
        group.status = super::proto_rpc::GroupStatus::InviteAccepted as i32;
        group.revision = group_info.revision;

        let invited = super::GroupInvited {
            sender_id: sender_id.to_bytes(),
            received_at: timestamp::Timestamp::get_timestamp(),
            group,
        };

        GroupStorage::save_invite(account_id.to_owned(), invited);
    }

    /// process incoming invite accept
    fn on_accpeted_invite(
        sender_id: &PeerId,
        account_id: &PeerId,
        resp: &super::proto_net::ReplyInvite,
    ) -> Result<bool, String> {
        // get group from data base
        let mut group;
        match GroupStorage::get_group(account_id.to_owned(), &resp.group_id) {
            Some(my_group) => group = my_group,
            None => return Err("group not found".to_string()),
        }
        let sender_id_bytes = sender_id.to_bytes();

        // check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow accept invite".to_string());
        }

        // check if user is invite pending state
        let member = group
            .members
            .get_mut(&sender_id_bytes)
            .ok_or_else(|| "member has no invite pending".to_string())?;
        if member.state > 0 {
            return Err("member has already joined".to_string());
        }

        // update group member state
        member.state = super::proto_rpc::GroupMemberState::Activated as i32;

        // update revision
        group.revision += 1;

        // save group
        let group_id = GroupId::from_bytes(&group.id).unwrap();
        GroupStorage::save_group(account_id.to_owned(), group);

        // save event
        Self::save_group_event_message(
            account_id,
            &group_id,
            sender_id,
            chat::rpc_proto::GroupEventType::Joined,
            sender_id.to_bytes(),
        );

        Ok(true)
    }

    /// process reject group invite
    fn on_declined_invite(
        sender_id: &PeerId,
        account_id: &PeerId,
        resp: &super::proto_net::ReplyInvite,
    ) -> Result<bool, String> {
        // get group from data base
        let mut group;
        match GroupStorage::get_group(account_id.to_owned(), &resp.group_id) {
            Some(my_group) => group = my_group,
            None => return Err("group not found".to_string()),
        }
        let sender_id_bytes = sender_id.to_bytes();

        // check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow accept invite".to_string());
        }

        // check if user is invite pending state
        if !group.members.contains_key(&sender_id_bytes) {
            return Err("member is not invite pending state".to_string());
        }
        let member = group.members.get(&sender_id_bytes).unwrap();
        if member.state > 0 {
            return Err("member is not invite pending state, it's joined".to_string());
        }

        group.members.remove(&sender_id_bytes);
        GroupStorage::save_group(account_id.to_owned(), group);

        Ok(true)
    }

    /// process accept or decline invite message from network
    pub fn on_reply_invite(
        sender_id: &PeerId,
        receiver_id: &PeerId,
        resp: &super::proto_net::ReplyInvite,
    ) -> Result<bool, String> {
        if resp.accept {
            Self::on_accpeted_invite(sender_id, receiver_id, resp)
        } else {
            if let Err(e) = Self::on_declined_invite(sender_id, receiver_id, resp) {
                log::error!("on_decline error {}", e);
            }
            Ok(false)
        }
    }

    /// user has been removed from group by administrator
    pub fn on_removed(
        sender_id: &PeerId,
        account_id: &PeerId,
        message: &super::proto_net::RemovedMember,
    ) -> Result<bool, String> {
        // get group from data base
        let mut group;
        match GroupStorage::get_group(account_id.to_owned(), &message.group_id) {
            Some(my_group) => group = my_group,
            None => return Err("group not found".to_string()),
        }
        let sender_id_bytes = sender_id.to_bytes();
        let account_id_bytes = account_id.to_bytes();

        // check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow user removal".to_string());
        }

        // check if sender is administrator
        match group.members.get(&sender_id_bytes) {
            Some(admin) => {
                if admin.role != super::proto_rpc::GroupMemberRole::Admin as i32 {
                    return Err("sender is not administrator".to_string());
                }
            }
            None => return Err("sender is not in group".to_string()),
        }

        // remove self from group
        group.members.remove(&account_id_bytes);

        // set group deactivation status
        group.status = super::proto_rpc::GroupStatus::Deactivated as i32;

        // save group
        let group_id = GroupId::from_bytes(&group.id).unwrap();
        GroupStorage::save_group(account_id.to_owned(), group);

        // save event
        Self::save_group_event_message(
            account_id,
            &group_id,
            sender_id,
            chat::rpc_proto::GroupEventType::Removed,
            account_id_bytes,
        );

        Ok(true)
    }
}
