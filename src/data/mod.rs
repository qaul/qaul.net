//! On-file data formats

mod blob;
mod kv;
mod tag;
mod loader;

use self::{blob::Blob, kv::Kv, tag::Tag};
use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted},
    error::Result,
    Id,
};

use async_std::{fs::File, io::ReadExt, sync::Arc};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};

/// A record header
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub id: Id,
    pub tags: BTreeSet<Tag>,
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
    pub t: Type,
    /// Total payload size
    pub size: u64,
    /// Beginning chunk markers
    pub chunks: Vec<u32>,
}

impl DetachedKey<KeyPair> for SecHeader {}

/// A record data body
#[derive(Debug, Serialize, Deserialize)]
pub enum Body {
    Kv(Kv),
    Blob(Blob),
}

impl DetachedKey<KeyPair> for Body {}

/// A single record in alexandria, defined by a header and body
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    /// The clear record header
    pub header: Header,
    /// A handle to the data body
    body: Encrypted<Body, KeyPair>,
}
