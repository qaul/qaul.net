// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! One connected peer: the protocol half of BleConnection.kt.
//!
//! A `Link` owns the send and receive queues for one peer and turns what
//! happens on the connection into what should be done about it. It has no
//! I/O: the caller feeds in events with the current time and this carries out the
//! actions that come back. the caller turns `Transmit` into a GATT write or a notify.
//!
//! # TODO: Ordering rules for the caller
//!
//! the link only guarantees the order of frames within each lane. The caller's
//! scheduler must:
//!
//! 1. Enqueue every action from one handle call before dispatching any of
//!    them. Otherwise an idle scheduler sends whichever frame arrives first,
//!    and a bulk chunk can go out ahead of an ACK in the same batch.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::{ChunkHeader, Frame};
use crate::constants::{
    DEFAULT_CHUNK_SIZE, LIVENESS_TIMEOUT_MS, MAX_CHUNK_SIZE, MEDIUM_MESSAGE_MAX_BYTES,
    PING_INTERVAL_MS, QAUL_ID_BYTES,
};
use crate::flc::FlcMessage;
use crate::frame::decode;
use crate::queue::receive_queue::{ReceiveEvent, ReceiveQueue};
use crate::queue::send_queue::SendQueue;
use crate::queue::send_queue::SendEvent;

/// Which scheduler lane a frame goes out on, highest priority first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    /// Flow control: identity, ACKs, chunk requests, connection establishment
    Control,
    /// Chunks of messages up to `MEDIUM_MESSAGE_MAX_BYTES`.
    Medium,
    /// Chunks of larger messages.
    Bulk,
}

/// Something that happened on the connection.
#[derive(Debug, PartialEq, Eq)]
pub enum LinkEvent {
    /// Frames can now reach the peer: services discovered (central) or the
    /// peer subscribed to notifications (peripheral).
    TransportReady,
    /// The ATT MTU was negotiated.
    MtuChanged(usize),
    /// The central read the peer's qaul ID from READ_CHAR.
    PeerIdRead([u8; QAUL_ID_BYTES]),
    /// A frame arrived from the peer.
    Received(Vec<u8>),
    /// libqaul wants this message sent to the peer.
    Send { message: Vec<u8>, message_id: String },
    /// Time has passed. Drives the liveness ping and timeout
    Tick,
}

/// Something the caller should do.
#[derive(Debug, PartialEq, Eq)]
pub enum LinkAction {
    /// Send this frame to the peer.
    Transmit { bytes: Vec<u8>, lane: Lane },
    /// A message from the peer arrived intact: hand it to libqaul.
    Deliver { message: Vec<u8>, from: [u8; QAUL_ID_BYTES] },
    /// A message we sent was delivered or failed.
    MessageResult { message_id: String, success: bool },
    /// We now know who the peer is. Emitted once per link.
    PeerIdentified([u8; QAUL_ID_BYTES]),
    /// The link is dead, caller should disconnect it.
    Close,
}

pub struct Link {
    our_id: [u8; QAUL_ID_BYTES],
    /// The peer's qaul ID, read from READ_CHAR or SEND_QAUL_ID.
    peer_id: Option<[u8; QAUL_ID_BYTES]>,
    /// Whether frames can reach the peer yet. Until then everything waits.
    ready: bool,
    /// When the last frame arrived from the peer. Starts when the transport is ready.
    last_received: Option<Instant>,
    /// When we last sent a liveness ping.
    last_ping: Option<Instant>,
    /// Whether our SEND_QAUL_ID has gone out. Set to false to send it again.
    id_sent: bool,
    send_queue: SendQueue,
    receive_queue: ReceiveQueue,
    /// Frames waiting to be transmitted, oldest first. Flow control goes here
    /// so it can be held until the transport is ready.
    outbox: VecDeque<(Vec<u8>, Lane)>,
}

impl Link {
    pub fn new(our_id: [u8; QAUL_ID_BYTES]) -> Self {
        Self {
            our_id,
            peer_id: None,
            ready: false,
            last_received: None,
            last_ping: None,
            id_sent: false,
            send_queue: SendQueue::new(our_id, DEFAULT_CHUNK_SIZE),
            receive_queue: ReceiveQueue::new(),
            outbox: VecDeque::new(),
        }
    }

    /// Handle one event and return what to do about it.
    pub fn handle(&mut self, event: LinkEvent, now: Instant) -> Vec<LinkAction> {
        let mut actions = Vec::new();
        match event {
            LinkEvent::TransportReady => {
                self.ready = true;
                self.last_received = Some(now);
            },
            LinkEvent::MtuChanged(mtu) => self.on_mtu_changed(mtu),
            LinkEvent::PeerIdRead(id) => self.identify(id, &mut actions),
            LinkEvent::Received(bytes) => {
                self.last_received = Some(now);
                self.on_chunk_received(&bytes, &mut actions);
            }
            LinkEvent::Send { message, message_id } => {
                self.send_queue.add_message(message, message_id)
            }
            LinkEvent::Tick => self.on_tick(now, &mut actions),
        }
        self.flush(&mut actions);
        actions
    }

    fn on_tick(&mut self, now: Instant, actions: &mut Vec<LinkAction>) {
        // The clocks only start once the transport is ready.
        if !self.ready {
            return;
        }

        // Mirrors Androids reapLiveness: nothing heard for too long, the link is dead.
        let timeout = Duration::from_millis(LIVENESS_TIMEOUT_MS);
        if self.last_received.is_some_and(|last| now.duration_since(last) >= timeout) {
            actions.push(LinkAction::Close);
            return;
        }

        // Mirrors Android's pingAll
        let interval = Duration::from_millis(PING_INTERVAL_MS);
        if self.last_ping.map_or(true, |last| now.duration_since(last) >= interval) {
            self.queue_flc(FlcMessage::LivenessPing);
            self.last_ping = Some(now);
        }
    }

    /// The link is gone. Every message not yet delivered has failed.
    pub fn disconnect(self) -> Vec<LinkAction> {
        self.send_queue
            .fail_all()
            .into_iter()
            .map(|message_id| LinkAction::MessageResult { message_id, success: false })
            .collect()
    }

    /// Mirrors `onMtuNegotiated`
    fn on_mtu_changed(&mut self, mtu: usize) {
        let chunk_size = mtu.saturating_sub(3).min(MAX_CHUNK_SIZE);
        self.send_queue.set_chunk_size(chunk_size);
    }

    /// Record the peer's qaul ID, telling the caller the first time only.
    fn identify(&mut self, id: [u8; QAUL_ID_BYTES], actions: &mut Vec<LinkAction>) {
        if self.peer_id.is_none() {
            self.peer_id = Some(id);
            actions.push(LinkAction::PeerIdentified(id));
        }
    }

    fn queue_flc(&mut self, flc: FlcMessage) {
        self.outbox.push_back((flc.encode(), Lane::Control));
    }

    /// A frame arrived from the peer. Mirrors `onChunkReceived`.
    fn on_chunk_received(&mut self, bytes: &[u8], actions: &mut Vec<LinkAction>) {
        // 1. Decode it with `frame::decode`.
        
        match decode(bytes) {
            Ok(Frame::Flc(flc)) => self.on_flc(flc, actions),
            Ok(Frame::Chunk { header, payload }) => self.on_chunk(header, payload, actions),
            Err(_) => {}  // undecodable: drop it, error?
        }

    }

    fn on_flc(&mut self, flc: FlcMessage, actions: &mut Vec<LinkAction>) {
        match flc {
            FlcMessage::SendQaulId(id) => {
                if let Ok(id) = id.try_into() {
                    self.identify(id, actions);
                }
            },
            FlcMessage::RequestQaulId => self.id_sent = false,
            FlcMessage::AckSuccess { queue_index } => self.on_ack(queue_index, true, actions),
            FlcMessage::AckError { queue_index, .. } => self.on_ack(queue_index, false, actions),
            FlcMessage::MissingChunks (missing) => {
                for chunk in self.send_queue.resend_chunks(&missing) {
                    self.outbox.push_back((chunk, Lane::Medium)); // TODO: the messages own lane, never Control
                }

            }
            _ => {} // ignoring other FLCs for now
        }
    }

    fn on_ack(&mut self, queue_index: u8, success: bool, actions: &mut Vec<LinkAction>) {
        match self.send_queue.on_ack(queue_index, success) {
            Some(SendEvent::Delivered { message_id }) => actions.push(LinkAction::MessageResult { message_id, success: true }),
            Some(SendEvent::Failed { message_id }) => actions.push(LinkAction::MessageResult { message_id, success: false }),
            _ => {}   
        }
    }

    fn on_chunk(&mut self, header: ChunkHeader, payload: &[u8], actions: &mut Vec<LinkAction>) {
        match self.receive_queue.handle_chunk(header, payload) {
            ReceiveEvent::Nothing => {},
            ReceiveEvent::NeedChunks { queue_index, chunks } => {
                let entries: Vec<u16> = chunks.iter().map(|&c| (u16::from(queue_index) << 11) | c).collect();
                for batch in entries.chunks(9) { // NOTE: could this be improved to reflect current MTU?
                    self.queue_flc(FlcMessage::MissingChunks(batch.to_vec()));
                }
            }
            ReceiveEvent::Failed { queue_index, error } => {
                let flc = FlcMessage::AckError { queue_index, error_code: 0 };
                self.queue_flc(flc);
            }
            ReceiveEvent::Received { queue_index, message, qaul_id } => {
                let flc = FlcMessage::AckSuccess { queue_index };
                self.queue_flc(flc);
                actions.push(LinkAction::Deliver { message, from: qaul_id });
            }
        }
    }

    /// Send whatever can be sent. Mirrors `flushSendQueue` + `getChunks`.
    fn flush(&mut self, actions: &mut Vec<LinkAction>) {
        // 1. Not ready yet: send nothing, everything stays queued.

        if !self.ready {
            return;
        }

        // 2. Our SEND_QAUL_ID first, if it has not gone out.

        if !self.id_sent {
            self.outbox.push_front((FlcMessage::SendQaulId(&self.our_id).encode(), Lane::Control));
            self.id_sent = true;
        }

        // 3. Drain everything in the outbox so far, in order.
        self.outbox.drain(..).for_each(|(bytes, lane)| {
            actions.push(LinkAction::Transmit { bytes, lane });
        });

        // 4. Start one waiting message, like getChunks does per flush:
        
        for event in self.send_queue.start_next_message() {
            match event {
                SendEvent::Chunks(chunks) => {
                    let size: usize = chunks.iter().map(|c| c.len()).sum();
                    let lane = if size <= MEDIUM_MESSAGE_MAX_BYTES { Lane::Medium } else { Lane::Bulk };
                    for chunk in chunks {
                        actions.push(LinkAction::Transmit { bytes: chunk, lane });
                    }
                }
                SendEvent::Failed { message_id } => {
                    actions.push(LinkAction::MessageResult { message_id, success: false });
                }
                SendEvent::Delivered { .. } => unreachable!("start_next_message never delivers"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID_A: [u8; QAUL_ID_BYTES] = [1, 1, 1, 1, 1, 1, 1, 1];
    const ID_B: [u8; QAUL_ID_BYTES] = [2, 2, 2, 2, 2, 2, 2, 2];

    /// Time is not used yet, so any instant will do.
    fn now() -> Instant {
        Instant::now()
    }

    /// A message of `len` bytes counting 0, 1, 2… so any misplaced byte shows.
    fn body(len: usize) -> Vec<u8> {
        (0..len).map(|i| i as u8).collect()
    }

    fn send(message: Vec<u8>, message_id: &str) -> LinkEvent {
        LinkEvent::Send { message, message_id: message_id.into() }
    }

    /// The frames among `actions`, with their lanes.
    fn frames(actions: &[LinkAction]) -> Vec<(Vec<u8>, Lane)> {
        actions
            .iter()
            .filter_map(|action| match action {
                LinkAction::Transmit { bytes, lane } => Some((bytes.clone(), *lane)),
                _ => None,
            })
            .collect()
    }

    /// Hand each frame to `link` as if it arrived over the air; collect everything it does.
    fn feed(link: &mut Link, frames: &[(Vec<u8>, Lane)]) -> Vec<LinkAction> {
        frames
            .iter()
            .flat_map(|(bytes, _)| link.handle(LinkEvent::Received(bytes.clone()), now()))
            .collect()
    }

    /// The chunk header of a frame, or `None` for a flow control frame.
    fn chunk_header(bytes: &[u8]) -> Option<ChunkHeader> {
        match decode(bytes) {
            Ok(Frame::Chunk { header, .. }) => Some(header),
            _ => None,
        }
    }

    /// Two links whose transports are up and who have exchanged qaul IDs.
    fn connected() -> (Link, Link) {
        let mut a = Link::new(ID_A);
        let mut b = Link::new(ID_B);
        let a_hello = frames(&a.handle(LinkEvent::TransportReady, now()));
        let b_hello = frames(&b.handle(LinkEvent::TransportReady, now()));
        assert!(feed(&mut b, &a_hello).contains(&LinkAction::PeerIdentified(ID_A)));
        assert!(feed(&mut a, &b_hello).contains(&LinkAction::PeerIdentified(ID_B)));
        (a, b)
    }

    #[test]
    fn nothing_goes_out_until_the_transport_is_ready() {
        let mut a = Link::new(ID_A);
        assert!(a.handle(send(body(10), "m"), now()).is_empty());

        let sent = frames(&a.handle(LinkEvent::TransportReady, now()));
        // Our qaul ID first, on the control lane, then the held message.
        assert_eq!(decode(&sent[0].0), Ok(Frame::Flc(FlcMessage::SendQaulId(&ID_A[..]))));
        assert_eq!(sent[0].1, Lane::Control);
        assert!(sent.len() > 1);
        assert!(sent[1..].iter().all(|(bytes, _)| chunk_header(bytes).is_some()));
    }

    #[test]
    fn mtu_sets_the_chunk_size() {
        let mut a = Link::new(ID_A);
        a.handle(LinkEvent::TransportReady, now());
        a.handle(LinkEvent::MtuChanged(517), now());
        // 400 bytes fit in one 509 byte chunk; at the default 20 it would take 23.
        assert_eq!(frames(&a.handle(send(body(400), "m"), now())).len(), 1);
    }

    #[test]
    fn peer_is_identified_once() {
        let mut b = Link::new(ID_B);
        let hello = vec![(FlcMessage::SendQaulId(&ID_A).encode(), Lane::Control)];
        assert_eq!(feed(&mut b, &hello), vec![LinkAction::PeerIdentified(ID_A)]);
        assert_eq!(feed(&mut b, &hello), vec![]);
    }

    #[test]
    fn request_for_our_id_sends_it_again() {
        let (mut a, _b) = connected();
        let request = vec![(FlcMessage::RequestQaulId.encode(), Lane::Control)];
        let sent = frames(&feed(&mut a, &request));
        assert_eq!(sent, vec![(FlcMessage::SendQaulId(&ID_A).encode(), Lane::Control)]);
    }

    #[test]
    fn message_round_trip_between_two_links() {
        let (mut a, mut b) = connected();

        let chunks = frames(&a.handle(send(body(100), "m"), now()));
        let at_b = feed(&mut b, &chunks);
        assert!(at_b.contains(&LinkAction::Deliver { message: body(100), from: ID_A }));

        // B's ACK goes back to A, which reports the message delivered.
        let at_a = feed(&mut a, &frames(&at_b));
        assert_eq!(at_a, vec![LinkAction::MessageResult { message_id: "m".into(), success: true }]);
    }

    #[test]
    fn lost_chunk_is_requested_and_resent_in_its_own_lane() {
        let (mut a, mut b) = connected();

        // 100 bytes at the default chunk size is 7 chunks. Chunk 2 is lost.
        let chunks = frames(&a.handle(send(body(100), "m"), now()));
        let arrived: Vec<_> = chunks
            .into_iter()
            .filter(|(bytes, _)| chunk_header(bytes).map(|h| h.chunk_index) != Some(2))
            .collect();

        // B notices the gap and asks for it, on the control lane.
        let request = frames(&feed(&mut b, &arrived));
        assert_eq!(request.len(), 1);
        assert_eq!(request[0].1, Lane::Control);

        // A resends chunk 2 with the resend bit, in the message's lane.
        let resend = frames(&feed(&mut a, &request));
        assert_eq!(resend.len(), 1);
        assert_eq!(resend[0].1, Lane::Medium, "a resend must never overtake on Control");
        let header = chunk_header(&resend[0].0).unwrap();
        assert_eq!((header.chunk_index, header.resend), (2, true));

        let at_b = feed(&mut b, &resend);
        assert!(at_b.contains(&LinkAction::Deliver { message: body(100), from: ID_A }));
        let at_a = feed(&mut a, &frames(&at_b));
        assert_eq!(at_a, vec![LinkAction::MessageResult { message_id: "m".into(), success: true }]);
    }

    #[test]
    fn disconnect_fails_every_unfinished_message() {
        let mut a = Link::new(ID_A);
        a.handle(LinkEvent::TransportReady, now());
        a.handle(send(body(10), "in flight"), now());
        assert_eq!(
            a.disconnect(),
            vec![LinkAction::MessageResult { message_id: "in flight".into(), success: false }]
        );
    }

    fn ping() -> (Vec<u8>, Lane) {
        (FlcMessage::LivenessPing.encode(), Lane::Control)
    }

    #[test]
    fn pings_every_interval_once_ready() {
        let t0 = Instant::now();
        let mut a = Link::new(ID_A);
        assert_eq!(a.handle(LinkEvent::Tick, t0), vec![], "no clocks before ready");

        a.handle(LinkEvent::TransportReady, t0);
        assert_eq!(frames(&a.handle(LinkEvent::Tick, t0)), vec![ping()]);
        assert_eq!(frames(&a.handle(LinkEvent::Tick, t0 + Duration::from_secs(1))), vec![]);
        assert_eq!(frames(&a.handle(LinkEvent::Tick, t0 + Duration::from_secs(6))), vec![ping()]);
    }

    #[test]
    fn silent_link_is_closed() {
        let t0 = Instant::now();
        let mut a = Link::new(ID_A);
        a.handle(LinkEvent::TransportReady, t0);
        assert_eq!(a.handle(LinkEvent::Tick, t0 + Duration::from_secs(31)), vec![LinkAction::Close]);
    }

    #[test]
    fn anything_received_keeps_the_link_alive() {
        let t0 = Instant::now();
        let mut a = Link::new(ID_A);
        a.handle(LinkEvent::TransportReady, t0);
        let ping = FlcMessage::LivenessPing.encode();
        a.handle(LinkEvent::Received(ping), t0 + Duration::from_secs(20));
        let actions = a.handle(LinkEvent::Tick, t0 + Duration::from_secs(31));
        assert!(!actions.contains(&LinkAction::Close));
    }
}
