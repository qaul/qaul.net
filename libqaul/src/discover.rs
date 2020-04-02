use crate::{
    messages::{MsgState, MsgUtils},
    utils::RunLock,
    Qaul,
};
use async_std::task;
use ratman::{netmod::Recipient, Identity, Router};
use std::{
    collections::BTreeMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

/// A thread-detached discovery service running inside libqaul
///
/// ## Required data
///
/// This internal service needs access to both the rest of the `Qaul`
/// structure to access external service registries and user stores,
/// as well as the underlying `Router` of a platform to send messages
/// to and receive from.
///
/// ## Startup
///
/// Startup procedure works pretty closely to how a `Router` is
/// initialised in `ratman`, where initialisation spawns threads, and
/// returns channel endpoints to send messages to the Discovery service.
///
/// Available messages are encoded in the DiscCmd enum.
#[derive(Clone)]
pub(crate) struct Discovery;

impl Discovery {
    /// Start a discovery service running inside libqaul
    pub(crate) fn start(qaul: Arc<Qaul>, router: Arc<Router>) {
        let run = Arc::new(RunLock::new(true));

        // Incoming message handler
        Self::inc_handler(Arc::clone(&qaul), Arc::clone(&router), Arc::clone(&run));

        // Handle new users
        task::spawn(async move {
            loop {
                let id = router.discover().await;
                info!(id = id.to_string().as_str(), "Discovered user!");
                qaul.users.discover(id);
            }
        });
    }

    /// Spawns a thread that listens to incoming messages
    #[instrument(skip(qaul, router, _lock), level = "info")]
    fn inc_handler(qaul: Arc<Qaul>, router: Arc<Router>, _lock: Arc<RunLock>) {
        task::spawn(async move {
            loop {
                let msg = router.next().await;

                info!("Receiving message...");
                let user = match msg.recipient {
                    Recipient::User(id) => id.clone(),
                    Recipient::Flood => unimplemented!(),
                };

                let msg = Arc::new(MsgUtils::process(user, msg));
                let associator = msg.associator.clone();

                qaul.messages
                    .insert(user, MsgState::Unread(Arc::clone(&msg)));
                qaul.services.push_for(associator, msg).unwrap();
                info!("Finished processing incoming message!");
            }
        });
    }
}
