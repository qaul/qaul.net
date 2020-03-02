//! Zone tables

use crate::Id;
use async_std::sync::{Arc, Mutex};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(crate) struct ZoneTable(BTreeMap<Id, BTreeSet<String>>);

impl ZoneTable {
    /// Create an empty zone table
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn insert<S>(&mut self, id: Id, zone: S)
    where
        S: Into<String>,
    {
        self.0.entry(id).or_default().insert(zone.into());
    }

    pub(crate) fn get(&self, id: Id) -> BTreeSet<String> {
        self.0.get(&id).unwrap().clone()
    }
}
