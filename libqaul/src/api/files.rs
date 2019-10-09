//! Service API file access
//!
//! Not to be confused with the `files` filesharing service. This is
//! basic fs abstraction access to allow services to store files in a
//! users namespace.

use super::models::{File, FileMeta, FileFilter, UserAuth};
use crate::{Qaul, QaulResult, User};

impl Qaul {
    /// Query available files to a user
    ///
    /// Optionally: use the `filter` parameter to exclude certain
    /// types of files, based on the lifecycle in the system.
    pub fn files_query(
        user: UserAuth,
        filter: Option<FileFilter>,
    ) -> QaulResult<impl Iterator<Item = FileMeta>> {
        unimplemented!()
    }

    /// Stream one particular file from storage
    ///
    /// Returns an error if the file couldn't be loaded because it's
    /// namespace doesn't exist or because it's not ready to load yet
    /// (FileMeta mismatch).
    ///
    /// **ASYNC THIS** currently this function avoids being blocking
    /// by returning a Lambda, but this is a _bad_ solution. A prime
    /// candidate for being an `async fn`!
    pub fn files_get<L>(user: UserAuth, file: &str) -> QaulResult<L>
    where
        L: FnOnce() -> QaulResult<File>,
    {
        unimplemented!()
    }

    /// Adds a new file to the local user's storage
    pub fn files_add(user: UserAuth, name: &str, file: File) -> QaulResult<()> {
        Ok(())
    }

    /// Delete a file from local storage
    ///
    /// If the deleted file was still `InProgress`, all subsequent
    /// received parts will be ignored, so that no invalid files are
    /// created in the store.
    pub fn files_delete(user: UserAuth, name: &str) -> QaulResult<()> {
        Ok(())
    }
}
