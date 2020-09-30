//! A tcp overlay netmod to connect router across the internet

mod error;
mod io;
mod peer;
mod proto;
mod ptr;
mod routes;
mod server;

pub use error::{Error, Result};

pub(crate) use io::IoPair;
pub(crate) use peer::{DstAddr, Peer, PeerState, SourceAddr};
pub(crate) use proto::{Packet, PacketBuilder};
pub(crate) use ptr::AtomPtr;
pub(crate) use routes::Routes;
pub(crate) use server::Server;

use async_std::sync::Arc;
use async_trait::async_trait;
use netmod::{self, Endpoint as EndpointExt, Frame, Target};
use std::net::SocketAddr;
use tracing::info;

/// Define the runtime mode for this endpount
///
/// In dynamic mode any new peer can introduce itself to start a link,
/// while in static mode only known peers will be accepted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Static,
    Dynamic,
}

#[derive(Clone)]
pub struct Endpoint {
    server: Arc<Server>,
    routes: Arc<Routes>,
}

impl Endpoint {
    /// Create a new endpoint on an interface and port
    #[tracing::instrument(level = "info")]
    pub async fn new(addr: &str, port: u16, name: &str, mode: Mode) -> Result<Arc<Self>> {
        info!("Initialising Tcp backend");

        let routes = Routes::new(port);
        let server = Server::new(Arc::clone(&routes), addr, port, mode).await?;

        server.run();
        Ok(Arc::new(Self { server, routes }))
    }

    /// Get the current runtime mode
    pub fn mode(&self) -> Mode {
        self.server.mode()
    }

    pub async fn stop(&self) {
        self.server.stop();
        self.routes.stop_all().await;
    }

    /// Insert a set of peers into the routing table
    ///
    /// Each peer will spawn a worker that periodically attempts to
    /// connect to it.  Connections might not be recipricated if the
    /// peer doesn't know the local IP or is rejecting unknown
    /// connections.
    pub async fn add_peers<I: Into<SocketAddr>>(&self, peers: Vec<I>) -> Result<()> {
        for peer in peers.into_iter() {
            self.routes.add_via_dst(peer.into()).await;
        }

        Ok(())
    }
}

#[async_trait]
impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&self, frame: Frame, target: Target) -> netmod::Result<()> {
        match target {
            Target::Flood => {
                let dsts = self.routes.all_dst().await;
                for peer in dsts {
                    peer.send(Packet::Frame(frame.clone())).await;
                }
            }
            Target::Single(id) => {
                let peer = match self.routes.get_peer(id as usize).await {
                    Some(p) => Ok(p),
                    None => Err(netmod::Error::ConnectionLost),
                }?;
                peer.send(Packet::Frame(frame)).await;
            }
        }

        Ok(())
    }

    async fn next(&self) -> netmod::Result<(Frame, Target)> {
        Ok(self.server.next().await)
    }
}
