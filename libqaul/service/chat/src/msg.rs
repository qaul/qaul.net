//! A helper to deal with unread counts and messages

use crate::{Chat, ChatMessage, RoomState, ASC_NAME};
use async_std::sync::Arc;
use chrono::Utc;
use conjoiner::{deserialise, serialise};
use libqaul::{
    error::Result,
    messages::{Message, MsgQuery},
    users::UserAuth,
};
use serde::{Deserialize, Serialize};

/// Get all chat messages for this service that are marked as "unread"
pub(crate) async fn unread(serv: &Arc<Chat>, user: UserAuth) -> Result<Vec<ChatMessage>> {
    Ok(serv
        .qaul
        .messages()
        .query(user, ASC_NAME, MsgQuery::new().unread())
        .await?
        .all()
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

// Purely here for field multiplexing on the message payload
#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    content: String,
    room: RoomState,
}

impl From<Message> for ChatMessage {
    fn from(msg: Message) -> Self {
        let Meta { content, room } = deserialise(&msg.payload).unwrap();
        Self {
            id: msg.id,
            sender: msg.sender,
            timestamp: Utc::now(),
            content,
            room,
        }
    }
}

impl ChatMessage {
    /// Generate a multiplexed payload for a libqaul message
    pub(crate) fn to_payload(&self) -> Vec<u8> {
        let ChatMessage { content, room, .. } = self.clone();
        serialise(&Meta { content, room }).unwrap()
    }
}
