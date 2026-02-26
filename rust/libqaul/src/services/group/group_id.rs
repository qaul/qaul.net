// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Group ID
//!
//! A group id is a 16 Byte UUID
//!
//! For group chats this is a random 16 Byte UUID
//!
//! For direct chats we use a predictable UUID,
//! which consists of the q8id of both users.
//! {smaller q8id}{bigger q8id}

use libp2p::PeerId;
use uuid::Uuid;

use crate::utilities::qaul_id::QaulId;

/// Chat Group ID Structure
#[derive(Clone, Debug)]
pub struct GroupId {
    /// group id byte vector
    pub id: Vec<u8>,
}

impl GroupId {
    pub const BYTE_LEN: usize = 16;

    /// creates a group id from a bytes vector
    pub fn from_bytes(id: &[u8]) -> Result<GroupId, String> {
        if id.len() == Self::BYTE_LEN {
            Ok(GroupId { id: id.to_vec() })
        } else {
            Err("invalid length".to_string())
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.id
    }

    /// create group id for a direct chat from two user ids
    ///
    /// This generates a predictable group id which
    /// is a combination of the q8id's of the users.
    pub fn from_peers(user_id_1: &PeerId, user_id_2: &PeerId) -> GroupId {
        let q8id_1 = QaulId::to_q8id(user_id_1.to_owned());
        let q8id_2 = QaulId::to_q8id(user_id_2.to_owned());

        Self::from_q8ids(q8id_1, q8id_2)
    }

    /// creates a group id from two q8id's
    pub fn from_q8ids(q8id_1: Vec<u8>, q8id_2: Vec<u8>) -> GroupId {
        let (first, second) = if q8id_1 <= q8id_2 {
            (q8id_1, q8id_2)
        } else {
            (q8id_2, q8id_1)
        };

        let mut group_id = Vec::with_capacity(first.len() + second.len());
        group_id.extend_from_slice(&first);
        group_id.extend_from_slice(&second);

        GroupId { id: group_id }
    }

    /// get the bytes vector from group id
    pub fn to_bytes(&self) -> Vec<u8> {
        self.id.clone()
    }

    /// get the base58 encoded group id
    #[allow(dead_code)]
    pub fn to_base58(&self) -> String {
        bs58::encode(self.to_bytes()).into_string()
    }

    /// get the group id as a hyphenated uuid string
    pub fn to_string(&self) -> String {
        let group_uuid;
        match Uuid::from_slice(&self.id) {
            Ok(uuid) => {
                group_uuid = uuid.hyphenated().to_string();
            }
            Err(e) => {
                log::error!("{}", e);
                group_uuid = "UUID_ERROR".to_string();
            }
        }

        group_uuid
    }

    /// split a direct group_id into it's q8id's
    ///
    /// returns a tuple with both id's
    pub fn to_q8ids(&self) -> (Vec<u8>, Vec<u8>) {
        (self.id[0..8].to_owned(), self.id[8..16].to_owned())
    }

    /// check if the group id is a direct chat
    ///
    /// Returns the q8id of the chat partner if yes.
    /// Returns None if it is not a direct chat ID
    pub fn is_direct(&self, account_id: PeerId) -> Option<Vec<u8>> {
        let (q8id_1, q8id_2) = self.to_q8ids();
        let account_q8id = QaulId::to_q8id(account_id);

        if q8id_1 == account_q8id {
            return Some(q8id_2);
        }

        if q8id_2 == account_q8id {
            return Some(q8id_1);
        }

        None
    }

    /// create an informational string from a slice
    ///
    /// This function is intended for log messages, and
    /// will return a string no matter what.
    ///
    /// If the provided vector is a valid UUID, the function will
    /// return a hyphenated UUID string.
    ///
    /// If the bytes are not a valid UUID, it will convert them into
    /// bs58 encoding.
    #[allow(dead_code)]
    pub fn slice_to_string(bytes: &[u8]) -> String {
        let string;
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => string = uuid.hyphenated().to_string(),
            Err(_) => string = bs58::encode(bytes).into_string(),
        }

        string
    }
}

impl PartialEq for GroupId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
