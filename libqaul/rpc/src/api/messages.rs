//! Messages API structures

use crate::QaulRpc;
use async_trait::async_trait;
use libqaul::{
    error::{Error, Result},
    helpers::{Subscription, Tag},
    messages::{IdType, Mode, MsgId, MsgQuery, MsgRef},
    users::UserAuth,
    Qaul,
};
use serde::{Deserialize, Serialize};

/// Send a raw payload message
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Send {
    auth: UserAuth,
    mode: Mode,
    id_type: IdType,
    service: String,
    #[serde(default)]
    tags: Vec<Tag>,
    payload: Vec<u8>,
}

#[async_trait]
impl QaulRpc for Send {
    type Response = Result<MsgId>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        let Self {
            auth,
            mode,
            id_type,
            service,
            tags,
            payload,
        } = self;

        qaul.messages()
            .send(auth, mode, id_type, service, tags, payload)
            .await
    }
}
