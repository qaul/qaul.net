//! `qaul.net` chat service

mod directory;
use directory::RoomDirectory;

mod msg;
mod protocol;
mod subs;
mod utils;
mod worker;

mod error;
pub use error::{Error, Result};

mod types;
pub(crate) use types::RoomState;
pub use types::{ChatMessage, Room, RoomDiff, RoomId, RoomMeta, Subscription};

use async_std::{sync::Arc, task};
use libqaul::{services::ServiceEvent, users::UserAuth, Identity, Qaul};
use std::collections::BTreeMap;

const ASC_NAME: &'static str = "net.qaul.chat";

pub(crate) mod tags {
    use {crate::RoomId, libqaul::helpers::Tag};
    pub(crate) const _META_NAME: &'static str = "room_list";
    pub(crate) const ROOM_LIST: &'static str = "net.qaul.chat.room_list";
    pub(crate) fn room_id(id: RoomId) -> Tag {
        Tag::new("room-id", id.as_bytes().to_vec())
    }
}

/// Messaging service state
#[derive(Clone)]
pub struct Chat {
    pub(crate) qaul: Arc<Qaul>,
    pub(crate) rooms: Arc<RoomDirectory>,
}

impl Chat {
    /// Create a new chat service instance
    pub async fn new(qaul: Arc<Qaul>) -> Result<Arc<Self>> {
        let rooms = Arc::new(RoomDirectory::new(Arc::clone(&qaul)));
        let this = Arc::new(Self { qaul, rooms });

        // Register the service event handle
        let sender = Arc::new(worker::run_asnc(Arc::clone(&this)));
        this.qaul
            .services()
            .register(ASC_NAME, move |cmd| {
                let sender = Arc::clone(&sender);
                task::block_on(async move {
                    match cmd {
                        ServiceEvent::Open(auth) => sender.send(worker::Command::Start(auth)).await,
                        ServiceEvent::Close(auth) => sender.send(worker::Command::Stop(auth)).await,
                    }
                })
            })
            .await?;

        Ok(this)
    }

    /// Get a list of available room metadata for a user
    ///
    /// The data returned is not comprehensive but considered enough
    /// to render a list of available rooms with how many messages
    /// they have available each.
    pub async fn rooms(self: &Arc<Chat>, user: UserAuth) -> Result<Vec<RoomMeta>> {
        let msgs = msg::unread(&self, user.clone()).await?;
        let rooms = dbg!(utils::room_map(self.rooms.get_all(user).await));

        Ok(msgs
            .into_iter()
            .fold(BTreeMap::new(), |mut map, msg| {
                let room_id = dbg!(msg).room.id();
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
    pub async fn get_room(&self, user: UserAuth, room: RoomId) -> Result<Room> {
        self.rooms.get(user, room).await.map(|r| Ok(r)).unwrap()
    }

    /// Start a new chat
    pub async fn start_chat(
        self: &Arc<Self>,
        user: UserAuth,
        friends: Vec<Identity>,
    ) -> Result<RoomId> {
        let friends = friends.into_iter().collect();

        if let Some(id) = Room::check(self, user.clone(), &friends).await {
            return Ok(id);
        }

        let room = Room::create(self, user.clone(), friends.clone(), None).await;
        let room_id = room.id();
        let payload = msg::gen_payload("", room);
        msg::dispatch_to(self, user, friends, payload, room_id).await?;
        Ok(room_id)
    }

    /// Send a normal chat message to a room
    pub async fn send_message(
        self: &Arc<Self>,
        user: UserAuth,
        room: RoomId,
        content: String,
    ) -> Result<()> {
        let friends = self.rooms.get(user.clone(), room).await?.users;
        let payload = msg::gen_payload(content, Room::resume(room));
        msg::dispatch_to(self, user, friends, payload, room).await
    }

    /// Subscribe to push updates for a particular room
    pub async fn subscribe(self: &Arc<Self>, user: UserAuth, room: RoomId) -> Result<Subscription> {
        msg::subscribe_for(self, user, room).await
    }

    /// Get all messages from a room
    pub async fn load_messages(
        self: &Arc<Self>,
        user: UserAuth,
        room: RoomId,
    ) -> Result<Vec<ChatMessage>> {
        msg::fetch_for(self, user, room).await
    }
}
