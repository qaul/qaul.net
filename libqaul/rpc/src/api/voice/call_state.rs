use super::VoiceRpc;
use async_std::sync::Arc;
use async_trait::async_trait;
use libqaul::{users::UserAuth, Identity};
use qaul_voice::{Call, CallEventSubscription, CallId, InvitationSubscription, Result, Voice};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StartCall {
    pub auth: UserAuth,
}

#[async_trait]
impl VoiceRpc for StartCall {
    type Response = Result<CallId>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice.start_call(self.auth).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetCalls {
    pub auth: UserAuth,
}

#[async_trait]
impl VoiceRpc for GetCalls {
    type Response = Result<Vec<Call>>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice.get_calls(self.auth).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetCall {
    pub auth: UserAuth,
    pub call: CallId,
}

#[async_trait]
impl VoiceRpc for GetCall {
    type Response = Result<Call>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice.get_call(self.auth, self.call).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InviteToCall {
    pub auth: UserAuth,
    pub call: CallId,
    pub friend: Identity,
}

#[async_trait]
impl VoiceRpc for InviteToCall {
    type Response = Result<()>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice
            .invite_to_call(self.auth, self.friend, self.call)
            .await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct JoinCall {
    pub auth: UserAuth,
    pub call: CallId,
}

#[async_trait]
impl VoiceRpc for JoinCall {
    type Response = Result<()>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice.join_call(self.auth, self.call).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LeaveCall {
    pub auth: UserAuth,
    pub call: CallId,
}

#[async_trait]
impl VoiceRpc for LeaveCall {
    type Response = Result<()>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice.leave_call(self.auth, self.call).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SubscribeInvites {
    pub auth: UserAuth,
}

#[async_trait]
impl VoiceRpc for SubscribeInvites {
    type Response = Result<InvitationSubscription>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice.subscribe_invites(self.auth).await
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SubscribeCallEvents {
    pub auth: UserAuth,
    pub call: CallId,
}

#[async_trait]
impl VoiceRpc for SubscribeCallEvents {
    type Response = Result<CallEventSubscription>;
    async fn apply(self, voice: &Arc<Voice>) -> Self::Response {
        voice.subscribe_call_events(self.auth, self.call).await
    }
}
