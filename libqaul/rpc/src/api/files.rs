//! File API structures

use async_trait::async_trait;
use crate::QaulRpc;
use libqaul::{Identity, users::UserAuth, files::FileFilter};

/// Send a file store query
pub struct Query {
    auth: UserAuth,
    filter: FileFilter,
}

/// List available files
pub struct List {
    auth: UserAuth,
}
