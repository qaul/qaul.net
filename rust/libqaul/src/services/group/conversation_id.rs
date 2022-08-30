// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Chat Conversation ID
//!
//! A conversation id is a 16 Byte UUID
//!
//! For group chats this is a random 16 Byte UUID
//!
//! For direct chats we use a predictable UUID,
//! which consists of the q8id of both users.
//! {smaller q8id}{bigger q8id}

use libp2p::PeerId;
use uuid::Uuid;

use crate::utilities::qaul_id::QaulId;

/// Chat Conversation ID Structure
pub struct ConversationId {
    /// conversation ID byte vector
    pub id: Vec<u8>,
}

impl ConversationId {
    /// creates a conversation ID from a bytes vector
    pub fn from_bytes(id: &Vec<u8>) -> Result<ConversationId, String> {
        if id.len() == 16 {
            Ok(ConversationId { id: id.clone() })
        } else {
            Err("invalid length".to_string())
        }
    }

    /// create conversation ID for a direct chat from two user ids
    ///
    /// This generates a predictable conversation ID which
    /// is a combination of the q8id's of the users.
    pub fn from_peers(user_id_1: &PeerId, user_id_2: &PeerId) -> ConversationId {
        let q8id_1 = QaulId::to_q8id(user_id_1.to_owned());
        let q8id_2 = QaulId::to_q8id(user_id_2.to_owned());

        Self::from_q8ids(q8id_1, q8id_2)
    }

    /// creates a conversation ID from two q8id's
    pub fn from_q8ids(q8id_1: Vec<u8>, q8id_2: Vec<u8>) -> ConversationId {
        let mut ids: Vec<Vec<u8>> = Vec::new();
        ids.push(q8id_1);
        ids.push(q8id_2);

        ids.sort();

        let mut conversation_id: Vec<u8> = Vec::new();
        conversation_id.extend(ids[0].clone());
        conversation_id.extend(ids[1].clone());

        ConversationId {
            id: conversation_id,
        }
    }

    /// get the bytes vector from conversation ID
    pub fn to_bytes(&self) -> Vec<u8> {
        self.id.clone()
    }

    /// get the base58 encoded conversation ID
    pub fn to_base58(&self) -> String {
        bs58::encode(self.to_bytes()).into_string()
    }

    /// get the conversation ID as a hyphenated uuid string
    pub fn to_string(&self) -> String {
        let conversation_uuid;
        match Uuid::from_slice(&self.id) {
            Ok(uuid) => {
                conversation_uuid = uuid.hyphenated().to_string();
            }
            Err(e) => {
                log::error!("{}", e);
                conversation_uuid = "UUID_ERROR".to_string();
            }
        }

        conversation_uuid
    }

    /// split a direct conversation_id into it's q8id's
    ///
    /// returns a tuple with both id's
    pub fn to_q8ids(&self) -> (Vec<u8>, Vec<u8>) {
        (self.id[0..8].to_owned(), self.id[8..16].to_owned())
    }

    /// check if the conversation ID is a direct chat
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
    pub fn slice_to_string(bytes: &Vec<u8>) -> String {
        let string;
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => string = uuid.hyphenated().to_string(),
            Err(_) => string = bs58::encode(bytes).into_string(),
        }

        string
    }
}

impl PartialEq for ConversationId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
