//! netmod-overlay is an internet overlay netmod for Qaul.net
#![allow(warnings)]

mod socket;
pub(crate) use socket::Socket;

use async_std::{sync::Arc, task};
use async_trait::async_trait;
use netmod::{Endpoint as EndpointExt, Frame, Recipient, Result, Target};

#[derive(Clone)]
pub struct Endpoint {
    socket: Arc<Socket>,
    server: String
}

impl Endpoint {
    /// Create a new endpoint binding the given address that connects to a given central
    /// server
    pub fn spawn(addr: &str, server: &str) -> Self {
        task::block_on(async move {
            Self {
                socket: Socket::with_addr(addr).await,
                server: server.to_string()
            }
        })
    }
}

#[async_trait]
impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&self, frame: Frame, _target: Target) -> Result<()> {
        self.socket
            .send(frame, &self.server)
            .await;
        Ok(())
    }

    async fn next(&self) -> Result<(Frame, Target)> {
        let frame = self.socket.next().await;
        Ok((frame, Target::default()))
    }
}

