//! A module that implements carrier formats for the qaul.net routing protocol
//!
//! These formats are router agnostic and can be implemented for various backends,
//! all that is required is a common `serde` compatibly exchange format.

/// A fingerprint buffer (8 bytes – 64bit)
pub type Fingerprint = [u8; 8];

/// A signature buffer (32 bytes – 256bit)
pub type Signature = [u8; 32];

/// An IPv4 address (TODO: Change this to IPv6 ?)
pub type IPAddress = [u8; 4];

/// A header contains package metadata and routing information
#[derive(Serialize, Deserialize)]
pub struct Header {
    /// A sender timestamp for ordering
    timestamp: u64,
    /// Signature of the message body (if applicable)
    signature: Signature,
    /// Cryptographic sender fingerprint ID
    sender_fp: Fingerprint,
    /// Receiver fingerprint ID
    receivr_fp: Fingerprint,
    /// Routing target IP
    target: IPAddress,
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
    Announce {},
    Farewell {},
    Payload {
        /// Indicates if data is base64 encoded
        encoded: bool,
        /// A data string
        data: String,
    },
    Empty,
}
