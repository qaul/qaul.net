//! TCP incoming connection server

use crate::{IoPair, Mode, Packet, PacketBuilder, Peer, PeerState, Result, Routes, SourceAddr};
use async_std::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use netmod::Frame;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tracing::{error, info, trace, warn};

/// The listening server part of the tcp driver
pub(crate) struct Server {
    alive: Arc<AtomicBool>,
    inner: TcpListener,
    routes: Arc<Routes>,
    port: u16,
    mode: Mode,
    incoming: IoPair<Frame>,
}

impl Server {
    /// Create a new tcp listening server, without running it
    pub(crate) async fn new(
        routes: Arc<Routes>,
        addr: &str,
        port: u16,
        mode: Mode,
    ) -> Result<Arc<Self>> {
        Ok(TcpListener::bind(format!("{}:{}", addr, port))
            .await
            .map(|inner| {
                Arc::new(Self {
                    alive: Default::default(),
                    incoming: IoPair::default(),
                    inner,
                    routes,
                    port,
                    mode,
                })
            })?)
    }

    fn alive(self: &Arc<Self>) -> bool {
        self.alive.load(Ordering::Relaxed)
    }

    /// Shut down the listening server
    pub(crate) fn stop(self: &Arc<Self>) {
        self.alive.fetch_and(false, Ordering::Relaxed);
    }

    /// Get the next available frame
    pub(crate) async fn next(self: &Arc<Self>) -> Frame {
        self.incoming.rx.recv().await.unwrap()
    }

    /// Spawn a handler task for incoming connections
    pub(crate) fn run(self: &Arc<Self>) {
        let s = Arc::clone(self);
        task::spawn(async move {
            let mut inc = s.inner.incoming();
            info!(
                "Listening on {:?} for incoming connections",
                s.inner.local_addr()
            );

            // For each connection, spawn a new worker task
            while let Some(Ok(stream)) = inc.next().await {
                if !s.alive() {
                    break;
                }

                let s = Arc::clone(&s);
                task::spawn(async move { s.accept_connection(stream).await });
            }

            info!("Terminating tcp accept loop!");
        });
    }

    /// loop over a stream of incoming data
    async fn accept_connection(self: Arc<Self>, mut stream: TcpStream) {
        let src_addr = match stream.peer_addr() {
            Ok(a) => a,
            Err(_) => {
                error!("Missing peer addr in stream; exiting!");
                return;
            }
        };

        // Find the correct peer or create a temporary one.  If we
        // create a temporary one, we will need to upgrade it before
        // being able to accept valid connections
        let pid = self
            .routes
            .find_via_src(&src_addr)
            .await
            .unwrap_or_else(|| task::block_on(async { self.routes.add_via_src(&src_addr).await }));
        let peer = self.routes.get_peer(pid).await.unwrap();

        // Loop forever
        loop {
            let mut fb = PacketBuilder::new(&mut stream);
            if let Err(_) = fb.parse().await {
                error!("Failed to read from incoming packet stream; dropping connection!");
                break;
            }

            let f = match fb.build() {
                Some(f) => f,
                None => {
                    error!("Malformed frame; skipping!");
                    continue;
                }
            };

            // Match on the peer-state, message payload tuple.  Each
            // scenario is documented on the handler function to keep
            // this match block as small and readable as possible.
            // Avoid useless logging in this block too!
            use Packet::*;
            use PeerState::*;
            match (peer.state(), f) {
                (Duplex, Frame(f)) | (RxOnly, Frame(f)) => self.handle_frame(f).await,
                (_, Hello { port }) => self.handle_hello(&src_addr, port).await,
                (RxOnly, KeepAlive) => self.rx_keepalive(),
                (Duplex, KeepAlive) => self.dup_keepalive(Arc::clone(&peer)),
                _ => todo!(),
            }
        }

        trace!("Exiting connetion work-loop; was there a connection drop?");
    }

    /// Handle an incoming frame message
    async fn handle_frame(self: &Arc<Self>, p: Frame) {
        self.incoming.tx.send(p).await;
    }

    /// A keepalive on an RXonly connection
    ///
    /// It means that currently the TX connection is down.  This
    /// function can't really do anything about that though, so we log
    /// the incident and hope that we will re-establish a connection
    /// soon.
    fn rx_keepalive(self: &Arc<Self>) {
        warn!("Received a Keep-alive, but don't have TX link! Waiting for introducer to do it's job...");
    }

    /// A keepalive on a valid duplex connection
    fn dup_keepalive(self: &Arc<Self>, peer: Arc<Peer>) {
        trace!("Receiving keep-alive and queueing reply!");
        task::spawn(async move { Self::send_keepalive(peer).await });
    }

    /// Handle an incoming HELLO message on Tx, or Rx only connections
    ///
    /// A hello can come from a peer that we have said hello to before
    /// (TxOnly), or a peer that has just introduced itself without us
    /// knowing it before (RxOnly).  If the node is running in dynamic
    /// mode, check if the peer is in the set of "theoretically known
    /// peers" before accepting the hello.
    async fn handle_hello(self: &Arc<Self>, src_addr: &SourceAddr, port: u16) {
        let maybe_id = self.routes.find_via_srcport(src_addr, port).await;
        let upm = "Received HELLO from unknown peer.";

        match (self.mode, maybe_id) {
            (Mode::Static, None) => {
                info!("{} Running STATIC: dropping packet!", upm);
                return;
            }
            (Mode::Dynamic, None) => {
                trace!("{} Running DYNAMIC: establishing reverse connection!", upm);
                let id = self.routes.find_via_src(src_addr).await.unwrap();
                self.routes.upgrade(id, port).await;
            }
            (mode, Some(id)) => {
                trace!(
                    "[Mode: {}] Received HELLO from known peer; responding with keep-alive",
                    match mode {
                        Mode::Dynamic => "dynamic",
                        Mode::Static => "static",
                    }
                );
                let peer = self.routes.get_peer(id).await.unwrap();
                task::spawn(async move { Self::send_keepalive(peer) });
            }
        }
    }

    /// Wait n seconds and then reply to a keep-alive
    async fn send_keepalive(peer: Arc<Peer>) {
        task::sleep(Duration::from_secs(10)).await;
        peer.send(Packet::KeepAlive).await;
    }
}
