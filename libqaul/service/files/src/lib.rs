//! `qaul.net` filesharing service

use std::sync::Arc;

use libqaul::{error::Result, Qaul};

pub mod types;

// TODO: Partial files
// TODO: file progress
// TODO: Download links with tokens

const ASC_NAME: &'static str = "net.qaul.filesharing";

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
        Ok(Self { qaul })
    }

    //     /// Announce a new file into a network
    //     pub fn announce<S>(&self, name: S) -> Result<FileId> {

    //     }
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
