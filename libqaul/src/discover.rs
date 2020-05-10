use crate::{messages::MsgUtils, users::TAG_PROFILE, Qaul};
use alexandria::utils::Tag;
use async_std::task;
use ratman::{netmod::Recipient, Router};
use std::sync::Arc;
use tracing::{info, warn, debug};

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
                let id = router.discover().await;
                debug!(id = id.to_string().as_str(), "Received announcement!");

                if !qaul.users.known_remote().await.contains(&id) {
                    info!(id = id.to_string().as_str(), "Discovered new user!");
                    qaul.users
                        .insert_profile(id, vec![Tag::empty(TAG_PROFILE)])
                        .await;
                }
            }
        });
    }

    /// Spawns a thread that listens to incoming messages
    #[tracing::instrument(skip(qaul, router), level = "info")]
    fn inc_handler(qaul: Arc<Qaul>, router: Arc<Router>) {
        task::spawn(async move {
            loop {
                let msg = router.next().await;
                let sender = msg.sender;

                println!("Receiving message by `{}`...", sender);
                let recp = match msg.recipient {
                    Recipient::User(id) => Some(id),
                    Recipient::Flood => None,
                };

                let msg = match MsgUtils::process(msg, &qaul.users).await {
                    Ok(msg) => Arc::new(msg),
                    Err(_) => {
                        println!("Skipping malformed message by `{}`", sender);
                        continue;
                    }
                };

                qaul.messages.insert_remote(recp, Arc::clone(&msg)).await;
                println!("Finished processing incoming message!");
            }
        });
    }
}
