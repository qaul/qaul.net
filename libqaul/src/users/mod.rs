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
/// This struct is used for both the local user (identified
/// by `UserAuth`) as well as remote users from the contacts book.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct UserData {
    /// A human readable display-name (like @foobar)
    pub display_name: Option<String>,
    /// A human's preferred call-sign ("Friends call me foo")
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

impl UserData {
    /// Create a new `UserData` with no data.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_display_name<S: Into<String>>(mut self, name: S) -> Self {
        self.display_name = Some(name.into());
        self
    }

    pub fn with_real_name<S: Into<String>>(mut self, name: S) -> Self {
        self.real_name = Some(name.into());
        self
    }

    pub fn with_bio_line<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.bio.insert(key.into(), value.into());
        self
    }

    pub fn with_service<S: Into<String>>(mut self, service: S) -> Self {
        self.services.push(service.into());
        self
    }

    pub fn with_avatar_data(mut self, data: Vec<u8>) -> Self {
        self.avatar = Some(data);
        self
    }
}
