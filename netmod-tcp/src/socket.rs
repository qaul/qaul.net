use crate::{Mode, Packet, PeerList, Result};
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

/// A wrapper around tcp socket logic
pub(crate) struct Socket {
    inner: TcpListener,
    streams: RwLock<BTreeMap<usize, TcpStream>>,
    port: u16,
}

impl Socket {
    pub(crate) async fn new(addr: &str, port: u16) -> Result<Arc<Self>> {
        Ok(TcpListener::bind(format!("{}:{}", addr, port))
            .await
            .map(|inner| {
                Arc::new(Self {
                    inner,
                    streams: RwLock::new(BTreeMap::new()),
                    port,
                })
            })?)
    }

    /// Send a fully encoded packet to a peer.  At this point
    /// connection checking has already been done
    pub(crate) async fn send(self: Arc<Self>, peer: SocketAddr, data: Frame) -> Result<()> {
        unimplemented!()
    }

    /// Start peering by sending a Hello packet
    pub(crate) async fn introduce(self: &Arc<Self>, id: usize, ip: SocketAddr) {
        if self.streams.read().await.contains_key(&id) {
            return;
        }

        let mut s = TcpStream::connect(ip).await.unwrap();
        self.streams.write().await.insert(id, s.clone());

        &self.streams.read().await;
        send_packet(&mut s, Packet::Hello { port: self.port }).await;
    }

    /// Run the async
    pub(crate) fn start(self: &Arc<Self>, mode: Mode, peers: &Arc<PeerList>) -> Receiver<Frame> {
        let socket = Arc::clone(&self);
        let peers = Arc::clone(peers);
        let (tx, rx) = channel(1);
        task::spawn(Self::run(tx, mode, socket, peers));
        rx
    }

    async fn run(tx: Sender<Frame>, mode: Mode, socket: Arc<Self>, peers: Arc<PeerList>) {
        let mut inc = socket.inner.incoming();

        println!("Listening for: {:?}", socket.inner.local_addr());
        while let Some(Ok(mut stream)) = inc.next().await {
            let mut addr = match stream.peer_addr() {
                Ok(a) => a,
                Err(_) => continue,
            };

            // Drop unknown connections
            if !peers.is_peer(&addr.ip()).await && !mode.dynamic() {
                println!("Connection from unknown peer `{}`, closing!", addr);
                break;
            }

            // Try to read a packet
            let mut fb = PacketBuilder::new(&mut stream);
            fb.parse().await.unwrap(); // TODO: Don't panic!
            let f = fb.build().unwrap();

            println!("Full packet `{:?}` received!", f);

            // Disambiguate differente packet types
            match f {
                // Forward to inbox
                Packet::Frame(f) => tx.send(f).await,

                // Add to stream/ peer list
                Packet::Hello { port } => {
                    addr.set_port(port);
                    match peers.add(&addr).await {
                        Some(id) => socket.introduce(id, addr).await,
                        None => {
                            let id = peers.get_id(&addr.ip()).await.unwrap();
                            let mut stream = get_stream(&socket.streams, id).await;
                            send_packet(&mut stream, Packet::KeepAlive).await;
                        }
                    }
                }

                // Reply to a keep-alive with 2seconds delay
                Packet::KeepAlive => {
                    let peers = Arc::clone(&peers);
                    let id = peers.get_id(&addr.ip()).await.unwrap();
                    let mut stream = get_stream(&socket.streams, id).await;
                    task::spawn(async move {
                        task::sleep(Duration::from_secs(1)).await;
                        println!("Replying to keep-alive!");
                        send_packet(&mut stream, Packet::KeepAlive).await;
                    });
                }
            }
        }
    }
}

async fn get_stream(streams: &RwLock<BTreeMap<usize, TcpStream>>, id: usize) -> TcpStream {
    let s = streams.read().await;
    s.get(&id)
        .expect(&format!("No stream for id: {}", id))
        .clone()
}

async fn send_packet(stream: &mut TcpStream, packet: Packet) {
    let vec = serialize(&packet).unwrap();
    match packet {
        Packet::Hello { .. } => println!("Sending HELLO to {:?}", stream.peer_addr()),
        Packet::KeepAlive => println!("Sending KEEP-ALIVE to {:?}", stream.peer_addr()),
        _ => {}
    }

    let mut buf = [0; 8];
    BigEndian::write_u64(&mut buf, vec.len() as u64);
    stream.write_all(&buf).await.unwrap();

    if let Err(_) = stream.write_all(&vec).await {
        println!("Failed to send data");
    }
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

    let s1 = Socket::new("127.0.0.1", 8000).await.unwrap();
    let s1_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000);
    let p1 = PeerList::new();
    p1.load(vec![s1_addr]).await.unwrap();
    s1.start(Mode::Static, &p1);

    let s2 = Socket::new("127.0.0.1", 9000).await.unwrap();
    let s2_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);
    let p2 = PeerList::new();
    p2.load(vec![s2_addr]).await.unwrap();
    s2.start(Mode::Static, &p2);

    // Let s1 -> s2
    let id = dbg!(p1.add(&s2_addr.clone().into()).await.unwrap());
    s1.introduce(id, s2_addr.into()).await;

    task::sleep(Duration::from_secs(10)).await;
}
