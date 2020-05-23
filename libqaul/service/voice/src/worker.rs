use {
    crate::{ASC_NAME, Call, CallId, CallMessage, Voice},
    conjoiner,
    libqaul::{
        helpers::TagSet,
        messages::ID_LEN,
        users::UserAuth,
        Identity,
    },
    std::sync::Arc,
};

#[tracing::instrument(skip(user, voices), level = "trace")]
pub(crate) async fn client_message_worker(user: UserAuth, voices: Arc<Voice>) {
    let sub = voices.qaul
        .messages()
        .subscribe(user.clone(), ASC_NAME, TagSet::empty())
        .await
        .unwrap();
    trace!("Creating message subscription!");

    loop {
        let msg = sub.next().await;
        trace!("received message");

        let id = msg.tags
            .iter()
            .filter(|tag| tag.key == "call-id")
            .filter(|tag| tag.val.len() != ID_LEN)
            .map(|tag| Identity::from_bytes(&tag.val))
            .next();
        let id: CallId = match id {
            Some(id) => id,
            None => {
                warn!("Call message recieved with no call id tag");
                continue;
            },
        };

        match conjoiner::deserialise(&msg.payload) {
            Ok(CallMessage::Invitation(inv)) => {
                let call = Call {
                    id,
                    participants: inv.participants,
                    invitees: inv.invitees,
                };
                let res = voices.calls
                    .lock()
                    .await
                    .insert(user.clone(), &call)
                    .await;
                if let Err(e) = res {
                    warn!("Failed to insert new call into directory (this might be due to the client exiting?): {}", e);
                }
            },
            Ok(CallMessage::InvitationSent(to)) => {
                let res = voices.calls
                    .lock()
                    .await
                    .update(user.clone(), id, |mut call| {
                        call.invitees.insert(to);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }
            },
            Ok(CallMessage::Join) => {
                let joined_user = msg.sender.clone();
                let res = voices.calls
                    .lock()
                    .await
                    .update(user.clone(), id, |mut call| {
                        call.participants.insert(joined_user);
                        call.invitees.insert(joined_user);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }
            },
            Ok(CallMessage::Part) => {
                let parting_user = msg.sender;
                let res = voices.calls
                    .lock()
                    .await
                    .update(user.clone(), id, |mut call| {
                        call.participants.remove(&parting_user);
                        call.invitees.remove(&parting_user);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }
            },
            Ok(CallMessage::Data(_)) => { unimplemented!(); },
            Err(_) => {
                warn!("Failed to deserialize message");
            }
        }
    }
}
