//! A protocol helper for the voice service
//!
//! The following documentation must be kept up to date with changes,
//! and outlines the basic workings of the protocol of this service.
//! Types that are sent over the wire are specified in `types.rs`,
//! their interactions are largely implemented in this file.

use crate::{error::Result, Call, CallId, CallInvitation, CallMessage, DirectoryRef};
use libqaul::{users::UserAuth, Identity, QaulRef};
use std::collections::BTreeSet;

impl Call {
    pub(crate) async fn new(calls: &DirectoryRef, user: &UserAuth) -> Self {
        let call = Call {
            id: CallId::random(),
            participants: BTreeSet::new(),
            invitees: vec![user.0].into_iter().collect(),
        };

        let _ = calls.lock().await.insert(user.clone(), &call).await;
        call
    }

    /// Get a call by it's Id
    pub(crate) async fn get(calls: &DirectoryRef, user: UserAuth, id: CallId) -> Result<Call> {
        calls.lock().await.get(user, id).await
    }

    pub(crate) async fn update(
        calls: &DirectoryRef,
        user: UserAuth,
        id: CallId,
        cb: impl FnOnce(Call) -> Call,
    ) -> Result<Call> {
        calls.lock().await.update(user, id, cb).await
    }

    /// Invite a new person to an existing call
    pub(crate) async fn invite_to(
        calls: &DirectoryRef,
        qaul: &QaulRef,
        user: UserAuth,
        friend: Identity,
        id: CallId,
    ) -> Result<()> {
        let call = Call::get(calls, user.clone(), id).await?;

        // first send the invitation to the user
        let message = CallMessage::Invitation(CallInvitation {
            participants: call.participants,
            invitees: call.invitees,
        });
        message.send_single(user.clone(), friend, id, &qaul).await?;

        // then add the user to the call's invitee list
        let call = Call::update(calls, user.clone(), id, |mut call| {
            call.invitees.insert(friend);
            call
        })
        .await?;

        // and tell others that the user was invited to the call
        //
        // this is crucial because it allows the users of the call to
        // completely change by the time the user joins the call and still
        // have them know who to talk to
        //
        // TODO: there's race conditions that could result in members of the
        // call having an inconsistant view of the call state
        // this could be resolved by having users occasionally randomly update
        // eachother on who they think is in the call
        let message = CallMessage::InvitationSent(friend);
        message
            .send_many(user.clone(), &call.invitees, id, &qaul)
            .await?;

        Ok(())
    }

    /// Join a call that's already in progress
    pub(crate) async fn join(
        calls: &DirectoryRef,
        qaul: &QaulRef,
        user: UserAuth,
        id: CallId,
    ) -> Result<()> {
        // we join ourselves
        let call = Call::update(calls, user.clone(), id, |mut call| {
            call.participants.insert(user.0);
            call
        })
        .await?;

        // and notify our peers
        let message = CallMessage::Join;
        message.send_many(user, &call.invitees, id, &qaul).await?;

        Ok(())
    }

    /// Leave a call
    pub(crate) async fn leave(
        calls: &DirectoryRef,
        qaul: &QaulRef,
        user: UserAuth,
        id: CallId,
    ) -> Result<()> {
        // remove ourselves from the call
        //
        // TODO: this should actually delete the call but...
        // i don't think alexandria can do that rn
        let call = Call::update(calls, user.clone(), id, |mut call| {
            call.participants.remove(&user.0);
            call.invitees.remove(&user.0);
            call
        })
        .await?;

        // send a goodbye to other members of the call
        let message = CallMessage::Part;
        message.send_many(user, &call.invitees, id, &qaul).await?;

        Ok(())
    }

    // TODO: move the worker code here at some point
}
