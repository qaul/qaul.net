//! A user profile change announcer

use crate::{
    messages::{Envelope, MsgUtils, RatMessageProto},
    users::{UserProfile, UserStore},
    Identity,
};
use async_std::{
    sync::{Arc, RwLock},
    task,
};
use ratman::{Message, Recipient, Router};
use std::{collections::BTreeSet, time::Duration};

pub(crate) struct Announcer {
    active: RwLock<BTreeSet<Identity>>,
}

impl Announcer {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            active: Default::default(),
        })
    }

    /// Check if a message is a user profile
    pub(crate) fn check_message(msg: &Message) -> Option<UserProfile> {
        let payload = MsgUtils::extract_simple_payload(&msg)?;
        bincode::deserialize(&payload).ok()
    }

    pub(crate) async fn online(
        self: &Arc<Self>,
        router: &Arc<Router>,
        store: UserStore,
        id: Identity,
    ) {
        self.active.write().await.insert(id);

        let this = Arc::clone(self);
        let router = Arc::clone(router);

        task::spawn(async move {
            while this.active.read().await.contains(&id) {
                let profile = store.get(id).await.unwrap();
                let payload = bincode::serialize(&profile).unwrap();

                let proto = RatMessageProto {
                    env: Envelope {
                        id: Identity::random(),
                        sender: id,
                        associator: "libqaul._int.announcer".into(),
                        payload,
                        tags: vec![],
                    },
                    recipient: Recipient::Flood,
                };

                MsgUtils::send(&store, &router, proto).await.unwrap();
                task::sleep(Duration::from_secs(30)).await;
            }
        });
    }

    pub(crate) async fn offline(self: &Arc<Self>, id: Identity) {
        self.active.write().await.remove(&id);
    }
}
