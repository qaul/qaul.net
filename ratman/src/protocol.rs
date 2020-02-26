//! Protocol generation module
//!
//! The routing protocol, and micro messages (analogous to micro
//! code), are much better documented in the `R.A.T.M.A.N.` design
//! specification/paper. But here's a brief overview, and
//! implementation:
//!
//! - `Announce` is sent when a node comes online
//! - `Sync` is a reply to an `Announce`, only omitted when `no_sync` is set

use crate::{
    data::{Message, MsgId},
    error::{Error, Result},
    Core,
};
use async_std::{
    sync::{Arc, Mutex},
    task,
};
use conjoiner;
use identity::Identity;
use netmod::{Frame, Recipient};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

/// A payload that represents a RATMAN-protocol message
#[derive(Debug, Serialize, Deserialize)]
enum ProtoPayload {
    /// A network-wide announcement message
    Announce { id: Identity, no_sync: bool },
}

/// Provide a builder API to construct different types of Messages
#[derive(Default)]
pub(crate) struct Protocol {
    online: Mutex<BTreeMap<Identity, Arc<AtomicBool>>>,
}

impl Protocol {
    pub(crate) fn new() -> Arc<Self> {
        Default::default()
    }

    /// Dispatch a task to announce a user periodically
    pub(crate) async fn online(self: Arc<Self>, id: Identity, core: Arc<Core>) -> Result<()> {
        let mut map = self.online.lock().await;
        if map.contains_key(&id) {
            return Err(Error::DuplicateUser);
        }

        let b = Arc::new(AtomicBool::new(true));
        map.insert(id, Arc::clone(&b));
        drop(map);

        task::spawn(async move {
            loop {
                core.send(Self::announce(id)).await;
                task::sleep(Duration::from_secs(2)).await;

                if !b.load(Ordering::Relaxed) && break {}
            }

            // Remove the runtime bool again
            self.online.lock().await.remove(&id);
        });

        Ok(())
    }

    pub(crate) async fn offline(&self, id: Identity) -> Result<()> {
        self.online
            .lock()
            .await
            .get(&id)
            .map(|b| b.swap(false, Ordering::Relaxed))
            .map_or(Err(Error::NoUser), |_| Ok(()))
    }

    /// Try to parse a frame as an announcement
    pub(crate) fn is_announce(f: &Frame) -> Option<Identity> {
        let Frame { ref payload, .. } = f;

        conjoiner::deserialise(payload)
            .map(|p| match p {
                ProtoPayload::Announce { id, .. } => id,
            })
            .ok()
    }

    /// Build an announcement message for a user
    fn announce(sender: Identity) -> Message {
        let payload = conjoiner::serialise(&ProtoPayload::Announce {
            id: sender,
            no_sync: true,
        })
        .unwrap();

        Message {
            id: MsgId::random(),
            recipient: Recipient::Flood,
            sender,
            payload,
        }
    }
}
