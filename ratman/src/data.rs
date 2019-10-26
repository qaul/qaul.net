//! Data exchange structures for `R.A.T.M.A.N.`

use conjoiner as conj;
use identity::Identity;
use netmod::Recipient;
use serde::{Deserialize, Serialize};
use std::hash::Hasher;
use twox_hash::XxHash64;

pub type Signature = Vec<u8>;

/// An atomic message, handed to a `Router` to deliver
///
/// Fundamentally a message has a `sender`, `recipient`
/// and is associated to some source, via `associator`.
/// This field is ignored by `R.A.T.M.A.N.` and verbatim
/// copied to the target system. It can thus be used to
/// associate different services messages and metadata,
/// outside of a complicated header structure.
///
/// A `Message` assumes that no transmission errors were made,
/// because this is guaranteed by the `netmod` `Frame` abstraction!
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Message {
    /// Sender of a message
    pub sender: Identity,
    /// Final recipient of a message
    pub recipient: Recipient,
    /// Outside service associative information
    pub associator: String,
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
    pub fn build_signed<S>(
        sender: Identity,
        recipient: Recipient,
        associator: S,
        data: Vec<u8>,
    ) -> Self
    where
        S: Into<String>,
    {
        let associator = associator.into();
        let payload = data;
        let signature = vec![1, 3, 1, 2];

        Self {
            sender,
            recipient,
            associator,
            payload,
            signature,
        }
    }
}
