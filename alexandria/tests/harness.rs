//! A test harness for alexandria tests

use alexandria::{utils::Id, Builder, Library};
use async_std::task::block_on;
use std::path::PathBuf;

pub const PASS: &'static str = "car horse battery staple";

pub struct Test {
    pub users: Vec<Id>,
    lib: Library,
}

impl Test {
    /// Create a test setup with a number of users
    pub fn new<P: Into<PathBuf>>(dir: P, userno: usize) -> Self {
        let lib = Builder::new().offset(dir.into().as_path()).build().unwrap();
        let users = (0..userno).fold(vec![], |mut vec, _| {
            vec.push(block_on(async {
                let id = Id::random();
                lib.user(id).create(PASS).await.map(|_| id).unwrap()
            }));
            vec
        });

        Self { users, lib }
    }

    pub fn lib(&self) -> &Library {
        &self.lib
    }
}

#[macro_export]
macro_rules! poll {
    ($x:expr) => {
        async_std::task::block_on(async { $x })
    };
}
