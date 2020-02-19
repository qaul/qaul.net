//! Messages API structures

use crate::QaulRpc;
use async_trait::async_trait;
use libqaul::{
    api::{Subscription, Tag},
    error::{Error, Result},
    messages::{Mode, MsgId, MsgQuery, MsgRef},
    users::UserAuth,
    Qaul,
};
use serde::{Deserialize, Serialize};

/// Send a raw payload message
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Send {
    auth: UserAuth,
    mode: Mode,
    service: String,
    #[serde(default)]
    tags: Vec<Tag>,
    payload: Vec<u8>,
}

#[async_trait]
impl QaulRpc for Send {
    type Response = Result<MsgId>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.messages()
            .send(self.auth, self.mode, self.service, self.tags, self.payload)
            .await
    }
}

/// Send a poll request to the message endpoint
///
/// Polling the API for changes might not be the most performant way
/// of getting new messages.  Instead, consider setting up a push
/// listener for your transport layer.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Poll {
    auth: UserAuth,
    service: String,
}

#[async_trait]
impl QaulRpc for Poll {
    type Response = Result<MsgRef>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.messages().poll(self.auth, self.service)
    }
}

/// Setup a listener/ push handler for messages
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Subscribe {
    auth: UserAuth,
    service: String,
    #[serde(default)]
    tags: Vec<Tag>,
}

#[async_trait]
impl QaulRpc for Subscribe {
    type Response = Result<Subscription<MsgRef>>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.messages()
            .subscribe(self.auth, self.service, self.tags)
    }
}

/// Query for a set of messages
///
/// Instead of subscribing to the set of message changes for a
/// service, query the existing set of messages, according to some
/// parameters
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Query {
    auth: UserAuth,
    service: String,
    query: MsgQuery,
}

#[async_trait]
impl QaulRpc for Query {
    type Response = Result<Vec<MsgRef>>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.messages().query(self.auth, self.service, self.query)
    }
}
