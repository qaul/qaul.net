use crate::{
    messages::{MsgState, MsgUtils},
    utils::RunLock,
    Qaul,
};
use async_std::task;
use ratman_netmod::{Recipient};
use ratman::{Identity, Message, Router};
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
    pub(crate) fn start(
        qaul: Arc<Qaul>,
        router: Arc<Router>,
        inc: Receiver<Message>,
    ) -> Sender<DiscCmd> {
        let run = Arc::new(RunLock::new(true));
        let (sender, rx) = channel();

        // Incoming message handler
        Self::inc_handler(qaul, inc, Arc::clone(&run));

        // Spawn the service communicator
        Self::service_handle(rx, router, run);

        sender
    }

    /// Spawns the service internal handler
    fn service_handle(rx: Receiver<DiscCmd>, router: Arc<Router>, run: Arc<RunLock>) {
        thread::spawn(move || {
            let run = Arc::clone(&run);
            let buf = Arc::new(RwLock::new(BTreeMap::new()));

            while let Ok(rscv) = rx.recv() {
                match rscv {
                    DiscCmd::Shutdown => {
                        run.set(false);
                        break;
                    }
                    DiscCmd::Start(id) => {
                        let router = Arc::clone(&router);
                        let run = Arc::clone(&run);

                        buf.write()
                            .unwrap()
                            .insert(id, Arc::new(RunLock::new(true)));
                        let buf = Arc::clone(&buf);

                        task::spawn(async move {
                            while active(&id, &buf).and(&run) {
                                task::sleep(Duration::from_secs(2)).await;
                                router.send(Protocol::announce(id.clone())).await.unwrap();
                            }
                        });
                    }
                    DiscCmd::Stop(id) => {
                        buf.write().unwrap().get_mut(&id).unwrap().set(false);
                    }
                }
            }
        });
    }

    /// Spawns a thread that listens to incoming messages
    fn inc_handler(qaul: Arc<Qaul>, inc: Receiver<Message>, _lock: Arc<RunLock>) {
        thread::spawn(move || {
            let qaul = Arc::clone(&qaul);

            while let Ok(msg) = inc.recv() {
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
