//! Library specific errors

use failure::Fail;
use std::fmt::{self, Display, Formatter};

/// Alexandria library API errors
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to add a user that already exits")]
    UserAlreadyExists,

    #[fail(display = "operation failed because user `{}` doesn't exist", id)]
    NoSuchUser { id: String },

    #[fail(display = "failed to initialise library at offset `{}`", offset)]
    InitFailed { offset: String },

    #[fail(display = "failed to perform action because user `{}` is locked", id)]
    UserNotOpen { id: String },

    #[fail(display = "bad unlock token (password?) for id `{}`", id)]
    UnlockFailed { id: String },

    #[fail(display = "tried to operate on locked encrypted state: {}", msg)]
    LockedState { msg: String },

    #[fail(display = "tried to unlock user Id `{}` twice", id)]
    AlreadyUnlocked { id: String },

    #[fail(display = "no such path: `{}`", path)]
    NoSuchPath { path: String },

    #[fail(display = "path exists already: {}", path)]
    PathExists { path: String },

    #[fail(display = "failed to load data: `{}`", msg)]
    LoadFailed { msg: String },

    #[fail(display = "failed to sync data: `{}`", msg)]
    SyncFailed { msg: String },

    #[fail(display = "tried to apply Diff of incompatible type")]
    BadDiffType,

    #[fail(display = "a Diff failed to apply: \n{}", msgs)]
    BadDiff { msgs: DiffErrs },

    #[doc(hidden)]
    #[fail(display = "An alexandria internal error occured: `{}`", msg)]
    InternalError { msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct DiffErrs(Vec<(usize, String)>);

impl DiffErrs {
    pub(crate) fn add(mut self, new: Self) -> Self {
        let mut ctr = self.0.len();
        new.0.into_iter().for_each(|(_, e)| {
            self.0.push((ctr, e));
            ctr += 1;
        });

        self
    }
}

impl Display for DiffErrs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |res, (num, msg)| {
            res.and_then(|_| write!(f, r#"{}: "{}""#, num, msg))
        })
    }
}

impl From<Vec<(usize, String)>> for DiffErrs {
    fn from(vec: Vec<(usize, String)>) -> Self {
        Self(vec)
    }
}

impl From<(usize, String)> for DiffErrs {
    fn from(tup: (usize, String)) -> Self {
        Self(vec![tup])
    }
}

impl From<DiffErrs> for Error {
    fn from(msgs: DiffErrs) -> Self {
        Error::BadDiff { msgs }
    }
}

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
