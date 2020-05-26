//! The file protocol implementation
//!
//! Underlying types used are defined in `types.rs`, interactions are
//! defined here for clarity.  Following is a textual explanation of
//! the dynamics of the protocol, what parts are implemented here, and
//! what parts are implemented via libqaul.

use libqaul::error::Result;
use libqaul::Identity;
use libqaul::users::UserAuth;

use crate::types::File;
use crate::types::FileFilter;
use crate::types::FileId;
use crate::types::FileMeta;
use crate::types::Files;

impl<'qaul> Files<'qaul> {
    /// Query the local file store for a specific constraint
    pub fn query<I>(&self, user: UserAuth, filter: FileFilter) -> Result<I>
        where
            I: Iterator<Item=FileMeta>,
    {
        // self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// List all available files
    pub fn list<I>(&self, user: UserAuth) -> Result<I>
        where
            I: Iterator<Item=FileMeta>,
    {
        // self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// Stream one particular file from storage
    pub async fn get(&self, user: UserAuth, file: FileId) -> Result<File> {
        // self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// Adds a new file to the local user's storage
    pub fn add(&self, user: UserAuth, name: &str, file: File) -> Result<FileId> {
        // self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// Delete a file from the local user store
    pub fn delete(&self, user: UserAuth, name: FileId) -> Result<()> {
        // self.q.auth.trusted(user)?;
        unimplemented!()
    }
}
