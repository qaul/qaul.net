// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Management Message Handling
//!
//! This file processes an incoming group management message.

use libp2p::PeerId;

use super::GroupStorage;

/// Group Message Structure
pub struct GroupMessage {}

impl GroupMessage {
    /// process group message from network
    pub fn on_message(
        sender_id: &PeerId,
        account_id: &PeerId,
        group_id: &Vec<u8>,
        message_id: &Vec<u8>,
    ) -> Result<bool, String> {
        let group;
        match GroupStorage::get_group(account_id.to_owned(), group_id.to_owned()) {
            Some(v) => group = v,
            None => return Err("group not found".to_string()),
        }

        // check member
        match group.get_member(&account_id.to_bytes()) {
            Some(_) => {}
            None => {
                return Err("you are not member in this group".to_string());
            }
        }

        let mut sender;
        // check if the sender is in group
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
        let group_crc = crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO).checksum(&group_id.clone());
        let sender_crc = crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO).checksum(&sender_id.to_bytes());
        let group_crc1 = u64::from_be_bytes(message_id[0..8].try_into().unwrap());
        let sender_crc1 = u64::from_be_bytes(message_id[8..16].try_into().unwrap());
        if group_crc != group_crc1 || sender_crc != sender_crc1 {
            return Err("invalid group message id-1".to_string());
        }
        let sender_msg_index = u32::from_be_bytes(message_id[16..20].try_into().unwrap());

        // change members status
        if sender_msg_index > sender.last_message_index {
            sender.last_message_index = sender_msg_index;
            super::Group::update_group_member(&account_id, group_id, &sender);
        }

        Ok(true)
    }
}
