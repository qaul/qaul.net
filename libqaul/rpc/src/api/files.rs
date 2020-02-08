//! File API structures


use libqaul::{Identity, users::UserAuth, files::FileFilter};
use crate::Change;

/// Send a file store query
pub struct Query {
    auth: UserAuth,
    filter: FileFilter,
}

/// List available files
pub struct List {
    auth: UserAuth,
}
