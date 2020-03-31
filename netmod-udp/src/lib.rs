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
    pub fn spawn(addr: &str) -> Self {
        task::block_on(async move {
            let addrs = Arc::new(AddrTable::new());
            Self {
                socket: Socket::with_addr(addr, Arc::clone(&addrs)).await,
                addrs,
            }
        })
    }

    #[cfg(test)]
    pub async fn peers(&self) -> usize {
        self.addrs.all().await.len()
    }
}

#[async_trait]
impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&self, frame: Frame, target: Target) -> Result<()> {
        match target {
            /// Sending to a user,
            Target::Single(ref id) => {
                self.socket
                    .send(&frame, self.addrs.ip(*id).await.unwrap())
                    .await
            }
            Target::Flood => {
                let addrs = self.addrs.all().await;
                self.socket.send_many(&frame, addrs).await;
            }
        }

        Ok(())
    }

    async fn next(&self) -> Result<(Frame, Target)> {
        let fe = self.socket.next().await;
        Ok((fe.0, fe.1))
    }
}

/// This test requires network access to set the multicast
#[test]
fn simple_two_way() {
    let a = Endpoint::spawn("0.0.0.0:9999");
    let b = Endpoint::spawn("0.0.0.0:8888");

    std::thread::sleep_ms(5000);
    
    assert_eq!(task::block_on(async { a.peers().await }), 1);
}
