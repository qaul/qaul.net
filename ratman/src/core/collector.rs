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
        let omap: ArcRw<BTreeMap<SeqId, Sender<Frame>>> = Default::default();

        task::spawn(async move {
            while let Some((fid, f)) = inc.recv().await {
                let mut map = omap.lock().await;

                // Check if a handler was already spawned for SeqId
                if map.contains_key(&fid) {
                    map.get(&fid).unwrap().send(f).await;
                } else {
                    // Otherwise we spawn the handler
                    let done = done.clone();
                    let (tx, rx) = channel(1);
                    map.insert(fid, tx);
                    let map = Arc::clone(&omap);

                    task::spawn(async move {
                        let mut buf: Vec<Frame> = vec![];
                        while let Some(f) = rx.recv().await {
                            match join_frames(&mut buf, f) {
                                // If the sequence was complete, clean up handlers
                                Some(msg) => {
                                    done.send(msg).await;
                                    map.lock().await.remove(&fid);
                                    break;
                                }
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
fn join_frames(buf: &mut Vec<Frame>, new: Frame) -> Option<Message> {
    // Insert the frame
    buf.push(new);

    // Sort by sequence numbers
    buf.sort_by(|a, b| a.seq.num.cmp(&b.seq.num));

    // The last frame needs to point to `None`
    if buf.last().unwrap().seq.next.is_some() {
        return None;
    }
    
    // Test inductive sequence number property
    if buf.iter().enumerate().fold(true, |status, (i, frame)| {
        status && (frame.seq.num == i as u32)
    }) {
        let id = buf[0].seq.seqid;
        let sender = buf[0].sender;
        let recipient = buf[0].recipient;
        let payload = SeqBuilder::restore(buf);

        Some(Message {
            id,
            sender,
            recipient,
            payload,
            signature: vec![],
        })
    } else {
        None
    }
}

#[test]
fn join_frame_simple() {
    use identity::Identity;
    use netmod::Recipient;

    let sender = Identity::random();
    let resp = Identity::random();
    let seqid = Identity::random();

    let mut seq = SeqBuilder::new(sender, Recipient::User(resp), seqid)
        .add((0..10).into_iter().collect())
        .add((10..20).into_iter().collect())
        .add((20..30).into_iter().collect())
        .build();

    // The function expects a filling buffer
    let mut buf = vec![];

    assert!(join_frames(&mut buf, seq.remove(0)) == None);
    assert!(join_frames(&mut buf, seq.remove(1)) == None); // Insert out of order
    assert!(join_frames(&mut buf, seq.remove(0)).is_some());
}
