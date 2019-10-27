use crate::error::{Error, Result};
use crate::messages::{MsgUtils, RatMessageProto};
use crate::qaul::{Identity, Qaul};
use crate::users::UserAuth;
use crate::utils::VecUtils;

use serde::{Deserialize, Serialize};

/// Signature trust level of an incoming `Message`
///
/// The three variants encode `trusted`, `unverified` and `invalid`,
/// according to signature verification of the internal keystore.
///
/// The `SigTrust::ok` convenience function can be used to reject
/// non-verifiable (unknown or bad) `Message` signatures.
#[derive(Serialize, Deserialize)]
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
            Self::Unverified => Err(Error::UnknownSign),
            Self::Invalid => Err(Error::BadSign),
        }
    }
}

/// Service message recipient
///
/// A recipient is either a single user or the entire network.  The
/// "flood" mechanic is passed through to `RATMAN`, which might
/// implement this in the networking module, or emulate
/// it. Performance may vary.
#[derive(Serialize, Deserialize)]
pub enum Recipient {
    /// A single user, known to this node
    User(Identity),
    /// A collection of users, sometimes called a Group
    Group(Vec<Identity>),
    /// Addressed to nobody, flooded into the network
    Flood,
}

/// A multipurpose service Message
///
/// The two states that are encoded in this enum are meant to
/// disambiguate between a `Message` being **sent into** the network,
/// vs. a `Message` that is **being received**. Fundamentally they are
/// the same, and share the same abstraction principles, but will
/// contain different data.
///
/// The `Out` variant will for example not contain `signature`
/// information, because by the time the object is made, this data has
/// not been computed yet. On the other hand the `In` variant won't
/// have a service `associator`, because this is abstracted away by
/// the `poll` and `listen` function endpoints.
///
/// ## Rationale
///
/// This approach was chosen over having multiple structs becauseit
/// makes storing this type in the internal data store easier, without
/// having to duplicate structures _too_ much.
#[derive(Serialize, Deserialize)]
pub enum Message {
    /// An incoming `Message`, received from someone else on the network
    In {
        /// The sender identity
        sender: Identity,
        /// Recipient information
        recipient: Recipient,
        /// Attached `Message` signature, built for all fields
        sign: SigTrust,
        /// A raw byte `Message` payload
        payload: Vec<u8>,
    },
    /// An outgoing `Message`, sent from a local user
    Out {
        /// The sender identity
        sender: Identity,
        /// Recipient information
        recipient: Recipient,
        /// The embedded service associator
        associator: String,
        /// A raw byte `Message` payload
        payload: Vec<u8>,
    },
}

impl Message {
    /// Returns the `sender` field for both `Message` variants
    pub fn sender(&self) -> &Identity {
        match self {
            Self::In { ref sender, .. } => sender,
            Self::Out { ref sender, .. } => sender,
        }
    }

    /// Returns the `recipient` field for both `Message` variants
    pub fn recipient(&self) -> &Recipient {
        match self {
            Self::In { ref recipient, .. } => recipient,
            Self::Out { ref recipient, .. } => recipient,
        }
    }

    /// Returns the `payload` field for both `Message` variants
    pub fn payload(&self) -> &Vec<u8> {
        match self {
            Self::In { ref payload, .. } => payload,
            Self::Out { ref payload, .. } => payload,
        }
    }

    /// Returns the `sign` field for the `Message::In` variant
    pub fn signature(&self) -> Option<&SigTrust> {
        match self {
            Self::In { ref sign, .. } => Some(sign),
            Self::Out { .. } => None,
        }
    }

    /// Returns the `associator` field for the `Message::Out` variant
    pub fn associator(&self) -> Option<&String> {
        match self {
            Self::Out { ref associator, .. } => Some(associator),
            Self::In { .. } => None,
        }
    }

    /// Construct a new `Recipient`, in reply to a `Message::In`
    ///
    /// If the `Message` was addressed to a single user, the sender is
    /// used. If it was addressed to a group, the sender is added, and
    /// self is removed from the `Group` set. If it was a flood, then
    /// the reply is a flood.
    pub fn reply(&self, id: &Identity) -> Recipient {
        let recipient = self.recipient();
        let sender = self.sender();

        use Recipient::*;
        match recipient {
            Group(ref group) => Group(group.clone().strip(id).add(*sender)),
            User(_) => User(*sender),
            Flood => Flood,
        }
    }
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
pub enum MessageQuery {
    /// Query by who a `Message` was composed by
    Sender(Identity),
    /// Query a Message by who it is addressed to
    Recipient(Recipient),
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
    pub fn send(
        &self,
        user: UserAuth,
        recipient: Recipient,
        service: String,
        payload: Vec<u8>,
    ) -> Result<()> {
        let (sender, _) = self.q.auth.trusted(user)?;
        let recipients = MsgUtils::readdress(&recipient);
        let associator = service;

        let msg = Message::Out {
            sender,
            recipient,
            associator,
            payload,
        };

        let signature = MsgUtils::sign(&msg);
        MsgUtils::send(
            &self.q.router,
            RatMessageProto {
                msg,
                recipients,
                signature,
            },
        )
    }

    /// Non-blockingly poll the API for the latest `Message` for a service
    ///
    /// Two notes on the data returned from this endpoint. For a more
    /// general `Message` query/ enumeration API, see
    /// `Messages::query` instead.
    ///
    /// 1. This will only receive new messages, since last checking
    ///    and can be used, while in active operation, to handle
    ///    incoming messages as they are received.
    /// 2. The `Message` variant returned from this endpoint will
    ///    **always** be `Message::In`, never an outgoing type.
    pub fn poll<S>(&self, user: UserAuth, service: S) -> Result<Message>
    where
        S: Into<String>,
    {
        unimplemented!()
    }

    /// Register a listener on new-message events for a service
    ///
    /// This function works very similarly to `Messages::poll`, except
    /// that it uses a lambda to call when a new `Message` is
    /// received.  Both caveats mentioned in the doc comment for
    /// `poll` apply here as well.
    pub fn listen<S, F>(&self, user: UserAuth, service: S, listener: F) -> Result<()>
    where
        S: Into<String>,
        F: Fn(Message) -> Result<()>,
    {
        unimplemented!()
    }

    /// Query for `Messages` from the store for a service
    pub fn query<S>(&self, user: UserAuth, service: S, query: MessageQuery) -> Result<Vec<Message>>
    where
        S: Into<Option<String>>,
    {
        let service = service.into();
        Ok(vec![])
    }
}
