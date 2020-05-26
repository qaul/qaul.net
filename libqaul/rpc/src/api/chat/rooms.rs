use super::ChatRpc;
use async_std::sync::Arc;
use async_trait::async_trait;
use libqaul::{
    helpers::{ItemDiff, ItemDiffExt, SetDiff, SetDiffExt},
    users::UserAuth,
    Identity,
};
use qaul_chat::{Chat, Result, Room, RoomId, RoomMeta};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct List {
    pub auth: UserAuth,
}

#[async_trait]
impl ChatRpc for List {
    type Response = Result<Vec<RoomMeta>>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.rooms(self.auth).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Get {
    pub id: RoomId,
    pub auth: UserAuth,
}

#[async_trait]
impl ChatRpc for Get {
    type Response = Result<Room>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.get_room(self.auth, self.id).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Create {
    pub auth: UserAuth,
    pub users: Vec<Identity>,
    #[serde(default)]
    pub name: Option<String>,
}

#[async_trait]
impl ChatRpc for Create {
    type Response = Result<RoomId>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.start_chat(self.auth, self.users, self.name).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Modify {
    pub auth: UserAuth,
    #[serde(default]
    pub users: Vec<SetDiff<Identity>>,
    #[serde(default]
    pub name: ItemDiff<String>,
}

#[async_trait]
impl ChatRpc for Modify {
    type Response = Result<Room>;
    async fn apply(self, chat: &Arc<Chat>) -> Self::Response {
        chat.start_chat(self.auth, self.users, self.name).await
    }
}
