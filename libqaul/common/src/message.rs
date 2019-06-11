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
    // TODO DESIGN: Include a fingerprint here?
    // TODO DESIGN: It's probably a good idea to add a signature in addition to
    // the error detection included in the Payload, to prevent tampering, unless
    // that's best left up to RATMAN.
    service: String,
    payload: Payload,
}
