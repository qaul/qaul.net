//! Internal message store wrapper

use crate::{
    helpers::QueryResult,
    messages::{Message, Mode, MsgQuery, MsgRef},
    services::Service,
    Identity,
};
use alexandria::{
    query::Query,
    utils::{Path, Tag, TagSet},
    Library, Session, GLOBAL,
};
use async_std::sync::Arc;

pub(crate) const TAG_FLOOD: &'static str = "libqaul._int.flood";
pub(crate) const TAG_READ: &'static str = "libqaul._int.read";
pub(crate) const TAG_SENDER: &'static str = "libqaul._int.sender";
pub(crate) const TAG_SERVICE: &'static str = "libqaul._int.service";

fn msg_path(msg_id: Identity) -> Path {
    Path::from(format!("/msg:{}", msg_id))
}

fn sender_tag(sender: Identity) -> Tag {
    Tag::new(TAG_SENDER, sender.as_bytes().to_vec())
}

fn service_tag(s: String) -> Tag {
    Tag::new(TAG_SERVICE, s.as_bytes().to_vec())
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
        tags.insert(sender_tag(user));
        tags.insert(service_tag(msg.associator.clone()));

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
    pub(crate) async fn insert_remote(&self, recipient: Option<Identity>, msg: MsgRef) {
        let mut tags = msg.tags.clone();
        tags.insert(sender_tag(msg.sender));
        tags.insert(service_tag(msg.associator.clone()));

        let diffs = msg.diff();
        let session = match recipient {
            Some(id) => Session::Id(id),
            None => {
                tags.insert(Tag::empty(TAG_FLOOD));
                GLOBAL
            }
        };

        self.inner
            .batch(session, msg_path(msg.id), tags, diffs)
            .await
            .unwrap();
    }

    /// Return items from alexandria via a user query
    pub(crate) async fn query(
        &self,
        user: Identity,
        service: Service,
        query: MsgQuery,
    ) -> QueryResult<Message> {
        // Add the service tag to the set
        let mut meta = match service {
            Service::Name(s) => service_tag(s).into(),
            Service::God => TagSet::empty(),
        };

        // Add the sender tag to the query
        if let Some(sender) = query.sender {
            meta.insert(sender_tag(sender));
        }

        // Add any other tags
        let tags = meta.merge(query.tags);

        // Make db query
        let mut glb = self
            .inner
            .query_iter(GLOBAL, Query::tags().subset(tags.clone()))
            .await
            .unwrap();
        let usr = self
            .inner
            .query_iter(user, Query::tags().subset(tags))
            .await
            .unwrap();

        glb.merge(usr).unwrap();
        QueryResult::new(glb)
    }
}

#[cfg(test)]
mod harness {
    use crate::{
        messages::{generator::MsgBuilder, Mode, MsgStore},
        security::Sec,
        users::{UserStore, TAG_PROFILE},
        Identity,
    };
    use alexandria::{
        utils::{Tag, TagSet},
        Builder,
    };
    use async_std::sync::Arc;
    use tempfile::tempdir;

    pub(super) use crate::error::Result;

    pub(super) struct Test {
        pub(super) store: MsgStore,
        usr: UserStore,
    }

    pub(super) fn init() -> Test {
        let dir = tempdir().unwrap();
        let lib = Builder::new().offset(dir.path()).build().unwrap();

        // Because the message store requires user namespaces, it
        // depends on the user store initialising first!  We then also
        // keep it around to insert users as needs be.
        Test {
            usr: UserStore::new(Arc::clone(&lib)),
            store: MsgStore::new(lib),
        }
    }

    /// Insert a new user into the store if required
    async fn maybe_make_user(state: &Test, id: Identity, local: bool) {
        match state.usr.get(id).await {
            Err(_) if local => {
                make_user(state, id).await;
            }
            Err(_) => {
                state.usr.insert_profile(id, Tag::empty(TAG_PROFILE)).await;
            }
            _ => {}
        }
    }

    /// Make a user to use in tests
    pub(super) async fn make_user(state: &Test, id: Identity) {
        let mut kid = Sec::new().generate().await;
        kid.id = id;
        state
            .usr
            .create_local(kid, "car horse battery staple")
            .await;
    }

    /// "Send" a random message with specific tags
    pub(super) async fn send_with_tags(
        state: &Test,
        user: Identity,
        tags: impl Into<TagSet>,
        mode: Mode,
    ) {
        maybe_make_user(state, user, true).await;

        let msg = MsgBuilder::new()
            .with_sender(user)
            .with_tags(tags.into())
            .generate();
        state.store.insert_local(user, Arc::new(msg), mode).await;
    }

    pub(super) async fn receive_with_tags(
        state: &Test,
        user: impl Into<Option<Identity>>,
        tags: impl Into<TagSet>,
    ) {
        let user = user.into();
        if let Some(user) = user {
            maybe_make_user(state, user, false).await;
        }

        let msg = match user {
            Some(id) => MsgBuilder::new().with_sender(id),
            None => MsgBuilder::new(),
        }
        .with_tags(tags.into())
        .generate();

        state.store.insert_remote(user.into(), Arc::new(msg)).await;
    }
}

#[async_std::test]
async fn simple_send() -> harness::Result<()> {
    let state = harness::init();
    let id = Identity::random();
    let tags = TagSet::empty();
    harness::send_with_tags(&state, id, tags, Mode::Std(Identity::random())).await;

    let result = state
        .store
        .query(id, Service::God, MsgQuery::new().sender(id))
        .await;
    assert_eq!(result.take(1).await?.len(), 1);
    Ok(())
}

#[async_std::test]
async fn simple_received() -> harness::Result<()> {
    let state = harness::init();
    let id = Identity::random();
    harness::make_user(&state, id).await;

    let tags = TagSet::empty();
    harness::receive_with_tags(&state, id, tags).await;

    let result = state
        .store
        .query(id, Service::God, MsgQuery::new().sender(id))
        .await;
    assert_eq!(result.take(1).await?.len(), 1);
    Ok(())
}
