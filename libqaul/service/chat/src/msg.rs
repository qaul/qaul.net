//! A helper to deal with unread counts and messages

use crate::{tags, Chat, ChatMessage, RoomId, RoomState, Subscription, ASC_NAME};
use async_std::sync::Arc;
use chrono::Utc;
use bincode::{deserialize, serialize};
use libqaul::{
    error::Result,
    messages::{Message, Mode, MsgQuery},
    users::UserAuth,
    Identity,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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
        let Meta { content, room } = deserialize(&msg.payload).unwrap();
        Self {
            id: msg.id,
            sender: msg.sender,
            timestamp: Utc::now(),
            content,
            room,
        }
    }
}

/// Generate a multiplexed payload for a libqaul message
pub(crate) fn gen_payload(content: impl Into<String>, room: RoomState) -> Vec<u8> {
    let content = content.into();
    serialize(&Meta { content, room }).unwrap()
}

/// Simple looping helper function that dispatches messages
pub(crate) async fn dispatch_to(
    serv: &Arc<Chat>,
    user: UserAuth,
    friends: BTreeSet<Identity>,
    payload: Vec<u8>,
    room: RoomId,
) -> Result<()> {
    for recp in friends {
        let mode = Mode::Std(recp);
        serv.qaul
            .messages()
            .send(
                user.clone(),
                mode,
                ASC_NAME,
                tags::room_id(room),
                payload.clone(),
            )
            .await?;
    }

    Ok(())
}

pub(crate) async fn subscribe_for(
    serv: &Arc<Chat>,
    user: UserAuth,
    room: RoomId,
) -> Result<Subscription> {
    let inner = serv
        .qaul
        .messages()
        .subscribe(user, ASC_NAME, tags::room_id(room))
        .await?;
    Ok(Subscription { inner })
}

pub(crate) async fn fetch_for(
    serv: &Arc<Chat>,
    user: UserAuth,
    room: RoomId,
) -> Result<Vec<ChatMessage>> {
    serv.qaul
        .messages()
        .query(user, ASC_NAME, MsgQuery::new().tag(tags::room_id(room)))
        .await?
        .all()
        .await?
        .into_iter()
        .map(|msg| Ok(msg.into()))
        .collect()
}
