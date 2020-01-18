use async_std::{sync::Arc, task};
use netmod::Frame;

/// Remote frame journal
pub(crate) struct Journal {}

impl Journal {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    /// Dispatches a long-running task to run the journal logic
    pub(crate) fn run(self: Arc<Self>) {
        task::spawn(async move { loop {} });
    }

    pub(crate) async fn queue(&self, frame: Frame) {}

    /// Checks if the provided frame ID is unique (not present) in the store
    pub(crate) async fn unique(&self, seqid: &[u8; 16]) -> bool {
        true
    }
}
