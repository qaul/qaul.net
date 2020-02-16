use {
    async_trait::async_trait,
    qaul_chat::Chat,
};

pub mod messages;
pub mod rooms;

#[async_trait]
pub trait ChatRPC {
    type Response;
    async fn apply(self, chat: &Chat) -> Self::Response;
}

#[async_trait]
pub trait ChatExt {
    async fn apply<R, T>(&self, r: T) -> R
    where 
        R: Send + Sync,
        T: Send + Sync + ChatRPC<Response = R>;
}

#[async_trait]
impl ChatExt for Chat {
    async fn apply<R, T>(&self, r: T) -> R
    where 
        R: Send + Sync,
        T: Send + Sync + ChatRPC<Response = R> 
    {
        r.apply(self).await
    }
}
