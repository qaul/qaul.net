//! Service contact book API

use super::{users::UserID, error::QResult};

pub fn add(id: UserID, contact: UserID) {}

pub fn get(id: UserID, contact: UserID) -> QResult<UserID> {
    unimplemented!()
}

pub fn remove(id: UserID, contact: UserID) {}

pub fn amend(id: UserID, contact: UserID) {}
