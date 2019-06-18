//! Types for representing a message to be sent between users.
use crate::payload::Payload;
use crate::UserID;

pub type MsgSignature = u64;

/// A `Message` to be sent from the `sender` to the `recipient`, associated
/// with the given `service`.
/// Differs from a RATMAN message; this is a user-to-user, fully encapsulated
/// message which does not care (or know) how it is delivered, merely that it
/// was sent by one user and is meant for another.
pub struct Message {
    sender: UserID,
    recipient: UserID,
    service: String,
    payload: Payload,
    signature: MsgSignature,
}
