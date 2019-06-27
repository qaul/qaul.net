//! Service contact book API

use common::{identity::UserID, error::{Error as QaulError, Result as QaulResult}};

pub fn add(id: UserID, contact: UserID) {}

pub fn get(id: UserID, contact: UserID) -> QaulResult<UserID> {
    unimplemented!()
}

pub fn remove(id: UserID, contact: UserID) {}

pub fn amend(id: UserID, contact: UserID) {}
