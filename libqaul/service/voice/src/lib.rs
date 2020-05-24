//! `qaul.net` voice service

#[macro_use] extern crate tracing;

mod directory;
mod error;
mod subs;
mod types;
mod worker;

pub use self::{
    error::Result,
    subs::{InvitationSubscription, CallEventSubscription},
    types::{Call, CallId, CallEvent},
};
pub(crate) use self::{
    types::{CallMessage, CallInvitation, CallUser},
};
use {
    async_std::{
        sync::{Mutex, RwLock},
        task,
    },
    futures::{
        channel::mpsc::channel,
        future::{
            abortable,
            AbortHandle,
        },
    },
    libqaul::{
        error::Error as QaulError,
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
    pub(crate) users: Arc<RwLock<BTreeMap<Identity, Arc<CallUser>>>>,
}

impl Voice {
    pub async fn new(qaul: Arc<Qaul>) -> Result<Arc<Self>> {
        let calls = Arc::new(Mutex::new(CallDirectory::new(qaul.clone())));
        let users = Arc::new(RwLock::new(BTreeMap::new()));
        let this = Arc::new(Self { 
            qaul, 
            calls,
            users,
        });

        let client_message_handles = SyncMutex::new(BTreeMap::new());
        let _this = this.clone();
        this.qaul
            .services()
            .register(ASC_NAME, move |cmd| match cmd {
                ServiceEvent::Open(auth) => {
                    let (fut, handle) = abortable(worker::client_message_worker(
                        auth.clone(), 
                        _this.clone(),
                        auth.0,
                    ));
                    task::spawn(fut);
                    client_message_handles.lock().unwrap().insert(auth.0, handle);
                },
                ServiceEvent::Close(auth) => {
                    task::block_on(_this.users.write()).remove(&auth.0);
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
        info!("User {:?} created call {:?}", user.0, call_id);
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
        info!("{:?} is inviting {:?} to call {:?}", user.0, friend, id);
        let call = self.get_call(user.clone(), id).await?;

        let message = CallMessage::Invitation(CallInvitation {
            participants: call.participants,
            invitees: call.invitees,
        });
        message.send(user.clone(), friend, id, &self.qaul).await?;

        let call = self.calls.lock().await.update(user.clone(), id, |mut call| {
            call.invitees.insert(friend);
            call
        }).await?;

        let message = CallMessage::InvitationSent(friend);
        message.send_to(user.clone(), &call.invitees, id, &self.qaul).await
    }

    pub async fn join_call(&self, user: UserAuth, id: CallId) -> Result<()> {
        info!("{:?} is joining call {:?}", user.0, id);
        let call = self.calls.lock().await.update(user.clone(), id, |mut call| {
            call.participants.insert(user.0);
            call
        }).await?;
        let message = CallMessage::Join;
        message.send_to(user, &call.invitees, id, &self.qaul).await
    }

    pub async fn leave_call(&self, user: UserAuth, id: CallId) -> Result<()> {
        info!("{:?} is leaving call {:?}", user.0, id);
        let call = self.calls.lock().await.update(user.clone(), id, |mut call| {
            call.participants.remove(&user.0);
            call.invitees.remove(&user.0);
            call
        }).await?;
        let message = CallMessage::Part;
        message.send_to(user, &call.invitees, id, &self.qaul).await
    }

    pub async fn subscribe_invites(&self, user: UserAuth) -> Result<InvitationSubscription> {
        let (sender, receiver) = channel(1);
        let user = self.users.read().await.get(&user.0).ok_or(QaulError::NoUser)?.clone();
        user.invitation_subs.write().await.push(sender);
        Ok(receiver)
    }

    pub async fn subscribe_call_events(&self, user: UserAuth, id: CallId) 
    -> Result<CallEventSubscription> {
        let (sender, receiver) = channel(1);
        let user = self.users.read().await.get(&user.0).ok_or(QaulError::NoUser)?.clone();
        let mut subs = user.call_event_subs.write().await;
        if let Some(mut v) = subs.get_mut(&id) {
            v.push(sender); 
        } else {
            subs.insert(id, vec![sender]);
        }
        Ok(receiver)
    }
}
