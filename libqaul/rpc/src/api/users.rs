//! Users API structures

use crate::QaulRpc;
use async_trait::async_trait;
use libqaul::{
    api::{ItemDiff, ItemDiffExt, MapDiff, MapDiffExt, SetDiff, SetDiffExt},
    error::Result,
    users::{UserAuth, UserProfile},
    Identity, Qaul,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Enumerate all publicly known users
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct List {}

#[async_trait]
impl QaulRpc for List {
    type Response = Vec<UserProfile>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().list()
    }
}

/// Enumerate all publicly known locally stored users
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ListLocal {}

#[async_trait]
impl QaulRpc for ListLocal {
    type Response = Vec<UserProfile>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().list_local()
    }
}
/// Enumerate all publicly known remote users
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ListRemote {}

#[async_trait]
impl QaulRpc for ListRemote {
    type Response = Vec<UserProfile>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().list_remote()
    }
}

/// Create a new user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Create {
    pw: String,
}

#[async_trait]
impl QaulRpc for Create {
    type Response = Result<UserAuth>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().create(&self.pw).await
    }
}

/// Delete a user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Delete {
    auth: UserAuth,
    /// Indicate whether local data should be deleted as well
    purge: bool,
}

#[async_trait]
impl QaulRpc for Delete {
    type Response = Result<()>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().delete(self.auth).await
        // TODO: Purge user if requestd
    }
}

/// Change the password on a user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ChangePw {
    auth: UserAuth,
    new: String,
}

#[async_trait]
impl QaulRpc for ChangePw {
    type Response = Result<()>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().change_pw(self.auth, &self.new)
    }
}

/// Create a new session for a user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Login {
    user: Identity,
    pw: String,
}

#[async_trait]
impl QaulRpc for Login {
    type Response = Result<UserAuth>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().login(self.user, &self.pw)
    }
}

/// Stop an existing session
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Logout {
    auth: UserAuth,
}

#[async_trait]
impl QaulRpc for Logout {
    type Response = Result<()>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().logout(self.auth)
    }
}

/// Get the user profile for any remote or local user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Get {
    user: Identity,
}

#[async_trait]
impl QaulRpc for Get {
    type Response = Result<UserProfile>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().get(self.user)
    }
}

/// Apply an update to your user profile
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Update {
    auth: UserAuth,
    #[serde(default)]
    display_name: ItemDiff<String>,
    #[serde(default)]
    real_name: ItemDiff<String>,
    #[serde(default)]
    bio: Vec<MapDiff<String, String>>,
    #[serde(default)]
    services: Vec<SetDiff<String>>,
    #[serde(default)]
    avatar: ItemDiff<Vec<u8>>,
}

#[async_trait]
impl QaulRpc for Update {
    type Response = Result<()>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        let Update {
            auth,
            display_name,
            real_name,
            bio,
            services,
            avatar,
        } = self;
        qaul.users().update(auth, move |profile| {
            display_name.apply(&mut profile.display_name);
            real_name.apply(&mut profile.real_name);
            profile.bio.apply(bio);
            profile.services.apply(services);
            avatar.apply(&mut profile.avatar);
        })
    }
}
