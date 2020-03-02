//! The library user table
//!
//! Users and scopes in Alexandria are secret.  They have a public Id, which get's opened with

use crate::{
    crypto::{
        aes::{self, Crypto},
        Encrypted,
    },
    Id,
};
use bincode::deserialize;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, mem::swap};

/// The hash of an Id which is used for external representation
pub(crate) type Hid = Id;

#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    /// The nested user token
    pub(crate) id: Id,
    // Encrypion key tree
    //pub(crate) keys: KeyTreePair,
}

/// An entry in the user table
#[derive(Serialize, Deserialize)]
pub(crate) enum Entry {
    /// A decrypted user metadata entry
    Open(User, Encrypted),
    /// An encrypted entry
    Closed(Encrypted),
}

/// A table of users in the database
#[derive(Default, Serialize, Deserialize)]
pub(crate) struct UserTable(BTreeMap<Hid, Entry>);

impl UserTable {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    /// Unlock a user entry in place
    ///
    /// The provided Id will be hashed, to corresponds to a `Hid`,
    /// which provides a layer of anonymity for users in the database.
    pub(crate) fn open_user(&mut self, id: Id, pw: &str) -> Option<()> {
        let k = aes::key_from_pw(id.to_string().as_str(), pw);

        let mut new = {
            let enc = match self.0.get_mut(&id) {
                Some(Entry::Closed(enc)) => enc,
                _ => return None,
            };

            let dec = enc.decrypt(&k);
            Entry::Open(deserialize(&dec).unwrap(), enc.clone())
        };

        swap(self.0.get_mut(&id).unwrap(), &mut new);
        Some(())
    }

    /// Re-seal the user metadata structure in place
    pub(crate) fn close_user(&mut self, id: Id) -> Option <()> {
        let mut enc = Entry::Closed(match self.0.get_mut(&id) {
            Some(Entry::Open(_, enc)) => enc.clone(),
            _ => return None,
        });

        swap(self.0.get_mut(&id).unwrap(), &mut enc);
        Some(())
    }
}

#[test]
fn oof() {
    let mut u = UserTable::new();
    u.open_user(Id::random(), "foobar");
}
