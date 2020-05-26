use chrono::{DateTime, Utc};
use libqaul::{
    helpers::{ItemDiff, SetDiff, Subscription as Sub},
    messages::{Message, MsgId},
    Identity,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A chat message, associated to a room full of comrades
///
/// The "RoomState" can be filled in to be several things: for one,
/// this representation is serialised and sent to other nodes, so this
/// is how room creates are propagated across the network.  This is
/// also how changes can be made to the room, by embedding a RoomDiff
/// into the message.  The chat service API returns this
/// representation when sending a message, but manages rooms via a
/// separate interface.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Unique message ID
    pub id: MsgId,
    /// Message sender ID
    pub sender: Identity,
    /// The timestamp at which the message was received (in utc)
    pub timestamp: DateTime<Utc>,
    /// Text payload
    pub content: String,
    /// Embedded or linked  information
    pub(crate) room: RoomState,
}

/// A unique identifier for a room
pub type RoomId = Identity;

/// An embeddable room update type that can be attached to a message
///
/// The room diff should be embedded into a message when updates are
/// sent across a room, or new people are invited (new invites get a
/// create, everyone else gets a Diff
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum RoomState {
    /// A simple chat message just needs the Room ID
    Id(RoomId),
    /// When creating a room while sending the first message
    Create(Room),
    /// A simple confirmation for receiving a particular command
    Confirm(RoomId, MsgId),
    /// Changes made to a room
    Diff(RoomDiff),
}

impl RoomState {
    /// Get the room ID from the RoomState state-machine
    pub(crate) fn id(&self) -> RoomId {
        match self {
            Self::Id(id) => *id,
            Self::Create(r) => r.id,
            Self::Confirm(id, _) => *id,
            Self::Diff(d) => d.id,
        }
    }
}

/// Some metadata for indexing rooms
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomMeta {
    /// Room ID
    pub id: Identity,
    /// Optional human readable room name
    pub name: Option<String>,
    /// Number of unread messages in a room
    pub unread: usize,
}

/// Abstraction over a chat room
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    /// The room ID
    pub id: RoomId,
    /// Set of users in the room
    pub users: BTreeSet<Identity>,
    /// A clear text room name
    pub name: Option<String>,
    /// The time at which this room was created
    pub create_time: DateTime<Utc>,
}

impl Room {
    pub(crate) fn apply(&mut self, diff: &RoomDiff) {
        diff.users.iter().for_each(|change| {
            match change {
                SetDiff::Add(id) => self.users.insert(*id),
                SetDiff::Remove(id) => self.users.remove(id),
                SetDiff::Ignore => false,
            };
        });

        match &diff.name {
            ItemDiff::Set(name) => self.name = Some(name.clone()),
            ItemDiff::Unset => self.name = None,
            ItemDiff::Ignore => {}
        }
    }
}

/// A set of changes made to a room
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomDiff {
    /// Associated room ID
    pub id: RoomId,
    /// Changes to room users
    pub users: Vec<SetDiff<Identity>>,
    /// Changes to room name
    pub name: ItemDiff<String>,
}

impl RoomDiff {
    pub(crate) fn named(id: RoomId, name: String) -> Self {
        Self {
            id,
            users: vec![],
            name: ItemDiff::Set(name),
        }
    }
}

/// A subscription handler that pushes out updates
pub struct Subscription {
    pub(crate) inner: Sub<Message>,
}

impl Subscription {
    pub(crate) fn new(inner: Sub<Message>) -> Self {
        Self { inner }
    }

    /// Get the next chat message
    pub async fn next(&self) -> ChatMessage {
        self.inner.next().await.into()
    }
}
