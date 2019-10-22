//! Data exchange structures for `R.A.T.M.A.N.`

use serde::{Deserialize, Serialize};
use identity::Identity;
use netmod::Recipient;
use std::hash::Hasher;
use twox_hash::XxHash64;
use conjoiner as conj;

pub type Signature = u64;

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
    pub payload: Payload,
    /// Source-verifiable payload signature data
    pub signature: Signature,
}

/// A raw, binary encoded message payload
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Payload {
    pub length: u64,
    pub data: Vec<u8>,
}

impl Message {
    /// Construct a signed message from raw data inputs
    ///
    /// The payload structure needs to provide a serializer, which
    /// allocates to be hashed for the XXHash signature of the
    /// Message.
    pub fn build_signed<S, V>(
        sender: Identity,
        recipient: Recipient,
        associator: S,
        payload: V,
    ) -> Self
    where
        S: Into<String>,
        V: Serialize,
    {
        #[derive(Serialize)]
        struct SkeletonMsg {
            sender: Identity,
            recipient: Recipient,
            associator: String,
            payload: Payload,
        };

        let mut hasher = XxHash64::with_seed(1312);
        let associator = associator.into();

        let vec = conj::serialise(&payload).unwrap();

        let payload = Payload {
            length: vec.len() as u64,
            data: vec,
        };

        let teeth_gang = SkeletonMsg {
            sender,
            recipient,
            associator,
            payload,
        };

        let vec = conj::serialise(&teeth_gang).unwrap();
        hasher.write(&vec);
        let signature = hasher.finish();

        // Destructure data to move into a new `Message`
        let SkeletonMsg {
            sender,
            recipient,
            associator,
            payload,
        } = teeth_gang;

        Self {
            sender,
            recipient,
            associator,
            payload,
            signature,
        }
    }
}
