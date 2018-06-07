//! Various data packages that are sent across the network
//!
//! A transaction usually correspnds to a query being sent, followed
//! by an answer by either a specific node or anyone in the network.
//!
//! The second (common) type of message is a direct data transfer, either
//! for text or video/ audio communication.
//!
//! A goal would be to make this protocol easily extendable via service
//! hooks that can cover other payload types.

use super::{
    meta::{FileMeta, UserMeta, UserMetaType}, Fingerprint,
};

#[derive(Serialize, Deserialize)]
pub enum AnounceType {
    /// A user metadata type (name, profile, ...)
    User(UserMeta),
    /// A regular file-share
    File(FileMeta),
}

/// Network query types
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
    Text { encrypted: bool, text: String },
    UserList { length: u64, list: Vec<Fingerprint> },
    FileList { length: u64, list: Vec<FileMeta> },
}
