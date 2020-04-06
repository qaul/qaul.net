//! netmod-overlay is an internet overlay netmod for Qaul.net
#![allow(warnings)]

mod socket;
pub(crate) use socket::Socket;

mod addrs;
pub(crate) use addrs::{Peer, AddrTable};

use async_std::{sync::Arc, task};
use async_trait::async_trait;
use netmod::{Endpoint as EndpointExt, Frame, Recipient, Result, Target, Error};

#[derive(Clone)]
pub struct Endpoint {
    socket: Arc<Socket>,
    server: Option<String>
}

impl Endpoint {
    /// Create a new endpoint binding the given address that connects to a given central
    /// server
    pub fn spawn(addr: &str) -> Self {
        task::block_on(async move {
            Self {
                socket: Socket::with_addr(addr).await,
                server: None
            }
        })
    }

    pub fn set_server(&mut self, server: &str) {
        self.server = Some(server.to_string());
    }
}

#[async_trait]
impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&self, frame: Frame, _target: Target) -> Result<()> {
        let server_addr = match self.server.clone() {
            Some(v) => v,
            None => { return Err(Error::NotSupported); }
        };
        self.socket
            .send(&frame, server_addr)
            .await;
        Ok(())
    }

    async fn next(&self) -> Result<(Frame, Target)> {
        let frame = self.socket.next().await;
        Ok((frame, Target::default()))
    }
}

