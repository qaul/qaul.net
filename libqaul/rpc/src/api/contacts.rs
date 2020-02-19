//! Contacts API structures

use async_trait::async_trait;
use crate::QaulRpc;
use libqaul::{
    api::{ItemDiff, ItemDiffExt}, 
    error::Result,
    contacts::{ContactQuery, ContactEntry}, 
    users::UserAuth, 
    Identity, Qaul, 
};
use serde::{Serialize, Deserialize};

/// Apply a modification to a contact entry
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Modify {
    auth: UserAuth,
    contact: Identity,
    #[serde(default)]
    nick: ItemDiff<String>,
    #[serde(default)]
    trust: Option<i8>,
    #[serde(default)]
    met: Option<bool>,
    #[serde(default)]
    location: ItemDiff<String>,
    #[serde(default)]
    notes: ItemDiff<String>,
}

#[async_trait]
impl QaulRpc for Modify {
    type Response = Result<()>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        let Modify { 
            auth, contact, nick, trust,
            met, location, notes } = self;
        qaul.contacts()
            .modify(auth, &contact, move |contact| {
                nick.apply(&mut contact.nick);
                if let Some(trust) = trust { contact.trust = trust; }
                if let Some(met) = met { contact.met = met; }
                location.apply(&mut contact.location);
                notes.apply(&mut contact.notes);
            })
    }
}

/// Get the contact entry for an identity
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Get {
    auth: UserAuth,
    contact: Identity,
}

#[async_trait]
impl QaulRpc for Get {
    type Response = Result<ContactEntry>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.contacts()
            .get(self.auth, &self.contact)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {
    auth: UserAuth,
    query: ContactQuery,
}

#[async_trait]
impl QaulRpc for Query {
    type Response = Result<Vec<Identity>>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.contacts()
            .query(self.auth, self.query)
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct All {
    auth: UserAuth
}

#[async_trait]
impl QaulRpc for All {
    type Response = Result<Vec<Identity>>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.contacts()
            .all(self.auth)
    }
}
