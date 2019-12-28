//! Socket handler module

use crate::{AddrTable, Envelope, FrameExt};
use conjoiner;
use netmod::Frame;
use std::net::UdpSocket;
use std::{
    collections::VecDeque,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, RwLock},
    thread
};

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
    pub(crate) fn with_addr(addr: &str, table: Arc<AddrTable>) -> Arc<Self> {
        let sock = UdpSocket::bind(addr).expect("Could not bind socket. Error");
        // sock.set_nonblocking(true)
        //     .expect("Could not set nonblocking on socket. Error");
        sock.join_multicast_v4(&MULTI, &SELF)
            .expect("Failed to join multicast. Error");
        sock.set_multicast_loop_v4(true).unwrap(); // only for testing

        let arc = Arc::new(Self {
            sock: Arc::new(RwLock::new(sock)),
            inbox: Default::default(),
        });

        Self::spawn(Arc::clone(&arc), table)
    }

    /// Send a message to one specific client
    pub(crate) fn send(&self, frame: Frame, ip: IpAddr) {
        let data = conjoiner::serialise(&frame).unwrap();
        self.sock
            .write()
            .unwrap()
            .send_to(&data, SocketAddr::new(ip, PORT))
            .unwrap();
    }

    /// Send a frame to many recipients
    pub(crate) fn send_many(&self, frame: Frame, ips: Vec<IpAddr>) {
        ips.into_iter().for_each(|ip| self.send(frame.clone(), ip));
    }

    /// Send a multicast with an envelope
    pub(crate) fn multicast(&self) {
        self.sock
            .write()
            .unwrap()
            .send_to(
                &vec![13, 12],
                SocketAddr::new(IpAddr::V4(MULTI.clone()), 12322),
            )
            .unwrap();
    }

    fn spawn(arc: Arc<Self>, table: Arc<AddrTable>) -> Arc<Self> {
        let arc2 = Arc::clone(&arc);
        thread::spawn(move || {
            loop {
                let mut buf = vec![0; 8192];
                let socket = arc.sock.write().expect("Socket mutex poisoned");
                match socket.recv_from(&mut buf) {
                    Ok((_, peer)) => {
                        let udp_env =
                            conjoiner::deserialise(&buf).expect("couldn't deserialise. error: ");
                        match udp_env {
                            Envelope::Announce => {
                                table.set(peer.ip());
                            },
                            Envelope::Data(vec) => {
                                let frame = conjoiner::deserialise(&vec)
                                    .expect("couldn't deserialise Frame");
                                dbg!(&frame);
                                let id = table.id(&peer.ip()).unwrap();
                                arc.inbox
                                    .write()
                                    .expect("Inbox mutex poisoned")
                                    .push_back(FrameExt(frame, id));
                            }
                        }
                    },
                    val => {
                        dbg!(val);
                        unimplemented!()
                    }
                }
            }
        });
        arc2
    }
}

#[test]
fn test_init() {
    let table = Arc::new(AddrTable::new());
    let sock = Socket::with_addr("0.0.0.0:12322", table);
    println!("Multicasting");
    sock.multicast();
}
