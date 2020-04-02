//! The collector worker

use super::{Locked, State};
use crate::{Message, Payload};
use async_std::sync::Arc;
use netmod::{Frame, SeqBuilder, SeqId};

/// A self contained sub-task that collects frames into messages
pub(super) struct Worker {
    /// The sequence of the message being collected
    seq: SeqId,
    /// The buffer of existing messages
    buf: Locked<Vec<Frame>>,
    /// Collector reference for control flow
    parent: Arc<State>,
}

impl Worker {
    /// Create a new collector task for a collector parent
    pub(super) fn new(seq: SeqId, parent: Arc<State>) -> Self {
        Self {
            seq,
            parent,
            buf: Default::default(),
        }
    }

    /// Poll for new frames to assemble from the frame pool
    #[instrument(skip(self), level = "info")]
    pub(crate) async fn poll(&self) -> Option<()> {
        info!("Polling for new work to be done");
        let frame = self.parent.get(&self.seq).await;
        let mut buf = self.buf.lock().await;

        info!("Joining frames");
        if let Some(msg) = join_frames(&mut buf, frame) {
            self.parent.finish(msg).await;
            None
        } else {
            Some(())
        }
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
        let layered = SeqBuilder::restore(buf);
        let Payload { payload, sign } = conjoiner::deserialise(&layered).unwrap();

        Some(Message {
            id,
            sender,
            recipient,
            payload,
            sign,
        })
    } else {
        None
    }
}

#[cfg(test)]
use identity::Identity;
#[cfg(test)]
use netmod::Recipient;

#[test]
fn join_frame_simple() {
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
