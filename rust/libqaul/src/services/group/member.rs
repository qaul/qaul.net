//use bs58::decode;
use libp2p::{
    PeerId,
};
use prost::Message;

use crate::{node::user_accounts::{UserAccount, UserAccounts}, utilities::timestamp};
use super::Group;
use super::Chat;

pub struct Member{}
impl Member{
    /// invite member from rpc command
    pub fn invite(my_user_id: &PeerId, group_id: &Vec<u8>, user_id: &Vec<u8>) ->Result<bool, String>{
        let groups = Group::get_groups_of_user(my_user_id);

        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0{
            return Err("can not find group".to_string());
        }
        let group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();

        //check admin permission
        if let Some(member) = group.get_member(&my_user_id.to_bytes()){
            if member.role != 255{
                return Err("you haven't permission for remove member".to_string());
            }
        }else{
            return Err("you are not member in this group".to_string());
        }

        //check user
        if let Some(_member) = group.get_member(user_id){
            return Err("user is already member in this group".to_string());
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

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id){
            let receiver = PeerId::from_bytes(user_id).unwrap();
            Group::send_group_message_through_message(&user_account, &receiver, &proto_message.encode_to_vec());
        }else{
            return Err("user account problem".to_string());
        }
        Ok(true)
    }


    /// reply to invited message from rpc command
    pub fn reply_invite(my_user_id: &PeerId, group_id: &Vec<u8>, conversation_id: &Vec<u8>, accept: bool) ->Result<bool, String>{
        //send invite.
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::ReplyInvite(
                super::proto_net::ReplyInvite {
                    group_id: group_id.clone(),
                    accept
                },
            )),
        };

        let mut message_buff = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut message_buff)
            .expect("Vec<u8> provides capacity as needed");

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id){
            let receiver = PeerId::from_bytes(conversation_id).unwrap();
            Group::send_group_message_through_message(&user_account, &receiver, &message_buff);

            //save invite chat message
            Chat::save_outgoing_group_invite_reply_message(my_user_id.clone(), receiver.clone(), 
                group_id, accept);

        }else{
            return Err("user account problem".to_string());
        }
        Ok(true)
    }

    /// remove member from rpc command
    pub fn remove(my_user_id: &PeerId, group_id: &Vec<u8>, user_id: &Vec<u8>) ->Result<bool, String>{
        let groups = Group::get_groups_of_user(my_user_id);
        let group_idx = groups.group_id_to_index(group_id);

        if group_idx == 0{
            return Err("can not find group".to_string());
        }
        let mut group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();

        if let Some(member) = group.get_member(&my_user_id.to_bytes()){
            if member.role != 255{
                return Err("you haven't permission for remove member".to_string());
            }
        }else{
            return Err("you are not member in this group".to_string());
        }

        if let Some(_member) = group.get_member(&user_id){
            // remove member
            group.members.remove(user_id);
            if let Err(error) = groups.db_ref.insert(&group_idx.to_be_bytes(), group){
                log::error!("group db updating error {}", error.to_string());                
            }
            Group::update_groups_of_user(my_user_id, groups);

        }else{
            return Err("this user is not member of this group".to_string());
        }

        //send direct message to removed user
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::Removed(
                super::proto_net::RemovedMember {
                    group_id: group_id.clone(),
                },
            )),
        };

        let mut message_buff = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut message_buff)
            .expect("Vec<u8> provides capacity as needed");

        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id){
            let receiver = PeerId::from_bytes(user_id).unwrap();
            Group::send_group_message_through_message(&user_account, &receiver, &message_buff);
        }else{
            return Err("user account has problem".to_string());
        }
        //send remove notify.
        Ok(true)
    }

    /// process group invite message from network
    pub fn on_be_invited(_user: &UserAccount, sender_id: &PeerId, receiver_id: &PeerId, req: &super::proto_net::InviteMember){
        //save chat message
        // Chat::save_incoming_group_invite_message(receiver_id.clone(), sender_id.clone(), 
        //     &req.group_id, req.group_name.clone(), req.created_at, 
        //     &req.admin_id, req.members_count, signature);
    }

    /// process accept invite message from network
    fn on_accpeted_invite(sender_id: &PeerId, receiver_id: &PeerId, resp: &super::proto_net::ReplyInvite)->Result<bool, String>{
        let groups = Group::get_groups_of_user(receiver_id);

        //add new member
        let new_member = super::GroupMember{
            user_id: sender_id.to_bytes(),
            role: 0,
            joined_at: timestamp::Timestamp::get_timestamp(),
            state: 0,
            last_message_index: 0,
        };

        let group_idx = groups.group_id_to_index(&resp.group_id);
        if group_idx == 0{
            return Err("can not find group".to_string());
        }

        let mut group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();
        if group.members.contains_key(&sender_id.to_bytes()){
            return Err("member already exists".to_string());
        }
        group.members.insert(sender_id.to_bytes(), new_member);
        if let Err(error) = groups.db_ref.insert(&group_idx.to_be_bytes(), group){
            log::error!("group db updating error {}", error.to_string());            
        }
        Group::update_groups_of_user(receiver_id, groups);
        Ok(true)
    }

    /// process accept or decline invite message from network
    pub fn on_reply_invite(sender_id: &PeerId, receiver_id: &PeerId, resp: &super::proto_net::ReplyInvite)->Result<bool, String>{
        if resp.accept{
            Self::on_accpeted_invite(sender_id, receiver_id, resp)
        }else{
            Ok(false)
        }
    }
}
