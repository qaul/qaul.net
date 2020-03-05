//! Zone tables

use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted},
    Id,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// A set of zones that a user has access to
#[derive(Default, Serialize, Deserialize)]
struct ZoneMap {
    map: BTreeSet<String>,
}

impl DetachedKey<KeyPair> for ZoneMap {}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct ZoneTable(BTreeMap<Id, Encrypted<ZoneMap, KeyPair>>);

impl ZoneTable {
    /// Create an empty zone table
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn insert<S>(&mut self, id: Id, zone: S)
    where
        S: Into<String>,
    {
        // self.0.entry(id).or_default().insert(zone.into());
    }

    // pub(crate) fn get(&self, id: Id) -> BTreeSet<String> {
    //     self.0.get(&id).unwrap().clone()
    // }
}
