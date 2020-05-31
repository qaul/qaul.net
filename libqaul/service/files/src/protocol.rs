//! The file protocol implementation
//!
//! Underlying types used are defined in `types.rs`, interactions are
//! defined here for clarity.  Following is a textual explanation of
//! the dynamics of the protocol, what parts are implemented here, and
//! what parts are implemented via libqaul.

use crate::{
    types::{File, FileFilter, FileId, FileMessage, FileMeta},
    Fileshare, ASC_NAME,
};
use async_std::task;
use libqaul::{
    error::Result,
    helpers::TagSet,
    messages::{IdType, Mode},
    users::UserAuth,
    Identity, Qaul,
};
use std::{
    fs,
    io::{Read, Write},
    sync::Arc,
};
use tracing::{info, warn};

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

    /// This function is called by the worker and does one of three
    /// things
    ///
    /// 1. For a request, check if we have this file.  If we do, reply with it.
    /// 2. For an ad, store the available file
    /// 3. For a file, store the file
    ///
    /// Any sending for replies happens internally to this function.
    ///
    /// Option questions:
    ///
    /// - how do we stop others from sending their files?
    pub(crate) async fn handle_incoming(self, auth: UserAuth, serv: Arc<Fileshare>) {
        let FileMessage {
            sender: friend,
            meta,
            ..
        } = self;

        match meta {
            // FIXME: we want to write ref meta @ here but we can't
            // because it's not availble on stable.  Change this as
            // soon as it becomes available.
            FileMeta::File(file) => {
                let path = serv.path.join(&file.name);
                if let Ok(mut f) = fs::File::create(path) {
                    let data = file.data;
                    task::spawn_blocking(move || f.write_all(&data))
                        .await
                        .unwrap();

                    // The type we store is "available", meaning the
                    // data is stored extrenally but we know about it
                    // and can recall the file-name just from the
                    // hash-id.  It also means that in the future we
                    // can change the file-name without anyone else
                    // needing to know about it.
                    serv.directory
                        .insert(
                            auth.clone(),
                            &FileMeta::Available {
                                hash_id: file.hash_id,
                                name: file.name,
                            },
                        )
                        .await;
                } else {
                    warn!(
                        "Failed to create file: {}.  File already exists!",
                        file.name
                    );
                }
            }
            FileMeta::Request { hash_id } => {
                if let Ok(FileMeta::Available { ref name, .. }) =
                    serv.directory.get(auth.clone(), hash_id).await
                {
                    let path = serv.path.join(&name);
                    if let Ok(mut f) = fs::File::open(path) {
                        let data = task::spawn_blocking(move || {
                            let mut data = vec![];
                            f.read_to_end(&mut data).unwrap();
                            data
                        })
                        .await;

                        // Build a reply file
                        let meta = FileMeta::File(File {
                            owner: auth.0,
                            hash_id,
                            name: name.clone(),
                            data,
                        });

                        // Send the reply off
                        meta.make_message(auth.0, Some(friend))
                            .send_off(auth, Arc::clone(&serv.qaul))
                            .await;
                    } else {
                        // In this block we want to delete the
                        // "available" flag from the database because
                        // the user deleted the file on disk.
                        serv.directory.delete(auth, &hash_id).await;
                    }
                }
                info!("Ignoring file request for: {}.  Not available!", hash_id);
            }
            ref meta @ FileMeta::Advertised { .. } => {
                // Store the advertised file if we don't know about it
                if serv.directory.get(auth.clone(), meta.id()).await.is_err() {
                    serv.directory.insert(auth, meta).await;
                }
            }
            meta => {
                warn!(
                    "Ignoring incoming meta because there are no rules for it: {:?}",
                    meta
                );
            }
        }
    }
}

impl FileMeta {
    pub(crate) fn id(&self) -> FileId {
        match self {
            Self::File(ref f) => f.hash_id,
            Self::Advertised { hash_id, .. } => *hash_id,
            Self::Available { hash_id, .. } => *hash_id,
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
