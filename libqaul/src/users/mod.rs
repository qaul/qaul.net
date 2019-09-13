//! User storage

use identity::Identity;
use rand::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

mod updates;
pub use updates::UserUpdate;
mod contacts;
pub use contacts::{ContactBook, ContactUpdate, LocalContactData};

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
    pub services: BTreeSet<String>,
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

    /// Apply the given UserUpdate to this UserData, returning the modified version.
    pub fn apply(self, update: UserUpdate) -> Self {
        let mut new = self;
        update.apply_to(&mut new);
        new
    }

    /// Check if the names of this UserData are similar to the query given, in order to
    /// facilitate searching.
    pub fn like_query(&self, query: &str) -> bool {
        let like_display_name = match &self.display_name {
            None => false,
            Some(v) => v.contains(query),
        };

        let like_real_name = match &self.real_name {
            None => false,
            Some(v) => v.contains(query),
        };

        like_display_name || like_real_name
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
        self.services.insert(service.into());
        self
    }

    pub fn with_avatar_data(mut self, data: Vec<u8>) -> Self {
        self.avatar = Some(data);
        self
    }
}

#[test]
fn like_query_fails_with_no_names() {
    assert!(!UserData::new().like_query(""));
}

#[test]
fn like_query_succeeds_with_exact() {
    assert!(UserData::new()
        .with_display_name("@dannydefault")
        .like_query("@dannydefault"));
    assert!(UserData::new()
        .with_real_name("Danny Default")
        .like_query("Danny Default"));
}

#[test]
fn like_query_succeeds_with_perfect_substring() {
    assert!(UserData::new()
        .with_display_name("@dannydefault")
        .like_query("danny"));
    assert!(UserData::new()
        .with_real_name("Danny Default")
        .like_query("Danny"));
}
