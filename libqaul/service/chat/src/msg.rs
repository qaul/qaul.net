//! Management datagrams to sync room states

use crate::room::RoomState;
use libqaul::{messages::MsgId, Identity};
use serde::{Deserialize, Serialize};

/// A chat message, associated to a room full of comrades
///
/// The "RoomState" can be filled in to be several things: for one,
/// this representation is serialised and sent to other nodes, so this
/// is how room creates are propagated across the network.  This is
/// also how changes can be made to the room, by embedding a RoomDiff
/// into the message.  The chat service API returns this
/// representation when sending a message, but manages rooms via a
/// separate interface.
#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    /// Unique message ID
    pub id: MsgId,
    /// Message sender ID
    pub sender: Identity,
    /// Embedded or linked  information
    pub room: RoomState,
    /// Text payload
    pub text: String,
    /// Optional list of attachments
    pub attachments: Vec<u8>,
}
