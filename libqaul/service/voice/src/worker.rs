use crate::{msgs, subs, Call, CallEvent, CallMessage, CallUser, Voice, ASC_NAME};
use async_std::sync::RwLock;
use libqaul::{helpers::TagSet, users::UserAuth, Identity};
use std::{collections::BTreeMap, sync::Arc};

/// The worker that handles incoming messages for a client
#[tracing::instrument(skip(auth, voices), level = "info")]
pub(crate) async fn client_message_worker(auth: UserAuth, voices: Arc<Voice>, user: Identity) {
    // store some information about the user and add it to the voice state
    let user = CallUser::new(auth);
    
    voices
        .users
        .write()
        .await
        .insert(user.auth.0, Arc::clone(&user));

    // subscribe to incoming messages
    let sub = voices
        .qaul
        .messages()
        .subscribe(user.auth.clone(), ASC_NAME, TagSet::empty())
        .await
        .unwrap();
    trace!("Creating message subscription!");

    loop {
        let msg = sub.next().await;

        // skip messages we've sent
        if msg.sender == user.auth.0 {
            continue;
        }

        // get the call id from the message tagset
        let id = match msgs::grab_call_id(&msg) {
            Some(id) => id,
            None => {
                warn!("Call message recieved with no call id tag");
                continue;
            }
        };

        let call_msg = match msgs::convert_message(&msg) {
            Some(msg) => msg,
            None => {
                warn!("Failed to deserialise message payload");
                continue;
            }
        };

        match call_msg {
            // if we have been invited to a call...
            CallMessage::Invitation(inv) => {
                info!("Recieved invitation for call {:?}", id);

                // add the call to our call database
                let mut call = Call {
                    id,
                    participants: inv.participants,
                    invitees: inv.invitees,
                };
                call.invitees.insert(user.auth.0);
                let res = voices
                    .calls
                    .lock()
                    .await
                    .insert(user.auth.clone(), &call)
                    .await;
                if let Err(e) = res {
                    warn!("Failed to insert new call into directory (this might be due to the client exiting?): {}", e);
                }

                // then push the event to any subscribers
                subs::notify_invites(&user, call).await;
            }

            // if someone else has been invited to a call...
            CallMessage::InvitationSent(to) => {
                info!(
                    "Recieved invitation notification for user {:?} on call {:?}",
                    to, id
                );

                // update the call in our database
                let res = voices
                    .calls
                    .lock()
                    .await
                    .update(user.auth.clone(), id, |mut call| {
                        call.invitees.insert(to);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }

                // then push the event to any subscribers
                subs::notify_call_events(&user, id, CallEvent::UserInvited(to)).await;
            }

            // if someone has joined the call...
            CallMessage::Join => {
                // note who it is
                let joined_user = msg.sender;

                info!(
                    "Recieved join message for user {:?} on call {:?}",
                    joined_user, id
                );

                // update the call in our database
                let res = voices
                    .calls
                    .lock()
                    .await
                    .update(user.auth.clone(), id, |mut call| {
                        call.participants.insert(joined_user);
                        call.invitees.insert(joined_user);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }

                // then push the event to any subscribers
                subs::notify_call_events(&user, id, CallEvent::UserJoined(joined_user)).await;
            }

            // if a user has left a call
            CallMessage::Part => {
                // note who they are
                let parting_user = msg.sender;

                info!(
                    "Receieved part message for user {:?} on call {:?}",
                    parting_user, id
                );

                // update our call database
                let res = voices
                    .calls
                    .lock()
                    .await
                    .update(user.auth.clone(), id, |mut call| {
                        call.participants.remove(&parting_user);
                        call.invitees.remove(&parting_user);
                        call
                    })
                    .await;
                if let Err(e) = res {
                    warn!("Failed to update call in directory (this might be due to the client exiting?): {}", e);
                }

                // then push the event to any subscribers
                subs::notify_call_events(&user, id, CallEvent::UserParted(parting_user)).await;
            }

            // actual call data
            CallMessage::Data(_) => unimplemented!(),
        }
    }
}
