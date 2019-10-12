//! Contact books for users, storing the users they know about.
//!
//! Users' contact books exist as:
//!
//! - lists of users they know about, by identity, plus
//! - local-only information about those users, like personal nicknames

use crate::{Identity, QaulError, QaulResult};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Wraps around user-local contact books
#[derive(Default, Debug)]
pub(crate) struct ContactStore {
    inner: Arc<Mutex<BTreeMap<Identity, ContactList>>>,
}

/// A collection of contacts associated with their local-only data.
pub(crate) type ContactList = BTreeMap<Identity, ContactData>;

/// Data about a contact that is relevant only from a single user's perspective.
#[derive(Default, Debug)]
pub(crate) struct ContactData {
    /// The name by which the associated contact is known by the owning user.
    nick: Option<String>,
    /// Set a user trust level
    trust: i8,
}

impl ContactStore {
    /// Modify a users personal contact entry via a callback
    ///
    /// `id` in this case is the current session user, `user` is the
    /// contact entry they want to modify. If none previously existed,
    /// a fresh one will be created.
    pub(crate) fn modify<F>(&self, id: &Identity, user: &Identity, modify: F) -> QaulResult<()>
    where
        F: Fn(&mut ContactData),
    {
        let mut inner = self.inner.lock().expect("Failed to lock ContactStore");
        let mut contact = inner
            .entry(id.clone())
            .or_insert(Default::default())
            .get_mut(user)
            .unwrap();
        modify(contact);
        Ok(())
    }
}
