//! # `libqaul` service API
//!
//! The idea behind this interface is further
//! documented in the `contribute` book. It goes
//! into detail about using it to write decentralised
//! networking services, using qaul.net as a backend.
//!
//! `qaul.net` itself provides a few primary services
//! for "messaging", "file sharing" and "VoIP",
//! as well as a sort of hidden, management "core"
//! service.
//! All of them are implemented via this API,
//! allowing external developers to write their own
//! services using qaul.net libraries and networks.
//!
//! ## Models
//!
//! Models defined in this submodule are different
//! from any other models defined in `libqaul`:
//! they are the public representations, i.e.
//! only fields that are relevant for service
//! developers to interact with, not including
//! shared service state or secrets.

use std::collections::BTreeMap;

mod models;
mod service;
pub use models::{Message, QaulError, QaulResult, SigTrust, UserAuth};

use crate::Qaul;
use crate::User;
pub use crate::{ContactBook, LocalContactData, UserData, UserUpdate};
use identity::Identity;

use argon2;
use base64::{encode_config, URL_SAFE};
use rand::{Rng, rngs::OsRng};

impl Qaul {
    /// Create a new user
    ///
    /// Generates a new `Identity` and takes a passphrase that is used
    /// to encrypt
    pub fn user_create(&self, pw: &str) -> QaulResult<UserAuth> {
        let user = User::new();
        let id = user.id.clone();
        let mut users = self.users.lock().unwrap();
        users.insert(id.clone(), user);

        let mut salt = [0; 16];
        OsRng.fill(&mut salt[..]); 
        // TODO: Use this error somehow
        let hash = argon2::hash_encoded(pw.as_bytes(), &salt, &argon2::Config::default()).unwrap();
        self.auth.lock().unwrap().insert(id.clone(), hash);

        let mut key = [0; 32];
        OsRng.fill(&mut key[..]);
        let key = encode_config(&key, URL_SAFE); 
        self.keys.lock().unwrap().insert(key.clone(), id.clone());

        Ok(UserAuth::Trusted(
            id,
            key,
        ))
    }

    /// Checks if a `UserAuth` is valid
    ///
    /// This means:
    /// - `id` points to a real user
    /// - `key` is a valid key for that user
    pub fn user_authenticate(&self, user: UserAuth) -> QaulResult<(Identity, String)> {
        let (user_id, key) = user.trusted()?;

        match self.keys.lock().unwrap().get(&key) {
            Some(id) if *id == user_id => Ok((user_id, key)),
            Some(id) => Err(QaulError::NotAuthorised),
            None => Err(QaulError::NotAuthorised),
        }

    }

    /// Inject a `UserAuth` into this `Qaul`.
    /// This is not, in general, a sensible thing for regular applications to do, but is
    /// necessary for testing.
    ///
    /// # Panics
    /// Panics if the provided `UserAuth` describes a user that is already known to this
    /// `Qaul` instance.
    /// Panics if the provided `UserAuth` users a key that is already known to this
    /// `Qaul` instance.
    pub fn user_inject(&self, user: UserAuth) -> QaulResult<UserAuth> {
        let (id, key) = user.trusted()?;
        let mut user = User::new();
        user.id = id;

        let mut users = self.users.lock().unwrap();
        if users.contains_key(&id) {
            panic!("The user {:?} already exists within the Qaul state.", id);
        }

        let mut keys = self.keys.lock().unwrap();
        if keys.contains_key(&key) {
            panic!("The key {:?} already exists within the Qaul state.", key);
        }

        users.insert(id.clone(), user);
        keys.insert(key.clone(), id.clone());
        Ok(UserAuth::Trusted(id, key))
    }

    /// Update an existing (logged-in) user to use the given details.
    pub fn user_update(&self, user: UserAuth, update: UserUpdate) -> QaulResult<User> {
        let (user_id, _) = self.user_authenticate(user)?;

        let mut users = self.users.lock().unwrap();
        let mut user = match users.get_mut(&user_id) {
            Some(v) => v,
            None => {
                return Err(QaulError::UnknownUser);
            }
        };

        update.apply_to(&mut user.data);

        Ok(user.clone())
    }

    /// Get information for any user
    pub fn user_get(&self, user: UserAuth) -> QaulResult<User> {
        let user_id = user.identity();
        let users = self.users.lock().unwrap();
        match users.get(&user_id) {
            Some(user) => Ok(user.clone()),
            None => Err(QaulError::UnknownUser),
        }
    }

    /// Delete the currently logged-in user
    pub fn user_delete(&self, user: UserAuth) -> QaulResult<()> {
        let (user_id, _) = self.user_authenticate(user)?;

        let mut users = self.users.lock().unwrap();
        if !users.contains_key(&user_id) {
            return Err(QaulError::UnknownUser);
        }
        users.remove(&user_id);
        Ok(())
    }

    /// Log-in to an existing user
    pub fn user_login(&self, id: Identity, pw: &str) -> QaulResult<UserAuth> {
        let auth = self.auth.lock().unwrap();
        let hash = match auth.get(&id) {
            Some(hash) => hash,
            None => { return Err(QaulError::UnknownUser); },
        };

        // TODO: Use this error somehow
        if !argon2::verify_encoded(hash, pw.as_bytes()).unwrap() {
            return Err(QaulError::NotAuthorised);
        }

        let mut key = [0; 32];
        OsRng.fill(&mut key[..]);
        let key = encode_config(&key, URL_SAFE); 
        self.keys.lock().unwrap().insert(key.clone(), id.clone());

        Ok(UserAuth::Trusted(
            id,
            key,
        ))
    }

    /// End a currently active user session
    pub fn user_logout(&self, user: UserAuth) -> QaulResult<()> {
        let (id, key) = self.user_authenticate(user)?;

        self.keys.lock().unwrap().remove(&key);

        Ok(())
    }

    /// Add a new contact to a user's known contacts
    pub fn contacts_add(&self, user: UserAuth, contact: User) -> QaulResult<()> {
        let (my_id, _) = user.trusted()?;
        let contact_id = contact.id.clone();

        // If the user isn't know to this libqaul instance, add it there first
        {
            let mut users = self.users.lock().expect("Users lock poisioned. Error");
            users.insert(contact_id.clone(), contact);
        }

        // Now, add it to the user's contact book.
        {
            let my_id = my_id.clone();
            let mut contacts = self.contacts.lock().expect("Contacts lock poisoned. Error");
            let mut contacts_book = contacts.entry(my_id).or_insert(ContactBook::new());
            contacts_book.insert(contact_id, LocalContactData::default());
        }

        Ok(())
    }

    /// Find a subset of contacts with some query
    pub fn contacts_find(&self, user: UserAuth, query: String) -> QaulResult<Vec<User>> {
        unimplemented!()
    }

    /// Enumerate all contacts known by a user
    pub fn contacts_get_all(&self, user: UserAuth) -> QaulResult<Vec<User>> {
        unimplemented!()
    }

    /// Send a message to another user
    pub fn message_send(
        &self,
        user: UserAuth,
        recipient: Identity,
        payload: Vec<u8>,
    ) -> QaulResult<()> {
        unimplemented!()
    }

    pub fn message_poll(&self, user: UserAuth) -> QaulResult<Vec<Message>> {
        unimplemented!()
    }

    /// Register a new service with this `qaul` instance
    ///
    /// Internally this function dispatches a query to a UI service
    /// (marked "primary" to allow the user to either verify or
    /// deny the registration request).
    pub fn service_register(&self, user: UserAuth, service_id: String) -> QaulResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn user_lifecycle() {
        let qaul = Qaul::start();

        // can we make a user?
        let auth = qaul.user_create("password").unwrap();

        // does something get added to the correct parts of state?
        {
            assert_eq!(qaul.auth.lock().unwrap().len(), 1);
            assert_eq!(qaul.keys.lock().unwrap().len(), 1);
            assert_eq!(qaul.users.lock().unwrap().len(), 1);
        }

        // are we trusted?
        let (id, key) = qaul.user_authenticate(auth.clone()).unwrap();

        // can we login as that user?
        let auth2 = qaul.user_login(id.clone(), "password").unwrap();

        // is the state updated appropriately?
        {
            assert_eq!(qaul.auth.lock().unwrap().len(), 1);
            assert_eq!(qaul.keys.lock().unwrap().len(), 2);
            assert_eq!(qaul.users.lock().unwrap().len(), 1);
        }

        // are we trusted?
        let (id2, key2) = qaul.user_authenticate(auth2.clone()).unwrap();

        // do we get back the same id?
        assert_eq!(id.clone(), id2);
        // do we get a different key?
        assert_ne!(key.clone(), key2);

        // can we log out?
        qaul.user_logout(auth2.clone());

        // is the state updated appropriately
        {
            assert_eq!(qaul.auth.lock().unwrap().len(), 1);
            assert_eq!(qaul.keys.lock().unwrap().len(), 1);
            assert_eq!(qaul.users.lock().unwrap().len(), 1);
        }

        // are we not trusted now?
        assert!(qaul.user_authenticate(auth2).is_err());

        // does logging in with the wrong password actually error?
        assert!(qaul.user_login(id.clone(), "not password").is_err());
    }
}
