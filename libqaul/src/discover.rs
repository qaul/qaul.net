use crate::{Qaul, utils::RunLock};
use async_std::task;
use ratman::{Identity, Protocol, Message, Router};
use std::{
    sync::{
        mpsc::{channel, Sender, Receiver},
        Arc, RwLock,
    },
    thread,
    time::Duration,
    collections::BTreeMap,
};

/// Encode available Discovery commands
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
    pub(crate) fn start(_qaul: Arc<Qaul>, router: Arc<Router>, inc: Receiver<Message>,) -> Sender<DiscCmd> {
        let run = Arc::new(RunLock::new(true));
        let (sender, rx) = channel();
        
        // Spawn the service communicator
        thread::spawn(move || {
            let run = Arc::clone(&run);
            let buf = Arc::new(RwLock::new(BTreeMap::new()));
            
            while let Ok(rscv) = rx.recv() {
                match rscv {
                    DiscCmd::Shutdown => {
                        run.set(false);
                        break;
                    },
                    DiscCmd::Start(id) => {
                        let router = Arc::clone(&router);
                        let run = Arc::clone(&run);

                        buf.write().unwrap().insert(id, Arc::new(RunLock::new(true)));
                        let buf = Arc::clone(&buf);
                        
                        task::spawn(async move {
                            while active(&id, &buf).and(&run) {
                                task::sleep(Duration::from_secs(2)).await;
                                router.send(Protocol::announce(id.clone())).unwrap();
                            }
                        });
                    },
                    DiscCmd::Stop(id) => {
                        buf.write().unwrap().get_mut(&id).unwrap().set(false);
                    },
                }
            }
        });

        // Incoming Message handler
        thread::spawn(move || {
            while let Ok(msg) = inc.recv() {
                dbg!(msg);
            }
        });

        sender
    }
}

/// Convenience function to get the RunLock for a specific user session
fn active(id: &Identity, buf: &Arc<RwLock<BTreeMap<Identity, Arc<RunLock>>>>) -> Arc<RunLock> {
    Arc::clone(&buf.read().unwrap().get(id).unwrap())
}
