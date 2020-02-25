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

/// Encode available  commands
#[allow(unused)]
pub(crate) enum DiscCmd {
    /// Start announcing a user ID
    Start(Identity),
    /// Stop announcing a user ID
    Stop(Identity),
    /// Signal the discovery to stop operations
    Shutdown,
}

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
    pub(crate) fn start(qaul: Arc<Qaul>, router: Arc<Router>) -> Sender<DiscCmd> {
        let run = Arc::new(RunLock::new(true));
        let (sender, rx) = channel();

        // Incoming message handler
        Self::inc_handler(qaul, Arc::clone(&router), Arc::clone(&run));

        sender
    }

    /// Spawns a thread that listens to incoming messages
    fn inc_handler(qaul: Arc<Qaul>, router: Arc<Router>, _lock: Arc<RunLock>) {
        task::spawn(async move {
            loop {
                let msg = router.next().await;

                println!("Receiving message...");
                let user = match msg.recipient {
                    Recipient::User(id) => id.clone(),
                    Recipient::Flood => unimplemented!(),
                };

                let msg = Arc::new(MsgUtils::process(msg, user));
                let associator = msg.associator.clone();

                qaul.messages
                    .insert(user, MsgState::Unread(Arc::clone(&msg)));
                qaul.services.push_for(associator, msg).unwrap();
                println!("Finished processing incoming message!");
            }
        });
    }
}

/// Convenience function to get the RunLock for a specific user session
fn active(id: &Identity, buf: &Arc<RwLock<BTreeMap<Identity, Arc<RunLock>>>>) -> Arc<RunLock> {
    Arc::clone(&buf.read().unwrap().get(id).unwrap())
}
