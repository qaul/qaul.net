//! Store for user profiles

use crate::error::{Error, Result};
use crate::qaul::Identity;
use crate::users::UserProfile;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// A small wrapper to express local vs. remote users
pub(crate) enum User {
    Local(UserProfile),
    #[allow(unused)]
    Remote(UserProfile),
}

impl User {
    #[allow(unused)]
    pub(crate) fn id(&self) -> &Identity {
        match self {
            User::Local(ref u) => &u.id,
            User::Remote(ref u) => &u.id,
        }
    }
}

/// User store responsible for tracking local and remote users
///
/// Also provides some facilities to create and delete local users,
/// providing persistent state for `Qaul`.
#[derive(Clone)]
pub(crate) struct UserStore {
    inner: Arc<Mutex<BTreeMap<Identity, User>>>,
}

impl UserStore {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    /// Add a new user (local or remote)
    pub(crate) fn add_user(&self, user: User) {
        let id = match user {
            User::Local(ref u) => u.id,
            User::Remote(ref u) => u.id,
        }
        .clone();

        let mut inner = self.inner.lock().expect("Failed to lock UserStore");
        inner.insert(id, user);
    }

    pub(crate) fn rm_user(&self, user: Identity) {
        self.inner.lock().unwrap().remove(&user);
    }

    /// Modify a single user inside the store in-place
    pub(crate) fn modify<F>(&self, id: &Identity, modifier: F) -> Result<()>
    where
        F: FnOnce(&mut UserProfile),
    {
        modifier(
            match self
                .inner
                .lock()
                .expect("Failed to lock user store")
                .get_mut(id)
                .map_or(Err(Error::NoUser), |x| Ok(x))?
            {
                User::Local(ref mut u) => u,
                User::Remote(ref mut u) => u,
            },
        );
        Ok(())
    }

    pub(crate) fn get(&self, id: &Identity) -> Result<UserProfile> {
        self.inner
            .lock()
            .expect("Failed to lock UserStore")
            .get(id)
            .map_or(Err(Error::NoUser), |x| {
                Ok(match x {
                    User::Local(ref u) => u,
                    User::Remote(ref u) => u,
                }
                .clone())
            })
    }

    /// Get all locally available users
    pub(crate) fn get_local(&self) -> Vec<UserProfile> {
        self.inner
            .lock()
            .expect("Failed to lock UserStore")
            .iter()
            .filter_map(|(_, u)| match u {
                User::Local(u) => Some(u.clone()),
                _ => None,
            })
            .collect()
    }

    /// Get all remote users this device knows about
    #[allow(unused)]
    pub(crate) fn get_remote(&self) -> Vec<UserProfile> {
        self.inner
            .lock()
            .expect("Failed to lock UserStore")
            .iter()
            .filter_map(|(_, u)| match u {
                User::Remote(u) => Some(u.clone()),
                _ => None,
            })
            .collect()
    }
}
