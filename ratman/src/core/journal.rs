use async_std::{sync::{Arc, RwLock}, task};
use netmod::{Frame, SeqId};
use std::collections::BTreeSet;

/// Remote frame journal
pub(crate) struct Journal {
    /// Keeps track of known frames to do reflood
    known: RwLock<BTreeSet<SeqId>>,
}

impl Journal {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            known: Default::default(),
        })
    }

    /// Dispatches a long-running task to run the journal logic
    pub(crate) fn run(self: Arc<Self>) {
        task::spawn(async move { loop {} });
    }

    /// Add a new frame to the known set
    pub(crate) async fn queue(&self, frame: Frame) {}

    /// Save a FrameID in the known journal page
    pub(crate) async fn save(&self, fid: &SeqId) {
        self.known.write().await.insert(fid.clone());
    }
    
    /// Checks if a frame ID has been seen before
    pub(crate) async fn known(&self, fid: &SeqId) -> bool {
        self.known.read().await.contains(fid)
    }
}
