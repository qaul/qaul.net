//! Protocol generation module
//!
//! The routing protocol, and micro messages (analogous to micro
//! code), are much better documented in the `R.A.T.M.A.N.` design
//! specification/paper. But here's a brief overview, and
//! implementation:
//!
//! - `Announce` is sent when a node comes online
//! - `Sync` is a reply to an `Announce`, only omitted when `no_sync` is set

use crate::data::{Message, MsgId};
use identity::Identity;
use netmod::Recipient;
use serde::{Deserialize, Serialize};

const ASSOCIATOR: &'static str = ".internal";

/// A payload that represents a RATMAN-protocol message
#[derive(Serialize, Deserialize)]
pub(crate) enum ProtoPayload {
    /// A network-wide announcement message
    Announce { id: Identity, no_sync: bool },
    /// A 1-to-1 routing table sync message
    Sync { id: Identity, table: Vec<Identity> },
}

/// Provide a builder API to construct different types of Messages
pub struct Protocol;

impl Protocol {
    /// Build an announcement message for a user
    pub fn announce(sender: Identity) -> Message {
        Message::build_signed(
            MsgId::random(),
            sender,
            Recipient::Flood,
            vec![0],
        )
    }

    /// Build a message that synchronises routing table state
    pub fn sync_rt(sender: Identity, recipient: Identity, _known: Vec<Identity>) -> Message {
        Message::build_signed(
            MsgId::random(),
            sender.clone(),
            Recipient::User(recipient),
            vec![0],
            // ProtoPayload::Sync {
            //     id: sender,
            //     table: known,
            // },
        )
    }
}
