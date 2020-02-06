//! Socket handler module

use crate::{AddrTable, Envelope, FrameExt};
use async_std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    sync::{Arc, RwLock},
    task,
};
use conjoiner;
use netmod::Frame;
use std::collections::VecDeque;

const PORT: u16 = 20120;
const MULTI: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);
const SELF: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

/// Wraps around a UDP socket an the input queue
pub(crate) struct Socket {
    sock: Arc<RwLock<UdpSocket>>,
    inbox: Arc<RwLock<VecDeque<FrameExt>>>,
}

impl Socket {
    /// Create a new socket handler and return a management reference
    pub(crate) async fn with_addr(addr: &str, table: Arc<AddrTable>) -> Arc<Self> {
        let sock = UdpSocket::bind(addr).await.unwrap();
        sock.join_multicast_v4(MULTI, SELF)
            .expect("Failed to join multicast. Error");
        sock.set_multicast_loop_v4(true).unwrap(); // only for testing

        let arc = Arc::new(Self {
            sock: Arc::new(RwLock::new(sock)),
            inbox: Default::default(),
        });

        Self::spawn(Arc::clone(&arc), table)
    }

    /// Send a message to one specific client
    pub(crate) async fn send(&self, frame: Frame, ip: IpAddr) {
        let data = conjoiner::serialise(&frame).unwrap();
        self.sock
            .write()
            .await
            .send_to(&data, SocketAddr::new(ip, PORT))
            .await
            .unwrap();
    }

    /// Send a frame to many recipients (via multicast)
    pub(crate) async fn send_many(&self, frame: Frame, ips: Vec<IpAddr>) {
        let data = conjoiner::serialise(&frame).unwrap();
        self.sock
            .write()
            .await
            .send_to(&data, SocketAddr::new(IpAddr::V4(MULTI.clone()), 12322))
            .await;
    }

    /// Send a multicast with an envelope
    pub(crate) async fn multicast(&self) {
        self.sock
            .write()
            .await
            .send_to(
                &vec![13, 12],
                SocketAddr::new(IpAddr::V4(MULTI.clone()), 12322),
            )
            .await;
    }

    fn spawn(arc: Arc<Self>, table: Arc<AddrTable>) -> Arc<Self> {
        let arc2 = Arc::clone(&arc);
        task::spawn(async move {
            loop {
                let mut buf = vec![0; 8192];
                let socket = arc.sock.write().await;
                match socket.recv_from(&mut buf).await {
                    Ok((_, peer)) => {
                        let udp_env =
                            conjoiner::deserialise(&buf).expect("couldn't deserialise. error: ");
                        match udp_env {
                            Envelope::Announce => {
                                table.set(peer.ip());
                            }
                            Envelope::Data(vec) => {
                                let frame = conjoiner::deserialise(&vec)
                                    .expect("couldn't deserialise Frame");
                                dbg!(&frame);
                                let id = table.id(&peer.ip()).await.unwrap();
                                arc.inbox.write().await.push_back(FrameExt(frame, id));
                            }
                        }
                    }
                    val => {
                        // TODO: handle errors more gracefully
                        dbg!(val).expect("Crashed UDP thread!");
                    }
                }
            }
        });
        arc2
    }
}

#[test]
fn test_init() {
    task::block_on(async move {
        let table = Arc::new(AddrTable::new());
        let sock = Socket::with_addr("0.0.0.0:12322", table).await;
        println!("Multicasting");
        sock.multicast();
    });
}
