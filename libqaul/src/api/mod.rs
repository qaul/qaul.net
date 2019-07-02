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
pub use models::{Message, QaulError, QaulResult, SigTrust, User, UserAuth};

use crate::Qaul;
pub use identity::Identity;

impl Qaul {
    /// Create a new user
    pub fn user_create(&self) -> QaulResult<UserAuth> {
        unimplemented!()
    }

    /// Update an existing (logged-in) user
    pub fn user_update(&self, user: UserAuth) -> QaulResult<()> {
        Ok(())
    }

    /// Get logged-in user info
    pub fn user_get(&self, user: UserAuth) -> QaulResult<User> {
        unimplemented!()
    }

    /// Delete the currently logged-in user
    pub fn user_delete(&self, user: UserAuth) -> QaulResult<()> {
        Ok(())
    }

    /// Log-in to an existing user
    pub fn user_login(&self, id: Identity) -> QaulResult<UserAuth> {
        unimplemented!()
    }

    /// End a currently active user session
    pub fn user_logout(&self, user: UserAuth) -> QaulResult<()> {
        Ok(())
    }

    /// Add a new contact to a user's known contacts
    pub fn contacts_add(&self, user: UserAuth, id: Identity) -> QaulResult<()> {
        unimplemented!()
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
