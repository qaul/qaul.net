//! Service API exchange models

use identity::Identity;

/// Convenience type for API functions
pub type QaulResult<T> = Result<T, QaulError>;

/// Service API error wrapper
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Clone)]
pub enum UserAuth {
    /// A user ID which has not been verified
    Untrusted(Identity),
    /// The user ID of the currently logged-in user
    Trusted(Identity, String),
}

impl UserAuth {
    /// Returns an error if the UserAuth isn't Trusted.
    pub fn trusted(self) -> QaulResult<(Identity, String)> {
        match self {
            UserAuth::Trusted(id, s) => Ok((id, s)),
            UserAuth::Untrusted(_) => Err(QaulError::NotAuthorised),
        }
    }

    /// Returns the interior identity, regardless of trust status.
    pub fn identity(self) -> Identity {
        match self {
            UserAuth::Trusted(id, _) => id,
            UserAuth::Untrusted(id) => id,
        }
    }
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
