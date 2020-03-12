//! Zone tables

use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted, EncryptedMap},
    error::{Error, Result},
    meta::users::User,
    Id,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use async_std::sync::Arc;

/// A set of zones that a user has access to
#[derive(Default, Serialize, Deserialize)]
struct ZoneMap {
    set: BTreeSet<String>,
}

impl DetachedKey<KeyPair> for ZoneMap {}

#[derive(Serialize, Deserialize)]
pub(crate) struct ZoneTable(EncryptedMap<Id, ZoneMap, KeyPair>);

impl ZoneTable {
    /// Create an empty zone table
    pub(crate) fn new() -> Self {
        Self(EncryptedMap::new())
    }

    /// Add a new user to the user table
    pub(crate) fn insert<S: Into<String>>(&mut self, user: &User, zone: S) -> Result<()> {
        let zone = zone.into();
        let id = user.id;

        // Load the existing zone map or create a new one
        let ref mut zm = self
            .0
            .entry(id)
            .or_insert(Encrypted::new(ZoneMap::default()))
            .deref_mut()?;

        // Return an error if the zone exsts
        if zm.set.contains(&zone) {
            return Err(Error::ZoneAlreadyExsts {
                id: id.to_string(),
                zone,
            });
        }

        zm.set.insert(zone);
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
