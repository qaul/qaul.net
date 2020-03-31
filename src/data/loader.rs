//! Data record loader abstraction
//!
//! This module contains a set of functions to load records from disk
//! in various circumstances. Some, like `Record::load` go
//! through the entire load operation, decrypting all metadata and
//! caching a whole record, while others just load certain aspects of
//! metadata.  The idea is that load operations be made as simple as
//! possible for the rest of the library internals.

use crate::{
    crypto::{asym::KeyPair, Encrypted},
    data::{Body, Header, Record, SecHeader, Tag, Type},
    error::Result,
    Id,
};
use async_std::{fs::File, io::ReadExt, path::Path};
use bincode;
use std::collections::BTreeSet;

impl Record {
    /// Lazy load a record from disk, depending on the payload
    pub(crate) async fn load<'p, P: Into<&'p Path>>(
        path: P,
        id: Id,
        key: &KeyPair,
    ) -> Result<Self> {
        let mut f = File::open(path.into()).await?;

        // Read the header lenth
        let mut buf: [u8; 8] = [0; 8];
        f.read_exact(&mut buf).await?;
        let hlen = u64::from_le_bytes(buf);

        // Read the pub header
        let mut hser = vec![0; hlen as usize];
        f.read_exact(&mut hser).await?;

        // Build the header
        let Header { id, tags, mut sec } = bincode::deserialize(&hser)?;

        // Attempt to decrypt the sec header
        sec.open(&key)?;
        let SecHeader {
            ref t,
            ref size,
            ref chunks,
        } = sec.deref()?;

        match t {
            /// We load kv records immediately
            Type::Kv => {
                let mut bser = vec![0; *size as usize];
                f.read_exact(&mut bser).await?;
                let mut body: Encrypted<Body, KeyPair> = bincode::deserialize(&bser)?;
                body.open(&key)?;

                Ok(Self {
                    header: Header { id, tags, sec },
                    body,
                })
            }
            /// We don't _really_ load blob records
            Type::Blob => unimplemented!(),
        }
    }
}

pub(crate) async fn load_tags<'p, P: Into<&'p Path>>(path: P) -> Result<BTreeSet<Tag>> {
    let mut f = File::open(path.into()).await?;

    // Read the header lenth
    let mut buf: [u8; 8] = [0; 8];
    f.read_exact(&mut buf).await?;
    let hlen = u64::from_le_bytes(buf);

    // Read the pub header
    let mut hser = vec![0; hlen as usize];
    f.read_exact(&mut hser).await?;

    // Build the header
    let Header { id, tags, sec } = bincode::deserialize(&hser)?;

    // Return the tags
    Ok(tags)
}
