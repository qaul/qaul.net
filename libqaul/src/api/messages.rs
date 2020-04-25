use crate::{
    error::{Error, Result},
    helpers::{QueryResult, Subscription, Tag, TagSet},
    messages::{Envelope, MsgUtils, RatMessageProto},
    qaul::{Identity, Qaul},
    services::Service,
    users::UserAuth,
};

use ratman::netmod::Recipient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A reference to an internally stored message object
pub type MsgRef = Arc<Message>;

/// Length of an `MsgId`, for converting to and from arrays
pub const ID_LEN: usize = 16;

/// A unique, randomly generated message ID
pub type MsgId = Identity;

/// Signature trust level of an incoming `Message`
///
/// The three variants encode `trusted`, `unverified` and `invalid`,
/// according to signature verification of the internal keystore.
///
/// The `SigTrust::ok` convenience function can be used to reject
/// non-verifiable (unknown or bad) `Message` signatures.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SigTrust {
    /// A verified signature by a known contact
    Trusted,
    /// An unverified signature by a known contact
    /// (pubkey not available!)
    Unverified,
    /// A fraudulent signature
    Invalid,
}

impl SigTrust {
    pub fn ok(&self) -> Result<()> {
        match self {
            Self::Trusted => Ok(()),
            Self::Unverified => Err(Error::NoSign),
            Self::Invalid => Err(Error::BadSign),
        }
    }
}

/// Specify the way that a message gets dispatched
///
/// This information is only needed during transmission, because the
/// message should later be associated with some other metadata
/// provided by your service (or just the message ID).
///
/// When sending a flooded message, it becomes publicly accessible for
/// everybody on this node, and will most likely be stored in plain
/// text on receiving nodes across the network.  Be aware of this!
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    /// Send a message to everybody
    Flood,
    /// Address only a single identity
    Std(Identity),
}

impl Mode {
    pub(crate) fn id(&self) -> Option<Identity> {
        match self {
            Self::Std(id) => Some(*id),
            Self::Flood => None,
        }
    }
}

impl From<Identity> for Mode {
    fn from(id: Identity) -> Self {
        Self::Std(id)
    }
}

impl From<Mode> for Recipient {
    fn from(sm: Mode) -> Self {
        match sm {
            Mode::Flood => Self::Flood,
            Mode::Std(id) => Self::User(id),
        }
    }
}

/// A query interface for the local message store
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MsgQuery {
    pub(crate) sender: Option<Identity>,
    pub(crate) tags: TagSet,
    pub(crate) skip: usize,
}

impl MsgQuery {
    /// Create a new, empty query
    pub fn new() -> Self {
        Self::default()
    }

    /// Query for messages by a specific sender
    pub fn sender(self, sender: Identity) -> Self {
        Self {
            sender: Some(sender),
            ..self
        }
    }

    /// Add a tag to the query that must be present
    ///
    /// Tag queries aim to be a subset in matching messages, which
    /// means that more tags can exist for a message, but all provided
    /// tags must be present.
    pub fn tag(mut self, t: Tag) -> Self {
        self.tags.insert(t);
        self
    }
}

/// A multi-purpose service Message
///
/// While this representation is quite "low level", i.e. forces a user
/// to deal with payload encoding themselves and provides no
/// functionality for async payloads (via filesharing, or similar), it
/// is quite a high level abstraction considering the data that needs
/// to be sent over the network in order for it to reach it's
/// recipient.
///
/// This type is both returned by `listen`, `poll`, as well as
/// specific message `queries`
///
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Message {
    /// A unique message ID
    pub id: MsgId,
    /// The sender identity
    pub sender: Identity,
    /// The embedded service associator
    pub associator: String,
    /// A tag store for persistent message metadata
    pub tags: TagSet,
    /// Verified signature data
    pub sign: SigTrust,
    /// A raw byte `Message` payload
    pub payload: Vec<u8>,
}

/// API scope type to access messaging functions
///
/// Used entirely to namespace API endpoints on `Qaul` instance,
/// without having long type identifiers.
///
/// ```norun
/// # use libqaul::{Qaul, Messages};
/// # let user = unimplemented!();
/// let q = Qaul::default();
/// q.messages().poll(user)?;
/// ```
///
/// It's also possible to `drop` the current scope, back into the
/// primary `Qaul` scope, although this is not often useful.
///
/// ```norun
/// # use libqaul::{Qaul, Messages};
/// # let q = Qaul::default();
/// q.messages().drop(); // Returns `&Qaul` again
/// ```
pub struct Messages<'chain> {
    pub(crate) q: &'chain crate::Qaul,
}

impl<'qaul> Messages<'qaul> {
    /// Drop this scope and return back to global `Qaul` scope
    pub fn drop(&'qaul self) -> &'qaul Qaul {
        self.q
    }

    /// Send a message with arbitrary payload into the network
    ///
    /// Because the term `Message` is overloaded slightly in
    /// `libqaul`, here is a small breakdown of what a message means
    /// in this context.
    ///
    /// The Service API provides an interface to communicate with
    /// other users on a qaul network. These messages are relatively
    /// low-level, meaning that their payload (for example) is simply
    /// a `Vec`, and it's left to a service to do anything meaningful
    /// with it.
    ///
    /// However when users write text-messages to each other in
    /// qaul.net, these are being sent via the `messaging` service,
    /// which implements it's own `Message`, on top of `libqaul`. In
    /// that case a message is plain text and can have binary
    /// attachments.
    ///
    /// Underlying `libqaul`, the routing layer (`RATMAN`) uses the
    /// term Message to refer to the same concept as a Service API
    /// message, with some more raw data inlined, such as signatures
    /// and checksums. Fundamentally they share the same idea of what
    /// a payload or recipient is however, and payloads that are
    /// unsecured in a Service API message will have been encrypted by
    /// the time that `RATMAN` handles them.
    pub async fn send<S, T>(
        &self,
        user: UserAuth,
        mode: Mode,
        service: S,
        tags: T,
        payload: Vec<u8>,
    ) -> Result<MsgId>
    where
        S: Into<String>,
        T: Into<TagSet>,
    {
        let (sender, _) = self.q.auth.trusted(user)?;
        let recipient = mode.into();
        let associator = service.into();
        let id = MsgId::random();
        let sign = SigTrust::Trusted;
        let tags: TagSet = tags.into();

        let env = Envelope {
            id,
            sender,
            associator: associator.clone(),
            payload: payload.clone(),
            tags: tags.iter().cloned().collect(),
        };

        self.q
            .messages
            .insert_local(
                sender,
                Arc::new(Message {
                    id,
                    sender,
                    associator,
                    tags,
                    payload,
                    sign,
                }),
                mode,
            )
            .await;

        MsgUtils::send(
            &self.q.users,
            &self.q.router,
            RatMessageProto { env, recipient },
        )
        .await
        .map(|_| id)
    }

    /// Subscribe to a stream of future message updates
    pub fn subscribe<S, T>(
        &self,
        _user: UserAuth,
        _service: S,
        _tags: T,
    ) -> Result<Subscription<Message>>
    where
        S: Into<Service>,
        T: IntoIterator<Item = Tag>,
    {
        unimplemented!()
    }

    /// Query for messages in the store, according to some parameters
    ///
    /// A query is always user authenticated, and normally associated
    /// to a service, but it doesn't have to be, if `god-mode` is
    /// enabled in the libqaul instance.
    ///
    /// The query parameters can be specified via the [`Query`]
    /// builder type which allows for very selective constraints.  The
    /// return of this function is a Wrapper around a result iterator
    /// that can return batches, or skip items dynamically.
    pub async fn query(
        &self,
        user: UserAuth,
        service: impl Into<Service>,
        query: MsgQuery,
    ) -> Result<QueryResult<Message>> {
        let (id, _) = self.q.auth.trusted(user)?;
        Ok(self.q.messages.query(id, service.into(), query).await)
    }
}
