//! The file protocol implementation
//!
//! Underlying types used are defined in `types.rs`, interactions are
//! defined here for clarity.  Following is a textual explanation of
//! the dynamics of the protocol, what parts are implemented here, and
//! what parts are implemented via libqaul.

use crate::{
    types::{File, FileFilter, FileId, FileMessage, FileMeta},
    ASC_NAME,
};
use libqaul::{
    error::Result,
    helpers::TagSet,
    messages::{IdType, Mode},
    users::UserAuth,
    Identity, Qaul,
};
use std::sync::Arc;

impl FileMessage {
    pub(crate) async fn send_off(&self, user: UserAuth, qaul: Arc<Qaul>) -> Result<()> {
        let payload = bincode::serialize(&self).unwrap();
        if let FileMeta::Advertised { .. } | FileMeta::Request { .. } = &self.meta {
            let mode = match self.recipient {
                Some(id) => Mode::Std(id),
                None => Mode::Flood,
            };

            qaul.messages()
                .send(
                    user,
                    mode,
                    IdType::unique(),
                    ASC_NAME,
                    TagSet::empty(),
                    payload,
                )
                .await?;
        } else if let file @ FileMeta::File(_) = &self.meta {
            let mode = Mode::Std(self.recipient.unwrap());
            qaul.messages()
                .send(
                    user,
                    mode,
                    IdType::unique(),
                    ASC_NAME,
                    TagSet::empty(),
                    payload,
                )
                .await?;
        }

        Ok(())
    }
}

impl FileMeta {
    pub(crate) fn id(&self) -> FileId {
        match self {
            Self::Advertised { hash_id, .. } => *hash_id,
            Self::File(ref f) => f.hash_id,
            Self::Request { hash_id, .. } => *hash_id,
        }
    }

    pub(crate) fn build_file(
        owner: Identity,
        hash_id: Identity,
        name: String,
        data: Vec<u8>,
    ) -> Self {
        Self::File(File {
            owner,
            hash_id,
            name,
            data,
        })
    }

    pub(crate) fn build_ad(owner: Identity, hash_id: Identity, name: String, size: u64) -> Self {
        Self::Advertised {
            owner,
            hash_id,
            name,
            size,
        }
    }

    pub(crate) fn build_request(hash_id: Identity) -> Self {
        Self::Request { hash_id }
    }

    pub(crate) fn make_message(self, sender: Identity, recipient: Option<Identity>) -> FileMessage {
        FileMessage {
            sender,
            recipient,
            meta: self,
        }
    }
}
