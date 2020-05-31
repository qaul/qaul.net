//! `qaul.net` filesharing service

// #![allow(unused)]
#![doc(html_favicon_url = "https://qaul.net/favicon.ico")]
#![doc(html_logo_url = "https://qaul.net/img/qaul_icon-128.png")]


mod directory;
mod protocol;
mod types;
mod worker;

use crate::directory::{DirectoryRef, FileDirectory};
use async_std::{sync::Arc, task};
use libqaul::{error::Result, services::ServiceEvent, users::UserAuth, Identity, Qaul};
use mime::Mime;
use std::path::PathBuf;

pub use crate::types::{File, FileFilter, FileId, FileMeta, Subscription};
pub mod error;

const ASC_NAME: &'static str = "net.qaul.fileshare";

pub(crate) mod tags {
    use {crate::FileId, libqaul::helpers::Tag};
    pub(crate) const _META_NAME: &'static str = "file_list";
    pub(crate) const FILE_LIST: &'static str = "net.qaul.files.file_list";
    pub(crate) fn file_id(id: FileId) -> Tag {
        Tag::new("file-id", id.as_bytes().to_vec())
    }
}

/// Filesharing service state
#[derive(Clone)]
pub struct Fileshare {
    qaul: Arc<Qaul>,
    directory: DirectoryRef,
    path: PathBuf,
}

impl Fileshare {
    /// Initialise the file-sharing service
    ///
    /// In order to initialise, a valid and running
    /// `Qaul` reference needs to be provided.
    pub fn new(qaul: Arc<Qaul>, path: PathBuf) -> Result<Arc<Self>> {
        let directory = FileDirectory::new(Arc::clone(&qaul));
        let this = Arc::new(Self {
            qaul,
            directory,
            path,
        });
        let sender = Arc::new(worker::run_asnc(Arc::clone(&this)));

        this.qaul.services().register(ASC_NAME, move |cmd| {
            let sender = Arc::clone(&sender);
            task::block_on(async move {
                match cmd {
                    ServiceEvent::Open(auth) => sender.send(worker::Command::Start(auth)).await,
                    ServiceEvent::Close(auth) => sender.send(worker::Command::Stop(auth)).await,
                }
            });
        });

        Ok(this)
    }

    /// Advertise a file into a network
    pub async fn advertise(
        &self,
        auth: UserAuth,
        name: String,
        size: u64,
        payload: Vec<u8>,
        // _mime: Mime, // TODO: implement mime stuff
    ) -> Result<FileId> {
        let hash_id = Identity::with_digest(&payload);

        let meta = FileMeta::build_ad(auth.0, hash_id, name, size);
        let msg = meta.make_message(auth.0, None);
        msg.send_off(auth, Arc::clone(&self.qaul)).await?;

        Ok(hash_id)
    }

    /// Advertise a file into a network
    pub async fn advertise_to_user(
        &self,
        auth: UserAuth,
        friend: Identity,
        name: String,
        size: u64,
        payload: Vec<u8>,
        mime: Mime,
    ) -> Result<FileId> {
        let hash_id = Identity::with_digest(&payload);

        // Build an advertising meta
        let meta = FileMeta::build_ad(auth.0, hash_id, name, size);
        let msg = meta.make_message(auth.0, Some(friend));
        msg.send_off(auth, Arc::clone(&self.qaul)).await?;

        Ok(hash_id)
    }

    /// Request a file with a given file id
    pub async fn request(&self, auth: UserAuth, file_id: FileId) -> Result<()> {
        let meta = FileMeta::build_request(file_id);
        let msg = meta.make_message(auth.0, None);
        msg.send_off(auth, Arc::clone(&self.qaul)).await?;
        Ok(())
    }

    /// Request a file transfer from a specific user
    pub async fn request_from_user(
        &self,
        auth: UserAuth,
        friend: Identity,
        file_id: FileId,
    ) -> Result<()> {
        let meta = FileMeta::build_request(file_id);
        let msg = meta.make_message(auth.0, Some(friend));
        Ok(())
    }
}
