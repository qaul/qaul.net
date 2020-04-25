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
//! create times.
//!
//! If self time is older, discard chat message and wait for
//! re-transmit.  Don't reply to Create request.
//!
//! If self time is younger, take messages from self from old room,
//! insert them into new room, swap room stored in libqaul storage.
//! Send Confirm message with MsgId of create request.
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


