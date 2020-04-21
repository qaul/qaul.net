//! Some simple integration tests for Alexandria

mod harness;
use harness::Test;
use tempfile::tempdir;

use alexandria::{
    record::kv::Value,
    utils::{Diff, DiffSeg, Path, Query, TagSet},
};

#[test]
fn insert_and_fetch() {
    let dir = tempdir().unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = TagSet::empty();
    let diff = Diff::from(("msg_count".into(), DiffSeg::Insert(Value::U64(0))));

    poll! { t.lib().data(t.users[0]).await.and_then(|api| poll! { api.insert(path.clone(), tags, diff).await }).unwrap() };
    poll! {
        t.lib().data(t.users[0]).await.and_then(|api| poll! {
            api.query(Query::Path(path.clone())).await
        }).unwrap()
    };
}

#[test]
fn batch_insert() {
    let dir = tempdir().unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = TagSet::empty();

    let diffs = vec![
        Diff::map().insert("id", "my-id"),
        Diff::map().insert("name", "spacekookie"),
    ];

    poll! { t.lib()
        .data(t.users[0])
        .await
        .and_then(|api| poll! { api.batch(path.clone(), tags, diffs).await })
            .unwrap()
    };
}
