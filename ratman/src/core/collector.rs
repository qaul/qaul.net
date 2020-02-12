use crate::Message;
use async_std::{
    sync::{channel, Arc, Mutex, Receiver, Sender},
    task,
};
use netmod::{Frame, SeqBuilder, SeqId};
use std::collections::BTreeMap;

type ArcRw<T> = Arc<Mutex<T>>;

/// Local frame collector
pub(crate) struct Collector {
    frags: Sender<(SeqId, Frame)>,
    done: Receiver<Message>,
}

impl Collector {
    pub(crate) fn new() -> Arc<Self> {
        let (frags, rx) = channel(5);
        let (tx, done) = channel(1);

        let this = Arc::new(Self { frags, done });
        Arc::clone(&this).run(rx, tx);
        this
    }

    /// Poll the collector for new messages addressed to local users
    pub(crate) async fn completed(&self) -> Message {
        self.done.recv().await.unwrap()
    }

    /// Dispatches a long-running task to run the collection logic
    pub(crate) fn run(self: Arc<Self>, inc: Receiver<(SeqId, Frame)>, done: Sender<Message>) {
        let map: ArcRw<BTreeMap<SeqId, Sender<Frame>>> = Default::default();

        task::spawn(async move {
            while let Some((fid, f)) = inc.recv().await {
                let mut map = map.lock().await;

                // Check if a handler was already spawned for SeqId
                if map.contains_key(&fid) {
                    map.get(&fid).unwrap().send(f).await;
                } else {
                    // Otherwise we spawn the handler
                    let done = done.clone();
                    let (tx, rx) = channel(1);
                    map.insert(fid, tx);

                    task::spawn(async move {
                        let mut buf: Vec<Frame> = vec![];
                        while let Some(f) = rx.recv().await {
                            match join_frames(&mut buf) {
                                Some(msg) => done.send(msg).await,
                                None => continue,
                            }
                        }
                    });
                }
            }
        });
    }

    /// Enqueue a frame to be desequenced
    pub(crate) async fn queue(&self, f: Frame) {
        let seqid = f.seq.seqid;
        self.frags.send((seqid, f)).await;
    }
}

/// Utility function that uses the SeqBuilder to rebuild Sequence
fn join_frames(buf: &mut Vec<Frame>) -> Option<Message> {
    None
}
