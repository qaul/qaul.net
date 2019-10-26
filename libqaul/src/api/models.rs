//! Service API exchange models

use std::fmt::{self, Debug, Formatter};

use identity::Identity;
use mime::Mime;

/// Convenience type for API functions
pub type QaulResult<T> = Result<T, QaulError>;

/// `libqaul` service API error states
///
/// All errors that can occur in interaction with the API are encoded
/// as variants on this enum. In most cases, no additional metadata is
/// provided and needs to be inferred from whatever context or
/// function call emitted the error. Check the variant doc comments
/// for a broad overview, as well as detailed usage instructions.
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
    /// Signature with an unknown public key
    UnknownSign,
    /// Fraudulent signature for a known public key
    BadSign,
}

// /// A security token to authenticate sessions
// #[derive(Clone, PartialEq, Eq)]
// pub struct Token(String);

// impl Debug for Token {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "<TOKEN>")
//     }
// }

// impl From<String> for Token {
//     fn from(s: String) -> Self {
//         assert!(s.len() == 64);
//         Token(s)
//     }
// }

pub type Token = String;

/// A wrapper around user authentication state
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserAuth {
    /// A user ID which has not been verified
    Untrusted(Identity),
    /// The user ID of the currently logged-in user
    Trusted(Identity, Token),
}

impl UserAuth {
    /// Returns an error if the UserAuth isn't Trusted.
    pub fn trusted(self) -> QaulResult<(Identity, Token)> {
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

    /// Returns the interior identity as an `Untrusted`, regardless of trust status.
    pub fn as_untrusted(&self) -> Self {
        UserAuth::Untrusted(self.clone().identity())
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

/// Local file abstraction
pub struct File {
    pub name: String,
    pub mime: Mime,
    pub data: Option<Vec<u8>>,
}

/// Describe a file's lifecycle
///
/// Not to be confused with `FileFilter`, which is part of public API
/// functions to allow users to easily filter for only certain types
/// of file data.
///
/// Filter functions then take a `Filter` and return a `Meta`.
pub enum FileMeta {
    /// Files owned by the current user
    Local(File),
    /// Network files, fully locally mirrored
    Available(File),
    /// Network files, still downloading
    InProgress {
        size: usize,
        local: usize,
        stalled: bool,
    },
    /// A network advertised file that hasn't started downloading
    Advertised { size: usize },
}

/// Describe a file's lifecycle
///
/// Filter functions for each time exist and enable
/// different sub-services based on which phase they
/// aim for.
pub enum FileFilter {
    Local,
    Available,
    InProgress,
}
