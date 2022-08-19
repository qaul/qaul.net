//use bs58::decode;
use super::chat;
use super::messaging;
use libp2p::PeerId;
use prost::Message;

use super::Group;
use crate::{node::user_accounts::UserAccounts, utilities::timestamp};

pub struct Member {}
impl Member {
    /// invite member from rpc command
    pub fn invite(
        my_user_id: &PeerId,
        group_id: &Vec<u8>,
        user_id: &PeerId,
    ) -> Result<bool, String> {
        //check group
        let groups = Group::get_groups_of_user(my_user_id);

        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0 {
            return Err("can not find group".to_string());
        }
        let mut group = groups
            .db_ref
            .get(&group_idx.to_be_bytes())
            .unwrap()
            .unwrap();

        //check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow this action".to_string());
        }

        //check admin permission
        if let Some(member) = group.get_member(&my_user_id.to_bytes()) {
            if member.role != 255 {
                return Err("you haven't permission for remove member".to_string());
            }
        } else {
            return Err("you are not member in this group".to_string());
        }

        //check user
        if let Some(member) = group.get_member(&user_id.to_bytes()) {
            if member.state > 0 {
                return Err("user is already member in this group".to_string());
            } else {
                return Err("already sent invite to member".to_string());
            }
        }

        //send invite.
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::InviteMember(
                super::proto_net::InviteMember {
                    group_id: group.id.clone(),
                    group_name: group.name.clone(),
                    admin_id: my_user_id.to_bytes(),
                    created_at: group.created_at,
                    members_count: group.members.len() as u32,
                },
            )),
        };

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id) {
            Group::send_group_message_through_message(
                &user_account,
                user_id,
                &proto_message.encode_to_vec(),
            );

            //save new user
            let member = super::GroupMember {
                user_id: user_id.to_bytes(),
                role: super::proto_rpc::GroupMemberRole::User.try_into().unwrap(),
                joined_at: timestamp::Timestamp::get_timestamp(),
                state: super::proto_rpc::GroupMemberState::Invited
                    .try_into()
                    .unwrap(),
                last_message_index: 0,
            };

            group.members.insert(user_id.to_bytes(), member);

            if let Err(_e) = groups
                .db_ref
                .insert(group_idx.to_be_bytes().to_vec(), group)
            {
                return Err("group updating error!".to_string());
            } else if let Err(_e) = groups.db_ref.flush() {
                return Err("group updating error!".to_string());
            }
        } else {
            return Err("user account problem".to_string());
        }
        Ok(true)
    }

    /// reply to invited message from rpc command
    pub fn reply_invite(
        my_user_id: &PeerId,
        group_id: &Vec<u8>,
        user_id: &PeerId,
        accept: bool,
    ) -> Result<bool, String> {
        //if already has not direct chat room, it's not allowed
        //check if already has direct chat room
        let conversation_id =
            super::messaging::ConversationId::from_peers(my_user_id, user_id).unwrap();
        if !Group::is_group_exist(my_user_id, &conversation_id.to_bytes()) {
            return Err("you have not been received group invite meesage".to_string());
        }

        let groups = Group::get_groups_of_user(my_user_id);

        //check if there is group invite
        if !groups.invited_ref.contains_key(group_id).unwrap() {
            return Err("you have not group invited".to_string());
        }
        let invited = groups.invited_ref.get(group_id).unwrap().unwrap();
        if PeerId::from_bytes(&invited.sender_id).unwrap() != *user_id {
            return Err("the group invite sender is different than inviter".to_string());
        }

        //send reply.
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::ReplyInvite(
                super::proto_net::ReplyInvite {
                    group_id: group_id.clone(),
                    accept,
                },
            )),
        };

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id) {
            Group::send_group_message_through_message(
                &user_account,
                &user_id,
                &proto_message.encode_to_vec(),
            );

            // remove invited
            if let Ok(_) = groups.invited_ref.remove(group_id) {
                if let Err(_) = groups.invited_ref.flush() {
                    log::error!("group invite removing error");
                }
            }
        } else {
            return Err("user account problem".to_string());
        }
        Ok(true)
    }

    /// remove member from rpc command
    pub fn remove(
        my_user_id: &PeerId,
        group_id: &Vec<u8>,
        user_id: &PeerId,
    ) -> Result<bool, String> {
        let groups = Group::get_groups_of_user(my_user_id);
        let group_idx = groups.group_id_to_index(group_id);

        if group_idx == 0 {
            return Err("can not find group".to_string());
        }
        let mut group = groups
            .db_ref
            .get(&group_idx.to_be_bytes())
            .unwrap()
            .unwrap();

        //check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow this action".to_string());
        }

        if let Some(member) = group.get_member(&my_user_id.to_bytes()) {
            if member.role != 255 {
                return Err("you haven't permission for remove member".to_string());
            }
        } else {
            return Err("you are not member in this group".to_string());
        }

        if let Some(_member) = group.get_member(&user_id.to_bytes()) {
            // remove member
            group.members.remove(&user_id.to_bytes());
            if let Err(error) = groups.db_ref.insert(&group_idx.to_be_bytes(), group) {
                log::error!("group db updating error {}", error.to_string());
            }
            Group::update_groups_of_user(my_user_id, groups);
        } else {
            return Err("this user is not member of this group".to_string());
        }

        //check if already has direct chat room
        let conversation_id =
            super::messaging::ConversationId::from_peers(my_user_id, user_id).unwrap();
        if !Group::is_group_exist(my_user_id, &conversation_id.to_bytes()) {
            super::Manage::create_new_direct_chat_group(my_user_id, user_id);
        }

        //send direct message to removed user
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::Removed(
                super::proto_net::RemovedMember {
                    group_id: group_id.clone(),
                },
            )),
        };

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id) {
            Group::send_group_message_through_message(
                &user_account,
                user_id,
                &proto_message.encode_to_vec(),
            );
        } else {
            return Err("user account has problem".to_string());
        }

        //save group event
        let event = chat::rpc_proto::GroupEvent {
            event_type: chat::rpc_proto::GroupEventType::GroupLeft
                .try_into()
                .unwrap(),
            user_id: user_id.to_bytes(),
        };
        chat::Chat::save_event(
            my_user_id,
            my_user_id,
            chat::rpc_proto::ContentType::GroupEvent.try_into().unwrap(),
            &event.encode_to_vec(),
            &messaging::ConversationId::from_bytes(group_id).unwrap(),
        );
        Ok(true)
    }

    /// process group invite message from network
    pub fn on_be_invited(
        sender_id: &PeerId,
        receiver_id: &PeerId,
        req: &super::proto_net::InviteMember,
    ) {
        let groups = Group::get_groups_of_user(receiver_id);
        let invited = super::GroupInvited {
            id: req.group_id.clone(),
            sender_id: sender_id.to_bytes(),
            received_at: timestamp::Timestamp::get_timestamp(),
            created_at: req.created_at,
            name: req.group_name.clone(),
            member_count: req.members_count,
        };

        if let Err(_e) = groups.invited_ref.insert(req.group_id.clone(), invited) {
            log::error!("group invite stroing error!");
        }
        if let Err(_e) = groups.invited_ref.flush() {
            log::error!("group invite stroing error!");
        }
    }

    /// process accept invite message from network
    fn on_accpeted_invite(
        sender_id: &PeerId,
        receiver_id: &PeerId,
        resp: &super::proto_net::ReplyInvite,
    ) -> Result<bool, String> {
        let groups = Group::get_groups_of_user(receiver_id);

        let group_idx = groups.group_id_to_index(&resp.group_id);
        if group_idx == 0 {
            return Err("can not find group".to_string());
        }

        let group = groups
            .db_ref
            .get(&group_idx.to_be_bytes())
            .unwrap()
            .unwrap();

        //check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow accept invite".to_string());
        }

        //check if already has direct chat room
        let conversation_id =
            super::messaging::ConversationId::from_peers(sender_id, receiver_id).unwrap();
        if !Group::is_group_exist(receiver_id, &conversation_id.to_bytes()) {
            return Err("you have not sent invite".to_string());
        }

        // check if user is invite pending state
        if !group.members.contains_key(&sender_id.to_bytes()) {
            return Err("member is not invite pending state".to_string());
        }

        let mut member = group.members.get(&sender_id.to_bytes()).unwrap().clone();
        if member.state > 0 {
            return Err("member is already joined".to_string());
        }
        member.state = super::proto_rpc::GroupMemberState::Activated
            .try_into()
            .unwrap();
        Group::update_group_member(receiver_id, &resp.group_id, &member);

        //save event
        let event = chat::rpc_proto::GroupEvent {
            event_type: chat::rpc_proto::GroupEventType::GroupJoined
                .try_into()
                .unwrap(),
            user_id: sender_id.to_bytes(),
        };
        chat::Chat::save_event(
            &receiver_id,
            &sender_id,
            chat::rpc_proto::ContentType::GroupEvent.try_into().unwrap(),
            &event.encode_to_vec(),
            &messaging::ConversationId::from_bytes(&resp.group_id).unwrap(),
        );

        Ok(true)
    }

    /// process accept invite message from network
    fn on_declined_invite(
        sender_id: &PeerId,
        receiver_id: &PeerId,
        resp: &super::proto_net::ReplyInvite,
    ) -> Result<bool, String> {
        let groups = Group::get_groups_of_user(receiver_id);

        let group_idx = groups.group_id_to_index(&resp.group_id);
        if group_idx == 0 {
            return Err("can not find group".to_string());
        }

        let mut group = groups
            .db_ref
            .get(&group_idx.to_be_bytes())
            .unwrap()
            .unwrap();

        //check it's direct chat room
        if group.is_direct_chat {
            return Err("direct chat room does not allow accept invite".to_string());
        }

        //check if already has direct chat room
        let conversation_id =
            super::messaging::ConversationId::from_peers(sender_id, receiver_id).unwrap();
        if !Group::is_group_exist(receiver_id, &conversation_id.to_bytes()) {
            return Err("you have not sent invite".to_string());
        }

        // check if user is invite pending state
        if !group.members.contains_key(&sender_id.to_bytes()) {
            return Err("member is not invite pending state".to_string());
        }
        let member = group.members.get(&sender_id.to_bytes()).unwrap();
        if member.state > 0 {
            return Err("member is not invite pending state, it's joined".to_string());
        }

        group.members.remove(&sender_id.to_bytes());
        Group::update_group(receiver_id, &group);

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
            Self::on_declined_invite(sender_id, receiver_id, resp);
            Ok(false)
        }
    }
}
