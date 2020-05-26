//! The chat protocol implementation
//!
//! Underlying types used are defined in `types.rs`, interactions are
//! defined here for clarity.  Following is a textual explanation of
//! the dynamics of the protocol, what parts are implemented here, and
//! what parts are implemented via libqaul.
//!
//! This documentation will go through a few workflows
//!
//! ## Creating a room
//!
//! Create a `Room` type, and update the service metadata store with
//! the list of available rooms.  Send a `ChatMessage` to the friend,
//! attach a `RoomState::Create` with the room metadata.
//!
//! When receiving a message with `RoomState::Create` => check if a
//! room already exists with that set of users.  If yes, compare the
//! room IDs numerically.
//!
//! If self id is larger, discard chat message.  Don't reply to Create
//! request.  It's assumed that self create message is still in
//! transit.
//!
//! If self id is smaller, take messages from self from old room,
//! insert them into new room, swap room stored in libqaul storage.
//!
//! ## Add or remove a person to a room
//!
//! Send message to room with `RoomState::Diff(_)`, where the RoomDiff
//! contains additional users.  Wait for Confirm from every member in
//! room.
//!
//! TODO: How to deal with updates that never get confirmed?
//!
//! ## Sending normal messages
//!
//! The room protocol is piggy-backed on the normal chat messages to
//! save space (and make the code simpler).  To send a normal message,
//! just set `RoomState::Id(_)` with the appropriate room ID.  Setting
//! the wrong room ID will get the message discarded on the other end.
//!
//! When receiving a message for a room ID where the sender is not in
//! the room: discard.

use crate::{msg, Chat, ChatMessage, Result, Room, RoomDiff, RoomId, RoomState};
use async_std::sync::Arc;
use chrono::Utc;
use libqaul::{helpers::ItemDiff, users::UserAuth, Identity};
use std::collections::BTreeSet;

impl Room {
    pub(crate) async fn check(
        serv: &Arc<Chat>,
        user: UserAuth,
        friends: &BTreeSet<Identity>,
    ) -> Option<RoomId> {
        let all = serv.rooms.get_all(user.clone()).await;
        all.into_iter().fold(None, |val, room| {
            val.or_else(|| {
                if &room.users == friends {
                    Some(room.id)
                } else {
                    None
                }
            })
        })
    }

    /// Continue a conversation in a room
    pub(crate) fn resume(id: RoomId) -> RoomState {
        RoomState::Id(id)
    }

    /// Create room, update room list, return RoomState for message
    pub(crate) async fn create(
        serv: &Arc<Chat>,
        user: UserAuth,
        mut users: BTreeSet<Identity>,
        name: Option<String>,
    ) -> RoomState {
        users.insert(user.0);

        let room = Self {
            id: RoomId::random(),
            users,
            name,
            create_time: Utc::now(),
        };

        serv.rooms.insert(user, &room).await;
        RoomState::Create(room)
    }

    /// Send a room state as an update to all participants in a room
    pub(crate) async fn send_to_participants(
        &self,
        serv: &Arc<Chat>,
        user: UserAuth,
        state: RoomState,
    ) -> Result<()> {
        let friends = &self.users;
        let room_id = self.id;
        let payload = msg::gen_payload("", state);

        msg::dispatch_to(&serv, user.clone(), friends.clone(), payload, room_id).await?;

        Ok(())
    }

    /// Add room, update room list, return RoomState for message
    pub(crate) async fn add_name(
        &self,
        serv: &Arc<Chat>,
        user: UserAuth,
        name: impl Into<String>,
    ) -> RoomState {
        let name = name.into();
        let new = Self {
            name: Some(name.clone()),
            ..self.clone()
        };
        serv.rooms.insert(user, &new).await;

        RoomState::Diff(RoomDiff {
            id: self.id,
            users: vec![],
            name: ItemDiff::Set(name),
        })
    }

    /// Handle an incoming message
    ///
    /// This can be one of four states as outlined in the module docs:
    ///
    /// - Id: normal chat message, no actions needed
    /// - Create: insert the provided room
    /// - Confirm: set the room metadata to confirmed by sender
    /// - Diff: apply the diff to the room
    ///
    /// In cases `Create` and `Diff` a Confirm will be generated
    pub(crate) async fn handle(
        serv: &Arc<Chat>,
        user: UserAuth,
        msg: &ChatMessage,
    ) -> Option<RoomState> {
        match msg.room {
            RoomState::Id(_) => None,
            RoomState::Create(ref room) => {
                serv.rooms.insert(user, room).await;
                None
            }
            RoomState::Confirm(_, _) => None,
            RoomState::Diff(ref diff) => {
                serv.rooms.apply_diff(user, diff).await;
                Some(RoomState::Confirm(diff.id, msg.id))
            }
        }
    }
}
