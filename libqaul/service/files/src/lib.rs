//! `qaul.net` filesharing service

// #![allow(unused)]
#![doc(html_favicon_url = "https://qaul.net/favicon.ico")]
#![doc(html_logo_url = "https://qaul.net/img/qaul_icon-128.png")]

use async_std::{sync::Arc, task};
use mime::Mime;

use libqaul::messages::{Message, MsgQuery};
use libqaul::users::UserAuth;
use libqaul::Identity;
use libqaul::{error::Result, Qaul};
use libqaul::services::ServiceEvent;

pub use crate::types::{File, FileFilter, FileId, Subscription, Files};

mod msg;
mod protocol;
pub mod types;
mod worker;

// these are original TODOs
// TODO: Partial files
// TODO: file progress
// TODO: Download links with tokens

const ASC_NAME: &'static str = "net.qaul.fileshare";

/// Filesharing service state
#[derive(Clone)]
pub struct Fileshare {
    qaul: Arc<Qaul>,
    advertised: Arc<Vec<FileId>>,
}

impl Fileshare {
    /// Initialise the file-sharing service
    ///
    /// In order to initialise, a valid and running
    /// `Qaul` reference needs to be provided.
    pub fn new(qaul: Arc<Qaul>, advertised: Arc<Vec<FileId>>) -> Result<Arc<Self>> {
        let this = Arc::new(Self {qaul, advertised});
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
    pub fn advertise(
        &self,
        file_name: String,
        file_id: FileId,
        file_size: usize,
        file_type: Mime,
    ) -> Result<Arc<Vec<FileId>>> {
        // TODO: Check if ok that it returns the `advertised` vector

        unimplemented!()
    }

    // Advertise to a single user
    pub fn advertise_to_user(
        &self,
        file_name: String,
        file_size: usize,
        file_type: Mime,
        send_to: UserAuth,
    ) -> Result<Arc<Vec<FileId>>> {
        unimplemented!()
    }

    /// Request a file with a given file id
    pub fn request(&self, file_id: FileId) -> Result<File> {
        unimplemented!()
    }
}

// impl<'q> Filesharing<'q> {
//     /// Send a single file to a group of people
//     pub fn send_file(
//         &self,
//         user: UserAuth,
//         recipients: Vec<Identity>,
//         file: File,
//     ) -> QaulResult<()> {
//         unimplemented!()
//     }

//     /// Get all files that were received since the last poll
//     pub fn poll_files(&self, user: UserAuth) -> QaulResult<Vec<File>> {
//         unimplemented!()
//     }
// }
