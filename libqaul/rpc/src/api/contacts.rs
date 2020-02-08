//! Contacts API structures

use libqaul::{Identity, users::UserAuth, contacts::ContactQuery};
use crate::Change;
use serde::{Serialize, Deserialize};

/// Apply a modification to a contact entry
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Modify {
    auth: UserAuth,
    contact: Identity,
    #[serde(default)]
    nick: Change<String>,
    #[serde(default)]
    trust: Change<i8>,
    #[serde(default)]
    met: Change<bool>,
    #[serde(default)]
    location: Change<String>,
    #[serde(default)]
    notes: Change<String>,
}

/// Get the contact entry for an identity
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Get {
    auth: UserAuth,
    contact: Identity,
}

pub struct Query<'a> {
    auth: UserAuth,
    query: ContactQuery<'a>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct All {
    auth: UserAuth
}
