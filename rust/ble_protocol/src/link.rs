// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! One connected peer: the protocol half of BleConnection.kt.
//!
//! A `Link` owns the send and receive queues for one peer and turns what
//! happens on the connection into what should be done about it. It has no
//! I/O: the caller feeds in events with the current time and this carries out the
//! actions that come back. the caller turns `Transmit` into a GATT write or a notify.
//!
//! # One frame at a time
//!
//! Frames wait here in one queue per lane, and only one frame is sent out at a
//! time: the next comes once the caller reports `FrameSent`. so an ACK waits for at most one bulk chunk,
//! not for every chunk queued before it. [ This Doesnt Work ]
//! per link: there's no global slot shared between links like in Android

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::{ChunkHeader, Frame};
use crate::constants::{
    DEFAULT_CHUNK_SIZE, IDENTITY_RETRY_MS, LIVENESS_TIMEOUT_MS, MAX_CHUNK_SIZE, MEDIUM_MESSAGE_MAX_BYTES, PING_INTERVAL_MS, QAUL_ID_BYTES, SETUP_TIMEOUT_MS, UNRESOLVED_TIMEOUT_MS,
};
use crate::flc::FlcMessage;
use crate::frame::decode;
use crate::queue::receive_queue::{ReceiveEvent, ReceiveQueue};
use crate::queue::send_queue::SendQueue;
use crate::queue::send_queue::{SendEvent, SendFailure};

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
    /// The last `Transmit` has been handed to the radio, the pipe can take the next one.
    FrameSent,
}

/// Something the caller should do.
#[derive(Debug, PartialEq, Eq)]
pub enum LinkAction {
    /// Send this frame to the peer.
    Transmit { bytes: Vec<u8>, lane: Lane },
    /// A message from the peer arrived intact: hand it to libqaul.
    Deliver { message: Vec<u8>, from: [u8; QAUL_ID_BYTES] },
    /// A message we sent was delivered or failed, and why.
    MessageResult { message_id: String, result: Result<(), SendFailure> },
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
    /// When the transport became ready. The unresolved reaper counts from here.
    ready_at: Option<Instant>,
    /// When our SEND_QAUL_ID last went out.
    id_sent_at: Option<Instant>,
    /// When the last frame arrived from the peer. Starts when the transport is ready.
    last_received: Option<Instant>,
    /// When we last sent a liveness ping.
    last_ping: Option<Instant>,
    /// Whether our SEND_QAUL_ID has gone out. Set to false to send it again.
    id_sent: bool,
    send_queue: SendQueue,
    receive_queue: ReceiveQueue,
    /// Frames waiting to be transmitted, one queue per lane, oldest first. Everything is held here until the transport is ready.
    lanes: [VecDeque<Vec<u8>>; 3],
    /// A `Transmit` is out and the caller hasn't reported `FrameSent` yet.
    in_flight: bool,
    /// Time when this link was created
    created_at: Instant,
    /// Every `Transmit` this link has handed out, for the status line.
    frames_sent: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkStats {
    /// Frames waiting in each lane: Control, Medium, Bulk.
    pub lanes: [usize; 3],
    /// A `Transmit` is out and its `FrameSent` hasn't come back yet.
    pub in_flight: bool,
    /// Every `Transmit` handed out since the link was made.
    pub frames_sent: u64,
    /// Messages holding a queue index: started and not ACKed yet (max 29).
    pub sending: usize,
    /// Messages not started yet, holding no queue index.
    pub waiting: usize,
}

/// The lanes, highest priority first.
const LANES: [Lane; 3] = [Lane::Control, Lane::Medium, Lane::Bulk];

impl Link {
    pub fn new(our_id: [u8; QAUL_ID_BYTES], now: Instant) -> Self {
        Self {
            our_id,
            peer_id: None,
            ready: false,
            ready_at: None,
            id_sent_at: None,
            last_received: None,
            last_ping: None,
            id_sent: false,
            send_queue: SendQueue::new(our_id, DEFAULT_CHUNK_SIZE),
            receive_queue: ReceiveQueue::new(),
            lanes: Default::default(),
            in_flight: false,
            created_at: now,
            frames_sent: 0,
        }
    }

    /// True once frames can reach the peer (TransportReady).
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// How busy the send side is right now.
    pub fn stats(&self) -> LinkStats {
        LinkStats {
            lanes: self.lanes.each_ref().map(VecDeque::len),
            in_flight: self.in_flight,
            frames_sent: self.frames_sent,
            sending: self.send_queue.sending_count(),
            waiting: self.send_queue.waiting_count(),
        }
    }

    /// Handle one event and return what to do about it.
    pub fn handle(&mut self, event: LinkEvent, now: Instant) -> Vec<LinkAction> {
        let mut actions = Vec::new();
        match event {
            LinkEvent::TransportReady => {
                self.ready = true;
                self.last_received = Some(now);
                self.ready_at = Some(now);
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
            LinkEvent::FrameSent => self.in_flight = false,
        }
        self.flush(&mut actions, now);
        actions
    }

    fn on_tick(&mut self, now: Instant, actions: &mut Vec<LinkAction>) {
        // The clocks only start once the transport is ready.
        if !self.ready {
            // Mirrors Android's setup timeout. TODO: Re assess usefulness
            let setup_timeout = Duration::from_millis(SETUP_TIMEOUT_MS);
            if now.duration_since(self.created_at) >= setup_timeout {
                actions.push(LinkAction::Close);
            }
            return;
        }

        if self.peer_id.is_none() {
            // Mirrors Android's reapUnresolved: no peer ID this long after ready, drop the link.
            let unresolved_timeout = Duration::from_millis(UNRESOLVED_TIMEOUT_MS);
            if self.ready_at.is_some_and(|ready| now.duration_since(ready) >= unresolved_timeout) {
                actions.push(LinkAction::Close);
                return;
            }

            // Mirrors Android's retryUnresolvedIdentity: resend ours, in case the exchange was
            // lost both ways (we can't tell whether ours arrived). flush does the sending. TODO: this and the android
            // one may be kind of useless if we only send based on theirs not arriving instead of ours.
            let retry = Duration::from_millis(IDENTITY_RETRY_MS);
            if self.id_sent_at.is_some_and(|sent| now.duration_since(sent) >= retry) {
                self.id_sent = false;
            }
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
            .map(|message_id| LinkAction::MessageResult { message_id, result: Err(SendFailure::LinkGone) })
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
        self.queue(flc.encode(), Lane::Control);
    }

    fn queue(&mut self, bytes: Vec<u8>, lane: Lane) {
        self.lanes[lane as usize].push_back(bytes);
    }

    /// The oldest frame of the highest lane that has one.
    fn next_frame(&mut self) -> Option<(Vec<u8>, Lane)> {
        LANES
            .into_iter()
            .find_map(|lane| Some((self.lanes[lane as usize].pop_front()?, lane)))
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
                    self.queue(chunk, Lane::Medium); // TODO: the messages own lane, never Control
                }
            }
            _ => {} // ignoring other FLCs for now
        }
    }

    fn on_ack(&mut self, queue_index: u8, success: bool, actions: &mut Vec<LinkAction>) {
        match self.send_queue.on_ack(queue_index, success) {
            Some(SendEvent::Delivered { message_id }) => actions.push(LinkAction::MessageResult { message_id, result: Ok(()) }),
            Some(SendEvent::Failed { message_id, reason }) => actions.push(LinkAction::MessageResult { message_id, result: Err(reason) }),
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
            ReceiveEvent::PartReceived { queue_index } => {
                self.queue_flc(FlcMessage::AckSuccess { queue_index });
            }
        }
    }

    /// Send whatever can be sent. Mirrors `flushSendQueue` + `getChunks`.
    fn flush(&mut self, actions: &mut Vec<LinkAction>, now: Instant) {
        // 1. Not ready yet: send nothing, everything stays queued.

        if !self.ready {
            return;
        }

        // 2. Our SEND_QAUL_ID first, if it has not gone out.

        if !self.id_sent {
            let id = FlcMessage::SendQaulId(&self.our_id).encode();
            self.lanes[Lane::Control as usize].push_front(id);
            self.id_sent = true;
            self.id_sent_at = Some(now);
        }

        // 3. Start one waiting message, like getChunks does per flush. Its chunks
        //    wait in their lane like everything else.
        for event in self.send_queue.start_next_message() {
            match event {
                SendEvent::Chunks(chunks) => {
                    let size: usize = chunks.iter().map(|c| c.len()).sum();
                    let lane = if size <= MEDIUM_MESSAGE_MAX_BYTES { Lane::Medium } else { Lane::Bulk };
                    for chunk in chunks {
                        self.queue(chunk, lane);
                    }
                }
                SendEvent::Failed { message_id, reason } => {
                    actions.push(LinkAction::MessageResult { message_id, result: Err(reason) });
                }
                SendEvent::Delivered { .. } => unreachable!("start_next_message never delivers"),
            }
        }

        // 4. If the pipe is free, hand it the next highest priority frame.
        if !self.in_flight {
            if let Some((bytes, lane)) = self.next_frame() {
                self.in_flight = true;
                self.frames_sent += 1;
                actions.push(LinkAction::Transmit { bytes, lane });
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

    /// Handle `event`, then report each frame sent as soon as it goes out, like an
    /// always free pipe, until the link has nothing left to send. Returns everything it did.
    fn handle_all(link: &mut Link, event: LinkEvent, at: Instant) -> Vec<LinkAction> {
        let mut actions = link.handle(event, at);
        loop {
            let more = link.handle(LinkEvent::FrameSent, at);
            if more.is_empty() {
                return actions;
            }
            actions.extend(more);
        }
    }

    /// Hand each frame to `link` as if it arrived over the air; collect everything it does.
    fn feed(link: &mut Link, frames: &[(Vec<u8>, Lane)]) -> Vec<LinkAction> {
        frames
            .iter()
            .flat_map(|(bytes, _)| handle_all(link, LinkEvent::Received(bytes.clone()), now()))
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
        let mut a = Link::new(ID_A, now());
        let mut b = Link::new(ID_B, now());
        let a_hello = frames(&handle_all(&mut a, LinkEvent::TransportReady, now()));
        let b_hello = frames(&handle_all(&mut b, LinkEvent::TransportReady, now()));
        assert!(feed(&mut b, &a_hello).contains(&LinkAction::PeerIdentified(ID_A)));
        assert!(feed(&mut a, &b_hello).contains(&LinkAction::PeerIdentified(ID_B)));
        (a, b)
    }

    #[test]
    fn nothing_goes_out_until_the_transport_is_ready() {
        let mut a = Link::new(ID_A, now());
        assert!(a.handle(send(body(10), "m"), now()).is_empty());

        let sent = frames(&handle_all(&mut a, LinkEvent::TransportReady, now()));
        // Our qaul ID first, on the control lane, then the held message.
        assert_eq!(decode(&sent[0].0), Ok(Frame::Flc(FlcMessage::SendQaulId(&ID_A[..]))));
        assert_eq!(sent[0].1, Lane::Control);
        assert!(sent.len() > 1);
        assert!(sent[1..].iter().all(|(bytes, _)| chunk_header(bytes).is_some()));
    }

    #[test]
    fn mtu_sets_the_chunk_size() {
        let mut a = Link::new(ID_A, now());
        handle_all(&mut a, LinkEvent::TransportReady, now());
        a.handle(LinkEvent::MtuChanged(517), now());
        // 400 bytes fit in one 509 byte chunk; at the default 20 it would take 23.
        assert_eq!(frames(&handle_all(&mut a, send(body(400), "m"), now())).len(), 1);
    }

    #[test]
    fn peer_is_identified_once() {
        let mut b = Link::new(ID_B, now());
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

        let chunks = frames(&handle_all(&mut a, send(body(100), "m"), now()));
        let at_b = feed(&mut b, &chunks);
        assert!(at_b.contains(&LinkAction::Deliver { message: body(100), from: ID_A }));

        // B's ACK goes back to A, which reports the message delivered.
        let at_a = feed(&mut a, &frames(&at_b));
        assert_eq!(at_a, vec![LinkAction::MessageResult { message_id: "m".into(), result: Ok(()) }]);
    }

    #[test]
    fn lost_chunk_is_requested_and_resent_in_its_own_lane() {
        let (mut a, mut b) = connected();

        // 100 bytes at the default chunk size is 7 chunks. Chunk 2 is lost.
        let chunks = frames(&handle_all(&mut a, send(body(100), "m"), now()));
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
        assert_eq!(at_a, vec![LinkAction::MessageResult { message_id: "m".into(), result: Ok(()) }]);
    }

    #[test]
    fn disconnect_fails_every_unfinished_message() {
        let mut a = Link::new(ID_A, now());
        a.handle(LinkEvent::TransportReady, now());
        a.handle(send(body(10), "in flight"), now());
        assert_eq!(
            a.disconnect(),
            vec![LinkAction::MessageResult { message_id: "in flight".into(), result: Err(SendFailure::LinkGone) }]
        );
    }

    fn ping() -> (Vec<u8>, Lane) {
        (FlcMessage::LivenessPing.encode(), Lane::Control)
    }

    /// A link that is ready at `t0` and has the peer's ID, so only liveness applies.
    fn identified(t0: Instant) -> Link {
        let mut a = Link::new(ID_A, t0);
        handle_all(&mut a, LinkEvent::TransportReady, t0);
        a.handle(LinkEvent::Received(FlcMessage::SendQaulId(&ID_B).encode()), t0);
        a
    }

    fn our_id() -> (Vec<u8>, Lane) {
        (FlcMessage::SendQaulId(&ID_A).encode(), Lane::Control)
    }

    #[test]
    fn pings_every_interval_once_ready() {
        let t0 = Instant::now();
        let mut a = Link::new(ID_A, t0);
        assert_eq!(a.handle(LinkEvent::Tick, t0), vec![], "no clocks before ready");

        let mut a = identified(t0);
        let mut tick = |at| frames(&handle_all(&mut a, LinkEvent::Tick, at));
        assert_eq!(tick(t0), vec![ping()]);
        assert_eq!(tick(t0 + Duration::from_secs(1)), vec![]);
        assert_eq!(tick(t0 + Duration::from_secs(6)), vec![ping()]);
    }

    #[test]
    fn silent_link_is_closed() {
        let t0 = Instant::now();
        let mut a = identified(t0);
        assert_eq!(a.handle(LinkEvent::Tick, t0 + Duration::from_secs(31)), vec![LinkAction::Close]);
    }

    #[test]
    fn unresolved_link_resends_its_id() {
        let t0 = Instant::now();
        let mut a = Link::new(ID_A, t0);
        handle_all(&mut a, LinkEvent::TransportReady, t0); // our ID goes out here
        let mut tick = |at| frames(&handle_all(&mut a, LinkEvent::Tick, at));
        assert_eq!(tick(t0 + Duration::from_millis(500)), vec![ping()]);
        assert_eq!(tick(t0 + Duration::from_millis(IDENTITY_RETRY_MS)), vec![our_id()]);
    }

    #[test]
    fn unresolved_link_is_closed() {
        let t0 = Instant::now();
        let mut a = Link::new(ID_A, t0);
        a.handle(LinkEvent::TransportReady, t0);
        let t1 = t0 + Duration::from_millis(UNRESOLVED_TIMEOUT_MS);
        assert_eq!(a.handle(LinkEvent::Tick, t1), vec![LinkAction::Close]);
    }

    #[test]
    fn unready_link_is_closed_after_setup_timeout() {
        let t0 = Instant::now();
        let mut a = Link::new(ID_A, t0);
        let timeout = Duration::from_millis(SETUP_TIMEOUT_MS);
        assert_eq!(a.handle(LinkEvent::Tick, t0 + timeout - Duration::from_millis(1)), vec![]);
        assert_eq!(a.handle(LinkEvent::Tick, t0 + timeout), vec![LinkAction::Close]);
    }

    #[test]
    fn ready_link_is_not_closed_by_setup_timeout() {
        let t0 = Instant::now();
        let t1 = t0 + Duration::from_secs(1);
        let mut a = Link::new(ID_A, t0);
        handle_all(&mut a, LinkEvent::TransportReady, t1);
        a.handle(LinkEvent::Received(FlcMessage::SendQaulId(&ID_B).encode()), t1);
        let later = t0 + Duration::from_millis(SETUP_TIMEOUT_MS + 1_000);
        assert!(!a.handle(LinkEvent::Tick, later).contains(&LinkAction::Close));
    }

    #[test]
    fn stats_show_what_is_queued_and_in_flight() {
        let mut a = Link::new(ID_A, now());
        handle_all(&mut a, LinkEvent::TransportReady, now()); // our SEND_ID
        // 100 bytes at the default chunk size is 7 chunks: one goes out, 6 wait.
        a.handle(send(body(100), "m"), now());
        assert_eq!(
            a.stats(),
            LinkStats { lanes: [0, 6, 0], in_flight: true, frames_sent: 2, sending: 1, waiting: 0 }
        );
    }

    #[test]
    fn identified_link_is_neither_retried_nor_reaped() {
        let t0 = Instant::now();
        let mut a = identified(t0);
        a.handle(LinkEvent::Tick, t0);
        let t1 = t0 + Duration::from_millis(UNRESOLVED_TIMEOUT_MS);
        assert_eq!(a.handle(LinkEvent::Tick, t1), vec![]);
    }

    #[test]
    fn anything_received_keeps_the_link_alive() {
        let t0 = Instant::now();
        let mut a = identified(t0);
        let ping = FlcMessage::LivenessPing.encode();
        a.handle(LinkEvent::Received(ping), t0 + Duration::from_secs(20));
        let actions = a.handle(LinkEvent::Tick, t0 + Duration::from_secs(31));
        assert!(!actions.contains(&LinkAction::Close));
    }

    #[test]
    fn only_one_frame_is_out_at_a_time() {
        let mut a = identified(now());
        // 3 chunks, but the pipe gets them one by one.
        assert_eq!(frames(&a.handle(send(body(50), "m"), now())).len(), 1);
        assert_eq!(frames(&a.handle(LinkEvent::Tick, now())), vec![], "ping waits for the pipe");
        assert_eq!(frames(&a.handle(LinkEvent::FrameSent, now())).len(), 1);
    }

    #[test]
    fn control_frame_overtakes_bulk_already_queued() {
        let mut a = identified(now());
        // The pipe takes the first bulk chunk; ~800 more wait in the bulk lane.
        let first = frames(&a.handle(send(body(MEDIUM_MESSAGE_MAX_BYTES + 1), "big"), now()));
        assert_eq!(first.iter().map(|(_, lane)| *lane).collect::<Vec<_>>(), vec![Lane::Bulk]);

        // Meanwhile B's small message arrives whole, so A owes it an ACK.
        let mut b = Link::new(ID_B, now());
        b.handle(send(body(10), "small"), now());
        let from_b = frames(&handle_all(&mut b, LinkEvent::TransportReady, now()));
        let at_a = feed_no_send(&mut a, &from_b);
        assert!(at_a.contains(&LinkAction::Deliver { message: body(10), from: ID_B }));

        // The first chunk is through: the ACK goes next, ahead of the rest of the bulk.
        let next = frames(&a.handle(LinkEvent::FrameSent, now()));
        assert_eq!(next.len(), 1);
        assert_eq!(next[0].1, Lane::Control);
        assert!(matches!(decode(&next[0].0), Ok(Frame::Flc(FlcMessage::AckSuccess { .. }))));
    }

    /// Like `feed`, but the pipe stays busy: nothing is reported sent.
    fn feed_no_send(link: &mut Link, frames: &[(Vec<u8>, Lane)]) -> Vec<LinkAction> {
        frames.iter().flat_map(|(bytes, _)| link.handle(LinkEvent::Received(bytes.clone()), now())).collect()
    }
}
