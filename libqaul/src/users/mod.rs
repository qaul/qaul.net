//! User storage

use identity::Identity;
use rand::prelude::*;
use std::collections::BTreeMap;

/// A complete user, with ID and data.
#[derive(Debug, PartialEq, Clone)]
pub struct User {
    /// A users network ID
    pub id: Identity,
    /// The user's information, such as name, bio, etc.
    pub data: UserData,
}

/// A public representation of user information
///
/// Apart from the user `id`, all fields are optional
/// and should not be assumed set. This struct is used
/// for both the local user (identified by `UserAuth`)
/// as well as remote users from the contacts book.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct UserData {
    /// A human readable display-name (like @foobar)
    pub display_name: Option<String>,
    /// A human's preferred call-signed ("Friends call be foo")
    pub real_name: Option<String>,
    /// A key-value list of things the user deems interesting
    /// about themselves. This could be stuff like "gender",
    /// "preferred languages" or whatever.
    pub bio: BTreeMap<String, String>,
    /// The set of services this user runs (should never be empty!)
    pub services: Vec<String>,
    /// A users profile picture (some people like selfies)
    pub avatar: Option<Vec<u8>>,
}

impl User {
    pub(crate) fn new() -> Self {
        let mut rng = rand::thread_rng();
        let buf: [u8; 12] = rng.gen();
        Self {
            id: buf.into(),
            data: Default::default(),
        }
    }
}
