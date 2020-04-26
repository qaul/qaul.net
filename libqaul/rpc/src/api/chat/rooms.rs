use super::ChatRpc;
use async_std::sync::Arc;
use async_trait::async_trait;
use libqaul::{
    error::Result,
    helpers::{ItemDiff, ItemDiffExt, SetDiff, SetDiffExt},
    users::UserAuth,
    Identity,
};
use qaul_chat::{Chat, Room, RoomId, RoomMeta};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct List {
    auth: UserAuth,
}

#[async_trait]
impl ChatRpc for List {
    type Response = Result<Vec<RoomMeta>>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.rooms(self.auth).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Get {
    id: RoomId,
    auth: UserAuth,
}

#[async_trait]
impl ChatRpc for Get {
    type Response = Result<Room>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.get_room(self.auth, self.id).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct StartChat {
    auth: UserAuth,
    #[serde(default)]
    users: Vec<Identity>,
}

#[async_trait]
impl ChatRpc for StartChat {
    type Response = Result<RoomId>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.start_chat(self.auth, self.users).await
    }
}
