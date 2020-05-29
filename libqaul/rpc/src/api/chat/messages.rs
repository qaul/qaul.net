use super::ChatRpc;
use crate::{api::Subscriber, Response, SubId};
use async_std::sync::Arc;
use async_trait::async_trait;
use futures::{future::FutureExt, stream::Stream};
use libqaul::users::UserAuth;
use qaul_chat::{Chat, ChatMessage, Result, RoomId, Subscription};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Subscribe {
    pub auth: UserAuth,
    pub room: RoomId,
}

#[async_trait]
impl ChatRpc for Subscribe {
    type Response = Result<Subscription>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.subscribe(self.auth, self.room).await
    }
}

#[async_trait]
impl Subscriber for Subscription {
    async fn next(&self) -> Option<Response> {
        Some(self.next().await.into())
    }
}

pub struct CancelSub {
    pub auth: UserAuth,
    pub id: SubId,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Get {
    pub auth: UserAuth,
    #[serde(rename = "id")]
    pub room: RoomId,
}

#[async_trait]
impl ChatRpc for Get {
    type Response = Result<Vec<ChatMessage>>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.load_messages(self.auth, self.room).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Create {
    pub auth: UserAuth,
    pub room: RoomId,
    pub text: String,
}

#[async_trait]
impl ChatRpc for Create {
    type Response = Result<ChatMessage>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.send_message(self.auth, self.room, self.text).await
    }
}
