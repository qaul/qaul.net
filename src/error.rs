//! Library specific errors

use failure::Fail;

/// Alexandria library API errors
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to add a user that already exits")]
    UserAlreadyExists,

    #[fail(display = "failed to perform action because user `{}` is locked", id)]
    UserNotOpen { id: String },

    #[fail(display = "operation failed because user `{}` doesn't exist", id)]
    NoSuchUser { id: String },

    #[fail(display = "failed to initialise library at offset `{}`", offset)]
    InitFailed { offset: String },

    #[fail(display = "bad unlock token (password?) for id `{}`", id)]
    UnlockFailed { id: String },

    #[fail(display = "tried to operate on locked encrypted state: {}", msg)]
    LockedState { msg: String },

    #[fail(display = "tried to unlock user Id `{}` twice", id)]
    AlreadyUnlocked { id: String },

    #[fail(display = "failed to sync data: `{}`", msg)]
    SyncFailed { msg: String },

    #[fail(display = "failed to load data: `{}`", msg)]
    LoadFailed { msg: String },

    #[fail(display = "user zone exists already: {}::{}", id, zone)]
    ZoneAlreadyExsts { id: String, zone: String },

    #[fail(display = "user zone does not exists: {}::{}", id, zone)]
    ZoneDoesNotExst { id: String, zone: String },

    #[doc(hidden)]
    #[fail(display = "An alexandria internal error occured: `{}`", msg)]
    InternalError { msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<bincode::Error> for Error {
    fn from(be: bincode::Error) -> Self {
        use bincode::ErrorKind::*;

        // FIXME: this isn't great but like... whatevs
        let msg = match *be {
            Io(e) => format!("I/O error: '{}'", e),
            SizeLimit => "Payload too large!".into(),
            SequenceMustHaveLength => "Internal sequencing error".into(),
            _ => "Unknown encoding error".into(),
        };

        Self::SyncFailed { msg }
    }
}

impl From<async_std::io::Error> for Error {
    fn from(e: async_std::io::Error) -> Self {
        Self::LoadFailed {
            msg: format!("{}", e),
        }
    }
}
