//! `qaul.net` filesharing service

use identity::Identity;
pub use mime::Mime;
use qaul::{Qaul, QaulResult, UserAuth};

/// A typed file that can be sent across the network
pub struct File {
    pub mime: Mime,
    pub data: Vec<u8>,
}

/// Filesharing service state
pub struct Filesharing<'q> {
    qaul: &'q Qaul,
}

impl<'q> Filesharing<'q> {
    /// Send a single file to a group of people
    pub fn send_file(
        &self,
        user: UserAuth,
        recipients: Vec<Identity>,
        file: File,
    ) -> QaulResult<()> {
        unimplemented!()
    }

    /// Get all files that were received since the last poll
    pub fn poll_files(&self, user: UserAuth) -> QaulResult<Vec<File>> {
        unimplemented!()
    }
}
