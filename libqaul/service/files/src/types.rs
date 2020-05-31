use serde::{Deserialize, Serialize};

use libqaul::error::Result;
use libqaul::helpers::Subscription as Sub;
use libqaul::messages::{Message, MsgId};
use libqaul::users::UserAuth;
use libqaul::Identity;

pub type FileId = Identity;

/// Local file abstraction
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct File {
    pub owner: Identity,
    pub hash_id: Identity, // hex
    pub name: String,
    pub data: Vec<u8>,
}

/// Describe a file's lifecycle
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FileMeta {
    /// Sending a file
    File(File),
    /// Telling poeple about files
    Advertised {
        owner: Identity,
        hash_id: Identity,
        name: String,
        size: u64,
    },
}

impl FileMeta {
    pub(crate) fn owner(&self) -> Identity {
        match self {
            Self::Advertised { owner, .. } => *owner,
            Self::File(ref f) => f.owner,
        }
    }

    pub(crate) fn id(&self) -> FileId {
        match self {
            Self::Advertised { hash_id, .. } => *hash_id,
            Self::File(ref f) => f.hash_id,
        }
    }
}

/// Describe a file's lifecycle
///
/// Filter functions for each time exist and enable
/// different sub-services based on which phase they
/// aim for.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FileFilter {
    Local,
    Advertised,
    InProgress,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct FileMessage {
    pub(crate) recipient: Option<Identity>,
    pub(crate) meta: FileMeta,
}

/// A subscription handler that pushes out updates
pub struct Subscription {
    pub(crate) inner: Sub<Message>,
}

impl Subscription {
    pub(crate) fn new(inner: Sub<Message>) -> Self {
        Self { inner }
    }

    /// Get the next chat message
    pub async fn next(&self) -> FileMeta {
        let Message { payload, .. } = self.inner.next().await;
        let fm: FileMessage = bincode::deserialize(&payload).unwrap();
        fm.meta
    }
}
