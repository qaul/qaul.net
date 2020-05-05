//! A tcp overlay netmod to connect router across the internet

mod error;
mod peers;
mod proto;
mod socket;

pub use error::{Error, Result};

pub(crate) use peers::{PeerList, PeerState};
pub(crate) use proto::Packet;
pub(crate) use socket::Socket;

use async_std::sync::{Arc, Receiver, RwLock};
use async_trait::async_trait;
use netmod::{self, Endpoint as EndpointExt, Frame, Target};
use std::net::SocketAddr;

/// Define the runtime mode for this endpount
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Static,
    Dynamic,
}

impl Mode {
    pub(crate) fn dynamic(&self) -> bool {
        self == &Mode::Dynamic
    }
}

#[derive(Clone)]
pub struct Endpoint {
    mode: Arc<RwLock<Mode>>,
    socket: Arc<Socket>,
    peers: Arc<PeerList>,
    inbox: Option<Receiver<Frame>>,
}

impl Endpoint {
    /// Create a new endpoint on an interface and port
    pub async fn new(addr: &str, port: u16) -> Result<Self> {
        Ok(Self {
            mode: Arc::new(RwLock::new(Mode::Static)),
            socket: Socket::new(addr, port, "").await?,
            peers: PeerList::new(),
            inbox: None,
        })
    }

    /// Set the runtime mode
    pub async fn mode(&self, mode: Mode) {
        *self.mode.write().await = mode;
    }

    /// Load a set of peers, replacing the old peer list
    pub async fn load_peers<I: Into<SocketAddr>>(&self, peers: Vec<I>) -> Result<()> {
        self.peers.load(peers).await?;
        Ok(())
    }

    pub async fn start(&mut self) {
        self.inbox = Some(self.socket.start(*self.mode.read().await, &self.peers));
    }
}

#[async_trait]
impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&self, _: Frame, _: Target) -> netmod::Result<()> {
        unimplemented!()
    }

    async fn next(&self) -> netmod::Result<(Frame, Target)> {
        unimplemented!()
    }
}
