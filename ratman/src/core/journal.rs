use async_std::{sync::Arc, task};

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
}
