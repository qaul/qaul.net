//! Types for representing a message to be sent through the network.
use crate::UserID;
use crate::payload::Payload;

pub struct Message {
    sender: UserID,
    recipient: UserID,
    // TODO DESIGN: Include a user fingerprint here?
    service: String,
    payload: Payload,
}
