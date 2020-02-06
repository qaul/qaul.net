//! netmod-udp is a UDP overlay for Ratman
#![allow(warnings)]

mod addrs;
pub(crate) use addrs::AddrTable;

mod socket;
pub(crate) use socket::Socket;

mod framing;
pub(crate) use framing::{Envelope, FrameExt};

use async_std::{sync::Arc, task};
use async_trait::async_trait;
use netmod::{Endpoint as EndpointExt, Frame, Recipient, Result, Target};

#[derive(Clone)]
pub struct Endpoint {
    socket: Arc<Socket>,
    addrs: Arc<AddrTable>,
}

impl Endpoint {
    /// Create a new endpoint and spawn a dispatch task
    pub fn spawn(addr: &str) -> Arc<Self> {
        task::block_on(async move {
            let addrs = Arc::new(AddrTable::new());
            Arc::new(Self {
                socket: Socket::with_addr(addr, Arc::clone(&addrs)).await,
                addrs,
            })
        })
    }
}

#[async_trait]
impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&mut self, frame: Frame, target: Target) -> Result<()> {
        match target {
            /// Sending to a user, 
            Target::Single(ref id) => {
                self.socket
                    .send(frame, self.addrs.ip(*id).await.unwrap())
                    .await
            },
            Target::Flood => {
                // self.socket.send_many
            }
        }

        Ok(())
    }

    async fn next(&mut self) -> Result<(Frame, Target)> {
        unimplemented!()
    }
}
