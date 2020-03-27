//! On-file data formats

mod inbox;
mod kv;
mod tag;

use crate::{
    crypto::{asym::KeyPair, Encrypted, DetachedKey},
    error::Result,
    Id,
};
use tag::Tag;

use async_std::{fs::File, io::ReadExt, sync::Arc};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};

/// A record header
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    id: Id,
    tags: BTreeSet<Tag>,
    sec: Encrypted<SecHeader, KeyPair>,
}

/// Distinguishes between the type of records
#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Kv,
    Blob,
}

/// The secret header is encrypted
#[derive(Debug, Serialize, Deserialize)]
pub struct SecHeader {
    /// Record type
    t: Type,
    /// Total payload size
    size: u64,
    /// Beginning chunk markers
    chunks: Vec<u32>,
}

impl DetachedKey<KeyPair> for SecHeader {}
