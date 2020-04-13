use crate::{
    delta::Delta,
    utils::{Path, Query},
};
use async_std::{
    sync::{channel, Arc, Receiver, RwLock, Sender},
    task,
};
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicUsize, Ordering},
};

pub type SubId = usize;

/// A management task collection for subscriptions
///
/// On every database operation, a Delta object is pushed into the
/// inbox, and then pushed out to all waiting subscription handlers.
/// They then internally filter based on their user query and notify
/// the waiter of changes.
pub(crate) struct SubHub {
    curr: AtomicUsize,
    inbox: Sender<Delta>,
    subs: RwLock<BTreeMap<SubId, Sender<Delta>>>,
}

impl SubHub {
    pub(crate) fn new() -> Arc<Self> {
        let (inbox, notify) = channel(2);

        let arc = Arc::new(Self {
            curr: 0.into(),
            inbox,
            subs: Default::default(),
        });

        {
            let arc = Arc::clone(&arc);
            task::spawn(async move {
                while let Some(d) = notify.recv().await {
                    let subs = arc.subs.read().await;
                    for (_, sub) in &*subs {
                        sub.send(d.clone()).await;
                    }
                }
            });
        }

        arc
    }

    pub(crate) async fn queue(&self, d: Delta) {
        self.inbox.send(d).await
    }

    pub(crate) async fn rm_sub(&self, id: SubId) {
        self.subs.write().await.remove(&id);
    }

    pub(crate) async fn add_sub(self: &Arc<Self>, query: Query) -> Subscription {
        let id = self.curr.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = channel(1);

        self.subs.write().await.insert(id, tx);
        Subscription::new(&self, id, query, rx)
    }
}

/// A subscription is created for a query filter and notifies pollers
pub struct Subscription {
    /// The query this subscription is filtering for
    pub query: Query,

    cb_hub: Arc<SubHub>,
    id: SubId,
    poll: Receiver<Path>,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        task::block_on(async move {
            self.cb_hub.rm_sub(self.id).await;
        })
    }
}

impl Subscription {
    pub(crate) fn new(hub: &Arc<SubHub>, id: SubId, query: Query, notify: Receiver<Delta>) -> Self {
        let query = query;
        let (re_notify, poll) = channel(1);

        {
            let query = query.clone();
            task::spawn(async move {
                while let Some(d) = notify.recv().await {
                    let d: Delta = d;
                    if match query {
                        Query::Path(ref p) => p == &d.path,
                        _ => unimplemented!(),
                    } {
                        re_notify.send(d.path).await;
                    }
                }
            });
        }

        Self {
            cb_hub: Arc::clone(hub),
            poll,
            query,
            id,
        }
    }

    /// Get the next `Path` segment in the query that had a transaction
    pub async fn next(&self) -> Path {
        self.poll.recv().await.unwrap()
    }
}

#[cfg(test)]
use crate::utils::TagSet;

#[cfg(test)]
struct SubTest {
    hub: Arc<SubHub>,
    path: Path,
}

#[cfg(test)]
impl SubTest {
    fn new<P: Into<Path>>(p: P) -> Self {
        SubTest {
            hub: SubHub::new(),
            path: p.into(),
        }
    }

    fn sub(&self, query: Query) -> Subscription {
        task::block_on(async { self.hub.add_sub(query).await })
    }

    fn insert<T: Into<Option<TagSet>>>(&self, ts: T) {
        use crate::delta::{DeltaBuilder, DeltaType};
        let delta = {
            let mut db = DeltaBuilder::new(None, DeltaType::Insert);
            db.path(&self.path);
            if let Some(ref ts) = ts.into() {
                db.tags(ts);
            }
            db.make()
        };

        let hub = Arc::clone(&self.hub);
        task::spawn(async move { hub.queue(delta).await });
    }

    fn path(&self) -> Path {
        self.path.clone()
    }
}

#[cfg(test)]
fn poll(sub: &Subscription) -> Path {
    task::block_on(async { sub.next().await })
}

#[test]
fn single_delta() {
    let test = SubTest::new("/msg:bob");

    let sub = test.sub(Query::Path(test.path.clone()));

    test.insert(None);

    assert_eq!(poll(&sub), test.path);
}
