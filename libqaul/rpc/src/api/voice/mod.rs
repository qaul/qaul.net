use async_std::sync::Arc;
use async_trait::async_trait;
use qaul_voice::Voice;

pub mod call_state;
mod subs;

#[async_trait]
pub trait VoiceExt {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + VoiceRpc<Response = R>;
}

#[async_trait]
impl<'a> VoiceExt for &'a Arc<Voice> {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + VoiceRpc<Response = R>,
    {
        r.apply(self).await
    }
}

#[async_trait]
pub trait VoiceRpc {
    type Response;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response;
}
