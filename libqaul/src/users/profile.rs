//! User abstraction

use crate::Identity;
use std::collections::{BTreeMap, BTreeSet};

/// A complete user profile with ID and metadata
///
/// This abstraction is used in the Service API (see `api` module),
/// but is important beyond the API functions, and as such is not part
/// of the API `models` module.
///
/// The user profile itself makes no destinction between local, remote
/// or self users (the latter being the currently active user in a
/// session)
#[derive(Clone)]
pub struct UserProfile {
    /// A user's network (node) ID
    pub id: Identity,
    /// A human readable display-name (like @foobar)
    pub display_name: Option<String>,
    /// A human's preferred call-sign ("Friends call me foo")
    pub real_name: Option<String>,
    /// A key-value list of things the user deems interesting about
    /// themselves. This could be stuff like "gender", "preferred
    /// languages" or whatever.
    pub bio: BTreeMap<String, String>,
    /// The set of services this user runs (should never be empty!)
    pub services: BTreeSet<String>,
    /// A users profile picture (some people like selfies)
    pub avatar: Option<Vec<u8>>,
}

impl UserProfile {
    /// Create a new user profile for a user ID
    pub fn new(id: Identity) -> Self {
        Self {
            id,
            display_name: None,
            real_name: None,
            bio: BTreeMap::new(),
            services: BTreeSet::new(),
            avatar: None,
        }
    }

    /// Apply the given UserUpdate to this UserUpdate in-place
    pub fn apply(self, update: UserUpdate) -> Self {
        let mut new = self;
        update.apply_to(&mut new);
        new
    }

    /// Do a contains-query on names to facilitate searching
    ///
    /// This means that the query string needs to be contained in it's
    /// entirety in the display or real name strings to return a
    /// match.
    pub fn contains_query(&self, query: &str) -> bool {
        (match &self.display_name {
            None => false,
            Some(v) => v.contains(query),
        }) || (match &self.real_name {
            None => false,
            Some(v) => v.contains(query),
        })
    }

    /// Do a fully fuzzy query on names to facilitate searching
    pub fn fuzzy_query(&self, _query: &str) -> bool {
        unimplemented!()
    }
}

/// All the ways a UserData can change, as individual events.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UserUpdate {
    /// Set or blank the User's display name.
    DisplayName(Option<String>),
    /// Set or blank the User's real name.
    RealName(Option<String>),
    /// Add or update a biography line with the given key to the given value.
    SetBioLine(String, String),
    /// Remove a biography line with the given key, or do nothing if it does not exist.
    RemoveBioLine(String),
    /// Add a service with the given name.
    AddService(String),
    /// Remove the service with the given name, or do nothing if it does not exist.
    RemoveService(String),
    /// Set or blank the User's avatar.
    AvatarData(Option<Vec<u8>>),
}

impl UserUpdate {
    /// Change the given UserProfile based on the instruction given by this UserUpdate.
    pub fn apply_to(self, data: &mut UserProfile) {
        use UserUpdate::*;
        match self {
            DisplayName(v) => data.display_name = v,
            RealName(v) => data.real_name = v,
            SetBioLine(k, v) => {
                data.bio.insert(k, v);
            }
            RemoveBioLine(k) => {
                data.services.remove(&k);
            }
            AddService(k) => {
                data.services.insert(k);
            }
            RemoveService(k) => {
                data.services.remove(&k);
            }
            AvatarData(v) => data.avatar = v,
        }
    }
}
