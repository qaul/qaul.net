//! netmod-udp is a UDP overlay for RATMAN

mod addrs;
use addrs::AddrTable;

mod socket;
use socket::Socket;

mod framing;
use framing::{Envelope, FrameExt};

use async_std::{io, task};
use conjoiner;

use netmod::{Endpoint, Frame, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, VecDeque},
    io::ErrorKind,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread,
};

/// An internal envelope that is used as a transfer protocol
#[derive(Serialize, Deserialize)]
enum UdpEnvelope {
    /// A tunneled data payload
    Data(Vec<u8>),
    /// Announce on broadcast that here be netmod's
    Announce,
}

#[derive(Debug, Clone)]
struct FrameEnvelope(Frame, u16);

/// Represents an IP network tunneled via UDP
pub struct UdpEndpoint {
    /// The raw UDP socket
    sock: Arc<Mutex<UdpSocket>>,
    /// An ID counter to keep track of assigned IDs
    curr_id: Mutex<u16>,
    /// A local IP space routing double reverse table
    ips: Arc<Mutex<BTreeMap<u16, IpAddr>>>,
    ids: Arc<Mutex<BTreeMap<IpAddr, u16>>>,
    /// Inbox of incoming frames
    inbox: Arc<Mutex<VecDeque<FrameEnvelope>>>,
}

impl UdpEndpoint {
    /// Increment and return the current ID
    fn id(&self) -> u16 {
        let mut f = self.curr_id.lock().unwrap();
        *f += 1;
        *f
    }

    /// Create a new UDP endpoint handler at the given address.
    pub fn with_addr(addr: &str) -> io::Result<Arc<Self>> {
        let socket = UdpSocket::bind(addr).expect("Could not bind socket. Error");
        socket
            .set_broadcast(true)
            .expect("Could not set broadcast on socket. Error");
        socket
            .set_nonblocking(true)
            .expect("Could not set nonblocking on socket. Error");

        let endpoint = Arc::new(Self {
            sock: Arc::new(Mutex::new(socket)),
            curr_id: Mutex::new(0),
            ips: Default::default(),
            ids: Default::default(),
            inbox: Default::default(),
        });

        Self::spawn(Arc::clone(&endpoint));
        Ok(endpoint)
    }

    /// Wrapper function to encapsulate the run thread
    fn spawn(endpoint: Arc<Self>) {
        thread::spawn(move || {
            loop {
                // TODO: How do we deal with larger sizes? We shouldn't have to because
                // the size_hint says 4k but we might have to.
                let mut buf = vec![0; 8192];

                let socket = endpoint.sock.lock().expect("Socket mutex poisoned");
                match socket.recv_from(&mut buf) {
                    Ok((_, peer)) => {
                        let udp_env =
                            conjoiner::deserialise(&buf).expect("couldn't deserialise. error: ");
                        match udp_env {
                            UdpEnvelope::Announce => {
                                let id = endpoint.id();
                                endpoint.ips.lock().unwrap().insert(id, peer.ip());
                                endpoint.ids.lock().unwrap().insert(peer.ip(), id);
                            }
                            UdpEnvelope::Data(vec) => {
                                let frame = conjoiner::deserialise(&vec)
                                    .expect("couldn't deserialise Frame");
                                let ip_id = *endpoint.ids.lock().unwrap().get(&peer.ip()).unwrap();
                                endpoint
                                    .inbox
                                    .lock()
                                    .expect("Inbox mutex poisoned")
                                    .push_back(FrameEnvelope(frame, ip_id));
                            }
                        }
                    }
                    Err(e) => match e.kind() {
                        ErrorKind::WouldBlock => {
                            #[allow(deprecated)]
                            thread::sleep_ms(10);
                        }
                        _ => {
                            panic!("Could not recv() on socket. Error: {:?}", e);
                        }
                    },
                }
                // TODO: This obviously shouldn't panic, we should do better error
                // handling here.
            }
        });
    }
}

impl Endpoint for UdpEndpoint {
    fn size_hint(&self) -> usize {
        1024 // just an arbitrary number for now
    }

    fn send(&mut self, frame: Frame, target: i16) -> Result<()> {
        let data = conjoiner::serialise(&frame).unwrap();

        match target {
            -1 => {
                // broadcast
            }
            val => {
                self.sock
                    .lock()
                    .unwrap()
                    .send_to(
                        &data,
                        SocketAddr::new(
                            self.ips.lock().unwrap().get(&(val as u16)).unwrap().clone(),
                            20120,
                        ),
                    )
                    .unwrap();
            }
        }
        Ok(())
    }

    fn poll(&mut self) -> Result<Option<(Frame, i16)>> {
        unimplemented!()
    }

    fn listen(&mut self, _: Box<dyn FnMut(Frame, i16) -> Result<()>>) -> Result<()> {
        unimplemented!()
    }
}
