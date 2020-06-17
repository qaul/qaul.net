//! A test harness for alexandria tests

use alexandria::{utils::Id, Builder, Library, Session};
use async_std::{sync::Arc, task::block_on};
use std::path::PathBuf;

pub const PASS: &'static str = "car horse battery staple";

pub struct Test {
    pub users: Vec<Session>,
    lib: Arc<Library>,
}

impl Test {
    /// Create a test setup with a number of users
    pub fn new<P: Into<PathBuf>>(dir: P, userno: usize) -> Self {
        let lib = Builder::new().offset(dir.into().as_path()).build().unwrap();
        let users = (0..userno).fold(vec![], |mut vec, _| {
            vec.push(block_on(async {
                let id = Id::random();
                Session::Id(lib.sessions().create(id, PASS).await.map(|_| id).unwrap())
            }));
            vec
        });

        Self { users, lib }
    }

    pub fn lib(&self) -> &Arc<Library> {
        &self.lib
    }
}
