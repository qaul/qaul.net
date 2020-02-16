use {
    async_trait::async_trait,
    super::ChatRPC,
    futures::{
        future::FutureExt,
        stream::Stream,
    },
    libqaul::{
        error::Result,
        users::UserAuth,
    },
    qaul_chat::{
        room::RoomId,
        Chat, ChatMessage,
    },
    serde::{Serialize, Deserialize},
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Next {
    auth: UserAuth,
    room: RoomId,
}

#[async_trait]
impl ChatRPC for Next {
    type Response = ChatMessage;
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.next(self.auth, self.room).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Subscribe {
    auth: UserAuth,
    room: RoomId,
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
    auth: UserAuth,
    room: RoomId,
    text: String,
}

#[async_trait]
impl ChatRPC for Send {
    type Response = Result<()>;
    async fn apply(self, chat: &Chat) -> Self::Response {
        chat.send(self.auth, self.room, self.text)
            .await
    }
}

