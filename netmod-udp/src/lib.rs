//! netmod-udp is a UDP overlay for RATMAN

mod addrs;
use addrs::AddrTable;

mod socket;
use socket::Socket;

mod framing;
use framing::{Envelope, FrameExt};

use netmod::{Endpoint as EndpointExt, Frame, Result};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct FrameEnvelope(Frame, u16);

/// Represents an IP network tunneled via UDP
pub struct Endpoint {
    socket: Socket,
    addrs: Arc<AddrTable>,
}

impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        1024 // just an arbitrary number for now
    }

    fn send(&mut self, frame: Frame, target: i16) -> Result<()> {
        match target {
            -1 => self.socket.send_many(frame, self.addrs.all()),
            id => self.socket.send(frame, self.addrs.ip(id as u16).unwrap()),
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
