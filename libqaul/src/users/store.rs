//! Store for user profiles

use crate::{
    error::{Error, Result},
    qaul::Identity,
    security::KeyId,
    users::UserProfile,
};
use alexandria::{utils::Id, Library};
use ed25519_dalek::Keypair;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// A small wrapper to express local vs. remote users
pub(crate) enum User {
    /// A local user has a full keypair
    Local {
        profile: UserProfile,
        keypair: Arc<Keypair>,
    },
    /// A remote user with optional pubkey
    Remote(UserProfile),
}

impl User {
    #[allow(unused)]
    pub(crate) fn id(&self) -> &Identity {
        match self {
            User::Local {
                ref profile,
                keypair: _,
            } => &profile.id,
            User::Remote(ref u) => &u.id,
        }
    }
}

/// A type wrapper around the alexandria storage library
#[derive(Clone)]
pub(crate) struct UserStore {
    inner: Arc<Library>,
    /// Map the key Ids to word-aligned alexandria Ids used in storage
    map: BTreeMap<Identity, Id>,
}

impl UserStore {
    /// Create a new type abstraction over an existing Alexandria lib
    pub(crate) fn new(inner: Arc<Library>) -> Self {
        Self {
            inner,
            map: Default::default(),
        }
    }

    /// Create a new storage user corresponding to a local user
    pub(crate) async fn create_user(&self, keyid: KeyId, pw: &str) {
        let KeyId { id, keypair } = keyid;
        self.inner.user(id).create(pw).await.unwrap();
    }

    /// Add a new user (local or remote)
    pub(crate) fn add_user(&self, user: User) {
        unimplemented!()
    }

    /// Convenience function around creating a new remote user
    pub(crate) fn discover(&self, id: Identity) {
        unimplemented!()
    }

    pub(crate) fn rm_user(&self, user: Identity) {
        unimplemented!()
    }

    /// Modify a single user inside the store in-place
    pub(crate) fn modify<F>(&self, id: &Identity, modifier: F) -> Result<()>
    where
        F: FnOnce(&mut UserProfile),
    {
        unimplemented!()
    }

    /// Don't call this on non-local users please
    pub(crate) fn get_key(&self, id: Identity) -> Option<Arc<Keypair>> {
        unimplemented!()
    }

    pub(crate) fn get(&self, id: &Identity) -> Result<UserProfile> {
        unimplemented!()
    }

    /// Get all locally available users
    pub(crate) fn get_local(&self) -> Vec<UserProfile> {
        unimplemented!()
    }

    /// Get all remote users this device knows about
    #[allow(unused)]
    pub(crate) fn get_remote(&self) -> Vec<UserProfile> {
        unimplemented!()
    }

    /// Get *all* users this device knows about
    #[allow(unused)]
    pub(crate) fn get_all(&self) -> Vec<UserProfile> {
        unimplemented!()
    }
}
