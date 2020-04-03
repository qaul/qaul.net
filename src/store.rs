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
#[derive(Default)]
pub(crate) struct Store {
    /// The shared datastore
    shared: BTreeMap<Path, Notify<Encrypted<Arc<Record>, SharedKey>>>,
    /// The per-user datastore
    usrd: BTreeMap<Id, Notify<BTreeMap<Path, Encrypted<Arc<Record>, KeyPair>>>>,
}

impl DetachedKey<SharedKey> for Arc<Record> {}

impl Store {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Get a single record from the store via the path
    ///
    /// If providing a user ID, check the user store first, before
    /// checking the shared store.
    pub(crate) fn get_path(&self, id: Option<Id>, path: &Path) -> Result<Arc<Record>> {
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
        &mut self,
        id: Option<Id>,
        path: &Path,
        tags: TagSet,
        diff: Diff,
    ) -> Result<Id> {
        // Check if the path exists already
        if id
            .and_then(|ref id| {
                self.usrd
                    .get(id)
                    .map_or(Some(false), |tree| Some(tree.contains_key(path)))
            })
            .or(Some(self.shared.contains_key(path)))
            .unwrap()
        {
            return Err(Error::NoSuchPath { msg: path.into() });
        }

        // Create a record
        let rec = Record::create(tags, diff)?;
        let rec_id = rec.header.id;
        let record = Encrypted::new(Arc::new(rec));

        match id {
            Some(id) => {
                let mut tree = self.usrd.entry(id).or_default();
                tree.insert(path.clone(), record);
                Notify::notify(&mut tree);
            }
            None => {
                self.shared.insert(path.clone(), Notify::new(record));
            }
        }

        Ok(rec_id)
    }
}

#[test]
fn store_insert() {
    use crate::{data::Value, diff::DiffSeg};

    let id = Id::random();
    let path = Path::from("/test:bob");
    let tags = TagSet::empty();
    let diff = Diff::from((
        "hello".into(),
        DiffSeg::Insert(Value::String("world".into())),
    ));

    let mut store = Store::new();
    let rec_id = store.insert(Some(id), &path, tags, diff).unwrap();

    assert_eq!(store.usrd.get(&id).unwrap().len(), 1);
    assert_eq!(store.shared.len(), 0);
    assert_eq!(
        store
            .usrd
            .get(&id)
            .unwrap()
            .get(&path)
            .unwrap()
            .deref()
            .unwrap()
            .header
            .id,
        rec_id
    );
}

#[test]
fn store_and_get() {
    use crate::{data::Value, diff::DiffSeg};

    let id = Id::random();
    let path = Path::from("/test:bob");
    let tags = TagSet::empty();
    let diff = Diff::from((
        "hello".into(),
        DiffSeg::Insert(Value::String("world".into())),
    ));

    let mut store = Store::new();
    let rec_id = store.insert(Some(id), &path, tags, diff).unwrap();

    assert_eq!(store.get_path(Some(id), &path).unwrap().header.id, rec_id);
}
