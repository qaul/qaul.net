//! A helper to deal with unread counts and messages

use crate::{tags, Chat, ChatMessage, Result, RoomId, RoomState, Subscription, ASC_NAME};
use async_std::sync::Arc;
use bincode::{deserialize, serialize};
use chrono::Utc;
use libqaul::{
    messages::{Message, Mode, MsgQuery},
    users::UserAuth,
    Identity,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use tracing::trace;

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
    trace!("Creating room with {:?}", friends);

    for recp in friends {
        // Skip self
        if recp == user.0 {
            continue;
        }

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
        .collect::<Result<Vec<_>>>()
        .map(|mut ok: Vec<ChatMessage>| {
            ok.sort_by_key(|msg| msg.timestamp);
            ok
        })
}

#[test]
fn sort_messages() {
    use crate::RoomState;
    use chrono::prelude::*;
    use libqaul::messages::MsgId;

    let room = RoomId::random();
    let friend = Identity::random();

    let mut msgs = vec![
        ChatMessage {
            id: MsgId::random(),
            sender: friend,
            room: RoomState::Id(room),
            content: "This is a middle message".into(),
            timestamp: Utc.ymd(2020, 5, 12).and_hms(14, 13, 23),
        },
        ChatMessage {
            id: MsgId::random(),
            sender: friend,
            room: RoomState::Id(room),
            content: "This is an old message".into(),
            timestamp: Utc.ymd(2020, 5, 12).and_hms(14, 13, 12),
        },
        ChatMessage {
            id: MsgId::random(),
            sender: friend,
            room: RoomState::Id(room),
            content: "This is a new message".into(),
            timestamp: Utc.ymd(2020, 5, 12).and_hms(14, 13, 37),
        },
    ];

    msgs.sort_by_key(|msg| msg.timestamp);

    assert_eq!(msgs[0].content, "This is an old message".to_owned());
    assert_eq!(msgs[1].content, "This is a middle message".to_owned());
    assert_eq!(msgs[2].content, "This is a new message".to_owned());
}
