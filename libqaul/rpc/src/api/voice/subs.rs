use crate::{api::Subscriber, Response};
use async_trait::async_trait;
use futures::stream::StreamExt;
use qaul_voice::{CallEventSubscription, InvitationSubscription};

#[async_trait]
impl Subscriber for InvitationSubscription {
    async fn next(&self) -> Option<Response> {
        self.next().await.into()
    }
}

#[async_trait]
impl Subscriber for CallEventSubscription {
    async fn next(&self) -> Option<Response> {
        self.next().await.into()
    }
}
