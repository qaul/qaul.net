//! A module that implements carrier formats for the qaul.net routing protocol
//!
//! These formats are router agnostic and can be implemented for various backends,
//! all that is required is a common `serde` compatibly exchange format.

/// A fingerprint buffer (8 bytes – 64bit)
pub type Fingerprint = [u8; 8];

/// A signature buffer (32 bytes – 256bit)
pub type Signature = [u8; 32];

/// An IP address, both for IPv4 and v6
#[derive(Serialize, Deserialize)]
pub enum IpAddress {
    V4([u8; 4]),
    V6([u8; 16]),
}

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

#[derive(Serialize, Deserialize)]
pub struct FileMeta {
    description: String,
    extention: String,
    hash: String,
    size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct UserMeta {
    username: String,
    fingerprint: String,
    pubkey: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum UserMetaType {
    Username,
    Pubkey,
    Avatar,
}

#[derive(Serialize, Deserialize)]
pub enum AnounceType {
    /// A user metadata type (name, profile, ...)
    User(UserMeta),
    /// A regular file-share
    File(FileMeta),
}

#[derive(Serialize, Deserialize)]
pub enum QueryType {
    File { hash: String },
    FileList { range: (u16, u16) },
    User { fp: String, _type: UserMetaType },
    UserList { range: (u16, u16) },
}

#[derive(Serialize, Deserialize)]
pub enum PayloadType {
    FileMeta(FileMeta),
    UserMeta(UserMeta),
    File { blob: Vec<u8> },
    UserList { length: u64, list: Vec<Fingerprint> },
    FileList { length: u64, list: Vec<FileMeta> },
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
