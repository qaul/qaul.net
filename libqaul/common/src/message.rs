//! Types for representing a message to be sent between users.
use crate::payload::Payload;
use crate::identity::UserID;

pub type MsgSignature = u64;

/// A `Message` with a `sender` and `recipient`, by a `service
///
/// Differs from a RATMAN message; this is a user-to-user,
/// fully encapsulated message which does not care (or know)
/// how it is delivered, merely that it was sent by one user
/// and is meant for another.
pub struct Message {
    sender: UserID,
    recipient: UserID,
    service: String,
    payload: Payload,
    signature: MsgSignature,
}
