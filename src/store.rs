//! The internal data store

use crate::{
    crypto::{
        asym::{KeyPair, SharedKey},
        DetachedKey, Encrypted,
    },
    data::{Record, TagSet},
    diff::Diff,
    notify::Notify,
    Error, Id, Path, Result,
};
use async_std::sync::Arc;
use std::collections::{BTreeMap, BTreeSet};

/// Main data store (mirrored to /records)
pub(crate) struct Store {
    /// Build a space index
    spaces: BTreeSet<Path>,

    /// The shared datastore
    shared: BTreeMap<Path, Notify<Encrypted<Arc<Record>, SharedKey>>>,
    /// The per-user datastore
    usrd: BTreeMap<Id, Notify<BTreeMap<Path, Encrypted<Arc<Record>, KeyPair>>>>,
}

impl DetachedKey<SharedKey> for Arc<Record> {}

impl Store {
    /// Get a single record from the store via the path
    ///
    /// If providing a user ID, check the user store first, before
    /// checking the shared store.
    pub(crate) async fn get_path(&self, id: Option<Id>, path: &Path) -> Result<Arc<Record>> {
        id.and_then(|ref id| self.usrd.get(id))
            .and_then(|tree| {
                tree.get(path)
                    .and_then(|e| e.deref().map(|ref rec| Arc::clone(&rec)).ok())
            })
            .or(self
                .shared
                .get(path)
                .and_then(|e| e.deref().map(|ref rec| Arc::clone(&rec)).ok()))
            .map_or(Err(Error::NoSuchPath { msg: path.into() }), |rec| Ok(rec))
    }

    /// Insert a record into the store
    ///
    /// This operation will fail if the path already exists
    pub(crate) fn insert(
        &self,
        id: Option<Id>,
        path: &Path,
        tags: TagSet,
        d: Diff,
    ) -> Result<()> {
        // Create a record
        
        
        Ok(())
    }
}
