//! Service data store access

use common::{identity::UserID, error::{Error as QaulError, Result as QaulResult}};

pub fn store(user: UserID, key: String, data: Vec<u8>) {}

pub fn get(user: UserID, key: String) -> QaulResult<Vec<u8>> {
    unimplemented!()
}

pub fn remove(user: UserID, key: String) {}

// FIXME: How can data be amended?
//        What is data even? How do we structure
//        this key-value store?
pub fn amend(user: UserID, key: String) {}
