//! The internal data store

use crate::{
    crypto::{
        asym::{KeyPair, SharedKey},
        DetachedKey, Encrypted,
    },
    delta::{DeltaBuilder, DeltaType},
    error::{Error, Result},
    notify::Notify,
    record::Record,
    utils::{Diff, Id, Path, TagSet},
    Session,
};
use async_std::sync::Arc;
use std::collections::BTreeMap;
use tracing::debug;

/// Main data store (mirrored to /records)
#[derive(Default)]
pub(crate) struct Store {
    /// The shared datastore
    shared: BTreeMap<Path, Notify<Encrypted<Arc<Record>, SharedKey>>>,
    /// The per-user datastore
    usrd: BTreeMap<Id, Notify<BTreeMap<Path, Notify<Encrypted<Arc<Record>, KeyPair>>>>>,
    /// Per-user GC locks
    gc_usr: BTreeMap<Id, BTreeMap<Path, GcReq>>,
    /// Shared-scope GC lock
    gc_shared: BTreeMap<Path, GcReq>,
}

/// A request for garbage collection wrapper
///
/// Specifies if an item should be held for GC, how many holders there
/// are and if the item should be deleted when the hold expires.
#[derive(Default)]
struct GcReq {
    /// Number of GC holders
    ctr: usize,
    /// Determine if the item should be deleted
    del: bool,
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
    pub(crate) fn get_path(&self, id: Session, path: &Path) -> Result<Arc<Record>> {
        id.id()
            .and_then(|ref id| self.usrd.get(id))
            .and_then(|tree| {
                tree.get(path)
                    .and_then(|e| e.deref().map(|ref rec| Arc::clone(&rec)).ok())
            })
            .or(self
                .shared
                .get(path)
                .and_then(|e| e.deref().map(|ref rec| Arc::clone(&rec)).ok()))
            .map_or(Err(Error::NoSuchPath { path: path.into() }), |rec| Ok(rec))
    }

    /// Similar to `insert`, but useful to seed an entire record from
    /// individual diffs at the same time
    #[tracing::instrument(skip(self, db, diffs), level = "debug")]
    pub(crate) fn batch(
        &mut self,
        db: &mut DeltaBuilder,
        id: Session,
        path: &Path,
        tags: TagSet,
        mut diffs: Vec<Diff>,
    ) -> Result<Id> {
        // Check if the path exists already
        if self.tree_mut(id).contains_key(path) {
            return Err(Error::PathExists { path: path.into() });
        }

        db.tags(&tags);
        db.path(&path);

        // Create a record
        let ulterior = diffs.split_off(1);
        let initial = diffs.remove(0);

        let mut rec = Record::create(tags, initial)?;
        let rec_id = rec.header.id;
        debug!("Created skeleton record `{}`", rec_id.to_string());
        
        for d in ulterior {
            rec.apply(d)?;
        }
        debug!("Applied diffs to skeleton record");

        let record = Notify::new(Encrypted::new(Arc::new(rec)));
        db.rec_id(rec_id);

        self.tree_mut(id).insert(path.clone(), record);
        self.wake_tree(id, path);

        Ok(rec_id)
    }

    /// Insert a record into the store
    ///
    /// This operation will fail if the path already exists
    #[tracing::instrument(skip(self, db, diff), level = "debug")]
    pub(crate) fn insert(
        &mut self,
        db: &mut DeltaBuilder,
        id: Session,
        path: &Path,
        tags: TagSet,
        diff: Diff,
    ) -> Result<Id> {
        // Check if the path exists already
        if self.tree_mut(id).contains_key(path) {
            return Err(Error::PathExists { path: path.into() });
        }

        db.tags(&tags);
        db.path(&path);

        // Create a record
        let rec = Record::create(tags, diff)?;
        let rec_id = rec.header.id;
        debug!("Seeded record `{}` from diff", rec_id);
        let record = Notify::new(Encrypted::new(Arc::new(rec)));
        db.rec_id(rec_id);

        self.tree_mut(id).insert(path.clone(), record);
        self.wake_tree(id, path);

        Ok(rec_id)
    }

    #[tracing::instrument(skip(self, db), level = "debug")]
    pub(crate) fn destroy(
        &mut self,
        db: &mut DeltaBuilder,
        id: Session,
        path: &Path,
    ) -> Result<()> {
        // Check if the path exists
        if !self.tree_mut(id).contains_key(path) {
            return Err(Error::NoSuchPath { path: path.into() });
        }

        db.path(&path);

        // Check if the path GC is locked and mark to delete
        if let Some(GcReq { ref mut del, .. }) = self.gc_set_mut(id).get_mut(path) {
            debug!("Marking path `{}` for future deletion", path);
            *del = true;
            return Ok(());
        }

        self.wake_tree(id, path);
        if let Ok(rec) = self.tree_mut(id).remove(path).unwrap().deref() {
            db.rec_id(rec.header.id);
            debug!("Deleting record `{}` from store", rec.header.id);
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, db, diff), level = "debug")]
    pub(crate) fn update(
        &mut self,
        db: &mut DeltaBuilder,
        id: Session,
        path: &Path,
        diff: Diff,
    ) -> Result<()> {
        // Check that the path actually exists
        if !self.tree_mut(id).contains_key(path) {
            return Err(Error::NoSuchPath { path: path.into() });
        }

        db.path(&path);

        // Make a copy of the underlying record
        let mut not: Notify<_> = self.tree_mut(id).remove(path).unwrap();
        let arc: &Arc<_> = not.deref()?;
        let mut rec: Record = (**arc).clone();

        db.rec_id(rec.header.id);

        // Apply changes
        rec.apply(diff)?;

        // Swap old and new records
        let mut arc = Arc::new(rec);
        not.swap(&mut arc);

        // Re-insert into the tree and wake pollers
        self.tree_mut(id).insert(path.clone(), not);
        self.wake_tree(id, path);
        Ok(())
    }

    /// Lock the GC for a set of paths
    #[tracing::instrument(skip(self), level = "debug")]
    pub(crate) fn gc_lock(&mut self, paths: &Vec<(Path, Session)>) {
        paths.iter().for_each(|(path, id)| {
            self.gc_set_mut(*id).entry(path.clone()).or_default().ctr += 1;
        });
    }

    /// Release the GC for a set of paths and delete them
    #[tracing::instrument(skip(self), level = "debug")]
    pub(crate) fn gc_release(&mut self, paths: &Vec<(Path, Session)>) -> Result<()> {
        paths.iter().fold(Ok(()), |res, (path, id)| {
            if let Some(GcReq {
                ref mut ctr,
                ref del,
            }) = self.gc_set_mut(*id).get_mut(&path)
            {
                // Decrement ctr
                *ctr -= 1;

                // If we were last, delete
                if *ctr == 0 && *del {
                    let mut db = DeltaBuilder::new(*id, DeltaType::Delete);
                    res.and_then(|_| self.destroy(&mut db, *id, path))
                } else {
                    res
                }
            } else {
                res
            }
        })
    }

    /// A helper to wake a tree, depending on Id
    fn wake_tree(&mut self, id: Session, path: &Path) {
        match id.id() {
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
        id: Session,
    ) -> &mut BTreeMap<Path, Notify<Encrypted<Arc<Record>, KeyPair>>> {
        match id.id() {
            Some(id) => self.usrd.entry(id).or_insert(Notify::new(BTreeMap::new())),
            None => &mut self.shared,
        }
    }

    /// A utility functiot to get the mutable gc lock, depending on id
    fn gc_set_mut(&mut self, id: Session) -> &mut BTreeMap<Path, GcReq> {
        match id.id() {
            Some(id) => self.gc_usr.entry(id).or_default(),
            None => &mut self.gc_shared,
        }
    }

    #[cfg(test)]
    #[allow(unused)]
    fn length(&mut self, id: Session) -> usize {
        self.tree_mut(id).len()
    }
}

///////////////////// Store tests

#[test]
fn store_insert() {
    use crate::{
        delta::{DeltaBuilder, DeltaType},
        record::kv::Value,
        utils::DiffSeg,
    };

    let id = Id::random();
    let path = Path::from("/test:bob");
    let tags = TagSet::empty();
    let diff = Diff::from((
        "hello".into(),
        DiffSeg::Insert(Value::String("world".into())),
    ));

    let mut db = DeltaBuilder::new(Session::Id(id), DeltaType::Insert);
    let mut store = Store::new();
    let rec_id = store
        .insert(&mut db, Session::Id(id), &path, tags, diff)
        .unwrap();

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
    use crate::{
        delta::{DeltaBuilder, DeltaType},
        record::kv::Value,
        utils::DiffSeg,
    };

    let id = Id::random();
    let path = Path::from("/test:bob");
    let tags = TagSet::empty();
    let diff = Diff::from((
        "hello".into(),
        DiffSeg::Insert(Value::String("world".into())),
    ));

    let mut db = DeltaBuilder::new(Session::Id(id), DeltaType::Insert);
    let mut store = Store::new();
    let rec_id = store
        .insert(&mut db, Session::Id(id), &path, tags, diff)
        .unwrap();

    assert_eq!(
        store.get_path(Session::Id(id), &path).unwrap().header.id,
        rec_id
    );
}

#[test]
fn store_and_update() {
    use crate::{
        delta::{DeltaBuilder, DeltaType},
        record::kv::Value,
        utils::DiffSeg,
    };

    let id = Id::random();
    let path = Path::from("/test:bob");
    let tags = TagSet::empty();
    let diff = Diff::from((
        "hello".into(),
        DiffSeg::Insert(Value::String("world".into())),
    ));

    let mut db = DeltaBuilder::new(Session::Id(id), DeltaType::Insert);
    let mut store = Store::new();
    let _ = store
        .insert(&mut db, Session::Id(id), &path, tags, diff)
        .unwrap();
    assert_eq!(
        store
            .usrd
            .get(&id)
            .unwrap()
            .get(&path)
            .unwrap()
            .deref()
            .unwrap()
            .kv()
            .len(),
        1
    );

    let diff2 = Diff::from((
        "saluton".into(),
        DiffSeg::Insert(Value::String("mondo".into())),
    ));

    let mut db = DeltaBuilder::new(Session::Id(id), DeltaType::Update);
    store
        .update(&mut db, Session::Id(id), &path, diff2)
        .unwrap();

    assert_eq!(store.usrd.get(&id).unwrap().len(), 1);
    assert_eq!(
        store
            .usrd
            .get(&id)
            .unwrap()
            .get(&path)
            .unwrap()
            .deref()
            .unwrap()
            .kv()
            .len(),
        2
    );
}

#[test]
fn store_and_delete() {
    use crate::{
        delta::{DeltaBuilder, DeltaType},
        record::kv::Value,
        utils::DiffSeg,
    };

    let id = Id::random();
    let path = Path::from("/test:bob");
    let tags = TagSet::empty();
    let diff = Diff::from((
        "hello".into(),
        DiffSeg::Insert(Value::String("world".into())),
    ));

    let mut store = Store::new();
    let mut db = DeltaBuilder::new(Session::Id(id), DeltaType::Insert);
    let _ = store
        .insert(&mut db, Session::Id(id), &path, tags, diff)
        .unwrap();
    assert_eq!(
        store
            .usrd
            .get(&id)
            .unwrap()
            .get(&path)
            .unwrap()
            .deref()
            .unwrap()
            .kv()
            .len(),
        1
    );

    let mut db = DeltaBuilder::new(Session::Id(id), DeltaType::Delete);
    store.destroy(&mut db, Session::Id(id), &path).unwrap();
    assert_eq!(store.usrd.get(&id).unwrap().len(), 0);
}

#[test]
fn insert_batch() {
    use crate::{
        delta::{DeltaBuilder, DeltaType},
        GLOBAL,
    };

    let vec = vec![
        Diff::map().insert("hello", "world"),
        Diff::map().insert("how", "are you?"),
    ];

    let path = Path::from("/test:bob");
    let tags = TagSet::empty();

    let mut store = Store::new();
    let mut db = DeltaBuilder::new(GLOBAL, DeltaType::Insert);

    let _ = store.batch(&mut db, GLOBAL, &path, tags, vec).unwrap();

    assert_eq!(
        store.shared.get(&path).unwrap().deref().unwrap().kv().len(),
        2
    );
}

#[test]
fn insert_batch_single() {
    use crate::{
        delta::{DeltaBuilder, DeltaType},
        GLOBAL,
    };

    let vec = vec![Diff::map().insert("hello", "world")];

    let path = Path::from("/test:bob");
    let tags = TagSet::empty();

    let mut store = Store::new();
    let mut db = DeltaBuilder::new(GLOBAL, DeltaType::Insert);

    let _ = store.batch(&mut db, GLOBAL, &path, tags, vec).unwrap();

    assert_eq!(
        store.shared.get(&path).unwrap().deref().unwrap().kv().len(),
        1
    );

    assert_eq!(store.length(GLOBAL), 1);
}
