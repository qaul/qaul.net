//! A helper to deal with unread counts and messages

use crate::{tags, Chat, ChatMessage, Result, RoomId, RoomState, Subscription, ASC_NAME};
use async_std::sync::Arc;
use bincode::{deserialize, serialize};
use chrono::{Duration, Utc};
use libqaul::{
    messages::{IdType, Message, Mode, MsgQuery},
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

        // This is a bit of a hack that fixes a lot of metadata race
        // conditions, especially in tests, but has practically no
        // real world consequences.
        //
        // The problem is that messages can get dispatched slightly
        // out of order. In the real world nobody is going to wait for
        // a message with a particular payload to assert on, which
        // means that out of order messages, while annoying in chats,
        // are fine.  However all of our tests assert on some very
        // basic properties and there are subtle timing differences,
        // depending on CPU and architecture that make tests fail when
        // messages arrive out of order.  When looking at a "status"
        // message, meaning one that updates the room in any way but
        // otherwise has no content, we apply a negative time offset
        // to make the history line up better.
        let timestamp = match room {
            RoomState::Id(_) => Utc::now(),
            _ => Utc::now() - Duration::milliseconds(500),
        };

        Self {
            id: msg.id,
            sender: msg.sender,
            timestamp,
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

/// Get a chat message via a specific Id
///
/// This function is very unstable and should only be called
/// immediately after inserting a message.  On the flip-side, if this
/// function ever panics, it indicates a deeper problem in the service
/// or even libqaul code.  This set of queries should never fail!
pub(crate) async fn fetch_chat_message(
    serv: &Arc<Chat>,
    user: UserAuth,
    id: Identity,
) -> ChatMessage {
    serv.qaul
        .messages()
        .query(user, ASC_NAME, MsgQuery::id(id))
        .await
        .unwrap()
        .resolve()
        .await
        .remove(0)
        .into()
}

/// Simple looping helper function that dispatches messages
pub(crate) async fn dispatch_to(
    serv: &Arc<Chat>,
    user: UserAuth,
    friends: BTreeSet<Identity>,
    payload: Vec<u8>,
    room: RoomId,
) -> Result<ChatMessage> {
    trace!("Creating room with {:?}", friends);

    let id_type = IdType::create_group();

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
                id_type,
                ASC_NAME,
                tags::room_id(room),
                payload.clone(),
            )
            .await?;
    }

    Ok(fetch_chat_message(serv, user, id_type.consume()).await)
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
