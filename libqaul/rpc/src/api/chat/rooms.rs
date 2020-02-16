use {
    super::ChatRPC,
    async_trait::async_trait,
    libqaul::{
        api::{
            ItemDiff, ItemDiffExt, 
            SetDiff, SetDiffExt,
        },
        error::Result,
        users::UserAuth,
        Identity,
    },
    qaul_chat::{
        room::{RoomId, Room},
        Chat,
    },
    serde::{Serialize, Deserialize},
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct List;

#[async_trait]
impl ChatRPC for List {
    type Response = Vec<RoomId>; 
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.rooms().list().await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Get {
    id: RoomId,
}

#[async_trait]
impl ChatRPC for Get {
    type Response = Room; 
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.rooms().get(self.id).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Create {
    auth: UserAuth,
    #[serde(default)]
    users: Vec<Identity>,
}

#[async_trait]
impl ChatRPC for Create {
    type Response = Result<RoomId>; 
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.rooms().create(self.auth, self.users).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Modify {
    auth: UserAuth,
    id: RoomId,
    #[serde(default)]
    users: Vec<SetDiff<Identity>>,
    #[serde(default)]
    name: ItemDiff<String>,
}

#[async_trait]
impl ChatRPC for Modify {
    type Response = Result<()>; 
    async fn apply(self, chat: &Chat) -> Self::Response {
        let Modify { auth, id, users, name } = self;
        chat.rooms()
            .modify(auth, id, move |room| {
                name.apply(&mut room.name);
                room.users.apply(users);
                Ok(())
            })
            .await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Delete {
    auth: UserAuth,
    id: RoomId,
}

#[async_trait]
impl ChatRPC for Delete {
    type Response = Result<()>;
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.rooms().delete(self.auth, self.id).await
    }
}
