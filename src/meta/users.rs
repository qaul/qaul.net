//! The library user table
//!
//! Users and scopes in Alexandria are secret.  They have a public Id, which get's opened with

use crate::{
    crypto::{
        aes::{Constructor, Key},
        DetachedKey, Encrypted,
    },
    error::{Error, Result},
    Id,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The hash of an Id which is used for external representation
pub(crate) type Hid = Id;

#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    /// The nested user token
    pub(crate) id: Id,
    // Encrypion key tree
    //pub(crate) keys: KeyTreePair,
}

#[derive(Serialize, Deserialize)]
struct UserWithKey {
    #[serde(skip)]
    key: Option<Key>,
    inner: User,
}

impl DetachedKey<Key> for UserWithKey {
    fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

/// A table of users in the database
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct UserTable(BTreeMap<Hid, Encrypted<UserWithKey, Key>>);

impl UserTable {
    /// Create a new empty user table
    pub(crate) fn new() -> Self {
        Default::default()
    }

    /// Load data from disk
    pub(crate) fn load(data: &[u8]) -> Self {
        unimplemented!()
        // deserialize(data).unwrap()
    }

    /// Add a new user to the user table
    pub(crate) fn add_user(&mut self, id: Id, pw: &str) -> Option<()> {
        if self.0.contains_key(&id) {
            return None;
        }

        let with_key = UserWithKey {
            key: Some(Key::from_pw(pw, &id.to_string())),
            inner: User { id },
        };

        self.0.insert(id, Encrypted::new(with_key));
        Some(())
    }

    /// Unlock a user entry in place
    ///
    /// The provided Id will be hashed, to corresponds to a `Hid`,
    /// which provides a layer of anonymity for users in the database.
    pub(crate) fn open_user(&mut self, id: Id, pw: &str) -> Result<()> {
        let k = Key::from_pw(id.to_string().as_str(), pw);

        // Unlocking happens in-place
        match self.0.get_mut(&id) {
            Some(ref mut e) => e.open(&k),
            None => Err(Error::UnlockFailed { user: id }),
        }
    }

    /// Re-seal the user metadata structure in place
    pub(crate) fn close_user(&mut self, id: Id) -> Result<()> {
        //  Unlocking happens in-place
        match self.0.get_mut(&id) {
            Some(e) => Ok(e.close_detached()?),
            None => Err(Error::UnlockFailed { user: id }),
        }
    }
}

#[test]
fn open_empty() {
    let mut u = UserTable::new();
    assert_eq!(u.open_user(Id::random(), "cool_pw").is_err(), true);
}

#[test]
fn create_clone_open() {
    let mut u = UserTable::new();
    let id = Id::random();
    let pw = "car horse battery staple";
    u.add_user(id, pw).unwrap();
    u.close_user(id).unwrap();
    u.open_user(id, pw).unwrap();
}
