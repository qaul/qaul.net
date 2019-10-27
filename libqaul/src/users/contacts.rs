//! Contact books for users, storing the users they know about.
//!
//! Users' contact books exist as:
//!
//! - lists of users they know about, by identity, plus
//! - local-only information about those users, like personal nicknames

use crate::error::{Error, Result};
use crate::qaul::Identity;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Wraps around user-local contact books
#[derive(Clone, Debug, Default)]
pub(crate) struct ContactStore {
    inner: Arc<Mutex<BTreeMap<Identity, ContactList>>>,
}

/// A collection of contacts associated with their local-only data.
pub(crate) type ContactList = BTreeMap<Identity, ContactData>;

/// Query structure to find contacts by
pub enum ContactQuery<'a> {
    /// A fuzzy nickname search
    Nick(&'a str),
    /// A fuzzy trust level search
    Trust { val: i8, fuz: i8 },
}

/// Data about a contact that is relevant only from a single user's perspective.
#[derive(Default, Debug)]
pub struct ContactData {
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
    pub(crate) fn modify<F>(&self, id: &Identity, user: &Identity, modify: F) -> Result<()>
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

    pub(crate) fn query(&self, id: &Identity, query: ContactQuery) -> Result<Vec<Identity>> {
        let mut inner = self.inner.lock().expect("Failed to lock ContactStore");
        Ok(inner
            .get(id)
            .map_or(Err(Error::UnknownUser), |x| Ok(x))?
            .iter()
            .filter(|(_, con)| match query {
                ContactQuery::Nick(nick) if con.nick.is_none() => false,
                ContactQuery::Nick(nick) => con.nick.as_ref().unwrap().contains(nick),
                ContactQuery::Trust { val, fuz } => con.trust + fuz < val || con.trust - fuz > val,
            })
            .map(|(id, _)| id.clone())
            .collect())
    }

    pub(crate) fn get_all(&self, id: &Identity) -> Result<Vec<Identity>> {
        let mut inner = self.inner.lock().expect("Failed to lock ContactStore");
        Ok(inner
            .get(id)
            .map_or(Err(Error::UnknownUser), |x| Ok(x))?
            .iter()
            .map(|(id, _)| id.clone())
            .collect())
    }
}
