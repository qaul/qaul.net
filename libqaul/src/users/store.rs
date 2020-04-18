//! Store for user profiles

use crate::{
    error::{Error, Result},
    qaul::Identity,
    security::KeyId,
    store::LocalUser,
    users::UserProfile,
};
use alexandria::{
    utils::{Id, Path, TagSet},
    Library,
};
use ed25519_dalek::Keypair;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

const KEY_PATH: &'static str = "/meta:keys";

fn profile_path(id: Id) -> Path {
    Path::from(format!("/users:{}", id))
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

    /// Create a new local user
    pub(crate) async fn create_local(&self, keyid: KeyId, pw: &str) {
        let KeyId { id, keypair } = keyid;
        self.inner.user(id).create(pw).await.unwrap();
        let local = LocalUser::new(id, keypair);

        // Store the key
        self.inner
            .data(id)
            .await
            .unwrap()
            .insert(Path::from(KEY_PATH), TagSet::empty(), local.meta_diff())
            .await
            .unwrap();

        // Then insert the user profile
        self.inner
            .data(None)
            .await
            .unwrap()
            .batch(profile_path(id), TagSet::empty(), local.profile.init_diff())
            .await
            .unwrap();
    }

    /// Add a newly discovered remote user
    pub(crate) fn discover(&self, id: Identity) {
        unimplemented!()
    }

    pub(crate) fn delete(&self, user: Identity) {
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
    pub(crate) fn all_local(&self) -> Vec<UserProfile> {
        unimplemented!()
    }

    /// Get all remote users this device knows about
    #[allow(unused)]
    pub(crate) fn all_remote(&self) -> Vec<UserProfile> {
        unimplemented!()
    }

    /// Get *all* users this device knows about
    #[allow(unused)]
    pub(crate) fn all(&self) -> Vec<UserProfile> {
        unimplemented!()
    }
}
