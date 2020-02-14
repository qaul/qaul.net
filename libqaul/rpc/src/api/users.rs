//! Users API structures

use std::collections::{BTreeMap, BTreeSet};
use libqaul::{Identity, users::UserAuth, api::{ItemDiff, SetDiff, MapDiff}};
use serde::{Serialize, Deserialize};

/// Enumerate all publicly known users
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct List;

/// Create a new user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Create {
    pw: String,
}

/// Delete a user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Delete {
    auth: UserAuth,
    /// Indicate whether local data should be deleted as well
    purge: bool,
}

/// Change the password on a user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ChangePw {
    auth: UserAuth,
    new: String,
}

/// Create a new session for a user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Login {
    user: Identity,
    pw: String
}

/// Stop an existing session
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Logout {
    auth: UserAuth
}

/// Get the user profile for any remote or local user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Get {
    user: Identity
}

/// Apply an update to your user profile
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Update {
    auth: UserAuth,

    #[serde(default)]
    display_name: ItemDiff<String>,
    #[serde(default)]
    real_name: ItemDiff<String>,
    #[serde(default)]
    bio: Vec<MapDiff<String, String>>,
    #[serde(default)]
    services: Vec<SetDiff<String>>,
    #[serde(default)]
    avatar: ItemDiff<Vec<u8>>,
}
