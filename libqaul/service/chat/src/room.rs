//! A module to handle chat rooms

use libqaul::{Identity, Qaul, error::Result};
use std::collections::BTreeSet;
use async_std::sync::Arc;
use serde::{Serialize, Deserialize};

/// A unique identifier for a room
pub type RoomId = Identity;

/// An embeddable room update type that can be attached to a message
///
/// The room diff should be embedded into a message when updates are
/// sent across a room, or new people are invited (new invites get a
/// create, everyone else gets a Diff
#[derive(Serialize, Deserialize)]
pub enum RoomState {
    /// A simple chat message just needs the Room ID
    Id(RoomId),
    /// When creating a room while sending the first message
    Create(Room),
    /// Changes made to a room
    Diff(RoomDiff),
    
}

/// A simple command type to make remote changes
#[derive(Serialize, Deserialize)]
pub(crate) enum Diff<T> {
    /// Add a new user to the set
    Add(T),
    /// Remove a user from the set
    Remove(T),
    /// Clear some Optional state value
    Clear,
}

/// Apply changes to a room by ID
#[derive(Serialize, Deserialize)]
pub struct RoomDiff {
    id: Identity,
    users: Vec<Diff<Identity>>,
    name: Diff<String>,
}

/// Abstraction over a chat room
#[derive(Serialize, Deserialize)]
pub struct Room {
    /// The room ID
    pub id: RoomId,
    /// Set of users in the room
    pub users: BTreeSet<Identity>,
    /// A clear text room name
    pub name: Option<String>,
}


impl Room {
    /// Create a new room builder
    pub fn new<S>(name: S) -> Self
    where
        S: Into<Option<String>>,
    {
        Self {
            id: Identity::random(),
            users: BTreeSet::default(),
            name: name.into(),
        }
    }

    /// Make a user join this room
    ///
    /// Nothing is changed remotely until `commit` is called
    pub fn join(&mut self, user: Identity) {
        self.users.insert(user);
    }

    /// Kick a user from this room
    ///
    /// Nothing is changed remotely until `commit` is called
    pub fn kick(&mut self, user: Identity) {
        self.users.remove(&user);
    }

    pub async fn commit(&self, qaul: Arc<Qaul>) -> Result<()> {
        
        
        Ok(())
    }
}
