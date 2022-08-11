//use bs58::decode;
use libp2p::PeerId;

use super::proto;
use super::Chat;
use super::Group;
use crate::{node::user_accounts::UserAccounts, utilities::timestamp};
use prost::Message;

pub struct GroupMessage {}
impl GroupMessage {
    /// process group message from network
    pub fn on_message(
        sender_id: &PeerId,
        receiver_id: &PeerId,
        group_id: &Vec<u8>,
        message_id: &Vec<u8>,
    ) -> Result<bool, String> {
        let group;
        match Group::get_group(receiver_id, group_id) {
            Ok(v) => {
                group = v;
            }
            Err(error) => {
                return Err(error);
            }
        }

        //check member
        match group.get_member(&receiver_id.to_bytes()) {
            Some(_) => {}
            None => {
                return Err("you are not member in this group".to_string());
            }
        }

        let mut sender;
        // checkif the sender is in group
        match group.get_member(&sender_id.to_bytes()) {
            Some(v) => {
                sender = v.clone();
            }
            None => {
                return Err("the sender is not member in this group".to_string());
            }
        }

        // check message id
        if message_id.len() != 20 {
            return Err("invalid group message id".to_string());
        }
        let group_crc = crc::crc64::checksum_iso(&group_id.clone());
        let sender_crc = crc::crc64::checksum_iso(&sender_id.to_bytes());
        let group_crc1 = u64::from_be_bytes(message_id[0..8].try_into().unwrap());
        let sender_crc1 = u64::from_be_bytes(message_id[8..16].try_into().unwrap());
        if group_crc != group_crc1 || sender_crc != sender_crc1 {
            return Err("invalid group message id-1".to_string());
        }
        let sender_msg_index = u32::from_be_bytes(message_id[16..20].try_into().unwrap());

        // change members status
        sender.last_message_index = sender_msg_index;
        super::Group::update_group_member(&receiver_id, group_id, &sender);
        Ok(true)
    }
}
