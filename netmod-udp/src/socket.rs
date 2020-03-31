//! Socket handler module

use crate::{AddrTable, Envelope, FrameExt};
use access_notifier::AccessNotifier as Notify;
use async_std::{
    future::{self, Future},
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    pin::Pin,
    sync::{Arc, RwLock},
    task::{self, Poll},
};
use conjoiner;
use netmod::{Frame, Target};
use std::collections::VecDeque;

const PORT: u16 = 20120;
const MULTI: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);
const SELF: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

/// Wraps around a UDP socket an the input queue
pub(crate) struct Socket {
    sock: Arc<RwLock<UdpSocket>>,
    inbox: Arc<RwLock<Notify<VecDeque<FrameExt>>>>,
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

        Self::incoming_handle(Arc::clone(&arc), table);
        arc.multicast(Envelope::Announce).await;
        dbg!();
        arc
    }

    /// Send a message to one specific client
    pub(crate) async fn send(&self, frame: &Frame, ip: IpAddr) {
        let data = Envelope::frame(frame);
        self.sock
            .write()
            .await
            .send_to(&data, SocketAddr::new(ip, PORT))
            .await
            .unwrap();
    }

    /// Send a frame to many recipients (via multicast)
    pub(crate) async fn send_many(&self, frame: &Frame, ips: Vec<IpAddr>) {
        let data = Envelope::frame(frame);
        for ip in ips.iter() {
            self.send(frame, *ip).await
        }
    }

    /// Send a multicast with an Envelope
    pub(crate) async fn multicast(&self, env: Envelope) {
        dbg!("Sending mulitcast {}", &env);
        self.sock
            .write()
            .await
            .send_to(
                &dbg!(env.as_bytes()),
                SocketAddr::new(IpAddr::V4(MULTI.clone()), 12322),
            )
            .await;
    }

    pub(crate) async fn next(&self) -> FrameExt {
        future::poll_fn(|ctx| {
            let lock = &mut self.inbox.write();
            match unsafe { Pin::new_unchecked(lock).poll(ctx) } {
                Poll::Ready(ref mut inc) => match inc.pop_front() {
                    Some(f) => Poll::Ready(f),
                    None => {
                        Notify::register_waker(inc, ctx.waker());
                        Poll::Pending
                    }
                },
                Poll::Pending => Poll::Pending,
            }
        })
        .await
    }

    fn incoming_handle(arc: Arc<Self>, table: Arc<AddrTable>) {
        task::spawn(async move {
            loop {
                // This is a bad idea
                let mut buf = vec![0; 8192];

                // Lock the socket, then poll the recv future _once_,
                // and move on.  This way we avoid perma-locking the
                // socket for writers, because UdpSocket doesn't
                // implement split(). . o O ( why not actually? )
                let socket = arc.sock.write().await;
                let mut fut = socket.recv_from(&mut buf);
                let res = future::poll_fn(move |ctx| {
                    match unsafe { Pin::new_unchecked(&mut fut).poll(ctx) } {
                        Poll::Ready(Ok((_, peer))) => Poll::Ready(Some(peer)),
                        _ => Poll::Ready(None),
                    }
                })
                .await;

                if let Some(peer) = res {
                    let env = dbg!(Envelope::from_bytes(&buf));
                    match env {
                        Envelope::Announce => {
                            dbg!("Receiving announce");
                            table.set(peer.ip()).await;
                            arc.multicast(Envelope::Reply).await;
                        }
                        Envelope::Reply => {
                            dbg!("Receiving announce");
                            table.set(peer.ip()).await;
                        }
                        Envelope::Data(_) => {
                            let frame = env.get_frame();
                            dbg!(&frame);

                            let id = table.id(&peer.ip()).await.unwrap();

                            // Append to the inbox and wake
                            let mut inbox = arc.inbox.write().await;
                            inbox.push_back(FrameExt(frame, Target::Single(id)));
                            Notify::wake_if_waker(&mut inbox);
                        }
                    }
                }
            }
        });
    }
}

#[test]
fn test_init() {
    task::block_on(async move {
        let table = Arc::new(AddrTable::new());
        let sock = Socket::with_addr("0.0.0.0:12322", table).await;
        println!("Multicasting");
        sock.multicast(Envelope::Announce);
    });
}
