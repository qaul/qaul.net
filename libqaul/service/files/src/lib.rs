//! `qaul.net` filesharing service
//!
//! Provides a simple interface to announce and broadcast binary
//! payload recipient messages.  Files can either be directly sent, or
//! they can be advertised, then pulled with a unique file ID,
//! contained in the announcement.

// #![allow(unused)]

use libqaul::{error::Result, Qaul};
use std::sync::Arc;

const ASC_NAME: &'static str = "net.qaul.filesharing";

// /// A typed file that can be sent across the network
// pub struct File {
//     pub name: String,
//     pub mime: Mime,
//     pub data: Vec<u8>,
// }

// // TODO: Partial files/ file progress
// // TODO: Download links with tokens

/// Filesharing service state
#[derive(Clone)]
pub struct Filesharing {
    qaul: Arc<Qaul>,
}

impl Filesharing {
    /// Initialise the filesharing service
    ///
    /// In order to initialise, a valid and running
    /// `Qaul` reference needs to be provided.
    pub fn new(qaul: Arc<Qaul>) -> Result<Self> {
        qaul.services().register(ASC_NAME)?;
        Ok(Self { qaul })
    }

    /// Announce a new file into a network
    pub fn announce<S>(&self, name: S) -> Result<FileId> {

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
