//! Contact books for users, storing the users they know about.
//!
//! Users' contact books exist as:
//! - lists of users they know about, by identity, plus
//! - local-only information about those users, like personal nicknames

use identity::Identity;
use std::collections::BTreeMap;

/// A collection of contacts associated with their local-only data.
pub type ContactBook = BTreeMap<Identity, LocalContactData>;

/// Data about a contact that is relevant only from a single user's perspective.
#[derive(Default, Debug)]
pub struct LocalContactData {
    /// The name by which the associated contact is known by the owning user.
    personal_nick: Option<String>,
}

/// All the ways in which a contact's local data can be changed.
#[derive(Debug)]
pub enum ContactUpdate {
    /// Set the local nickname of a contact.
    SetPersonalNickname(String),
    /// Clear the local nickname of a contact.
    ClearPersonalNickname,
}

impl ContactUpdate {
    pub fn apply_to(self, data: &mut LocalContactData) {
        use ContactUpdate::*;
        match self {
            SetPersonalNickname(s) => data.personal_nick = Some(s),
            ClearPersonalNickname => data.personal_nick = None,
        }
    }
}
