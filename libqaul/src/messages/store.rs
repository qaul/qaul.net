use crate::{
    messages::{Message, MsgId, MsgRef},
    users::UserAuth,
    Identity,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

type Synced<T> = Arc<RwLock<T>>;

/// A searchable index of messages encountered by this system
#[derive(Clone)]
pub(crate) struct MsgStore {
    /// Owns Message references by their ID
    by_id: Synced<BTreeMap<MsgId, MsgRef>>,
    /// By-user reference table to improve search performance
    by_user: Synced<BTreeMap<Identity, Vec<MsgRef>>>,
}

impl MsgStore {
    pub(crate) fn new() -> Self {
        Self {
            by_id: Arc::new(RwLock::new(BTreeMap::default())),
            by_user: Arc::new(RwLock::new(BTreeMap::default())),
        }
    }

    /// Permanently store a whole Message
    ///
    /// For this function it doesn't matter if the `Message` was
    /// dispatched by this system or has come in from outside.
    pub(crate) fn insert(&self, user: Identity, msg: Message) {
        let id = msg.id.clone();
        let arc = Arc::new(msg);

        self.by_user
            .write()
            .expect("Failed to lock MsgStore!")
            .entry(user)
            .or_insert(vec![])
            .push(Arc::clone(&arc));

        self.by_id
            .write()
            .expect("Failed to lock MsgStore!")
            .insert(id, arc);
    }

    pub(crate) fn get_user(&self, user: UserAuth) -> Option<Vec<Arc<Message>>> {
        let UserAuth(ref user, _) = user;

        self.by_user
            .read()
            .unwrap()
            .get(user)
            .map(|vec| vec.iter().map(Arc::clone).collect())
    }

    pub(crate) fn get_id(&self, id: &MsgId) -> Option<Arc<Message>> {
        self.by_id.read().unwrap().get(id).map(Arc::clone)
    }
}
