use {
    async_trait::async_trait,
    failure::format_err,
    futures::stream::StreamExt,
    libqaul::{
        users::UserAuth,
        Identity,
    },
    qaul_voices::{
        api::{StreamMetadata, CallId, IncomingCall, CallStatus},
        Voices, Result,
    },
    serde::{Serialize, Deserialize},
};

/// An extension trait for the Voices service to make applying RPC requests
/// straight forward
#[async_trait]
pub trait VoicesExt {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + VoicesRpc<Response = R>;
}

#[async_trait]
impl VoicesExt for Voices {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + VoicesRpc<Response = R> 
    {
        r.apply(self).await
    }
}

/// A trait for RPC requests that can be applied to an instance of Voices
#[async_trait]
pub trait VoicesRpc {
    type Response;
    async fn apply(self, voices: &Voices) -> Self::Response;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MakeCall {
    auth: UserAuth,
    user: Identity,
    metadata: StreamMetadata,
}

#[async_trait]
impl VoicesRpc for MakeCall {
    type Response = Result<CallId>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.call(self.auth, self.user, self.metadata).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AcceptCall {
    auth: UserAuth,
    call: CallId,
    metadata: StreamMetadata,
}

#[async_trait]
impl VoicesRpc for AcceptCall {
    type Response = Result<()>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.accept(self.auth, self.call, self.metadata).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RejectCall {
    auth: UserAuth,
    call: CallId,
}

#[async_trait]
impl VoicesRpc for RejectCall {
    type Response = Result<()>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.reject(self.auth, self.call).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HangUp {
    auth: UserAuth,
    call: CallId,
}

#[async_trait]
impl VoicesRpc for HangUp {
    type Response = Result<()>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.hang_up(self.auth, self.call).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NextIncoming {
    auth: UserAuth,
}

#[async_trait]
impl VoicesRpc for NextIncoming {
    type Response = Result<IncomingCall>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.next_incoming(self.auth).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMetadata {
    auth: UserAuth,
    call: CallId,
}

#[async_trait]
impl VoicesRpc for GetMetadata {
    type Response = Result<StreamMetadata>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.get_metadata(self.auth, self.call).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushVoice {
    auth: UserAuth,
    call: CallId,
    // TODO: probably want to have custom serialization for this
    // also maybe we should switch the whole stack to f32 to mitigate
    // endianness issues in the encoding
    data: Vec<i16>,
}

#[async_trait]
impl VoicesRpc for PushVoice {
    type Response = Result<()>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.push_voice(self.auth, self.call, self.data).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStatus {
    auth: UserAuth,
    call: CallId,
}

#[async_trait]
impl VoicesRpc for GetStatus {
    type Response = Result<CallStatus>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.get_status(self.auth, self.call).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NextVoice {
    auth: UserAuth,
    call: CallId,
}

#[async_trait]
impl VoicesRpc for NextVoice {
    type Response = Result<Vec<i16>>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.subscribe_to_voice(self.auth, self.call)
            .await?
            .next()
            .await
            .ok_or(format_err!("Stream closed"))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnHangup {
    auth: UserAuth,
    call: CallId,
}

#[async_trait]
impl VoicesRpc for OnHangup {
    type Response = Result<()>;
    async fn apply(self, voices: &Voices) -> Self::Response {
        voices.on_hangup(self.auth, self.call).await
    }
}
