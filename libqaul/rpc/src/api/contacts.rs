//! Contacts API structures

use libqaul::{Identity, users::UserAuth, contacts::ContactQuery};
use crate::Change;

/// Apply a modification to a contact entry
pub struct Modify {
    auth: UserAuth,
    contact: Identity,
    nick: Change<String>,
    trust: Change<i8>,
    met: Change<bool>,
    location: Change<String>,
    notes: Change<String>,
}

/// Get the contact entry for an identity
pub struct Get {
    auth: UserAuth,
    contact: Identity,
}

pub struct Query<'a> {
    auth: UserAuth,
    query: ContactQuery<'a>,
}

pub struct All {
    auth: UserAuth
}
