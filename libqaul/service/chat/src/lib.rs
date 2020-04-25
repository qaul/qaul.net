//! `qaul.net` chat service

mod types;
pub use types::{ChatMessage, Room, RoomDiff, RoomId, RoomMeta};

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
