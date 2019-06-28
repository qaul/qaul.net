//! Data exchange structures for `R.A.T.M.A.N.`

use identity::Identity;

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
#[derive(PartialEq, Eq, Debug)]
pub struct Message {
    /// Sender of a message
    pub sender: Identity,
    /// Final recipient of a message
    pub recipient: Identity,
    /// Outside service associative information
    pub associator: String,
    /// Some raw message payload
    pub payload: Payload,
    /// Source-verifiable payload signature data
    pub signature: Signature,
}

/// A raw, binary encoded message payload
#[derive(PartialEq, Eq, Debug)]
pub struct Payload {
    pub length: u64,
    pub data: Vec<u8>,
}
