use {
    async_trait::async_trait,
    qaul_chat::Chat,
};

pub mod messages;
pub mod rooms;

/// A trait to extend the Chat service state with RPC functions
#[async_trait]
pub trait ChatExt {
    async fn apply<R, T>(&self, r: T) -> R
    where 
        R: Send + Sync,
        T: Send + Sync + ChatRpc<Response = R>;
}

#[async_trait]
impl ChatExt for Chat {
    async fn apply<R, T>(&self, r: T) -> R
    where 
        R: Send + Sync,
        T: Send + Sync + ChatRpc<Response = R> 
    {
        r.apply(self).await
    }
}

/// The RPC trait that get's access to the Chat service state
#[async_trait]
pub trait ChatRpc {
    type Response;
    async fn apply(self, chat: &Chat) -> Self::Response;
}

