use {
    super::ChatRpc,
    async_trait::async_trait,
    futures::{future::FutureExt, stream::Stream},
    libqaul::{error::Result, users::UserAuth},
    qaul_chat::{room::RoomId, Chat, ChatMessage},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Next {
    pub auth: UserAuth,
    pub room: RoomId,
}

#[async_trait]
impl ChatRpc for Next {
    type Response = ChatMessage;
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.next(self.auth, self.room).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Subscribe {
    pub auth: UserAuth,
    pub room: RoomId,
}

//#[async_trait]
//impl ChatRPC for Subscribe {
//    type Response = Result<impl Stream<Item = ChatMessage> + Unpin>;
//    async fn apply(self, chat: &Chat) -> Self::Response {
//        chat.subscribe(self.user, self.room)
//            .await
//    }
//}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Send {
    pub auth: UserAuth,
    pub room: RoomId,
    pub text: String,
}

#[async_trait]
impl ChatRpc for Send {
    type Response = Result<()>;
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.send(self.auth, self.room, self.text).await
    }
}
