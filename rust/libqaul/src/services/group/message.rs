//use bs58::decode;
use libp2p::{
    PeerId,
};

use prost::Message;
use crate::{node::user_accounts::{UserAccounts}, utilities::timestamp};
use super::Group;
use super::Chat;

pub struct GroupMessage{}
impl GroupMessage{
    /// send group message from rpc command
    pub fn send(my_user_id: &PeerId, group_id: &Vec<u8>, message: String )  ->Result<bool, String>{
        let groups = Group::get_groups_of_user(my_user_id.clone());
        let group_idx = groups.group_id_to_index(group_id);
        if group_idx == 0{
            return Err("can not find group".to_string());
        }
        let group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();

        //check member
        match group.get_member(&my_user_id.to_bytes()){
            Some(_)=>{},
            None =>{
                return Err("you are not member in this group".to_string());
            }
        }

        //create group message
        let proto_message = super::proto_net::GroupContainer {
            message: Some(super::proto_net::group_container::Message::GroupMessage(
                super::proto_net::GroupMessage {
                    group_id: group_id.clone(),
                    content: message.clone(),
                    sent_at: timestamp::Timestamp::get_timestamp(),
                },
            )),
        };

        let mut message_buff = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut message_buff)
            .expect("Vec<u8> provides capacity as needed");


        //broad cast to all group members
        if let Some(user_account) = UserAccounts::get_by_id(*my_user_id){
            for user_id in group.members.keys(){
                let receiver = PeerId::from_bytes(&user_id.clone()).unwrap();                
                if receiver != *my_user_id{
                    Group::send_group_message_through_message(&user_account, receiver, &group.id, &message_buff);
                }                
            }
        }
        Ok(true)
    }

    /// process group message from network
    pub fn on_message(sender_id: &Vec<u8>, receiver_id: &Vec<u8>, chat_message: &super::proto_net::GroupMessage)  ->Result<bool, String>{
        let user_id = PeerId::from_bytes(receiver_id).unwrap();
        // check group and member        
        let groups = Group::get_groups_of_user(user_id);
        let group_idx = groups.group_id_to_index(&chat_message.group_id);
        if group_idx == 0{
            return Err("can not find group".to_string());
        }

        let group = groups.db_ref.get(&group_idx.to_be_bytes()).unwrap().unwrap();

        //check member
        match group.get_member(receiver_id){
            Some(_)=>{},
            None =>{
                return Err("you are not member in this group".to_string());
            }
        }
        Ok(true)
    }

}
