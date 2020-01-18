use async_std::{
    sync::{Arc, Mutex},
    task,
};
use netmod::{Frame, SeqId as SeqData};
use std::collections::BTreeMap;
use crate::{Message, MsgId};

/// This module is the only one that addresses this as the seqId, and
/// the overall SeqChain as SeqData.
type SeqId = [u8; 16];

/// Local frame collector
#[derive(Default)]
pub(crate) struct Collector {
    frags: Mutex<BTreeMap<SeqId, Vec<Frame>>>,
}

impl Collector {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Poll the collector for new messages addressed to local users
    pub(crate) async fn completed(&self) -> Message {
        unimplemented!()
    }

    /// Dispatches a long-running task to run the collection logic
    pub(crate) fn run(self: Arc<Self>) {
        task::spawn(async move {});
    }

    /// Enqueue a frame to be desequenced
    pub(crate) async fn queue(&self, f: Frame) {
        let seqid = f.seqid.seqid;
        self.frags.lock().await.entry(seqid).or_default().push(f);
    }
}
