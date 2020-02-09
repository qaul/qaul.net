//! libqaul api scopes

pub use contacts::Contacts;
pub(crate) mod contacts;

pub use files::Files;
pub(crate) mod files;

pub use messages::Messages;
pub(crate) mod messages;

pub use services::Services;
pub(crate) mod services;

pub use users::Users;
pub(crate) mod users;

use serde::{Serialize, Deserialize};

/// A generic metadata tag
///
/// Because searching through message or file payloads might be slow,
/// and I/O intensive (especially within thi secret storage module),
/// all public types have a tag metadata interface.  These are
/// included in the wire-format, meaning that they will get
/// transferred across to another node.
///
/// This can be used to implement things like conversation ID's,
/// In-Reply-To, and more.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Tag {
    /// A string key for a tag
    pub key: String,
    /// Some binary data that is up to a service to interpret
    pub val: Vec<u8>,
}

impl Tag {
    /// Create a new MsgTag with key and value
    pub fn new<K, I>(key: K, val: I) -> Self
    where
        K: Into<String>,
        I: IntoIterator<Item = u8>,
    {
        Self {
            key: key.into(),
            val: val.into_iter().collect(),
        }
    }
}
