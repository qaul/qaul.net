use identity::Identity;
use netmod::Recipient;
use serde::{Deserialize, Serialize};

/// A unique, randomly generated message ID
pub type MsgId = Identity;

/// An atomic message with a variable sized payload
///
/// A message is only ever addressed to a single node, or everyone on
/// the network.  The signature is required to be present, if a
/// payload is.  The payload can be empty, which can be used to create
/// a ping, or using the 16 byte MsgId as payload.  In these cases,
/// the sigature can also be empty.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Message {
    /// A random message ID
    pub id: MsgId,
    /// Sender of a message
    pub sender: Identity,
    /// Final recipient of a message
    pub recipient: Recipient,
    /// Some raw message payload
    pub payload: Vec<u8>,
    /// Signature data for userspace layers
    pub sign: Vec<u8>,
}

/// A wrapper around payload and signature 
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct Payload {
    pub(crate)  payload: Vec<u8>,
    pub(crate) sign: Vec<u8>,
}
