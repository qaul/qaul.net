//! `qaul.net` chat service

#![doc(html_favicon_url = "https://qaul.net/favicon.ico")]
#![doc(html_logo_url = "https://qaul.net/img/qaul_icon-128.png")]

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
        let rooms = utils::room_map(self.rooms.get_all(user).await);

        let smsg = msgs.into_iter().fold(BTreeMap::new(), |mut map, msg| {
            let room_id = msg.room.id();
            *map.entry(room_id).or_default() += 1;
            map
        });

        Ok(rooms
            .into_iter()
            .map(|(id, room)| RoomMeta {
                id,
                unread: *smsg.get(&id).unwrap_or(&0),
                name: room.name.clone(),
                users: room.users,
                create_time: room.create_time,
            })
            .collect())
    }

    /// Get all metadata about a specific room
    pub async fn get_room(self: &Arc<Chat>, user: UserAuth, room: RoomId) -> Result<RoomMeta> {
        self.rooms
            .get(user.clone(), room)
            .await
            .map(|r| {
                Ok(RoomMeta {
                    id: r.id,
                    unread: utils::get_unread_message_count(self, user, room),
                    name: r.name,
                    users: r.users,
                    create_time: r.create_time,
                })
            })
            .unwrap()
    }

    /// Create a chat with a group of people
    ///
    /// A chat has to be between at least two people, meaning that the
    /// set of friends given to this function needs to not be empty.
    pub async fn start_chat(
        self: &Arc<Self>,
        user: UserAuth,
        friends: Vec<Identity>,
        name: Option<String>,
    ) -> Result<RoomMeta> {
        let friends = friends.into_iter().collect();

        if let Some(id) = RoomMeta::check(self, user.clone(), &friends).await {
            return self.get_room(user.clone(), id).await;
        }

        let room = RoomMeta::create(self, user.clone(), friends.clone(), name).await;
        let room_id = room.id();
        let payload = msg::gen_payload("", room);
        msg::dispatch_to(self, user.clone(), friends, payload, room_id).await?;
        self.get_room(user, room_id).await
    }

    /// Send a normal chat message to a room
    pub async fn send_message(
        self: &Arc<Self>,
        user: UserAuth,
        room: RoomId,
        content: String,
    ) -> Result<ChatMessage> {
        let friends = self.rooms.get(user.clone(), room).await?.users;
        let payload = msg::gen_payload(content, RoomMeta::resume(room));
        msg::dispatch_to(self, user, friends, payload, room).await
    }

    /// Subscribe to push updates for a particular room
    pub async fn subscribe(self: &Arc<Self>, user: UserAuth, room: RoomId) -> Result<Subscription> {
        msg::subscribe_for(self, user, room).await
    }

    /// Set a name for an existing room, overriding the previous name
    ///
    /// This is a convenience function for `modify_room(self, user, room, diff)`!
    pub async fn set_name(
        self: &Arc<Self>,
        user: UserAuth,
        room: RoomId,
        name: String,
    ) -> Result<()> {
        self.modify_room(user, room, RoomDiff::named(room, name))
            .await?;
        Ok(())
    }

    /// Apply some changes to a room
    pub async fn modify_room(
        self: &Arc<Self>,
        user: UserAuth,
        room: RoomId,
        diff: RoomDiff,
    ) -> Result<RoomMeta> {
        let mut room = self.get_room(user.clone(), room).await?;
        let state = room.modify(self, user.clone(), diff).await;
        room.send_to_participants(self, user, state).await?;
        Ok(room)
    }

    /// Subscriber function that notifies the caller when a new room is discovered
    ///
    /// **Warning:** this API creates a side-channel attack
    /// opportunity to spy on other users in this service, because
    /// there's no name-spacing between room identities!  For the
    /// limited testing purposes that this function is used in at the
    /// moment this is fine.  Ultimately the underlying room directory
    /// should be able to namespace notifies, or this function should
    /// be removed from the public fascade (`doc(hidden)` exists too)
    pub async fn next_rooms(self: &Arc<Self>) -> RoomId {
        self.rooms.poll_new().await
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
