//! `qaul.net` voice service

#[macro_use] extern crate tracing;

mod directory;
mod error;
mod types;
mod worker;

pub use self::error::Result;
pub(crate) use self::{
    types::{Call, CallId, CallMessage, CallInvitation},
};
use {
    async_std::{
        sync::Mutex,
        task,
    },
    futures::future::{
        abortable,
        AbortHandle,
    },
    libqaul::{
        services::ServiceEvent,
        users::UserAuth,
        Identity, Qaul,
    },
    self::directory::CallDirectory,
    std::{
        collections::{BTreeSet, BTreeMap},
        sync::{Arc, Mutex as SyncMutex},
    },
};

const ASC_NAME: &'static str = "net.qaul.voice";

pub(crate) mod tags {
    use {crate::CallId, libqaul::helpers::Tag};
    pub(crate) const CALL_LIST: &'static str = "net.qaul.voice.call_list";
    pub(crate) fn call_id(id: CallId) -> Tag {
        Tag::new("call-id", id.as_bytes().to_vec())
    }
} 

#[derive(Clone)]
pub struct Voice {
    pub(crate) qaul: Arc<Qaul>,
    pub(crate) calls: Arc<Mutex<CallDirectory>>,
}

impl Voice {
    pub async fn new(qaul: Arc<Qaul>) -> Result<Arc<Self>> {
        let calls = Arc::new(Mutex::new(CallDirectory::new(qaul.clone())));
        let this = Arc::new(Self { 
            qaul, 
            calls,
        });

        let client_message_handles = SyncMutex::new(BTreeMap::new());
        let _this = this.clone();
        this.qaul
            .services()
            .register(ASC_NAME, move |cmd| match cmd {
                ServiceEvent::Open(auth) => {
                    let (fut, handle) = abortable(worker::client_message_worker(
                        auth.clone(), 
                        _this.clone()
                    ));
                    task::spawn(fut);
                    client_message_handles.lock().unwrap().insert(auth.0, handle);
                },
                ServiceEvent::Close(auth) => {
                    if let Some(handle) = client_message_handles.lock().unwrap().remove(&auth.0) {
                        handle.abort();
                    }
                },
            })
            .await?;
        Ok(this)
    }

    pub async fn start_call(&self, user: UserAuth) -> Result<CallId> {
        let call = Call {
            id: CallId::random(),
            participants: BTreeSet::new(),
            invitees: Some(user.0).into_iter().collect(),
        };
        let call_id = call.id.clone();
        self.calls.lock().await.insert(user, &call).await?;
        Ok(call_id)
    }

    pub async fn get_calls(&self, user: UserAuth) -> Result<Vec<Call>> {
        self.calls.lock().await.get_all(user).await
    }

    pub async fn get_call(&self, user: UserAuth, id: CallId) -> Result<Call> {
        self.calls.lock().await.get(user, id).await
    }

    pub async fn invite_to_call(&self, user: UserAuth, friend: Identity, id: CallId) -> Result<()> {
        let call = self.get_call(user.clone(), id).await?;

        let message = CallMessage::Invitation(CallInvitation {
            participants: call.participants,
            invitees: call.invitees,
        });
        message.send(user.clone(), friend, id, &self.qaul).await?;

        let message = CallMessage::InvitationSent(friend);
        message.send(user, friend, id, &self.qaul).await
    }

    pub async fn join_call(&self, user: UserAuth, id: CallId) -> Result<()> {
        let call = self.get_call(user.clone(), id.clone()).await?;
        let message = CallMessage::Join;
        message.send_to(user, &call.invitees, id, &self.qaul).await
    }

    pub async fn leave_call(&self, user: UserAuth, id: CallId) -> Result<()> {
        let call = self.get_call(user.clone(), id.clone()).await?;
        let message = CallMessage::Part;
        message.send_to(user, &call.invitees, id, &self.qaul).await
    }
}
