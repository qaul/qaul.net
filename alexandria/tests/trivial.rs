//! Some dead simple integration tests for alexandria
//!
//! These tests are deemed trivial because they're non-concurrent,
//! linear and non-malicious.  A test simply makes sure that the
//! public API surface is in fact holding up it's guaratees, without
//! breaking under simple load.  These tests are also meant to be
//! usage examples for newcomers.  None of these tests should be
//! considered valuable for finding bugs.

mod harness;
use harness::Test;

use alexandria::{
    query::{Query, SetQuery},
    record::kv::Value,
    utils::{Diff, DiffSeg, Path, Tag, TagSet},
};
use tempfile::tempdir;

#[async_std::test]
async fn insert_and_fetch() {
    let dir = tempdir().unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = TagSet::empty();
    let diff = Diff::from(("msg_count".into(), DiffSeg::Insert(Value::U64(0))));

    t.lib()
        .insert(t.users[0], path.clone(), tags, diff)
        .await
        .unwrap();

    t.lib()
        .query(t.users[0], Query::Path(path.clone()))
        .await
        .unwrap();
}

#[async_std::test]
async fn batch_insert() {
    let dir = tempdir().unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = TagSet::empty();

    let diffs = vec![
        Diff::map().insert("id", "my-id"),
        Diff::map().insert("name", "spacekookie"),
    ];

    t.lib()
        .batch(t.users[0], path.clone(), tags, diffs)
        .await
        .unwrap();
}

#[async_std::test]
async fn insert_delete() {
    let dir = tempdir().unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = TagSet::empty();
    let diff = Diff::from(("msg_count".into(), DiffSeg::Insert(Value::U64(0))));

    t.lib()
        .insert(t.users[0], path.clone(), tags, diff)
        .await
        .unwrap();

    t.lib().delete(t.users[0], path.clone()).await.unwrap()
}

#[async_std::test]
async fn insert_delete_failed_query() {
    let dir = tempdir().unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = TagSet::empty();
    let diff = Diff::from(("msg_count".into(), DiffSeg::Insert(Value::U64(0))));

    t.lib()
        .insert(t.users[0], path.clone(), tags, diff)
        .await
        .unwrap();

    t.lib().delete(t.users[0], path.clone()).await.unwrap();

    assert!(t
        .lib()
        .query(t.users[0], Query::Path(path.clone()))
        .await
        .is_err());
}

#[async_std::test]
async fn simple_subscription() {
    let dir = tempdir().unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = vec![Tag::empty("marked")];
    let diff = Diff::from(("msg_count".into(), DiffSeg::Insert(Value::U64(0))));

    let sub = t
        .lib()
        .subscribe(
            t.users[0],
            Query::Tag(SetQuery::Partial(tags.clone().into())),
        )
        .await
        .unwrap();

    // Insert some data
    t.lib()
        .insert(t.users[0], path.clone(), tags, diff)
        .await
        .unwrap();

    assert_eq!(sub.next().await, path);
}
