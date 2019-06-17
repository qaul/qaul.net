//! Types for representing a message to be sent between users.
use crate::UserID;
use crate::payload::Payload;

/// A `Message` to be sent from the `sender` to the `recipient`, associated
/// with the given `service`.
/// Differs from a RATMAN message; this is a user-to-user, fully encapsulated
/// message which does not care (or know) how it is delivered, merely that it
/// was sent by one user and is meant for another.
pub struct Message {
    sender: UserID,
    recipient: UserID,
    // TODO: Add some kind of cryptographic signing here
    service: String,
    payload: Payload,
}
