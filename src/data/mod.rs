//! Record data formats and utility types

mod blob;
mod kv;
mod loader;
mod tag;

pub use self::{
    blob::Blob,
    kv::{Kv, Value},
    tag::{Tag, TagSet},
};
use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted},
    Diff, Id, Result,
};

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A record header
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    /// A unique record ID
    pub id: Id,
    /// Public set of search tags
    pub tags: BTreeSet<Tag>,
    /// The encrypted header
    sec: Encrypted<SecHeader, KeyPair>,
}

/// Distinguishes between the type of records
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Type {
    /// Key-value mapped store
    Kv,
    /// Large binary object
    Blob,
}

/// The secret header is encrypted
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SecHeader {
    /// Record type
    pub(crate) t: Type,
    /// Total payload size
    pub(crate) size: u64,
    /// Beginning chunk markers
    pub(crate) chunks: Vec<u32>,
}

impl DetachedKey<KeyPair> for SecHeader {}

/// A record data body
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Body {
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

impl Record {
    pub(crate) fn create(tags: TagSet, diff: Diff) -> Result<Self> {
        // let (t, body) = match diff {
        //     Diff::Map(kv) => (Type::Kv, Body::Kv(Kv::new().apply(diff)?)),
        //     Diff::Binary(blob) => unimplemented!(),
        // };

        unimplemented!()
    }
}
