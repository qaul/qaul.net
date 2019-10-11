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

        // Computes and stores the pw hash
        self.auth.set_pw(id.clone(), pw);

        // Then generate a token
        let token = self.auth.new_login(id.clone(), pw)?;
        Ok(UserAuth::Trusted(id, token))
    }

    /// Checks if a `UserAuth` is valid
    ///
    /// This means:
    /// - `id` points to a real user
    /// - `key` is a valid key for that user
    pub fn user_authenticate(&self, user: UserAuth) -> QaulResult<(Identity, String)> {
        let (user_id, token) = user.trusted()?;
        self.auth.verify_token(&user_id, &token)?;
        Ok((user_id, token))
    }

    /// Get a list of available users
    pub fn user_get_all(&self) -> Vec<Identity> {
        self.users.lock().unwrap().keys().cloned().collect()
    }


    /// Inject a `UserAuth` into this `Qaul`.
    ///
    /// This is not, in general, a sensible thing for regular
    /// applications to do, but is necessary for testing.
    ///
    /// ## Note
    ///
    /// In it's current form, this function can not be implemented
    /// with the new `AuthStore` backend, because it doesn't map
    /// tokens to users, but the other way around. The code is still
    /// in the repo, albeit commented out. We should check what this
    /// function should actually do and if it can be aproximated
    /// better.
    ///
    /// # Panics
    /// Panics if the provided `UserAuth` describes a user that is already known to this
    /// `Qaul` instance.
    /// Panics if the provided `UserAuth` users a key that is already known to this
    /// `Qaul` instance.
    pub fn user_inject(&self, user: UserAuth) -> QaulResult<UserAuth> {
        // let (id, key) = user.trusted()?;
        // let mut user = User::new();
        // user.id = id;

        // let mut users = self.users.lock().unwrap();
        // if users.contains_key(&id) {
        //     panic!("The user {:?} already exists within the Qaul state.", id);
        // }

        // let mut keys = self.keys.lock().unwrap();
        // if keys.contains_key(&key) {
        //     panic!("The key {:?} already exists within the Qaul state.", key);
        // }

        // users.insert(id.clone(), user);
        // keys.insert(key.clone(), id.clone());
        // Ok(UserAuth::Trusted(id, key))
        unimplemented!()
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
        let token = self.auth.new_login(id, pw)?;
        Ok(UserAuth::Trusted(id, token))
    }

    /// End a currently active user session
    pub fn user_logout(&self, user: UserAuth) -> QaulResult<()> {
        let (id, token) = self.user_authenticate(user)?;
        self.auth.logout(&id, &token)
    }
}
