//! Service API: user endpoints

use super::models::{QaulError, QaulResult, UserAuth};
use crate::{
    auth::{AuthStore, PwHash},
    qaul::Qaul,
    users::{User, UserProfile, UserUpdate},
    utils, Identity,
};

impl Qaul {
    /// Create a new fouser
    ///
    /// Generates a new `Identity` and takes a passphrase that is used to encrypt
    pub fn user_create(&self, pw: &str) -> QaulResult<UserAuth> {
        // FIXME: Generate ID from pubkey
        let id = Identity::truncate(&utils::random(16));
        let user = User::Local(UserProfile::new(id.clone()));

        self.users.add_user(user);
        self.auth.set_pw(id.clone(), pw);
        self.auth
            .new_login(id, pw)
            .map(|token| UserAuth::Trusted(id, token))
    }

    /// Get a list of all available local users
    pub fn user_local_users(&self) -> Vec<UserProfile> {
        self.users.get_local()
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
    pub fn user_inject(&self, _user: UserAuth) -> QaulResult<UserAuth> {
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
    pub fn user_update(&self, user: UserAuth, update: UserUpdate) -> QaulResult<UserProfile> {
        // let (user_id, _) = self.user_authenticate(user)?;

        // let mut users = self.users.lock().unwrap();
        // let user = match users.get_mut(&user_id) {
        //     Some(v) => v,
        //     None => {
        //         return Err(QaulError::UnknownUser);
        //     }
        // };

        // update.apply_to(&mut user.data);

        // Ok(user.clone())
        unimplemented!()
    }

    /// Get information for any user
    pub fn user_get(&self, user: UserAuth) -> QaulResult<UserProfile> {
        unimplemented!()
        // let user_id = user.identity();
        // let users = self.users.lock().unwrap();
        // match users.get(&user_id) {
        //     Some(user) => Ok(user.clone()),
        //     None => Err(QaulError::UnknownUser),
        // }
    }

    /// Delete the currently logged-in user
    pub fn user_delete(&self, user: UserAuth) -> QaulResult<()> {
        unimplemented!()
        // let (user_id, _) = self.user_authenticate(user)?;

        // let mut users = self.users.lock().unwrap();
        // if !users.contains_key(&user_id) {
        //     return Err(QaulError::UnknownUser);
        // }
        // users.remove(&user_id);
        // Ok(())
    }

    /// Log-in to an existing user
    pub fn user_login(&self, id: Identity, pw: &str) -> QaulResult<UserAuth> {
        unimplemented!()
        // let token = self.auth.new_login(id, pw)?;
        // Ok(UserAuth::Trusted(id, token))
    }

    /// End a currently active user session
    pub fn user_logout(&self, user: UserAuth) -> QaulResult<()> {
        unimplemented!()
        // let (id, token) = self.user_authenticate(user)?;
        // self.auth.logout(&id, &token)
    }
}
