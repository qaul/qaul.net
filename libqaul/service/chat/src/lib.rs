//! `qaul.net` chat service
#![allow(unused)]

mod msg;
pub use msg::ChatMessage;

pub mod room;
use room::{Room, RoomId};

use async_std::{stream::Stream, sync::Arc};
use futures::stream::StreamExt;
use libqaul::{api::Subscription, error::Result, users::UserAuth, Identity, Qaul, Tag};

const ASC_NAME: &'static str = "net.qaul.chat";

/// Messaging service state
#[derive(Clone)]
pub struct Chat {
    qaul: Arc<Qaul>,
}

impl Chat {
    /// Create a new chat service instance
    pub fn new(qaul: Arc<Qaul>) -> Result<Arc<Self>> {
        qaul.services().register(ASC_NAME)?;
        Ok(Arc::new(Self { qaul }))
    }

    /// Access room function scope
    pub fn rooms<'s>(&'s self) -> Rooms<'s> {
        Rooms { chat: self }
    }

    /// Get the next available chat message
    pub async fn next(&self, auth: UserAuth, room: RoomId) -> ChatMessage {
        unimplemented!()
    }

    /// Subscribe to any future messages that are sent to a room
    pub async fn subscribe(
        &self,
        auth: UserAuth,
        room: RoomId,
    ) -> Result<impl Stream<Item = ChatMessage> + Unpin> {
        self.qaul
            .messages()
            .subscribe(auth, ASC_NAME, Some(Tag::new("room_id", room)))
            .map(|sub_stream| sub_stream.map(|msg| unimplemented!()))
    }

    /// Send a message into a conversation
    pub async fn send<S>(&self, auth: UserAuth, room: RoomId, text: S) -> Result<()>
    where
        S: Into<String>,
    {
        unimplemented!()
    }
}

/// Small API wrapper for room management
pub struct Rooms<'c> {
    chat: &'c Chat,
}

impl<'c> Rooms<'c> {
    /// Get a list of available rooms by ID
    pub async fn list(&self) -> Vec<RoomId> {
        vec![]
    }

    /// Get all state information by room ID
    pub async fn get(&self, id: RoomId) -> Room {
        unimplemented!()
    }

    /// Create a new room
    pub async fn create<I>(&self, auth: UserAuth, users: I) -> Result<RoomId>
    where
        I: IntoIterator<Item = Identity>,
    {
        Ok(RoomId::random())
    }

    /// Make modifications to an existing room
    pub async fn modify<F>(&self, auth: UserAuth, _id: RoomId, _f: F) -> Result<()>
    where
        F: FnOnce(&mut Room) -> Result<()>,
    {
        unimplemented!()
    }

    /// Delete a room locally
    pub async fn delete(&self, auth: UserAuth, _id: RoomId) -> Result<()> {
        unimplemented!()
    }
}
