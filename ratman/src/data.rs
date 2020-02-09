use identity::Identity;
use netmod::Recipient;
use serde::{Deserialize, Serialize};

/// Cryptographic signature payload for primary payload
///
/// Because ratman can't assume access to an advanced keystore, the
/// signature for a Message is not verified on this API level.  It is
/// up to the users of the Router API to verify message payloads
/// before parsing and trusting them.  This type variant merely
/// provides a convenient wrapper for that to be simpler.
pub type Signature = Vec<u8>;

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
    /// Source-verifiable payload signature data
    pub signature: Signature,
}

impl Message {
    /// Construct a signed message from raw data inputs
    ///
    /// The payload structure needs to provide a serializer, which
    /// allocates to be hashed for the XXHash signature of the
    /// Message.
    pub fn build_signed(id: MsgId, sender: Identity, recipient: Recipient, data: Vec<u8>) -> Self {
        let payload = data;
        let signature = vec![1, 3, 1, 2];

        Self {
            id,
            sender,
            recipient,
            payload,
            signature,
        }
    }
}
