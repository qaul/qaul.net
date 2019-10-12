//! Store for user profiles

use crate::{Identity, QaulError, QaulResult, UserProfile};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// A small wrapper to express local vs. remote users
pub(crate) enum User {
    Local(UserProfile),
    Remote(UserProfile),
}

/// User store responsible for tracking local and remote users
///
/// Also provides some facilities to create and delete local users,
/// providing persistent state for `Qaul`.
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

    /// Modify a single user inside the store in-place
    pub fn modify<F>(&self, id: &Identity, modifier: F) -> QaulResult<()>
    where
        F: Fn(&mut UserProfile),
    {
        modifier(
            match self
                .inner
                .lock()
                .expect("Failed to lock user store")
                .get_mut(id)
                .map_or(Err(QaulError::UnknownUser), |x| Ok(x))?
            {
                User::Local(ref mut u) => u,
                User::Remote(ref mut u) => u,
            },
        );
        Ok(())
    }

    /// Get all locally available users
    pub fn get_local(&self) -> Vec<UserProfile> {
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
    pub fn get_remote(&self) -> Vec<UserProfile> {
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
