//! Basic package layout (header and core)

use super::{
    data::{AnounceType, PayloadType, QueryType}, Fingerprint, IpAddress, Signature,
};

/// A header contains package metadata and routing information
#[derive(Serialize, Deserialize)]
pub struct Header {
    /// A sender timestamp for ordering
    timestamp: u32,
    /// Signature of the message body (if applicable)
    signature: Option<Signature>,
    /// Cryptographic sender fingerprint ID
    sender_fp: Fingerprint,
    /// Receiver fingerprint ID (if applicable)
    target_fp: Option<Fingerprint>,
    /// Sender IP address
    sender: IpAddress,
    /// Routing target IP
    target: IpAddress,
}

/// Represents a base message sent via the qaul.net protocol
#[derive(Serialize, Deserialize)]
pub struct Message {
    head: Header,
    body: Body,
}

/// A message body can be one of several types that contain
/// structure data, depending on their use
#[derive(Serialize, Deserialize)]
pub enum Body {
    /// An announcement message sent on-connect
    Announce(AnounceType),
    /// Asking messages into the network
    Query(QueryType),
    /// Responses to queries
    Payload {
        size: u64,
        data: PayloadType,
    },
    Empty,
}
