//! TCP internal protocol used to share connection state

use crate::LinkType;
use async_std::{
    io::{self, prelude::ReadExt},
    net::TcpStream,
};
use bincode::{deserialize, serialize};
use byteorder::{BigEndian, ByteOrder};
use netmod::Frame;
use serde::{Deserialize, Serialize};

/// An internally used packet format
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Packet {
    /// A general keep alive message
    ///
    /// Because tcp assumes a client-server architecture, an incoming
    /// connection can't accept data without one device explicitly
    /// being the server, and one being the client.  To build a p2p
    /// network we create reverse connections.  When establishing a
    /// connection, the hello message contains the port which is
    /// swapped into the source address to connect to.
    Hello { port: u16, _type: LinkType },
    /// Response to a Hello on the sending stream
    Ack,
    /// An actual data packet
    Frame(Frame),
}

impl Packet {
    /// Serialises the packet into a length prepended data stream
    pub(crate) fn serialize(&self) -> Vec<u8> {
        let mut vec = serialize(self).unwrap();
        let mut buf = vec![0; 8];
        BigEndian::write_u64(&mut buf, vec.len() as u64);
        buf.append(&mut vec);
        buf
    }
}

/// A utility to read packets from an incoming TCP stream
pub(crate) struct PacketBuilder<'s> {
    stream: &'s mut TcpStream,
    data: Option<Vec<u8>>,
}

impl<'s> PacketBuilder<'s> {
    /// Create a new frame builder from a stream
    pub(crate) fn new(stream: &'s mut TcpStream) -> Self {
        Self { stream, data: None }
    }

    /// Parse incoming data and initialise the builder
    pub(crate) async fn parse(&mut self) -> io::Result<()> {
        let mut len_buf = [0; 8];
        self.stream.read_exact(&mut len_buf).await?;
        let len = BigEndian::read_u64(&len_buf);

        let mut data_buf = vec![0; len as usize];
        self.stream.read_exact(&mut data_buf).await?;
        self.data = Some(data_buf);
        Ok(())
    }

    /// Consume the builder and maybe return a frame
    pub(crate) fn build(self) -> Option<Packet> {
        self.data.and_then(|vec| deserialize(&vec).ok())
    }
}
