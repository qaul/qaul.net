//! netmod-udp is a UDP overlay for Ratman
#![allow(warnings)]

#[macro_use] extern crate tracing;

mod addrs;
pub(crate) use addrs::{AddrTable, Peer};

mod socket;
pub(crate) use socket::Socket;

mod framing;
pub(crate) use framing::{Envelope, FrameExt};

use async_std::{sync::Arc, task};
use async_trait::async_trait;
use netmod::{Endpoint as EndpointExt, Frame, Recipient, Result, Target};
use std::net::ToSocketAddrs;

#[derive(Clone)]
pub struct Endpoint {
    socket: Arc<Socket>,
    addrs: Arc<AddrTable>,
}

impl Endpoint {
    /// Create a new endpoint and spawn a dispatch task
    pub fn spawn(port: u16) -> Self {
        task::block_on(async move {
            let addrs = Arc::new(AddrTable::new());
            Self {
                socket: Socket::with_port(port, Arc::clone(&addrs)).await,
                addrs,
            }
        })
    }

    /// Manually introduce this endpoint to other endpoints
    pub async fn introduce<A: ToSocketAddrs>(&self, addr: A) -> std::io::Result<()> {
        for addr in addr.to_socket_addrs()? {
            self.addrs.set(addr).await;
        }
        Ok(())
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
fn discover() {
    task::block_on(async {
        use async_std::net::{IpAddr, Ipv4Addr};

        let p1 = Peer {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 11000,
        };
        let p2 = Peer {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 11001,
        };

        let e1 = Endpoint::spawn(&p1.to_string());
        let e2 = Endpoint::spawn(&p2.to_string());

        std::thread::sleep_ms(5000);

        assert_eq!(task::block_on(async { e1.peers().await }), 1);
    })
}
