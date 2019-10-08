//! Service API: user endpoints

use super::models::{QaulError, QaulResult, UserAuth};
use crate::{Qaul, User, UserUpdate};
use identity::Identity;

use argon2;
use base64::{encode_config, URL_SAFE};
use rand::{rngs::OsRng, Rng};

impl Qaul {
    /// Create a new fouser
    ///
    /// Generates a new `Identity` and takes a passphrase that is used to encrypt
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

        Ok(UserAuth::Trusted(id, key))
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
            Some(_) => Err(QaulError::NotAuthorised),
            None => Err(QaulError::NotAuthorised),
        }
    }

    /// Get a list of available users
    pub fn user_get_all(&self) -> Vec<Identity> {
        self.users.lock().unwrap().keys().cloned().collect()
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
        let user = match users.get_mut(&user_id) {
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
            None => {
                return Err(QaulError::UnknownUser);
            }
        };

        // TODO: Use this error somehow
        if !argon2::verify_encoded(hash, pw.as_bytes()).unwrap() {
            return Err(QaulError::NotAuthorised);
        }

        let mut key = [0; 32];
        OsRng.fill(&mut key[..]);
        let key = encode_config(&key, URL_SAFE);
        self.keys.lock().unwrap().insert(key.clone(), id.clone());

        Ok(UserAuth::Trusted(id, key))
    }

    /// End a currently active user session
    pub fn user_logout(&self, user: UserAuth) -> QaulResult<()> {
        let (id, key) = self.user_authenticate(user)?;

        self.keys.lock().unwrap().remove(&key);

        Ok(())
    }
}
