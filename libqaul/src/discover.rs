//!

use ratman::Router;
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

#[derive(Clone)]
pub(crate) struct Discovery {
    router: Option<Arc<Router>>,
    worker: Option<Arc<JoinHandle<()>>>,
}

impl Discovery {
    /// Create empty Discovery daemon
    ///
    /// Don't forget to initialise later!
    pub(crate) fn missing() -> Self {
        Self {
            router: None,
            worker: None,
        }
    }

    /// Initialise and start Discovery feature
    pub(crate) fn new(r: Arc<Router>) -> Self {
        Self {
            router: Some(r),
            worker: Some(Arc::new(thread::spawn(|| {
                // Handle incoming `Announce` messages here
            }))),
        }
    }
}
