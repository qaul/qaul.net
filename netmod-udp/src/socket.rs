//! Socket handler module

use crate::{FrameExt, Envelope};
use std::net::UdpSocket;
use netmod::Frame;
use conjoiner;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
    net::{IpAddr, SocketAddr},
};

const PORT: u16 = 20120;

/// Wraps around a UDP socket an the input queue
pub(crate) struct Socket {
    sock: Arc<RwLock<UdpSocket>>,
    inbox: Arc<RwLock<VecDeque<FrameExt>>>,
}

impl Socket {

    /// Create a new socket handler and return a management reference
    pub(crate) fn with_addr(addr: &str) -> Arc<Self> {
        let sock = UdpSocket::bind(addr).expect("Could not bind socket. Error");
        sock
            .set_broadcast(true)
            .expect("Could not set broadcast on socket. Error");
        sock
            .set_nonblocking(true)
            .expect("Could not set nonblocking on socket. Error");

        let arc = Arc::new(Self {
            sock: Arc::new(RwLock::new(sock)),
            inbox: Default::default(),
        });

        Self::spawn(Arc::clone(&arc))
    }

    /// Send a message to one specific client
    pub(crate) fn send(&self, frame: Frame, ip: IpAddr) {
        let data = conjoiner::serialise(&frame).unwrap();
        self.sock.write().unwrap().send_to(&data, SocketAddr::new(ip, PORT));
    }

    /// Send a frame to many recipients
    pub(crate) fn send_many(&self, frame: Frame, ips: Vec<IpAddr>) {
        ips.into_iter().for_each(|ip| self.send(frame.clone(), ip));
    }

    /// Send a multicast with an envelope
    pub(crate) fn multicast(&self, env: Envelope) {

    }

    fn spawn(arc: Arc<Self>) -> Arc<Self> {
        arc
    }
}
