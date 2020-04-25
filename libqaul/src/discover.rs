use crate::{
    messages::{MsgUtils},
    Qaul,
};
use alexandria::utils::Tag;
use async_std::task;
use ratman::{netmod::Recipient, Router};
use std::sync::Arc;
use tracing::info;

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
        // Incoming message handler
        Self::inc_handler(Arc::clone(&qaul), Arc::clone(&router));

        // Handle new users
        task::spawn(async move {
            loop {
                let id = router.discover().await; // FIXME: Do we still need this?
                info!(id = id.to_string().as_str(), "Discovered user!");
                qaul.users
                    .insert_profile(id, vec![Tag::empty("profile")])
                    .await;
            }
        });
    }

    /// Spawns a thread that listens to incoming messages
    #[tracing::instrument(skip(qaul, router), level = "info")]
    fn inc_handler(qaul: Arc<Qaul>, router: Arc<Router>) {
        task::spawn(async move {
            loop {
                let msg = router.next().await;

                info!("Receiving message...");
                let recp = match msg.recipient {
                    Recipient::User(id) => Some(id),
                    Recipient::Flood => None,
                };

                let msg = Arc::new(MsgUtils::process(msg));
                let associator = msg.associator.clone();

                qaul.messages
                    .insert_remote(recp, Arc::clone(&msg))
                    .await;
                qaul.services.push_for(associator, msg).unwrap();
                info!("Finished processing incoming message!");
            }
        });
    }
}
