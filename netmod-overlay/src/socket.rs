//! Socket handler module

use async_std::{
    prelude::*,
    future::{self, Future},
    net::{ToSocketAddrs, SocketAddr, UdpSocket},
    pin::Pin,
    sync::{Arc, RwLock},
    task::{self, Poll},
};
use std::time::Duration;
use conjoiner;
use netmod::{Frame, Target};
use std::collections::VecDeque;
use crate::{Peer, AddrTable};

/// Wraps around a UDP socket an the input queue
#[derive(Clone)]
pub(crate) struct Socket {
    sock: Arc<RwLock<UdpSocket>>,
    inbox: Arc<RwLock<VecDeque<Frame>>>,
    addrs: Arc<AddrTable>,
}

impl Socket {
    /// Create a new socket handler and return a management reference
    pub(crate) async fn with_addr(addr: &str) -> Arc<Self> {
        let sock = UdpSocket::bind(addr).await.unwrap();

        let arc = Arc::new(Self {
            sock: Arc::new(RwLock::new(sock)),
            inbox: Default::default(),
            addrs: Arc::new(AddrTable::new())
        });

        Self::incoming_handle(arc.clone());
        arc
    }

    /// Send a message to one specific client
    pub(crate) async fn send<A: ToSocketAddrs>(&self, frame: &Frame, addr: A) {
        let data = conjoiner::serialise(frame).unwrap();
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

    fn incoming_handle(arc: Arc<Self>) {
        task::spawn(async move {
            loop {
                // This is a bad idea
                let mut buf = vec![0; 8192];

                {
                    let socket = arc.sock.write().await;

                    // recieving is lower priority than sending, so time out and just wait until later
                    // if need be
                    match socket.recv_from(&mut buf).timeout(Duration::from_millis(10)).await {
                        Ok(v) => match v {
                            Ok((_, peer)) => {
                                let frame: Frame =
                                    conjoiner::deserialise(&buf).expect("couldn't deserialise frame. error: ");
                                arc.inbox.write().await.push_back(frame);
                            }
                            val => {
                                // TODO: handle errors more gracefully
                                dbg!(val).expect("Crashed UDP thread!");
                            }
                        },
                        Err(_) => {
                            task::sleep(Duration::from_millis(10));
                        }
                    }
                }
            }
        });
    }
}

