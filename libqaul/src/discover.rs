use async_std::task;
use ratman::{Protocol, Message, Router, Identity};
use std::{
    sync::{mpsc::Receiver, Arc},
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Clone)]
pub(crate) struct Discovery {
    router: Arc<Router>,
    worker: Arc<JoinHandle<()>>,
}

impl Discovery {

    /// Enable the announcement cycle for an active User session
    ///
    /// Spawns an async task on the local executor. Currently there's
    /// no way of ending an active session and there's no validation
    /// done by this task to identity when an Announce should be
    /// stopped.  This needs to be refactored but can be done at a
    /// later point.
    pub(crate) fn start_announce(&self, id: Identity) {
        let router = Arc::clone(&self.router);
        task::spawn(async move {
            loop {
                task::sleep(Duration::from_secs(2)).await;
                router.send(Protocol::announce(id.clone())).unwrap();
            }
        });
    }
    
    /// Initialise and start Discovery feature
    pub(crate) fn new(router: Arc<Router>, recvr: Receiver<Message>) -> Self {
        Self {
            router,
            worker: Arc::new(thread::spawn(move || {
                while let Ok(msg) = recvr.recv() {
                    dbg!(msg);
                }
            })),
        }
    }
}
