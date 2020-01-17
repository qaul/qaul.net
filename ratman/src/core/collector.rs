use async_std::{sync::Arc, task};

/// Local frame collector
pub(crate) struct Collector {
    frags: Vec<()>,
}

impl Collector {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self { frags: vec![] })
    }

    /// Poll the collector for new messages addressed to local users
    pub(crate) async fn completed(&self) -> () {}

    /// Dispatches a long-running task to run the collection logic
    pub(crate) fn run(self: Arc<Self>) {
        task::spawn(async move { loop {} });
    }
}
