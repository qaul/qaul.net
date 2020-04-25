//! Users API structures

use crate::QaulRpc;
use async_trait::async_trait;
use futures::future;
use libqaul::{
    helpers::{ItemDiff, ItemDiffExt, MapDiff, MapDiffExt, SetDiff, SetDiffExt},
    error::Result,
    users::{UserAuth, UserProfile, UserUpdate},
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
        qaul.users().list().await
    }
}

/// Enumerate all publicly known locally stored users
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ListLocal {}

#[async_trait]
impl QaulRpc for ListLocal {
    type Response = Vec<UserProfile>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().list_local().await
    }
}

/// Enumerate all publicly known remote users
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ListRemote {}

#[async_trait]
impl QaulRpc for ListRemote {
    type Response = Vec<UserProfile>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().list_remote().await
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
        qaul.users().login(self.user, &self.pw).await
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
        qaul.users().logout(self.auth).await
    }
}

/// Get the user profile for any remote or local user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Get {
    id: Identity,
}

#[async_trait]
impl QaulRpc for Get {
    type Response = Result<UserProfile>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.users().get(self.id).await
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
        let mut changes = vec![];

        match display_name {
            ItemDiff::Ignore => {}
            ItemDiff::Set(name) => changes.push(UserUpdate::DisplayName(Some(name))),
            ItemDiff::Unset => changes.push(UserUpdate::DisplayName(None)),
        }

        match real_name {
            ItemDiff::Ignore => {}
            ItemDiff::Set(name) => changes.push(UserUpdate::RealName(Some(name))),
            ItemDiff::Unset => changes.push(UserUpdate::RealName(None)),
        }

        bio.into_iter().for_each(|bio| match bio {
            MapDiff::Ignore => {}
            MapDiff::Add { key, value } => changes.push(UserUpdate::SetBioLine(key, value)),
            MapDiff::Remove(key) => changes.push(UserUpdate::RemoveBioLine(key)),
        });

        services.into_iter().for_each(|serv| match serv {
            SetDiff::Ignore => {}
            SetDiff::Add(val) => changes.push(UserUpdate::AddService(val)),
            SetDiff::Remove(val) => changes.push(UserUpdate::RemoveService(val)),
        });

        let users = qaul.users();
        future::join_all(changes.into_iter().fold(vec![], |mut vec, u| {
            vec.push(users.update(auth.clone(), u));
            vec
        }))
        .await
        .into_iter()
        .fold(Ok(()), |prev, res| prev.and_then(|_| res))
    }
}
