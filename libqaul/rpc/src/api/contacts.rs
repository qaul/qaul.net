//! Contacts API structures

use libqaul::{Identity, users::UserAuth, contacts::ContactQuery, api::ItemDiff};
use serde::{Serialize, Deserialize};

/// Apply a modification to a contact entry
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Modify {
    auth: UserAuth,
    contact: Identity,
    #[serde(default)]
    nick: ItemDiff<String>,
    #[serde(default)]
    trust: ItemDiff<i8>,
    #[serde(default)]
    met: ItemDiff<bool>,
    #[serde(default)]
    location: ItemDiff<String>,
    #[serde(default)]
    notes: ItemDiff<String>,
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
