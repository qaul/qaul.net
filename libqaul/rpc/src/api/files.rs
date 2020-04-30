//! File API structures

use crate::QaulRpc;
use async_trait::async_trait;
use libqaul::{files::FileFilter, users::UserAuth, Identity};

/// Send a file store query
#[derive(PartialEq)]
pub struct Query {
    auth: UserAuth,
    filter: FileFilter,
}

/// List available files
#[derive(PartialEq)]
pub struct List {
    auth: UserAuth,
}
