//! An internal worker that subscribes service events
//!
//! This stateless worker is given UserAuth objects to subscribe on,
//! then spawns extra tasks to listen for messages.

use crate::{msg, Chat, ChatMessage, Room, ASC_NAME};
use async_std::{
    sync::{channel, Arc, RwLock, Sender},
    task,
};
use libqaul::{helpers::TagSet, users::UserAuth};
use std::collections::BTreeSet;
use tracing::{trace, debug};

/// A command to the internal worker
pub(crate) enum Command {
    Start(UserAuth),
    Stop(UserAuth),
}

impl Command {
    #[inline]
    fn to_str(&self) -> &'static str {
        match self {
            Self::Start(_) => "START",
            Self::Stop(_) => "STOP",
        }
    }
}

type RunMap = Arc<RwLock<BTreeSet<UserAuth>>>;

/// Spawn the async machinery that runs the chat service
pub(crate) fn run_asnc(serv: Arc<Chat>) -> Sender<Command> {
    let (tx, rx) = channel(1);

    task::spawn(async move {
        let map: RunMap = Default::default();

        if let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Start(auth) => {
                    trace!("Receiving libqaul user {} START event!", auth.0);
                    map.write().await.insert(auth.clone());
                    task::spawn(run_user(auth, Arc::clone(&serv), Arc::clone(&map)));
                }
                Command::Stop(auth) => {
                    trace!("Receiving libqaul user {} STOP event!", auth.0);
                    map.write().await.remove(&auth);
                }
            }
        }

        // Stop all remaining workers
        map.write().await.clear();
    });

    tx
}

/// Run a worker that subscribes to all events for a user
#[tracing::instrument(skip(serv, run, user), level = "trace")]
pub(crate) async fn run_user(user: UserAuth, serv: Arc<Chat>, run: RunMap) {
    let sub = serv
        .qaul
        .messages()
        .subscribe(user.clone(), ASC_NAME, TagSet::empty())
        .await
        .unwrap();
    trace!("Creating message subscription!");

    while run.read().await.contains(&user) {
        if let Some(msg) = sub.next().await {
            let chat_msg: ChatMessage = msg.into();
            debug!("Handling incoming text message in service worker");

            // If we get a room state back, we send a reply message
            if let Some(rs) = Room::handle(&serv, user.clone(), &chat_msg).await {
                let friends = serv.rooms.get(user.clone(), rs.id()).await.unwrap().users;
                let room_id = rs.id();
                msg::dispatch_to(
                    &serv,
                    user.clone(),
                    friends,
                    msg::gen_payload("", rs),
                    room_id,
                )
                .await
                .unwrap();
            }
        }
    }
}
