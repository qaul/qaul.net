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

mod models;
mod service;
pub use models::{Message, QaulError, QaulResult, SigTrust, UserAuth};

use crate::users::UserData;
use crate::Qaul;
use crate::User;
use identity::Identity;

impl Qaul {
    /// Create a new user
    ///
    /// Generates a new `Identity` and takes a passphrase that is used
    /// to encrypt
    pub fn user_create(&self, _pw: &str) -> QaulResult<UserAuth> {
        let user = User::new();
        let id = user.id.clone();
        let mut users = self.users.lock().unwrap();
        users.insert(id.clone(), user);

        Ok(UserAuth::Trusted(
            id,
            unimplemented!("Key material generation"),
        ))
    }

    /// Inject a `UserAuth` into this `Qaul`.
    /// This is not, in general, a sensible thing for regular applications to do, but is
    /// necessary for testing.
    ///
    /// # Panics
    /// Panics if the provided `UserAuth` describes a user that is already known to this
    /// `Qaul` instance.
    pub fn user_inject(&self, user: UserAuth) -> QaulResult<UserAuth> {
        let (id, passphrase) = user.trusted()?;
        let mut user = User::new();
        user.id = id;

        let mut users = self.users.lock().unwrap();
        if users.contains_key(&id) {
            panic!("The user {:?} already exists within the Qaul state.", id);
        }

        users.insert(id.clone(), user);
        Ok(UserAuth::Trusted(id, passphrase))
    }

    /// Update an existing (logged-in) user to use the given details.
    pub fn user_update(&self, user: UserAuth, data: UserData) -> QaulResult<()> {
        let (user_id, _) = user.trusted()?;
        let mut users = self.users.lock().unwrap();
        let mut user = match users.get_mut(&user_id) {
            Some(v) => v,
            None => {
                return Err(QaulError::UnknownUser);
            }
        };

        user.data = data;

        Ok(())
    }

    /// Get logged-in user info
    pub fn user_get(&self, user: UserAuth) -> QaulResult<User> {
        let (user_id, _) = user.trusted()?;
        let users = self.users.lock().unwrap();
        match users.get(&user_id) {
            Some(user) => Ok(user.clone()),
            None => Err(QaulError::UnknownUser),
        }
    }

    /// Delete the currently logged-in user
    pub fn user_delete(&self, user: UserAuth) -> QaulResult<()> {
        let (user_id, _) = user.trusted()?;
        let mut users = self.users.lock().unwrap();
        if !users.contains_key(&user_id) {
            return Err(QaulError::UnknownUser);
        }
        users.remove(&user_id);
        Ok(())
    }

    /// Log-in to an existing user
    pub fn user_login(&self, id: Identity, pw: &str) -> QaulResult<UserAuth> {
        unimplemented!()
    }

    /// End a currently active user session
    pub fn user_logout(&self, user: UserAuth) -> QaulResult<()> {
        Ok(())
    }

    /// Add a new contact to a user's known contacts
    pub fn contacts_add(&self, user: UserAuth, contact: User) -> QaulResult<()> {
        let (my_id, _) = user.trusted()?;
        let mut users = self.users.lock().unwrap();
        let contact_id = contact.id.clone();
        users.insert(contact_id, contact);
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
