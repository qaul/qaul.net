//! Zone tables

use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted, EncryptedMap},
    error::{Error, Result},
    meta::users::User,
    utils::Id,
};
use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A set of paths that a user has access to
#[derive(Clone, Default, Serialize, Deserialize)]
struct PathMap {
    set: BTreeSet<String>,
}

impl DetachedKey<KeyPair> for PathMap {}

#[derive(Serialize, Deserialize)]
pub(crate) struct PathTable(EncryptedMap<Id, PathMap, KeyPair>);

impl PathTable {
    /// Create an empty zone table
    pub(crate) fn new() -> Self {
        Self(EncryptedMap::new())
    }

    /// Add a new user to the user table
    pub(crate) fn insert<S: Into<String>>(&mut self, user: &User, path: S) -> Result<()> {
        let path = path.into();
        let id = user.id;

        // Load the existing zone map or create a new one
        let ref mut zm = self
            .0
            .entry(id)
            .or_insert(Encrypted::new(PathMap::default()))
            .deref_mut()?;

        // Return an error if the zone exsts
        if zm.set.contains(&path) {
            return Err(Error::PathExists { path });
        }

        zm.set.insert(path);
        Ok(())
    }

    /// Unlock a user entry in place
    ///
    /// The provided Id will be hashed, to corresponds to a `Hid`,
    /// which provides a layer of anonymity for users in the database.
    pub(crate) fn open(&mut self, user: &User) -> Result<()> {
        self.0.open(user.id, &*user.key)
    }

    /// Re-seal the user metadata structure in place
    pub(crate) fn close(&mut self, user: &User) -> Result<()> {
        self.0.close(user.id, Arc::clone(&user.key))
    }
}
