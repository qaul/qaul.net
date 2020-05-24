//! `qaul.net` voice call service
//!
//! A call can be established between two or more participants on the
//! network.  Audio data is streamed into and out from conversations.

#[macro_use]
extern crate tracing;

mod directory;
mod msgs;
mod protocol;
mod subs;
mod types;
mod worker;

pub mod error;
pub use self::{
    subs::{EventSubscription, InvitationSubscription},
    types::{Call, CallEvent, CallId},
};

pub(crate) use self::{
    directory::{CallDirectory, DirectoryRef},
    types::{CallInvitation, CallMessage, CallUser},
};

use crate::error::Result;
use async_std::{
    sync::{channel, Mutex, RwLock},
    task,
};
use futures::future::abortable;
use libqaul::{error::Error as QaulError, services::ServiceEvent, users::UserAuth, Identity, Qaul};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex as SyncMutex},
};

pub(crate) const ASC_NAME: &'static str = "net.qaul.voice";

pub(crate) mod tags {
    use {crate::CallId, libqaul::helpers::Tag};

    /// Service specific list of calls stored in libqaul
    pub(crate) const CALL_LIST: &'static str = "net.qaul.voice.call_list";

    /// Generate a call-id tag
    pub(crate) fn call_id(id: CallId) -> Tag {
        Tag::new("call-id", id.as_bytes().to_vec())
    }
}

/// Voice service state
#[derive(Clone)]
pub struct Voice {
    /// libqaul instance we're running on
    pub(crate) qaul: Arc<Qaul>,
    /// Where we store calls
    ///
    /// This is behind a mutex because simultaneous access to
    /// alexandria could result in race conditions
    pub(crate) calls: DirectoryRef,
    /// where we store user data for easy access
    pub(crate) users: Arc<RwLock<BTreeMap<Identity, Arc<CallUser>>>>,
}

impl Voice {
    /// Create a new voice service instance
    pub async fn new(qaul: Arc<Qaul>) -> Result<Arc<Self>> {
        let calls = Arc::new(Mutex::new(CallDirectory::new(qaul.clone())));
        let users = Arc::new(RwLock::new(BTreeMap::new()));
        let this = Arc::new(Self { qaul, calls, users });

        // this is where we store the per-client worker abort handles to
        // allow future cleanup when the client logs out
        let client_message_handles = SyncMutex::new(BTreeMap::new());
        let _this = this.clone();
        this.qaul
            .services()
            .register(ASC_NAME, move |cmd| match cmd {
                // when a user logs in...
                ServiceEvent::Open(auth) => {
                    // start up a worker
                    let (fut, handle) = abortable(worker::client_message_worker(
                        auth.clone(),
                        _this.clone(),
                        auth.0, //this field is just to add what we want to instrument
                    ));
                    task::spawn(fut);

                    // and keep track of the abort handle for later
                    client_message_handles
                        .lock()
                        .unwrap()
                        .insert(auth.0, handle);
                }
                // when a user logs out...
                ServiceEvent::Close(auth) => {
                    // remove the user from the user map
                    task::block_on(_this.users.write()).remove(&auth.0);
                    // and then abort the worker
                    if let Some(handle) = client_message_handles.lock().unwrap().remove(&auth.0) {
                        handle.abort();
                    }
                }
            })
            .await?;
        Ok(this)
    }

    /// Start a new call with at least one friend to invite
    ///
    /// While it's technically possible to have a call without any
    /// participants (other than ourselves), to keep the API surface a
    /// bit cleaner, we take a friend here to auto-invite after the
    /// call was created.
    ///
    /// ## Call example
    ///
    /// ```rust
    /// # async fn testing() -> qaul_voice::error::Result<()> {
    /// # use {libqaul::{Qaul, Identity}, qaul_voice::{Voice, CallEvent}};
    /// # use async_std::sync::Arc;
    /// # let qaul = Qaul::dummy();
    /// # let user = qaul.users().create("abcdefg").await.unwrap();
    /// # let friend = Identity::random();
    /// # let voice = Voice::new(Arc::clone(&qaul)).await?;
    /// // Create a call
    /// let id = voice.start_call(user.clone(), friend).await?;
    ///
    /// // Get full call metadata via id
    /// let call = voice.get_call(user.clone(), id).await?;
    ///
    /// // Listen for reply events from friend
    /// let sub = voice.subscribe_call_events(user, id).await?;
    /// while let Some(event) = sub.next().await {
    ///   match event {
    ///     CallEvent::UserInvited(id) => println!("Someone invited {}", id),
    ///     CallEvent::UserJoined(id) => println!("User accepted invitation: {}", id),
    ///     CallEvent::UserParted(id) => println!("User quit: {}", id),
    ///   }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_call(&self, user: UserAuth, friend: Identity) -> Result<CallId> {
        let call = Call::new(&self.calls, &user).await;
        info!("User {:?} created call {:?}", user.0, call.id);

        self.invite_to_call(user, friend, call.id).await?;
        Ok(call.id)
    }

    /// Get all current calls that are known for a user session
    ///
    /// Note that calls are namespaced by user session, meaning that
    /// when a user is using two different devices, for the sake of
    /// call states, they are two separate users.
    pub async fn get_calls(&self, user: UserAuth) -> Result<Vec<Call>> {
        self.calls.lock().await.get_all(user).await
    }

    /// Get call metadata via the call Id
    pub async fn get_call(&self, user: UserAuth, id: CallId) -> Result<Call> {
        Call::get(&self.calls, user, id).await
    }

    /// Invite a user to an ongoing call
    ///
    /// This will not immediately make the requested friend available
    /// in the call, but instead dispatch an invitation that needs to
    /// be accepted first (and can be rejected).  Any UI code needs to
    /// take this into account.
    pub async fn invite_to_call(&self, user: UserAuth, friend: Identity, id: CallId) -> Result<()> {
        info!("{:?} is inviting {:?} to call {:?}", user.0, friend, id);
        Call::invite_to(&self.calls, &self.qaul, user, friend, id).await
    }

    /// Join an in-progress call
    ///
    /// This function should be called as a response to a call invite
    /// event received via `subscribe _invites`.  It will signal to
    /// everybody in the call that a user has accepted the call and
    /// audio data will be sent to them.
    pub async fn join_call(&self, user: UserAuth, id: CallId) -> Result<()> {
        info!("{:?} is joining call {:?}", user.0, id);
        Call::join(&self.calls, &self.qaul, user, id).await
    }

    /// Leave a call
    ///
    /// Send a signal to all call participants that the user is
    /// leaving and ignoring subsequent voice data and invite
    /// requests.
    pub async fn leave_call(&self, user: UserAuth, id: CallId) -> Result<()> {
        info!("{:?} is leaving call {:?}", user.0, id);
        Call::leave(&self.calls, &self.qaul, user, id).await
    }

    /// Subscribe to call invitation events for a particular user
    ///
    /// NOTE: This will not notify you about the creation of your own
    /// calls, only external invites
    pub async fn subscribe_invites(&self, user: UserAuth) -> Result<InvitationSubscription> {
        // if the worker hasn't added the user we'll error because
        // there's not much else we can do
        let user: Arc<CallUser> = self
            .users
            .read()
            .await
            .get(&user.0)
            .ok_or(QaulError::NoUser)?
            .clone();

        let (tx, rx) = channel(1);
        let sub_id = user.add_invitation_sub(tx).await;
        Ok(InvitationSubscription::new(Arc::clone(&user), rx, sub_id))
    }

    /// Subscribe to call events
    ///
    /// This will notify you when other users join, leave, or are
    /// invited to the call, ignoring events that are emitted locally.
    pub async fn subscribe_call_events(
        &self,
        user: UserAuth,
        id: CallId,
    ) -> Result<EventSubscription> {
        // if the worker hasn't added the user we'll error because
        // there's not much else we can do
        let user: Arc<CallUser> = self
            .users
            .read()
            .await
            .get(&user.0)
            .ok_or(QaulError::NoUser)?
            .clone();

        // Create a channel and store it in the CallUser.  Give the
        // other side to the subscriber
        let (tx, rx) = channel(1);
        let sub_id = user.add_event_sub(id, tx).await;
        Ok(EventSubscription::new(Arc::clone(&user), rx, id, sub_id))
    }

    /// Send a buffer with audio data to a call
    pub async fn push_voice_data(
        &self,
        _user: UserAuth,
        _id: CallId,
        _buffer: Vec<u8>,
    ) -> Result<()> {
        unimplemented!()
    }
}
