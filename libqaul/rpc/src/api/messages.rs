//! Messages API structures

use async_trait::async_trait;
use crate::QaulRPC;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    stream::Stream,
};
use libqaul::{
    api::Tag,
    messages::{MsgQuery, Mode, MsgId, MsgRef},
    users::UserAuth,
    error::{Error, Result},
    Qaul,
};
use serde::{Serialize, Deserialize};

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
impl QaulRPC for Send {
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
impl QaulRPC for Poll {
    type Response = Result<MsgRef>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.messages()
            .poll(self.auth, self.service)
    }
}

/// Setup a listener/ push handler for messages
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Subscribe {
    auth: UserAuth,
    service: String,
    listener_id: String,
}

#[async_trait]
impl QaulRPC for Subscribe {
    type Response = Result<UnboundedReceiver<MsgRef>>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        let (send, recv) = unbounded(); 
        qaul.messages()
            .listen(self.auth, self.service, move |msg| {
                send.unbounded_send(msg)
                    .map_err(|_| Error::CommFault)
            })
            .map(|_| recv)
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
impl QaulRPC for Query {
    type Response = Result<Vec<MsgRef>>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        qaul.messages()
            .query(self.auth, self.service, self.query)
    }
}
