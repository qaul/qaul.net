//! Service API exchange models

use std::fmt::{self, Debug, Formatter};
use ratman::Identity;
use mime::Mime;


/// Signature trust information embedded into service messages
pub enum SigTrust {
    /// A verified signature by a known contact
    Trusted(Identity),
    /// An unverified signature by a known contact
    /// (pubkey not available!)
    Unverified(Identity),
    /// A fraudulent signature
    Invalid,
}

/// Local file abstraction
pub struct File {
    pub name: String,
    pub mime: Mime,
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
