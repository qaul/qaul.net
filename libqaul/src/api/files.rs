#![allow(unused)]

use crate::Identity;
use crate::error::Result;
use crate::users::UserAuth;
use serde::{Deserialize, Serialize};

pub type FileId = Identity;

/// Local file abstraction
pub struct File {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

/// Describe a file's lifecycle
///
/// Not to be confused with `FileFilter`, which is part of public API
/// functions to allow users to easily filter for only certain types
/// of file data.
///
/// Filter functions then take a `Filter` and return a `Meta`.
pub enum FileMeta {
    /// Files owned by the current user
    Local(File),
    /// Network files, fully locally mirrored
    Available(File),
    /// Network files, still downloading
    InProgress {
        size: usize,
        local: usize,
        stalled: bool,
    },
    /// A network advertised file that hasn't started downloading
    Advertised { size: usize },
}

/// Describe a file's lifecycle
///
/// Filter functions for each time exist and enable
/// different sub-services based on which phase they
/// aim for.
pub enum FileFilter {
    Local,
    Available,
    InProgress,
}

/// API scope type to access file functions
///
/// Used entirely to namespace API endpoints on `Qaul` instance,
/// without having long type identifiers.
///
/// ```norun
/// # use libqaul::{Qaul, Files};
/// # let user = unimplemented!();
/// let q = Qaul::default();
/// q.files().list(user)?;
/// ```
///
/// It's also possible to `drop` the current scope, back into the
/// primary `Qaul` scope, although this is not often useful.
///
/// ```norun
/// # use libqaul::{Qaul, Messages};
/// # let q = Qaul::default();
/// q.files().drop(); // Returns `&Qaul` again
/// ```
pub struct Files<'chain> {
    pub(crate) q: &'chain crate::Qaul,
}

impl<'qaul> Files<'qaul> {
    /// Query the local file store for a specific constraint
    pub fn query<I>(&self, user: UserAuth, filter: FileFilter) -> Result<I>
    where
        I: Iterator<Item = FileMeta>,
    {
        self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// List all available files
    pub fn list<I>(&self, user: UserAuth) -> Result<I>
    where
        I: Iterator<Item = FileMeta>,
    {
        self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// Stream one particular file from storage
    pub async fn get(&self, user: UserAuth, file: FileId) -> Result<File> {
        self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// Adds a new file to the local user's storage
    pub fn add(&self, user: UserAuth, name: &str, file: File) -> Result<FileId> {
        self.q.auth.trusted(user)?;
        unimplemented!()
    }

    /// Delete a file from the local user store
    pub fn delete(&self, user: UserAuth, name: FileId) -> Result<()> {
        self.q.auth.trusted(user)?;
        unimplemented!()
    }
}
