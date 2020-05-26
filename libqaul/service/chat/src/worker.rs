//! An internal worker that subscribes service events
//!
//! This stateless worker is given UserAuth objects to subscribe on,
//! then spawns extra tasks to listen for messages.

use crate::{msg, Chat, Room, Subscription, ASC_NAME};
use async_std::{
    sync::{channel, Arc, RwLock, Sender},
    task,
};
use libqaul::{helpers::TagSet, users::UserAuth, Identity};
use std::collections::BTreeSet;
use tracing::{debug, info, trace};

/// A command to the internal worker
pub(crate) enum Command {
    Start(UserAuth),
    Stop(UserAuth),
}

type RunMap = Arc<RwLock<BTreeSet<Identity>>>;

/// Spawn the async machinery that runs the chat service
#[tracing::instrument(skip(serv), level = "trace")]
pub(crate) fn run_asnc(serv: Arc<Chat>) -> Sender<Command> {
    let (tx, rx) = channel(1);

    task::spawn(async move {
        let map: RunMap = Default::default();
        while let Some(cmd) = rx.recv().await {
            let map = Arc::clone(&map);
            match cmd {
                Command::Start(auth) => {
                    trace!("Receiving libqaul user {} START event!", auth.0);
                    map.write().await.insert(auth.0);
                    task::spawn(run_user(auth, Arc::clone(&serv), Arc::clone(&map)));
                }
                Command::Stop(auth) => {
                    trace!("Receiving libqaul user {} STOP event!", auth.0);
                    map.write().await.remove(&auth.0);
                }
            }
        }

        // Stop all remaining workers
        info!("Deallocating subscription workers");
        map.write().await.clear();
    });

    tx
}

/// Run a worker that subscribes to all events for a user
#[tracing::instrument(skip(serv, run, user), level = "trace")]
pub(crate) async fn run_user(user: UserAuth, serv: Arc<Chat>, run: RunMap) {
    let sub = Subscription::new(
        serv.qaul
            .messages()
            .subscribe(user.clone(), ASC_NAME, TagSet::empty())
            .await
            .unwrap(),
    );
    trace!("Creating message subscription!");

    while run.read().await.contains(&user.0) {
        let chat_msg = sub.next().await;
        if chat_msg.sender == user.0 && continue {}

        // If we get a room state back, we send a reply message
        if let Some(rs) = Room::handle(&serv, user.clone(), &chat_msg).await {
            trace!("Sending confirmation message to room state change");
            let room = serv.rooms.get(user.clone(), rs.id()).await.unwrap();
            room.send_to_participants(&serv, user.clone(), rs)
                .await
                .unwrap();
        }
    }
}
