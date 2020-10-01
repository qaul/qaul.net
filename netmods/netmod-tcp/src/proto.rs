//! TCP internal protocol used to share connection state

use async_std::{
    io::{self, prelude::ReadExt},
    net::TcpStream,
};
use bincode::deserialize;
use byteorder::{BigEndian, ByteOrder};
use netmod::Frame;
use serde::{Deserialize, Serialize};
use tracing::trace;

/// An internally used packet format
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Packet {
    /// A general handshake message
    ///
    /// If this message get's ignored the connection will time out
    /// automatically and the peer should be marked as "unreachable"
    /// in the peer list.
    Hello { port: u16 },
    /// A periodic update to or from a peer
    KeepAlive,
    /// An actual data packet
    Frame(Frame),
}

/// A utility to read packets from an incoming TCP stream
pub(crate) struct PacketBuilder<'s> {
    stream: &'s mut TcpStream,
    data: Option<Vec<u8>>,
}

impl<'s> PacketBuilder<'s> {
    /// Create a new frame builder from a stream
    pub(crate) fn new(stream: &'s mut TcpStream) -> Self {
        tracing::warn!("Creating a parser...");
        Self { stream, data: None }
    }

    /// Parse incoming data and initialise the builder
    pub(crate) async fn parse(&mut self) -> io::Result<()> {
        trace!("Starting to parse a packet!");
        let mut len_buf = [0; 8];
        self.stream.read_exact(&mut len_buf).await?;
        let len = BigEndian::read_u64(&len_buf);

        trace!("Got length of {}", len);
        
        let mut data_buf = vec![0; len as usize];
        self.stream.read_exact(&mut data_buf).await?;
        self.data = Some(data_buf);
        trace!("Read data to buffer!");
        Ok(())
    }

    /// Consume the builder and maybe return a frame
    pub(crate) fn build(self) -> Option<Packet> {
        trace!("Building packet from stream...");
        self.data.and_then(|vec| deserialize(&vec).ok())
    }
}
