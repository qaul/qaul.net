// Copyright (c) 2022 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group Management Message Handling
//!
//! This file processes an incoming group management message.

use libp2p::PeerId;

use super::GroupStorage;

const GROUP_MESSAGE_ID_LEN: usize = 20;

struct ParsedGroupMessageId {
    group_crc: u64,
    sender_crc: u64,
    sender_msg_index: u32,
}

impl ParsedGroupMessageId {
    fn parse(message_id: &[u8]) -> Result<Self, String> {
        if message_id.len() != GROUP_MESSAGE_ID_LEN {
            return Err("invalid group message id".to_string());
        }

        Ok(Self {
            group_crc: u64::from_be_bytes(message_id[0..8].try_into().unwrap()),
            sender_crc: u64::from_be_bytes(message_id[8..16].try_into().unwrap()),
            sender_msg_index: u32::from_be_bytes(message_id[16..20].try_into().unwrap()),
        })
    }

    fn validate(self, group_id: &[u8], sender_id: &[u8]) -> Result<u32, String> {
        let group_crc = crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO).checksum(group_id);
        let sender_crc = crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO).checksum(sender_id);

        if group_crc != self.group_crc || sender_crc != self.sender_crc {
            return Err("invalid group message id-1".to_string());
        }

        Ok(self.sender_msg_index)
    }
}

/// Group Message Structure
pub struct GroupMessage {}

impl GroupMessage {
    /// process group message from network
    pub fn on_message(
        sender_id: &PeerId,
        account_id: &PeerId,
        group_id: &[u8],
        message_id: &[u8],
    ) -> Result<bool, String> {
        let group = GroupStorage::get_group(account_id.to_owned(), group_id)
            .ok_or_else(|| "group not found".to_string())?;
        let account_id_bytes = account_id.to_bytes();
        let sender_id_bytes = sender_id.to_bytes();

        // check member
        if group.get_member(&account_id_bytes).is_none() {
            return Err("you are not member in this group".to_string());
        }

        // check if the sender is in group
        let mut sender = group
            .get_member(&sender_id_bytes)
            .cloned()
            .ok_or_else(|| "the sender is not member in this group".to_string())?;

        let sender_msg_index =
            ParsedGroupMessageId::parse(message_id)?.validate(group_id, &sender_id_bytes)?;

        // change members status
        if sender_msg_index > sender.last_message_index {
            sender.last_message_index = sender_msg_index;
            super::Group::update_group_member(account_id, group_id, &sender);
        }

        Ok(true)
    }
}
