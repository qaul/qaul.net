//! Library specific errors

use failure::Fail;
use identity::Identity as Id;

/// Alexandria library API errors
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to initialise library at offset `{}`", offset)]
    InitFailed { offset: String },
    #[fail(display = "bad unlock token (password?) for user `{}`", user)]
    UnlockFailed { user: Id },
    #[fail(display = "failed to sync data: `{}`", msg)]
    SyncFailed { msg: String },
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
