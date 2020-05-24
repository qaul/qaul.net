use crate::{Call, CallEvent, CallId, CallUser};
use async_std::{
    future,
    sync::{Arc, Receiver},
    task,
};
use std::time::Duration;

/// A subscriber for call invitation events
pub struct InvitationSubscription {
    parent: Arc<CallUser>,
    io: Receiver<Call>,
    sub_id: usize,
}

impl Drop for InvitationSubscription {
    fn drop(&mut self) {
        task::block_on(async {
            self.parent
                .invitation_subs
                .write()
                .await
                .remove(&self.sub_id);
        });
    }
}

impl InvitationSubscription {
    pub(crate) fn new(parent: Arc<CallUser>, io: Receiver<Call>, sub_id: usize) -> Self {
        Self { io, parent, sub_id }
    }

    /// Wait for the next event to come
    pub async fn next(&self) -> Call {
        self.io.recv().await.unwrap()
    }

    /// Wait for the next event, up to a timeout limit
    ///
    /// This function returns `None` when the timeout was reached and
    /// may panic when a service internal error occured.
    pub async fn next_timeout(&self, time: Duration) -> Option<Call> {
        future::timeout(time, async { self.io.recv().await.unwrap() })
            .await
            .ok()
    }
}

/// A subscriber for in-call user generated events
pub struct EventSubscription {
    parent: Arc<CallUser>,
    io: Receiver<CallEvent>,
    call_id: CallId,
    sub_id: usize,
}

// We implement drop so that the subscription can deallocate the
// senders when we go out of scope.
impl Drop for EventSubscription {
    fn drop(&mut self) {
        task::block_on(async {
            if let Some(map) = self
                .parent
                .call_event_subs
                .write()
                .await
                .get_mut(&self.call_id)
            {
                map.remove(&self.sub_id);
            }
        });
    }
}

impl EventSubscription {
    pub(crate) fn new(
        parent: Arc<CallUser>,
        io: Receiver<CallEvent>,
        call_id: CallId,
        sub_id: usize,
    ) -> Self {
        Self {
            io,
            call_id,
            sub_id,
            parent,
        }
    }

    /// Wait for the next event to come
    ///
    /// The subscriber should be dropped when it starts sending `None`
    /// because that means that the sender has been deallocated.
    pub async fn next(&self) -> Option<CallEvent> {
        self.io.recv().await
    }

    /// Wait for the next event, up to a timeout limit
    ///
    /// This function returns `None` when the timeout was reached or
    /// the sender has been deallocated.
    // FIXME: maybe introduce a timeout error or something?
    pub async fn next_timeout(&self, time: Duration) -> Option<CallEvent> {
        future::timeout(time, async { self.io.recv().await.unwrap() })
            .await
            .ok()
    }
}

pub(crate) async fn notify_call_events(user: &Arc<CallUser>, id: CallId, event: CallEvent) {
    if let Some(subs) = user.call_event_subs.read().await.get(&id) {
        subs.iter().for_each(|(_, sub)| {
            // FIXME: in case subscriptions aren't being polled this
            // will fail!  instead we should spawn here and
            // async-deliver messages
            task::block_on(async {
                sub.send(event.clone()).await;
            })
        });
    }
}

pub(crate) async fn notify_invites(user: &Arc<CallUser>, call: Call) {
    user.invitation_subs
        .read()
        .await
        .iter()
        .for_each(|(_, tx)| {
            task::block_on(async {
                tx.send(call.clone()).await;
            })
        });
}
