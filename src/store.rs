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
    usrd: BTreeMap<Id, Notify<BTreeMap<Path, Notify<Encrypted<Arc<Record>, KeyPair>>>>>,
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
        if self.tree(id).map(|tree| tree.contains_key(path))? {
            return Err(Error::NoSuchPath { msg: path.into() });
        }

        // Create a record
        let rec = Record::create(tags, diff)?;
        let rec_id = rec.header.id;
        let record = Notify::new(Encrypted::new(Arc::new(rec)));

        self.tree_mut(id).insert(path.clone(), record);
        self.wake_tree(id, path);

        Ok(rec_id)
    }

    pub(crate) fn destroy(&mut self, id: Option<Id>, path: &Path) -> Result<()> {
        Ok(())
    }

    pub(crate) fn update(&mut self, id: Option<Id>, path: &Path, diff: Diff) -> Result<()> {
        Ok(())
    }

    /// A helper to wake a tree, depending on Id
    fn wake_tree(&mut self, id: Option<Id>, path: &Path) {
        match id {
            Some(ref id) => {
                let tree = self
                    .usrd
                    .get_mut(id)
                    .expect("Don't try to wake something that doen't exist!");
                Notify::notify(tree);

                let rec = tree
                    .get_mut(path)
                    .expect("Don't try to wake something that doen't exist!");
                Notify::notify(rec);
            }
            None => {
                let tree = self
                    .shared
                    .get_mut(path)
                    .expect("Don't try to wake something that doen't exist!");
                Notify::notify(tree);
            }
        }
    }

    /// A utility function to get the mutable tree, depending on id
    fn tree_mut(
        &mut self,
        id: Option<Id>,
    ) -> &mut BTreeMap<Path, Notify<Encrypted<Arc<Record>, KeyPair>>> {
        match id {
            Some(id) => self.usrd.entry(id).or_default(),
            None => &mut self.shared,
        }
    }

    /// A utility function to get the tree, depending on id
    fn tree(
        &self,
        id: Option<Id>,
    ) -> Result<&BTreeMap<Path, Notify<Encrypted<Arc<Record>, KeyPair>>>> {
        match id {
            Some(ref id) => self
                .usrd
                .get(id)
                .map_or(Err(Error::NoSuchUser { id: id.to_string() }), |tree| {
                    Ok(tree)
                }),
            None => Ok(&self.shared),
        }
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
