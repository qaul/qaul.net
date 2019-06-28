//! Service API exchange models

use identity::Identity;
use std::collections::BTreeMap;

/// Convenience type for API functions
pub type QaulResult<T> = Result<T, QaulError>;

/// Service API error wrapper
pub enum QaulError {
    /// Not authorised to perform this action
    NotAuthorised,
    /// The desired user was not known
    UnknownUser,
    /// Invalid search query
    InvalidQuery,
    /// Invalid payload (probably too big)
    InvalidPayload,
    /// A function callback timed out
    CallbackTimeout,
}

/// A wrapper around user authentication state
pub enum UserAuth {
    /// A user ID which has not been verified
    Default(Identity),
    /// The user ID of the currently logged-in user
    Trusted(Identity, String),
}

/// Signature trust information embedded into service messages
pub enum SigTrust {
    /// A verified signature by a known contact
    Trusted(Identity),
    /// An unverified signature by a known contact
    /// (pubkey not available!)
    Unverified(Identity),
    /// A fraudulent signature
    Invalid,
}

/// A service message
///
/// Differs from the `RATMAN` abstraction for messages
/// because it's signature has already been verified.
/// Instead of delivering the raw signature to a service,
/// this message only embeds validity information.
///
/// This makes it easier for service authors to trust
/// data provided by `libqaul`, without having to do
/// calls into some crypto library themselves.
///
/// In comparison to the `RATMAN` message, the `associator`
/// has also been removed because at this stage, only the
/// relevant related service is being handed a message anyway.
pub struct Message {
    sender: Identity,
    recipient: Identity,
    payload: Vec<u8>,
    signature: SigTrust,
}

/// A public representation of user metadata
///
/// Apart from the user `id`, all fields are optional
/// and should not be assumed set. This struct is used
/// for both the local user (identified by `UserAuth`)
/// as well as remote users from the contacts book.
pub struct User {
    /// A users network ID
    id: Identity,
    /// A human readable display-name (like @foobar)
    display_name: Option<String>,
    /// A human's preferred call-signed ("Friends call be foo")
    real_name: Option<String>,
    /// A key-value list of things the user deems interesting
    /// about themselves. This could be stuff like "gender",
    /// "preferred languages" or whatever.
    bio: BTreeMap<String, String>,
    /// The set of services this user runs (should never be empty!)
    services: Vec<String>,
    /// A users profile picture (some people like selfies)
    avatar: Option<Vec<u8>>,
}
