//!

use ratman::{Message, Router};
use std::{
    sync::{mpsc::Receiver, Arc},
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
    pub(crate) fn new(r: Arc<Router>, router: Receiver<Message>) -> Self {
        Self {
            router: Some(r),
            worker: Some(Arc::new(thread::spawn(move || {
                while let Ok(msg) = router.recv() {
                    dbg!(msg);
                }
            }))),
        }
    }
}
