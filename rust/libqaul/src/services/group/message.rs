//use bs58::decode;
use libp2p::{
    PeerId,
};

use prost::Message;
use crate::{node::user_accounts::{UserAccounts}, utilities::timestamp};
use super::Group;
use super::proto;
use super::Chat;


pub struct GroupMessage{}
impl GroupMessage{
    /// send group message from rpc command
    pub fn send(my_user_id: &PeerId, group_id: &Vec<u8>, message: String )  ->Result<bool, String>{
        let groups = Group::get_groups_of_user(my_user_id);
        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0{
            return Err("can not find group".to_string());
        }
        let mut group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();
        let mut member;
        //check member
        match group.get_member(&my_user_id.to_bytes()){
            Some(m)=>{
                member = m.clone();
            },
            None =>{
                return Err("you are not member in this group".to_string());
            }
        }

        let last_index = member.last_message_index + 1;

        let timestamp = timestamp::Timestamp::get_timestamp();
        let conversation_id = super::messaging::ConversationId::from_bytes(&group.id).unwrap();
        let message_id = super::messaging::Messaging::generate_group_message_id(&group.id, my_user_id, last_index);

        // pack message
        let common_message = proto::CommonMessage{
            message_id: message_id.clone(),
            conversation_id: conversation_id.to_bytes(),
            sent_at: timestamp,
            payload: Some(proto::common_message::Payload::ChatMessage(
                        proto::ChatMessage {
                            content: message.clone(),
                        }
                    ))
        };

        // save outgoing message
        Chat::save_outgoing_message(my_user_id, my_user_id, &conversation_id, &message_id, &common_message.encode_to_vec(), 0);

        //broad cast to all group members
        if let Some(user_account) = UserAccounts::get_by_id(my_user_id.clone()){
            for user_id in group.members.keys(){
                let receiver = PeerId::from_bytes(&user_id.clone()).unwrap();
                if receiver != *my_user_id{
                    if let Err(error) = Chat::send(&user_account, &receiver, &common_message){
                        log::error!("sending group message error {}", error);
                    }
                }                
            }
        }

        //update member state
        member.last_message_index = last_index;
        group.members.insert(member.user_id.clone(), member);
        groups.db_ref.insert(&group_idx.to_be_bytes(), group);
        groups.db_ref.flush();

        Ok(true)
    }

    /// process group message from network
    pub fn on_message(sender_id: &PeerId, receiver_id: &PeerId, group_id: &Vec<u8>, message_id: &Vec<u8>)  ->Result<bool, String>{

        let group;
        match Group::get_group(receiver_id, group_id){
            Ok(v)=>{
                group = v;
            },
            Err(error)=>{
                return Err(error);
            }
        }

        //check member
        match group.get_member(&receiver_id.to_bytes()){
            Some(_)=>{},
            None =>{
                return Err("you are not member in this group".to_string());
            }
        }

        let mut sender;
        // checkif the sender is in group
        match group.get_member(&sender_id.to_bytes()){
            Some(v)=>{
                sender = v.clone();
            },
            None =>{
                return Err("the sender is not member in this group".to_string());
            }
        }

        // check message id
        if message_id.len() != 20{
            return Err("invalid group message id".to_string());
        }
        let group_crc = crc::crc64::checksum_iso(&group_id.clone());
        let sender_crc = crc::crc64::checksum_iso(&sender_id.to_bytes());
        let group_crc1 = u64::from_be_bytes(message_id[0..8].try_into().unwrap());
        let sender_crc1 = u64::from_be_bytes(message_id[8..16].try_into().unwrap());
        if group_crc != group_crc1 || sender_crc != sender_crc1{
            return Err("invalid group message id-1".to_string());
        }
        let sender_msg_index = u32::from_be_bytes(message_id[16..20].try_into().unwrap());

        // change members status
        sender.last_message_index = sender_msg_index;
        super::Group::update_group_member(&receiver_id, group_id, &sender);
        Ok(true)
    }

}
