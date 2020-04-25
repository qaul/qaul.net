//! Internal message store wrapper

use crate::{
    messages::{Mode, MsgRef},
    helpers::QueryResult,
    Identity,
};
use alexandria::{
    utils::{Path, Tag},
    Library, Session, GLOBAL,
};
use async_std::sync::Arc;

pub(crate) const TAG_FLOOD: &'static str = "libqaul._int.flood";
pub(crate) const TAG_READ: &'static str = "libqaul._int.read";

fn msg_path(msg_id: Identity) -> Path {
    Path::from(format!("/msg:{}", msg_id))
}

#[derive(Clone)]
pub(crate) struct MsgStore {
    inner: Arc<Library>,
}

impl MsgStore {
    pub(crate) fn new(inner: Arc<Library>) -> Self {
        Self { inner }
    }

    /// Insert a message that was sent locally
    ///
    /// This message will be marked as "read" immediately, and
    /// inserted into either the user or global store, depending on
    /// wether it was a Flooded message.
    pub(crate) async fn insert_local(&self, user: Identity, msg: MsgRef, mode: Mode) {
        let mut tags = msg.tags.clone().merge(Tag::empty(TAG_READ));
        let diffs = msg.diff();
        let session = match mode {
            Mode::Flood => {
                tags.insert(Tag::empty(TAG_FLOOD));
                GLOBAL
            }
            Mode::Std(_) => Session::Id(user),
        };

        self.inner
            .batch(session, msg_path(msg.id), tags, diffs)
            .await
            .unwrap();
    }

    /// Insert a message captured from the network
    ///
    /// The primary difference to `insert_local()` is that the
    /// inserted message will not be marked as "read" and can be
    /// retrieved via the "unread messages" query.
    pub(crate) async fn insert_remote(&self, user: Option<Identity>, msg: MsgRef, mode: Mode) {
        let mut tags = msg.tags.clone();
        let diffs = msg.diff();
        let session = match mode {
            Mode::Flood => {
                tags.insert(Tag::empty(TAG_FLOOD));
                GLOBAL
            }
            Mode::Std(_) => Session::Id(user.unwrap()),
        };

        self.inner
            .batch(session, msg_path(msg.id), tags, diffs)
            .await
            .unwrap();
    }
}

// use crate::{
//     error::Result,
//     messages::{MsgId, MsgQuery, MsgRef},
//     Identity, Tag,
// };
// use std::{
//     collections::{BTreeMap, BTreeSet},
//     sync::{Arc, RwLock},
// };

// type MsgTree<K, V> = Arc<RwLock<BTreeMap<K, V>>>;

// /// A query object that get's built and then executed asynchronously
// pub(crate) struct StoreQuery<'store> {
//     store: &'store MsgStore,
//     user: Identity,
//     unread: bool,
//     tags: BTreeSet<Tag>,
//     service: Option<String>,
//     query: Option<MsgQuery>,
//     limit: Option<usize>,
// }

// impl<'store> StoreQuery<'store> {
//     /// Filter messages for unreads only
//     pub(crate) fn unread(self) -> Self {
//         Self {
//             unread: true,
//             ..self
//         }
//     }

//     /// Filter messages by association with a service
//     ///
//     /// This lookup uses message ispection and might be generally
//     /// slower than others.
//     pub(crate) fn service<S>(self, service: S) -> Self
//     where
//         S: Into<String>,
//     {
//         Self {
//             service: Some(service.into()),
//             ..self
//         }
//     }

//     pub(crate) fn limit(self, limit: usize) -> Self {
//         Self {
//             limit: Some(limit),
//             ..self
//         }
//     }

//     /// Filter messages additionally with a user provided query
//     pub(crate) fn constraints(self, query: MsgQuery) -> Self {
//         Self {
//             query: Some(query),
//             ..self
//         }
//     }

//     pub(crate) fn tags<I: IntoIterator<Item = Tag>>(mut self, tags: I) -> Self {
//         self.tags.extend(tags.into_iter());
//         self
//     }

//     /// Execute the query against the store
//     pub(crate) fn exec(self) -> Result<Vec<MsgRef>> {
//         let StoreQuery {
//             store,
//             user,
//             query,
//             unread,
//             tags,
//             service,
//             limit,
//         } = self;

//         store
//             .by_user
//             .write()
//             .unwrap()
//             .get_mut(&user)
//             .map_or(Ok(vec![]), |set| {
//                 Ok(set
//                     .iter_mut()
//                     // Conditional filters that are applied only if the query matches
//                     .filter(|msg| if unread { msg.unread() } else { true })
//                     .filter(|msg| {
//                         if let Some(ref s) = service {
//                             &msg.inner().associator == s
//                         } else {
//                             true
//                         }
//                     })
//                     .filter(|msg| msg.inner().tags.is_superset(&tags))
//                     .filter(|msg| match query {
//                         Some(MsgQuery::Id(ref id)) => &msg.inner().id == id,
//                         Some(MsgQuery::Sender(ref sender)) => &msg.inner().sender == sender,
//                         Some(MsgQuery::Tag(ref tag)) => msg.inner().tags.contains(tag),
//                         None => true,
//                     })
//                     .take(limit.unwrap_or(usize::max_value()))
//                     .map(|msg| msg.read())
//                     .collect())
//             })
//     }
// }

// /// Encodes the read-state of a Message
// ///
// /// The state is transformed when a query yields in this message being
// /// returned to an endpoint. At no point can the internal message
// /// store keep track if a message has actually been shown to a
// /// human. As such, the accuracy of this data might be flawed.
// #[derive(Clone)]
// pub(crate) enum MsgState {
//     /// A previously read message
//     Read(MsgRef),
//     /// An unread message
//     Unread(MsgRef),
// }

// impl MsgState {
//     /// Change state to read, while returning reference to inner message data
//     fn read(&mut self) -> MsgRef {
//         let msg = match self {
//             Self::Unread(msg) => msg,
//             Self::Read(msg) => msg,
//         };

//         let msg_ref = Arc::clone(&msg);
//         *self = Self::Read(Arc::clone(&msg));
//         msg_ref
//     }

//     fn unread(&self) -> bool {
//         match self {
//             Self::Unread(_) => true,
//             _ => false,
//         }
//     }

//     fn inner(&self) -> &MsgRef {
//         match self {
//             Self::Unread(msg) => msg,
//             Self::Read(msg) => msg,
//         }
//     }
// }

// /// A searchable index of messages encountered by this system
// #[derive(Clone)]
// pub(crate) struct MsgStore {
//     /// Owns Message references by their ID
//     by_id: MsgTree<MsgId, MsgState>,
//     /// By-user reference table to improve search performance
//     by_user: MsgTree<Identity, Vec<MsgState>>,
// }

// impl MsgStore {
//     pub(crate) fn new() -> Self {
//         Self {
//             by_id: Arc::new(RwLock::new(BTreeMap::default())),
//             by_user: Arc::new(RwLock::new(BTreeMap::default())),
//         }
//     }

//     /// Start a query for a given user
//     pub(crate) fn query(&self, user: Identity) -> StoreQuery {
//         StoreQuery {
//             user,
//             store: self,
//             unread: false,
//             tags: BTreeSet::new(),
//             service: None,
//             query: None,
//             limit: None,
//         }
//     }

//     /// Permanently store a whole Message
//     ///
//     /// For this function it doesn't matter if the `Message` was
//     /// dispatched by this system or has come in from outside.
//     pub(crate) fn insert(&self, user: Identity, msg: MsgState) {
//         let id = msg.inner().id.clone();

//         self.by_user
//             .write()
//             .expect("Failed to lock MsgStore!")
//             .entry(user)
//             .or_insert(vec![])
//             .push(msg.clone());

//         self.by_id
//             .write()
//             .expect("Failed to lock MsgStore!")
//             .insert(id, msg);
//     }
// }
