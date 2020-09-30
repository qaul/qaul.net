//! TCP incoming connection server

use crate::{Mode, PacketBuilder, Result, Routes};
use async_std::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tracing::{error, info, trace};

/// The listening server part of the tcp driver
pub(crate) struct Server {
    alive: Arc<AtomicBool>,
    inner: TcpListener,
    routes: Arc<Routes>,
    port: u16,
    mode: Mode,
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

            // Match on the peer-state, message payload tuple
            match (peer.state(), f) {
                _ => todo!(),
            }
        }

        trace!("Exiting connetion work-loop; was there a connection drop?");
    }
}
