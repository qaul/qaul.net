//! `qaul.net` chat service

mod types;
pub use types::{ChatMessage, Room, RoomDiff, RoomId, RoomMeta, RoomState};

use async_std::sync::Arc;
use libqaul::{error::Result, users::UserAuth, Identity, Qaul};

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
    pub fn rooms(&self, user: UserAuth) -> Result<Vec<RoomMeta>> {
        Ok(vec![])
    }

    /// Get all metadata about a specific room
    pub fn metadata(&self, user: UserAuth, room: RoomId) -> Result<Room> {
        unimplemented!()
    }

    /// Start a new chat
    pub fn start_chat(&self, user: UserAuth, friends: Vec<Identity>) -> Result<RoomId> {
        unimplemented!()
    }

    pub fn send_message(&self, user: UserAuth, room: RoomId) -> Result<()> {
        unimplemented!()
    }

    /// Subscribe to any future messages that are sent to a room
    pub async fn subscribe(&self, auth: UserAuth, room: RoomId) -> () {
        // self.qaul
        //     .messages()
        //     .subscribe(auth, ASC_NAME, Some(Tag::new("room_id", room)))
        //     .map(|sub_stream| sub_stream.map(|msg| unimplemented!()))
        unimplemented!()
    }

    /// Get all messages from a room
    pub fn query(&self, user: UserAuth, room: RoomId) -> Result<Vec<ChatMessage>> {
        unimplemented!()
    }
}
