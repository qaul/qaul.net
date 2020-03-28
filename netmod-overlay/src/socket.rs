//! Socket handler module

use async_std::{
    future::{self, Future},
    net::{ToSocketAddrs, SocketAddr, UdpSocket},
    pin::Pin,
    sync::{Arc, RwLock},
    task::{self, Poll},
};
use conjoiner;
use netmod::{Frame, Target};
use std::collections::VecDeque;

/// Wraps around a UDP socket an the input queue
pub(crate) struct Socket {
    sock: Arc<RwLock<UdpSocket>>,
    inbox: Arc<RwLock<VecDeque<Frame>>>,
}

impl Socket {
    /// Create a new socket handler and return a management reference
    pub(crate) async fn with_addr(addr: &str) -> Arc<Self> {
        let sock = UdpSocket::bind(addr).await.unwrap();

        let arc = Arc::new(Self {
            sock: Arc::new(RwLock::new(sock)),
            inbox: Default::default(),
        });

        Self::spawn(Arc::clone(&arc))
    }

    /// Send a message to one specific client
    pub(crate) async fn send<A: ToSocketAddrs>(&self, frame: Frame, addr: A) {
        let data = conjoiner::serialise(&frame).unwrap();
        self.sock
            .write()
            .await
            .send_to(&data, addr)
            .await
            .unwrap();
    }

    pub(crate) async fn next(&self) -> Frame {
        future::poll_fn(|ctx| {
            let lock = &mut self.inbox.write();
            match unsafe { Pin::new_unchecked(lock).poll(ctx) } {
                Poll::Ready(mut inc) => match inc.pop_front() {
                    Some(f) => Poll::Ready(f),
                    None => Poll::Pending,
                },
                Poll::Pending => Poll::Pending,
            }
        })
        .await
    }

    fn spawn(arc: Arc<Self>) -> Arc<Self> {
        let arc2 = Arc::clone(&arc);
        task::spawn(async move {
            loop {
                let mut buf = vec![0; 8192];
                let socket = arc.sock.write().await;
                match socket.recv_from(&mut buf).await {
                    Ok((_, peer)) => {
                        let vec =
                            conjoiner::deserialise(&buf).expect("couldn't deserialise. error: ");
                        let frame = conjoiner::deserialise(&vec)
                            .expect("couldn't deserialise Frame");
                        dbg!(&frame);
                        arc.inbox
                            .write()
                            .await
                            .push_back(frame);
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

