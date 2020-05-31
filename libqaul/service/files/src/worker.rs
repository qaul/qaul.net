use crate::{Fileshare, Subscription, ASC_NAME};
use async_std::{
    sync::{channel, Arc, RwLock, Sender},
    task,
};
use libqaul::{helpers::TagSet, users::UserAuth, Identity};
use std::collections::BTreeSet;
use tracing::{debug, info, trace};

pub(crate) enum Command {
    Start(UserAuth),
    Stop(UserAuth),
}

type RunMap = Arc<RwLock<BTreeSet<Identity>>>;

pub(crate) fn run_asnc(file_serv: Arc<Fileshare>) -> Sender<Command> {
    let (tx, rx) = channel(1);

    task::spawn(async move {
        let map: RunMap = Default::default();
        while let Some(cmd) = rx.recv().await {
            let map = Arc::clone(&map);
            match cmd {
                Command::Start(auth) => {
                    trace!("Receiving libqaul user {} START event!", auth.0);
                    map.write().await.insert(auth.0);
                    task::spawn(run_user(auth, Arc::clone(&file_serv), Arc::clone(&map)));
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

pub(crate) async fn run_user(user: UserAuth, file_serv: Arc<Fileshare>, run: RunMap) {
    let sub = Subscription::new(
        file_serv
            .qaul
            .messages()
            .subscribe(user.clone(), ASC_NAME, TagSet::empty())
            .await
            .unwrap(),
    );
    trace!("Creating message subscription!");

    while run.read().await.contains(&user.0) {
        let f_msg = sub.next().await;

        // Skip messages that we sent (this is a libqaul bug)
        if f_msg.sender == user.0 && continue {}

        trace!("Receiving file message from: {}", f_msg.sender);

        // Handling the message does some weird I/O so we spawn an I/O
        // task here to not block our main subscriber.
        let auth = user.clone();
        let serv = Arc::clone(&file_serv);
        task::spawn(async move { f_msg.handle_incoming(auth, serv).await });
    }
}
