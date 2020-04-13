//! netmod-udp is a UDP overlay for Ratman
#![allow(warnings)]

#[macro_use]
extern crate tracing;

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

/// A test that makes two instances on the same device see each other
///
/// In theory this test is a good idea, but in practise it doesn't
/// work.  The multicast protocol doesn't filter by port, but the
/// implementation on Linux does.  This means that unless all
/// participants on the same device are on the same multicast port, we
/// can't get the multicast messages from each other because they're
/// being filtered by the Kernel.
///
/// We still wanna keep this test around just in case we can run on a
/// platform that doesn't do this, or when we can set the
/// non-exclusive port option.
#[test]
#[ignore]
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

        let e1 = Endpoint::spawn(&p1.port);
        let e2 = Endpoint::spawn(&p2.port);

        std::thread::sleep_ms(5000);

        assert_eq!(task::block_on(async { e1.peers().await }), 1);
    })
}
