//! TCP ovelay specific error handling

use failure::Fail;

pub type Result<T> = std::result::Result<T, Error>;

/// A generic initialisation error
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "a set of provided peers already existed")]
    DuplicatePeers { peers: PeerErrs },
    #[fail(display = "the selected mode does not allow for this operation")]
    InvalidMode,
    #[fail(display = "failed to initialise socket: invalid address")]
    InvalidAddr,
}

use std::net::SocketAddr;

#[derive(Debug)]
pub struct PeerErrs(Vec<SocketAddr>);

impl PeerErrs {
    pub(crate) fn new(first: SocketAddr) -> std::result::Result<(), Self> {
        Err(Self(vec![first]))
    }

    pub(crate) fn append(mut self, new: SocketAddr) -> Self {
        self.0.push(new);
        self
    }
}

impl From<async_std::io::Error> for Error {
    fn from(e: async_std::io::Error) -> Self {
        use async_std::io::ErrorKind::*;
        match e.kind() {
            PermissionDenied | AddrInUse | AddrNotAvailable => Self::InvalidAddr,
            e => panic!("Unhandled io error: `{:?}`", e),
        }
    }
}

impl From<PeerErrs> for Error {
    fn from(peers: PeerErrs) -> Self {
        Self::DuplicatePeers { peers }
    }
}
