//! `qaul.net` chat service

mod directory;
use directory::RoomDirectory;

mod msg;
mod protocol;
mod utils;

mod types;
pub(crate) use types::RoomState;
pub use types::{ChatMessage, Room, RoomDiff, RoomId, RoomMeta};

use async_std::sync::Arc;
use libqaul::{error::Result, users::UserAuth, Identity, Qaul};
use std::collections::BTreeMap;

const ASC_NAME: &'static str = "net.qaul.chat";

pub(crate) mod tags {
    pub(crate) const _META_NAME: &'static str = "room_list";
    pub(crate) const ROOM_LIST: &'static str = "net.qaul.chat.room_list";
}

/// Messaging service state
#[derive(Clone)]
pub struct Chat {
    pub(crate) qaul: Arc<Qaul>,
    pub(crate) rooms: Arc<RoomDirectory>,
}

impl Chat {
    /// Create a new chat service instance
    pub fn new(qaul: Arc<Qaul>) -> Result<Arc<Self>> {
        qaul.services().register(ASC_NAME)?;
        let rooms = Arc::new(RoomDirectory::new(Arc::clone(&qaul)));
        Ok(Arc::new(Self { qaul, rooms }))
    }

    /// Get a list of available room metadata for a user
    ///
    /// The data returned is not comprehensive but considered enough
    /// to render a list of available rooms with how many messages
    /// they have available each.
    pub async fn rooms(self: Arc<Chat>, user: UserAuth) -> Result<Vec<RoomMeta>> {
        let msgs = msg::unread(&self, user.clone()).await?;
        let rooms = utils::room_map(self.rooms.get_all(user).await);

        Ok(msgs
            .into_iter()
            .fold(BTreeMap::new(), |mut map, msg| {
                let room_id = msg.room.id();
                *map.entry(room_id).or_default() += 1;
                map
            })
            .into_iter()
            .map(|(id, unread)| RoomMeta {
                id,
                unread,
                name: rooms.get(&id).unwrap().name.clone(),
            })
            .collect())
    }

    /// Get all metadata about a specific room
    pub fn get_room(&self, user: UserAuth, room: RoomId) -> Result<Room> {
        unimplemented!()
    }

    /// Start a new chat
    pub fn start_chat(&self, user: UserAuth, friends: Vec<Identity>) -> Result<RoomId> {
        unimplemented!()
    }

    /// Send a normal chat message to a room
    pub fn send_message(&self, user: UserAuth, room: RoomId, content: String) -> Result<()> {
        unimplemented!()
    }

    /// Subscribe to push updates for a particular room
    pub async fn subscribe(&self, auth: UserAuth, room: RoomId) -> Result<()> {
        unimplemented!()
    }

    /// Stop receiving push updates for a room
    pub async fn unsubscribe(&self, auth: UserAuth, room: RoomId) -> Result<()> {
        unimplemented!()
    }

    /// Get all messages from a room
    pub fn get_messages(&self, user: UserAuth, room: RoomId) -> Result<Vec<ChatMessage>> {
        unimplemented!()
    }
}
