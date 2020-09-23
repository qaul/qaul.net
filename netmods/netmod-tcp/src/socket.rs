use crate::{Error, LinkState, Mode, Packet, Peer, Peers, Result};
use async_std::{
    io::{
        self,
        prelude::{ReadExt, WriteExt},
    },
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    sync::{channel, Arc, Receiver, RwLock, Sender},
    task,
};
use bincode::{deserialize, serialize};
use byteorder::{BigEndian, ByteOrder};
use netmod::Frame;
use std::{collections::HashMap, net::SocketAddr, time::Duration};
use tracing::{error, info, trace, warn};

/// A wrapper around tcp socket logic
pub(crate) struct Socket {
    inner: TcpListener,
    uplinks: RwLock<HashMap<SocketAddr, TcpStream>>,
    port: u16,
    pub id: String,
}

impl Socket {
    pub(crate) async fn new(addr: &str, port: u16, id: &str) -> Result<Arc<Self>> {
        Ok(TcpListener::bind(format!("{}:{}", addr, port))
            .await
            .map(|inner| {
                Arc::new(Self {
                    inner,
                    uplinks: RwLock::new(Default::default()),
                    port,
                    id: id.into(),
                })
            })?)
    }

    /// Send a fully encoded packet to a peer.  At this point
    /// connection checking has already been done
    pub(crate) async fn send(self: &Arc<Self>, addr: SocketAddr, data: Frame) -> Result<()> {
        let mut stream = get_stream(&self.uplinks, &addr).await;
        let res = send_packet(&mut stream, Packet::Frame(data), &self.id)
            .await
            .map_err(|_| Error::FailedToSend);

        if res.is_err() {
            self.uplinks.write().await.remove(&addr);
        }

        res
    }

    pub(crate) async fn send_all(self: &Arc<Self>, data: Frame) -> Result<()> {
        let streams = self.uplinks.read().await;
        let ids: Vec<_> = streams.iter().map(|(id, _)| id).collect();
        for id in ids {
            self.send(*id, data.clone()).await?;
        }

        Ok(())
    }

    /// Start peering by sending a Hello packet
    pub(crate) async fn introduce(self: &Arc<Self>, id: usize, ip: SocketAddr) -> Option<()> {
        // Skip introduction if there is already an outgoing stream
        if self.uplinks.read().await.contains_key(&ip) {
            return None;
        }

        // In case we fail to immediately connect, we want to just
        // retry the same peer again and again, until it works.  We
        // spawn a long-running task here to accomplish this.
        let socket = Arc::clone(&self);
        task::spawn(async move {
            let mut ctr: u16 = 0;

            loop {
                let pre = match ctr {
                    0 => "".into(),
                    n => format!("[retry #{}]", n),
                };

                if socket.uplinks.read().await.contains_key(&ip) {
                    trace!(
                        "Peer '{}' (ID {}) is already connected!\
                         Ceasing reconnect loop",
                        ip.to_string(),
                        id
                    );
                    break;
                }

                trace!(
                    "{}, Attempting connection to peer '{}'",
                    pre,
                    ip.to_string()
                );
                let mut s = match TcpStream::connect(ip).await {
                    Ok(s) => {
                        trace!("Successfully connected to peer '{}'", ip.to_string());
                        s
                    }
                    Err(e) => {
                        error!(
                            "Failed to connect to peer '{}': {}",
                            ip.to_string(),
                            e.to_string()
                        );
                        // FIXME: make this configurable
                        task::sleep(Duration::from_secs(5)).await;
                        ctr += 1;

                        trace!(
                            "Retry timeout expired for peer '{}', proceeding with retry {}",
                            ip.to_string(),
                            ctr
                        );
                        continue;
                    }
                };

                // We have a connection!  Save the uplink for later
                socket.uplinks.write().await.insert(ip, s.clone());

                // Try to send a hello packet
                match send_packet(&mut s, Packet::Hello { port: socket.port }, &socket.id).await {
                    Ok(()) => break,
                    Err(e) => {
                        error!("Failed to send packet: {}", e.to_string());
                        // FIXME :make this configurable
                        task::sleep(Duration::from_secs(10)).await;
                        ctr += 1;
                        continue;
                    }
                }
            }
        });

        Some(())
    }

    /// Run the async listener
    #[tracing::instrument(skip(self, peers), level = "trace")]
    pub(crate) fn start(
        self: &Arc<Self>,
        mode: Mode,
        peers: &Arc<Peers>,
    ) -> Receiver<(Frame, usize)> {
        let socket = Arc::clone(&self);
        let peers = Arc::clone(peers);
        let (tx, rx) = channel(1);
        trace!("Starting Tcp listener...");
        task::spawn(Self::run(tx, mode, socket, peers));
        rx
    }

    /// Inner run-block which is executed on an async task
    async fn run(tx: Sender<(Frame, usize)>, mode: Mode, socket: Arc<Self>, peers: Arc<Peers>) {
        let mut inc = socket.inner.incoming();

        info!(
            "{} Listening for: {:?}",
            socket.id,
            socket.inner.local_addr()
        );

        while let Some(Ok(mut stream)) = inc.next().await {
            task::spawn(handle_incoming_stream(
                socket.clone(),
                stream,
                mode,
                peers.clone(),
                tx.clone(),
            ));
        }

        info!("{} Exited connection-appept loop!", socket.id);
    }
}

/// Handle a single incoming stream
async fn handle_incoming_stream(
    socket: Arc<Socket>,
    mut stream: TcpStream,
    mode: Mode,
    peers: Arc<Peers>,
    tx: Sender<(Frame, usize)>,
) {
    let src_addr = match stream.peer_addr() {
        Ok(a) => a,
        Err(_) => return,
    };

    // Get the peer associated with the source address and handle
    // various errors that can occur during this process
    let mut peer = match peers.peer_by_src(&src_addr).await {
        Some(p) => p,
        None => {
            match peers
                .add_peer(Peer {
                    id: 0,
                    src: Some(src_addr),
                    dst: None,
                    known: false,
                })
                .await
            {
                Ok(vid) => match peers.peer_by_id(vid).await {
                    Some(peer) => peer,
                    None => {
                        error!("Peer '{}' (src '{}') disappeared between being added and being handled. \
                                Someone removed it? Breaking.", vid, src_addr);
                        return;
                    }
                },
                Err(e) => {
                    error!(
                        "Failed to record new peer with src addr: '{}': {:?}.\
                            THis is not an anticipated condition.",
                        src_addr, e
                    );
                    return;
                }
            }
        }
    };

    loop {
        trace!(
            "Waiting for packet from peer '{}' (src '{}', dst '{}'). \
                State is '{:?}'.",
            peer.id,
            peer.src.map(|a| a.to_string()).unwrap_or("unknown".into()),
            peer.dst.map(|a| a.to_string()).unwrap_or("unknown".into()),
            peer.link_state()
        );

        // Try to read a packet
        let mut fb = PacketBuilder::new(&mut stream);
        if let Err(e) = fb.parse().await {
            use std::io::ErrorKind::*;
            match e.kind() {
                UnexpectedEof => {
                    error!(
                        "Stream is at end of file, lost communication with '{}'.",
                        src_addr
                    );
                    break;
                }
                _ => {
                    error!(
                        "failed to parse incoming message, dropping conniect with '{}': {:?}",
                        src_addr, e
                    );
                    break;
                }
            }
        }

        let f = fb.build().unwrap();
        trace!("Full frame: `{:?}`", f);

        // Disambiguate different packet types
        use LinkState::*;
        match (f, peer.link_state()) {
            // Packets containing a regular frame, from a DUPLEX
            // connection are forwarded to the arrival layer
            (Packet::Frame(f), Duplex) => {
                let id = peers.peer_by_src(&src_addr).await.unwrap().id;
                tx.send((f, id)).await;
            }

            // Packets containing a regular frame, from non-duplex
            // connections are a garbage; they need to HELLO us first
            (Packet::Frame(_), DownOnly) => {
                warn!(
                    "Dropping invalid data frame because peer {:?} is DOWN only.",
                    &src_addr
                );
                continue;
            }

            // Packets containing a HELLO from DOWN only connections
            // can be upgraded to DUPLEX connections
            (Packet::Hello { port }, DownOnly) => {
                if !peer.known && !mode.dynamic() {
                    warn!(
                        "Receiving HELLO from unknown peer.  OP Mode is not 'dynamic'; closing..."
                    );
                }

                let mut dst = src_addr;
                dst.set_port(port);
                match peers.change_dst(peer.id, &dst).await {
                    Ok(p) => peer = p,
                    Err(e) => {
                        error!(
                            "Could not upgrade dst address for peer '{}' src '{:?}' dst '{:?}' \
                             to new destination '{}': {:?}",
                            peer.id, peer.src, peer.dst, dst, e
                        );

                        continue;
                    }
                }

                trace!(
                    "Accepted HELLO from peer '{}'. New dst is '{:?}'",
                    peer.id,
                    peer.dst
                );

                // When we have already introduced ourselves, but the
                // peer wasn't previously verified because we hadn't
                // gotten a message from them, we need to cancel
                // introduction and just switch to sending a
                // keep-alive
                // TODO: is this correct still?
                if socket.introduce(peer.id, dst).await.is_none() {
                    let mut s = get_stream(&socket.uplinks, &dst).await;
                    match send_packet(&mut s, Packet::KeepAlive, &socket.id).await {
                        Ok(_) => (),
                        Err(e) => {
                            error!(
                                "Failed to send KEEP-ALIVE packet to newly introduced \
                                    peer: '{}', at '{}': {}",
                                peer.id,
                                dst,
                                e.to_string(),
                            );
                            break;
                        }
                    }
                }
            }

            // HELLO from a peer we are already in a DULEX connection
            // with are redundant and can be dropped
            (Packet::Hello { port: _ }, Duplex) => {
                warn!(
                    "Received a HELLO packet from a DUPLEX peer connection at '{}'",
                    src_addr
                );
                let id = peers.peer_by_src(&src_addr).await.unwrap().id;
                let mut s = get_stream(&socket.uplinks, &peer.dst.unwrap()).await;
                match send_packet(&mut s, Packet::KeepAlive, &socket.id).await {
                    Ok(_) => (),
                    Err(e) => {
                        error!(
                            "Failed to send KEEP-ALIVE packet to known DUPLEX peer (ID: {}): '{}'",
                            id,
                            e.to_string()
                        );
                        break;
                    }
                }
            }

            // Reply to a KEEP-ALIVE to UpLink and Duplex connections
            // with a small delay
            (Packet::KeepAlive, UpOnly) | (Packet::KeepAlive, Duplex) => {
                let peers = Arc::clone(&peers);
                let mut s = get_stream(&socket.uplinks, &peer.dst.unwrap()).await;
                let node_id = socket.id.clone();
                match send_packet(&mut s, Packet::KeepAlive, &socket.id).await {
                    Ok(_) => (),
                    Err(e) => {
                        error!(
                            "Failed to reply to KEEP-ALIVE from: '{}', on socket ID {}: {}",
                            src_addr,
                            node_id,
                            e.to_string()
                        );
                        break;
                    }
                }
            }

            // Everything else _shouldn't_ happen, but if it does we
            // need to add more valid states
            (packet, state) => panic!(format!(
                "Hit unreachable code-point!.  Parse tuple: ({:?}, {:?})",
                packet, state
            )),
        }
    }

    trace!(
        "Exiting read-work loop for peer '{}' (addr '{}').",
        socket.id,
        src_addr
    );
}

/// Get the upload stream for a particular socket address
async fn get_stream(
    streams: &RwLock<HashMap<SocketAddr, TcpStream>>,
    addr: &SocketAddr,
) -> TcpStream {
    let s = streams.read().await;
    s.get(addr)
        .expect(&format!("No stream for addr: {}", addr))
        .clone()
}

/// Send a packet to a particular stream
#[tracing::instrument(skip(stream, packet), level = "trace")]
async fn send_packet(stream: &mut TcpStream, packet: Packet, id: &str) -> io::Result<()> {
    let mut vec = serialize(&packet).unwrap();
    match packet {
        Packet::Hello { .. } => trace!(
            "{} Sending HELLO to {:?}",
            id,
            stream.peer_addr()?.to_string()
        ),
        Packet::KeepAlive => trace!(
            "{} Sending KEEP-ALIVE to {:?}",
            id,
            stream.peer_addr()?.to_string()
        ),
        _ => {}
    }

    let mut buf = vec![0; 8];
    BigEndian::write_u64(&mut buf, vec.len() as u64);
    buf.append(&mut vec);

    stream.write_all(&buf).await
}

struct PacketBuilder<'s> {
    stream: &'s mut TcpStream,
    data: Option<Vec<u8>>,
}

impl<'s> PacketBuilder<'s> {
    /// Create a new frame builder from a stream
    fn new(stream: &'s mut TcpStream) -> Self {
        Self { stream, data: None }
    }

    /// Parse incoming data and initialise the builder
    async fn parse(&mut self) -> io::Result<()> {
        let mut len_buf = [0; 8];
        self.stream.read_exact(&mut len_buf).await?;
        let len = BigEndian::read_u64(&len_buf);

        let mut data_buf = vec![0; len as usize];
        self.stream.read_exact(&mut data_buf).await?;
        self.data = Some(data_buf);
        Ok(())
    }

    /// Consume the builder and maybe return a frame
    fn build(self) -> Option<Packet> {
        self.data.and_then(|vec| deserialize(&vec).ok())
    }
}

#[async_std::test]
async fn simple_send() {
    use std::net::{Ipv4Addr, SocketAddrV4};

    let s1 = Socket::new("127.0.0.1", 10010, "A =").await.unwrap();
    let s1_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 10010);
    let p1 = PeerList::new();

    let s2 = Socket::new("127.0.0.1", 10011, "B =").await.unwrap();
    let s2_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 10011);
    let p2 = PeerList::new();

    // Make p1 load p2's address, and vice versa
    p1.load(vec![s2_addr]).await.unwrap();
    p2.load(vec![s1_addr]).await.unwrap();

    s1.start(Mode::Static, &p1);
    s2.start(Mode::Static, &p2);

    // Make p1 introduce itself to p2
    let id = p1.get_id_by_dst(&s2_addr.into()).await.unwrap();
    s1.introduce(id, s2_addr.into()).await;

    // Give the test some time to run
    task::sleep(Duration::from_secs(2)).await;

    assert_eq!(p1.peer_state(&s2_addr.into()).await, PeerState::Valid);
}
