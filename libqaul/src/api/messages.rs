use crate::error::{Error, Result};
use crate::messages::{Envelope, MsgUtils, RatMessageProto};
use crate::messages::message_generation;
use crate::qaul::{Identity, Qaul};
use crate::users::UserAuth;
use crate::utils::VecUtils;

use serde::{
    Deserialize, Serialize,
    de::{Deserializer},
    ser::{Serializer},
};
use std::{
    fmt::{Display, Debug, Formatter, self},
    sync::Arc,
};
use hex;

/// A reference to an internally stored message object
pub type MsgRef = Arc<Message>;

/// Length of an `MsgId`, for converting to and from arrays
pub const ID_LEN: usize = 16;

/// A unique, randomly generated message ID
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MsgId(pub(crate) [u8; ID_LEN]);

impl MsgId {
    /// Generate a new **random** message ID
    pub(crate) fn new() -> Self {
        crate::utils::random(ID_LEN)
            .into_iter()
            .zip(0..ID_LEN)
            .fold(Self([0; ID_LEN]), |mut acc, (x, i)| {
                acc.0[i] = x;
                acc
            })
    }
}

impl Debug for MsgId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<MSG ID: {}>", hex::encode_upper(self))
    }
}

impl Display for MsgId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = hex::encode_upper(self);
        let mut v = s
            .as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap()
            .join(" ");
        v.insert(20, ' ');
        write!(f, "{}", v)
    }
}


/// Implement RAW `From` binary array
impl From<[u8; ID_LEN]> for MsgId {
    fn from(i: [u8; ID_LEN]) -> Self {
        Self(i)
    }
}

/// Implement RAW `From` binary (reference) array
impl From<&[u8; ID_LEN]> for MsgId {
    fn from(i: &[u8; ID_LEN]) -> Self {
        Self(i.clone())
    }
}

/// Implement binary array `From` RAW
impl From<MsgId> for [u8; ID_LEN] {
    fn from(i: MsgId) -> Self {
        i.0
    }
}

/// Implement binary array `From` RAW reference
impl From<&MsgId> for [u8; ID_LEN] {
    fn from(i: &MsgId) -> Self {
        i.0.clone()
    }
}

/// Implement RAW identity to binary array reference
impl AsRef<[u8]> for MsgId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl serde::ser::Serialize for MsgId {
    fn serialize<S>(&self, ser: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if ser.is_human_readable() {
            ser.serialize_str(&self.to_string())
        } else {
            ser.serialize_bytes(&self.0)
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for MsgId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor, SeqAccess};
        use std::result::Result;

        struct IdentityVisitor;

        impl IdentityVisitor {
            fn from_str<E: Error>(v: &str) -> Result<MsgId, E> {
                let v: Vec<u8> = v
                    .split(" ")
                    .map(|s| hex::decode(s).map_err(|e| E::custom(e)))
                    // I don't like this way of propagating errors up but the alternative
                    // is a for loop which i also don't like
                    .collect::<Result<Vec<Vec<u8>>, E>>()?
                    .into_iter()
                    .flatten()
                    .collect();

                Self::from_bytes(&v)
            }

            fn from_bytes<E: Error, V: AsRef<[u8]>>(v: V) -> Result<MsgId, E> {
                let v = v.as_ref();
                if v.len() != ID_LEN {
                    return Err(E::custom(format!(
                        "Expected {} bytes, got {}",
                        ID_LEN,
                        v.len()
                    )));
                }

                Ok(MsgId(v.iter().enumerate().take(ID_LEN).fold(
                    [0; ID_LEN],
                    |mut buf, (i, u)| {
                        buf[i] = *u;
                        buf
                    },
                )))
            }
        }

        impl<'de> Visitor<'de> for IdentityVisitor {
            type Value = MsgId;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(
                    f,
                    "Either a {l} byte array or a hex string representing {l} bytes",
                    l = ID_LEN
                )
            }

            fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
                Self::from_str(v)
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Self::from_str(&v)
            }

            fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                Self::from_bytes(v)
            }

            fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Self::from_bytes(v)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut v = Vec::new();
                while let Some(b) = seq.next_element::<u8>()? {
                    v.push(b);
                }

                Self::from_bytes(v)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(IdentityVisitor)
        } else {
            deserializer.deserialize_bytes(IdentityVisitor)
        }
    }
}

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

/// Service message recipient
///
/// A recipient is either a single user or the entire network.  The
/// "flood" mechanic is passed through to `RATMAN`, which might
/// implement this in the networking module, or emulate
/// it. Performance may vary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Recipient {
    /// A single user, known to this node
    User(Identity),
    /// A collection of users, sometimes called a Group
    Group(Vec<Identity>),
    /// Addressed to nobody, flooded into the network
    Flood,
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
    /// Recipient information
    pub recipient: Recipient,
    /// The embedded service associator
    pub associator: String,
    /// Verified signature data
    pub sign: SigTrust,
    /// A raw byte `Message` payload
    pub payload: Vec<u8>,
}

impl Message {
    /// Construct a new `Recipient`, in reply to a Message
    ///
    /// If the `Message` was addressed to a single user, the sender is
    /// used. If it was addressed to a group, the sender is added, and
    /// self is removed from the `Group` set. If it was a flood, then
    /// the reply is a flood.
    pub fn reply(&self, id: &Identity) -> Recipient {
        let recipient = &self.recipient;
        let sender = self.sender;

        use Recipient::*;
        match recipient {
            Group(ref group) => Group(group.clone().strip(id).add(sender)),
            User(_) => User(sender),
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageQuery {
    /// Single query for the exact message ID
    Id(MsgId),
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
    pub async fn send<S>(
        &self,
        user: UserAuth,
        recipient: Recipient,
        service: S,
        payload: Vec<u8>,
    ) -> Result<MsgId>
    where
        S: Into<String>,
    {
        let (sender, _) = self.q.auth.trusted(user)?;
        let recipients = MsgUtils::readdress(&recipient);
        let associator = service.into();
        let id = MsgId::new();
        let sign = SigTrust::Trusted;

        let env = Envelope {
            id,
            sender,
            associator: associator.clone(),
            payload: payload.clone(),
        };

        let signature = MsgUtils::sign(&env);

        self.q.messages.insert(
            sender,
            crate::messages::MsgState::Read(Arc::new(Message {
                id,
                sender,
                recipient,
                associator,
                payload,
                sign,
            })),
        );

        MsgUtils::send(
            &self.q.router,
            RatMessageProto {
                env,
                recipients,
                signature,
            },
        ).await
        .map(|_| id)
    }

    /// Non-blockingly poll the API for the latest `Message` for a service
    ///
    /// For a more general `Message` query/ enumeration API, see
    /// `Messages::query` instead.
    pub fn poll<S>(&self, user: UserAuth, service: S) -> Result<MsgRef>
    where
        S: Into<String>,
    {
        let (id, _) = self.q.auth.trusted(user)?;
        self.q
            .messages
            .query(id)
            .service(service)
            .unread()
            .limit(1)
            .exec()
            .map(|vec| match vec.into_iter().nth(0) {
                Some(msg) => Ok(msg),
                None => Err(Error::NoData),
            })?
    }

    /// Register a listener on new-message events for a service
    ///
    /// This function works very similarly to `Messages::poll`, except
    /// that it uses a lambda to call when a new `Message` is
    /// received.  Both caveats mentioned in the doc comment for
    /// `poll` apply here as well.
    pub fn listen<S, F: 'static + Send + Sync>(
        &self,
        user: UserAuth,
        service: S,
        listener: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Fn(MsgRef) -> Result<()>,
    {
        self.q.auth.trusted(user)?;
        self.q.services.add_listener(service.into(), listener)
    }

    /// Retrieve locally stored messages from the store
    ///
    /// A query is made in relation to an associated service
    /// handle. It isn't possible to query all messages for all
    /// services in an efficient manner due to how messages are stored
    /// in a node.
    pub fn query<S>(&self, user: UserAuth, service: S, query: MessageQuery) -> Result<Vec<MsgRef>>
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
