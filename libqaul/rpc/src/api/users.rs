//! Users API structures

use std::collections::{BTreeMap, BTreeSet};
use libqaul::{Identity, users::UserAuth};
use crate::Change;

/// Enumerate all publicly known users
pub struct List;

/// Create a new user
pub struct Create {
    pw: String,
}

/// Delete a user
pub struct Delete {
    auth: UserAuth,
    /// Indicate whether local data should be deleted as well
    purge: bool,
}

/// Change the password on a user
pub struct ChangePw {
    auth: UserAuth,
    new: String,
}

/// Create a new session for a user
pub struct Login {
    user: Identity,
    pw: String
}

/// Stop an existing session
pub struct Logout {
    auth: UserAuth
}

/// Get the user profile for any remote or local user
pub struct Get {
    user: Identity
}

/// Apply an update to your user profile
pub struct Update {
    auth: UserAuth,

    display_name: Change<String>,
    real_name: Change<String>,
    /// Can either be a set of insertions, or a set of deletions
    bio: Change<BTreeMap<String, String>>,
    services: Change<BTreeSet<String>>,
    avatar: Change<Vec<u8>>,
}
