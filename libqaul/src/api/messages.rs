use crate::{
    api::{SubId, Subscription},
    error::{Error, Result},
    messages::{Envelope, MsgUtils, RatMessageProto},
    qaul::{Identity, Qaul},
    users::UserAuth,
    Tag,
};

use async_std::{
    future::{self, Future},
    task::{self, Poll},
};
use ratman::netmod::Recipient;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, iter::FromIterator, sync::Arc, time::Duration};

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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    /// Send a message to everybody
    Flood,
    /// Address only a single identity
    Std(Identity),
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
    pub tags: BTreeSet<Tag>,
    /// Verified signature data
    pub sign: SigTrust,
    /// A raw byte `Message` payload
    pub payload: Vec<u8>,
}

/// A query interface for the local `Message` store
///
/// Important to consider that a `Query` can only be applied to the
/// set of messages that the user has access to. User access
/// information is not encoded in this enum, but rather passed to the
/// `Messages::query` function as a first parameter.
///
/// While `Query` objects can't be combined (yet), it is also possible
/// to pass an `Option<String>` as service filter, meaning that only
/// messages addressed to the appropriate service will be
/// returned. Without this parameter, all messages will be returned.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgQuery {
    /// Query for the exact message ID
    Id(MsgId),
    /// Query by who a `Message` was composed by
    Sender(Identity),
    /// Search for a set of tag values
    Tag(Tag),
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

    /// Send a message into the network
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
        T: IntoIterator<Item = Tag>,
    {
        let (sender, _) = self.q.auth.trusted(user)?;
        let recipient = mode.into();
        let associator = service.into();
        let id = MsgId::random();
        let sign = SigTrust::Trusted;
        let tags = BTreeSet::from_iter(tags.into_iter());

        let env = Envelope {
            id,
            sender,
            associator: associator.clone(),
            payload: payload.clone(),
            tags: tags.iter().cloned().collect(),
        };

        self.q.messages.insert(
            sender,
            crate::messages::MsgState::Read(Arc::new(Message {
                id,
                sender,
                associator,
                tags,
                payload,
                sign,
            })),
        );

        MsgUtils::send(
            &self.q.users,
            &self.q.router,
            RatMessageProto { env, recipient },
        )
        .await
        .map(|_| id)
    }

    /// Poll the API for the next `Message` for a service
    ///
    /// For a more general `Message` query/ enumeration API, see
    /// `Messages::query` instead.
    pub async fn next<S, I>(&self, user: UserAuth, service: S, tags: I) -> Result<MsgRef>
    where
        S: Into<String>,
        I: IntoIterator<Item = Tag>,
    {
        let tags = tags.into_iter().collect::<BTreeSet<Tag>>();
        let (id, _) = self.q.auth.trusted(user)?;
        let service: String = service.into();

        // This whole unholy mess needs to be rewritten, when we have
        // an async message store ... yikes

        future::poll_fn(move |ctx| {
            match self
                .q
                .messages
                .query(id)
                .service(service.as_str())
                .tags(tags.clone())
                .unread()
                .limit(1)
                .exec()
            {
                Ok(msg) if msg.len() >= 1 => Poll::Ready(Ok(msg.into_iter().nth(0).unwrap())),
                _ => {
                    let waker = ctx.waker().clone();
                    task::spawn(async move {
                        task::sleep(Duration::from_millis(10)).await;
                        waker.wake();
                    });
                    Poll::Pending
                }
            }
        })
        .await
    }

    /// Subscribe to a stream of future message updates
    ///
    /// A subscription is an async stream of messages, that is
    /// specific to a user, service token, set of store search tags,
    /// and subscription tag.  The subscription tag is generated for
    /// each subscription and can later on be used to cancel a stream.
    pub fn subscribe<S, T>(
        &self,
        user: UserAuth,
        service: S,
        tags: T,
    ) -> Result<Subscription<MsgRef>>
    where
        S: Into<String>,
        T: IntoIterator<Item = Tag>,
    {
        unimplemented!()
    }

    /// Cancel a previous subscription by Id
    ///
    /// Messages can still be queried or polled, but will stop being
    /// streamed to the registered receiver.
    pub fn unsubscribe(&self, user: UserAuth, id: SubId) -> Result<()> {
        unimplemented!()
    }

    /// Retrieve locally stored messages from the store
    ///
    /// A query is made in relation to an associated service
    /// handle. It isn't possible to query all messages for all
    /// services in an efficient manner due to how messages are stored
    /// in a node.
    pub fn query<S>(&self, user: UserAuth, service: S, query: MsgQuery) -> Result<Vec<MsgRef>>
    where
        S: Into<String>,
    {
        let (id, _) = self.q.auth.trusted(user)?;
        self.q
            .messages
            .query(id)
            .constraints(query)
            .service(service)
            .exec()
    }
}
