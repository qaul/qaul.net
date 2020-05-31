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
    Qaul,
};
use std::sync::Arc;

impl FileMessage {
    pub(crate) async fn send_off(&self, user: UserAuth, qaul: Arc<Qaul>) -> Result<()> {
        let payload = bincode::serialize(&self).unwrap();
        if let ad @ FileMeta::Advertised { .. } = &self.meta {
            qaul.messages()
                .send(
                    user,
                    Mode::Flood,
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
