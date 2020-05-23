use crate::{Error, Mode, Packet, PeerList, PeerState, Result};
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
use std::{collections::BTreeMap, net::SocketAddr, time::Duration};
use tracing::{debug, error};

/// A wrapper around tcp socket logic
pub(crate) struct Socket {
    inner: TcpListener,
    streams: RwLock<BTreeMap<usize, TcpStream>>,
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
                    streams: RwLock::new(BTreeMap::new()),
                    port,
                    id: id.into(),
                })
            })?)
    }

    /// Send a fully encoded packet to a peer.  At this point
    /// connection checking has already been done
    pub(crate) async fn send(self: &Arc<Self>, peer: usize, data: Frame) -> Result<()> {
        let mut stream = get_stream(&self.streams, peer).await;
        send_packet(&mut stream, Packet::Frame(data), &self.id)
            .await
            .map_err(|_| Error::FailedToSend)
    }

    pub(crate) async fn send_all(self: &Arc<Self>, data: Frame) -> Result<()> {
        let streams = self.streams.read().await;
        let ids: Vec<_> = streams.iter().map(|(id, _)| id).collect();
        for id in ids {
            self.send(*id, data.clone()).await?;
        }

        Ok(())
    }

    /// Start peering by sending a Hello packet
    pub(crate) async fn introduce(self: &Arc<Self>, id: usize, ip: SocketAddr) -> Option<()> {
        if self.streams.read().await.contains_key(&id) {
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

                if socket.streams.read().await.contains_key(&id) {
                    debug!("Peer '{}' (ID {}) appears to already have a connection. Ceasing reconnect loop.", ip.to_string(), id);
                    break;
                }

                debug!("{}, Attempting connection to peer '{}'", pre, ip.to_string());
                let mut s = match TcpStream::connect(ip).await {
                    Ok(s) => { debug!("Successfully connected to peer '{}'", ip.to_string()); s },
                    Err(e) => {
                        error!("Failed to connect to peer '{}': {}", ip.to_string(), e.to_string());
                        task::sleep(Duration::from_secs(30)).await;
                        debug!("Retry timeout expired for peer '{}', proceeding with retry {}", ip.to_string(), ctr);
                        continue;
                    }
                };

                socket.streams.write().await.insert(id, s.clone());
                &socket.streams.read().await;

                match send_packet(&mut s, Packet::Hello { port: socket.port }, &socket.id).await {
                    Ok(()) => break,
                    Err(e) => {
                        error!("Failed to send packet: {}", e.to_string());
                        task::sleep(Duration::from_secs(10)).await;
                        ctr += 1;
                        continue;
                    }
                }
            }
        });

        Some(())
    }

    /// Run the async
    pub(crate) fn start(
        self: &Arc<Self>,
        mode: Mode,
        peers: &Arc<PeerList>,
    ) -> Receiver<(Frame, usize)> {
        let socket = Arc::clone(&self);
        let peers = Arc::clone(peers);
        let (tx, rx) = channel(1);
        task::spawn(Self::run(tx, mode, socket, peers));
        rx
    }

    async fn run(tx: Sender<(Frame, usize)>, mode: Mode, socket: Arc<Self>, peers: Arc<PeerList>) {
        let mut inc = socket.inner.incoming();

        println!(
            "{} Listening for: {:?}",
            socket.id,
            socket.inner.local_addr()
        );
        while let Some(Ok(mut stream)) = inc.next().await {
            loop {
                let addr = match stream.peer_addr() {
                    Ok(a) => a,
                    Err(_) => continue,
                };

                // Drop unknown connections
                let state = peers.peer_state(&addr).await;
                if state == PeerState::Unknown && !mode.dynamic() {
                    println!(
                        "{} Connection from unknown peer `{}`, closing!",
                        socket.id, addr
                    );
                    break;
                }

                // Try to read a packet
                let mut fb = PacketBuilder::new(&mut stream);
                if let Err(e) = fb.parse().await {
                    error!("Failed to parse incoming message: {:?}... dropping connection", e);
                    break;
                }

                let f = fb.build().unwrap();

                println!("{} Full packet `{:?}` received!", socket.id, f);

                // Disambiguate differente packet types
                match (f, state) {
                    // Forward to inbox
                    (Packet::Frame(f), PeerState::Valid) => {
                        let id = peers.get_id_by_src(&addr).await.unwrap();
                        tx.send((f, id)).await;
                    }
                    (Packet::Frame(_), PeerState::Unverified) => {
                        println!(
                            "Dropping incoming packet because peer {:?} is unverified!",
                            &addr
                        );
                    }

                    // Add to stream/ peer list
                    (Packet::Hello { port }, PeerState::Unverified) => {
                        let id = peers.add_src(&addr, port).await.unwrap();
                        let dst = peers.get_dst_by_src(&addr).await.unwrap();

                        // When we have already introduced ourselves, but
                        // the peer wasn't previously verified because we
                        // hadn't gotten a message from them, we need to
                        // cancel introduction and just switch to sending
                        // a keep-alive.
                        if socket.introduce(id, dst).await.is_none() {
                            let mut stream = get_stream(&socket.streams, id).await;
                            match send_packet(&mut stream, Packet::KeepAlive, &socket.id).await {
                                Ok(_) => (),
                                Err(e) => error!("Failed to send KeepAlive packet to newly introduced socket ID {}: {}", socket.id, e.to_string())
                            };
                        }
                    }
                    (Packet::Hello { port: _ }, PeerState::Valid) => {
                        let id = peers.get_id_by_src(&addr).await.unwrap();
                        let mut stream = get_stream(&socket.streams, id).await;
                        match send_packet(&mut stream, Packet::KeepAlive, &socket.id).await {
                            Ok(_) => (),
                            Err(e) => error!("Failed to send KeepAlive packet to known valid socket ID {}: {}", socket.id, e.to_string())
                        };
                    }

                    // Reply to a keep-alive with 2seconds delay
                    (Packet::KeepAlive, _) => {
                        let peers = Arc::clone(&peers);
                        let id = peers.get_id_by_src(&addr).await.unwrap();
                        let mut stream = get_stream(&socket.streams, id).await;
                        let node_id = socket.id.clone();
                        task::spawn(async move {
                            task::sleep(Duration::from_secs(2)).await;
                            println!("{} Replying to keep-alive!", node_id);
                            match send_packet(&mut stream, Packet::KeepAlive, &node_id).await {
                                Ok(_) => (),
                                Err(e) => error!("Failed to reply to KeepAlive packet from socket ID {}: {}", node_id, e.to_string())
                            };
                        });
                    }
                    (packet, state) => {
                        panic!(format!("Failed with tuple: {:?}, {:?}", packet, state))
                    }
                }
            }

            println!("{} Exited read-work loop!", socket.id);
        }

        println!("{} Exited listen loop!", socket.id);
    }
}

async fn get_stream(streams: &RwLock<BTreeMap<usize, TcpStream>>, id: usize) -> TcpStream {
    let s = streams.read().await;
    s.get(&id)
        .expect(&format!("No stream for id: {}", id))
        .clone()
}

#[tracing::instrument(skip(stream, packet), level = "debug")]
async fn send_packet(stream: &mut TcpStream, packet: Packet, id: &str) -> io::Result<()> {
    let mut vec = serialize(&packet).unwrap();
    match packet {
        Packet::Hello { .. } => debug!(
            "{} Sending HELLO to {:?}",
            id,
            stream.peer_addr()?.to_string()
        ),
        Packet::KeepAlive => debug!(
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
