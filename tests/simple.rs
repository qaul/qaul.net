//! Some simple integration tests for Alexandria

mod harness;
use harness::Test;

use tempdir::TempDir;

use alexandria::{
    record::kv::Value,
    utils::{Diff, DiffSeg, Path, TagSet},
};

#[test]
fn scaffold_lib() {
    let dir = TempDir::new("alexandria").unwrap();
    let _ = Test::new(dir.path(), 1);
}

#[test]
fn insert_and_fetch() {
    let dir = TempDir::new("alexandria").unwrap();
    let t = Test::new(dir.path(), 1);

    let path = Path::from("/msg:alice");
    let tags = TagSet::empty();
    let diff = Diff::from(("msg_count".into(), DiffSeg::Insert(Value::U64(0))));

    poll! { t.lib.data(t.users[0]).await.and_then(|api| poll! { api.insert(path, tags, diff).await }).unwrap() };
}
