use std::{collections::{HashMap, HashSet}, time::{Duration, Instant}};
use std::hash::Hash;

use crate::{QAUL_ID_ADVERT_BYTES, constants::{WRONG_ROLE_DEFER_MS}};
use crate::constants::MAX_CONNECTIONS;
use crate::gossip::{Gossip, NeighbourList, Prefix};
use crate::link::{LinkAction, LinkEvent};
pub use crate::link::LinkStats;
pub use crate::queue::send_queue::SendFailure;
use crate::{QAUL_ID_BYTES, link::Link};

// Sans I/O engine for driving all ble protocol elements

/// Our side of a link: we connected to them, or they connected to us.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// We connected out: GATT client
    Central,
    /// They connected to us: our GATT server
    Peripheral,
}

/// What the engine needs from a peer key. The platform picks the type
/// (Linux: address + role), the engine only asks it for its role.
pub trait Peer: Copy + Eq + Hash {
    fn role(&self) -> Role;
}

/// Engine events are the engines input, something that I/O wants done.
#[derive(Debug, PartialEq)]
pub enum EngineEvent<P> {
    /// Peer subscribed / central finished discovery
    PeerReady { peer: P, mtu: usize },        
    /// Received a frame, process.
    Frame { peer: P, bytes: Vec<u8> }, 
    /// We read the peer's qaul ID from its READ_CHAR (central only).
    PeerIdRead { peer: P, id: [u8; QAUL_ID_BYTES] },
    PeerGone { peer: P },
    Send { to: [u8; QAUL_ID_BYTES], message: Vec<u8>, message_id: Vec<u8> },
    /// A qaul advert was heard; `peer` is the key we'd use if we connect.
    Discovered {peer: P, id_prefix: [u8; QAUL_ID_ADVERT_BYTES], rssi: i16},
    /// A `Connect` we asked for didn't produce a link.
    ConnectFailed { peer: P },
    Tick,
    /// Last transmit for a link was sent to the radio, we can now retrieve the next
    FrameSent { peer:P }, 
}

/// Engine Actions are the engines output, data ready for use in the ble_module.
#[derive(Debug, PartialEq)]
pub enum EngineAction<P> {
    Transmit   { peer: P, bytes: Vec<u8> },   // → outbox
    Disconnect { peer: P },                   // → drop the outbox / remove the device
    PeerUp     { id: [u8; QAUL_ID_BYTES] },   // → send_device_found
    PeerDown   { id: [u8; QAUL_ID_BYTES] },   // → send_device_unavailable
    Connect    { peer: P },                   // → start a central connection
    Deliver    { from: [u8; QAUL_ID_BYTES], message: Vec<u8> },
    SendResult { message_id: Vec<u8>, result: Result<(), SendFailure> },
}

/// One link as the engine sees it, for status logs and debug views.
#[derive(Debug, PartialEq)]
pub struct LinkSnapshot<P> {
    pub peer: P,
    /// The peers qaul id once identified.
    pub id: Option<[u8; QAUL_ID_BYTES]>,
    /// Frames can reach the peer.
    pub ready: bool,
    /// How busy its send side is.
    pub stats: LinkStats,
}

/// What the engine knows right now. Data only: each platform decides how to show it.
#[derive(Debug, PartialEq)]
pub struct Snapshot<P> {
    pub our_id: [u8; QAUL_ID_BYTES],
    pub links: Vec<LinkSnapshot<P>>,
    /// Connects asked for that haven't become a link or failed yet.
    pub connecting: usize,
}

pub struct Engine<P> {
    qaul_id: [u8; QAUL_ID_BYTES],
    links: HashMap<P, Link>,
    /// Each identified peer's qaul ID. see Mac rotation
    ids: HashMap<P, [u8; QAUL_ID_BYTES]>,
    /// Connects weve asked for that haven't become a link or failed yet.
    connecting: HashSet<P>,
    /// When we first heard a node who should be connecting to us, used in the wrong role fallback.
    deferred_since: HashMap<[u8; QAUL_ID_ADVERT_BYTES], Instant>,
    /// Neighbour gossip: our own list and the relay of everyone elses.
    gossip: Gossip,
    /// Our set of identified neighbours changed, so our list goes out after this event.
    neighbours_changed: bool,
    /// Lists that arrived during this event, to relay after it.
    gossip_in: Vec<(P, NeighbourList)>,
}

impl<P: Peer> Engine<P> {
    /// `gossip_seq` is where our neighbour list seq counts up from. The engine has no clock, so the
    /// platform passes one that moves ahead across restarts, otherwise peers
    /// may drop our lists as stale for up to LINK_STATE_TIMEOUT_MS after a restart. 
    pub fn new(qaul_id: [u8; QAUL_ID_BYTES], gossip_seq: u16) -> Self {
        Self {
            qaul_id,
            links: HashMap::new(),
            ids: HashMap::new(),
            connecting: HashSet::new(),
            deferred_since: HashMap::new(),
            gossip: Gossip::new(prefix(&qaul_id), gossip_seq),
            neighbours_changed: false,
            gossip_in: Vec::new(),
        }
    }

    /// Translate all engine events into actions.
    pub fn handle(&mut self, event: EngineEvent<P>, now: Instant) -> Vec<EngineAction<P>> {
        let mut out = self.handle_event(event, now);
        out.extend(self.gossip(now));
        out
    }

    /// Relay the lists that arrived, then send our own if our neighbours changed or the keepalive
    /// is due. Mirrors ConnectionPools relayNeighbourList and broadcastNeighbourList.
    fn gossip(&mut self, now: Instant) -> Vec<EngineAction<P>> {
        let mut out = Vec::new();
        for (from, list) in std::mem::take(&mut self.gossip_in) {
            if let Some(relay) = self.gossip.receive(list, now) {
                // Not back to the link it came from.
                let to: Vec<P> = self.links.keys().copied().filter(|p| *p != from).collect();
                out.extend(self.send_gossip(&to, relay.encode(), now));
            }
        }
        if self.links.is_empty() {
            self.neighbours_changed = false;
        } else if self.neighbours_changed || self.gossip.keepalive_due(now) {
            self.neighbours_changed = false;
            let mut neighbours: Vec<Prefix> = self.ids.values().map(prefix).collect();
            // A peer linked in both roles is one neighbour.
            neighbours.sort();
            neighbours.dedup();
            let ours = self.gossip.ours(neighbours, now).encode();
            // To every link, identified or not, like Android.
            let to: Vec<P> = self.links.keys().copied().collect();
            out.extend(self.send_gossip(&to, ours, now));
        }
        out
    }

    fn send_gossip(&mut self, to: &[P], frame: Vec<u8>, now: Instant) -> Vec<EngineAction<P>> {
        let mut out = Vec::new();
        for &peer in to {
            if let Some(link) = self.links.get_mut(&peer) {
                let actions = link.handle(LinkEvent::Gossip(frame.clone()), now);
                out.extend(self.on_link_actions(peer, actions));
            }
        }
        out
    }

    fn handle_event(&mut self, event: EngineEvent<P>, now: Instant) -> Vec<EngineAction<P>> {
        let mut out = Vec::new();
        match event {
            EngineEvent::PeerReady {peer, mtu} => {
                // A connect we started has finished.
                self.connecting.remove(&peer);
                let Some(link) = self.link_for(peer, now) else {
                    out.push(EngineAction::Disconnect { peer });
                    return out;
                };
                let mut link_actions = link.handle(LinkEvent::MtuChanged(mtu), now);
                link_actions.extend(link.handle(LinkEvent::TransportReady, now));
                out.extend(self.on_link_actions(peer, link_actions));
            }
            EngineEvent::Frame { peer, bytes } => {
                let Some(link) = self.link_for(peer, now) else {
                    out.push(EngineAction::Disconnect { peer });
                    return out;
                };
                let link_actions = link.handle(LinkEvent::Received(bytes), now); // Be careful with borrowing self and link
                out.extend(self.on_link_actions(peer, link_actions));
            }
            EngineEvent::PeerIdRead { peer, id } => {
                // link_for, like Frame: the ID lands even if the read beats PeerReady.
                let Some(link) = self.link_for(peer, now) else {
                    out.push(EngineAction::Disconnect { peer });
                    return out;
                };
                let link_actions = link.handle(LinkEvent::PeerIdRead(id), now);
                out.extend(self.on_link_actions(peer, link_actions));
            }
            EngineEvent::PeerGone { peer } => {
                self.connecting.remove(&peer);
                let actions = self.remove_peer(peer);
                out.extend(actions)
            }
            EngineEvent::Send { to, message, message_id } => {
                let Some(peer) = self.ids.iter().find_map(|(p, id)| (*id == to).then_some(*p)) else {
                    // not identified (or never connected) so this message fails
                    out.push(EngineAction::SendResult { message_id, result: Err(SendFailure::NotIdentified) });
                    return out;
                };
                let Some(link) = self.links.get_mut(&peer) else {
                    // link doesnt exist, we dont create as this isnt an allowed setup link step
                    out.push(EngineAction::SendResult { message_id, result: Err(SendFailure::NoLink) });
                    return out;
                };
                let link_actions = link.handle(LinkEvent::Send{message, message_id: to_hex(&message_id)}, now); // Be careful with borrowing self and link
                out.extend(self.on_link_actions(peer, link_actions));   

            }
            EngineEvent::Discovered { peer, id_prefix, .. } => {
                if self.should_connect(peer, id_prefix, now) {
                    self.connecting.insert(peer);
                    out.push(EngineAction::Connect { peer });
                }
            }
            EngineEvent::ConnectFailed { peer } => {
                self.connecting.remove(&peer);
            }
            EngineEvent::Tick => {
                self.gossip.expire(now);
                // Two passes: handling actions needs &mut self and a Close
                // would remove from links, which can't happen while iterating it.
                let ticked: Vec<_> = self.links
                    .iter_mut()
                    .map(|(p, link)| (*p, link.handle(LinkEvent::Tick, now)))
                    .collect();
                for (peer, actions) in ticked {
                    out.extend(self.on_link_actions(peer, actions));  
                }
            }
            EngineEvent::FrameSent { peer} => {
                if let Some(link) = self.links.get_mut(&peer){
                    let link_actions = link.handle(LinkEvent::FrameSent, now);
                    out.extend(self.on_link_actions(peer, link_actions));  
                };
            }
        }
        out
    }

    /// The peer's Link, created on its first event of any kind as it could be SEND_QAUL_ID before subscribe
    /// (Android alway writes SEND_QAUL_ID before it subscribes).
    /// `None` if it would be a new link while we're at MAX_CONNECTIONS: a central that connected to us
    /// anyway, e.g. from an advert seen just before we stopped advertising.
    fn link_for(&mut self, peer: P, now: Instant) -> Option<&mut Link> {
        if !self.links.contains_key(&peer) && !self.connecting.contains(&peer) && !self.has_room() {
            return None;
        }
        let our_id = self.qaul_id;
        Some(self.links.entry(peer).or_insert_with(|| Link::new(our_id, now)))
    }

    /// Below MAX_CONNECTIONS, counting links and connects still in progress. The platform
    /// advertises only while this is true.
    pub fn has_room(&self) -> bool {
        let connecting = self.connecting.iter().filter(|p| !self.links.contains_key(p)).count();
        self.links.len() + connecting < MAX_CONNECTIONS
    }

    /// Turn what a peer's Link asked for into engine actions.
    fn on_link_actions(&mut self, peer: P, actions: Vec<LinkAction>) -> Vec<EngineAction<P>> {
        let mut out = Vec::new();
        for action in actions {
            match action {
                // The lane is dropped on purpose: FIFO until the scheduler lands.
                LinkAction::Transmit { bytes, .. } => out.push(EngineAction::Transmit { peer, bytes }),
                LinkAction::PeerIdentified(id) => {
                    self.ids.insert(peer, id);
                    self.neighbours_changed = true;
                    out.push(EngineAction::PeerUp { id });
                }
                LinkAction::Neighbours(list) => self.gossip_in.push((peer, list)),
                LinkAction::Deliver { message, from } => {
                    out.push(EngineAction::Deliver { from, message })
                }
                LinkAction::MessageResult { message_id, result } => {
                    out.push(EngineAction::SendResult { message_id: from_hex(&message_id), result })
                }
                LinkAction::Close => {
                    out.push(EngineAction::Disconnect { peer });
                    out.extend(self.remove_peer(peer));
                }
            }
        }
        out
    }

    /// Forget a peer: fail its in-flight messages, and tell libqaul it's down
    /// if it was ever identified.
    fn remove_peer(&mut self, peer: P) -> Vec<EngineAction<P>> {
        let mut out = Vec::new();
        if let Some(link) = self.links.remove(&peer) {
            out.extend(self.on_link_actions(peer, link.disconnect()));
        }
        if let Some(id) = self.ids.remove(&peer) {
            self.deferred_since.remove(&prefix(&id));

            self.neighbours_changed = true;
            out.push(EngineAction::PeerDown { id });
        }
        out
    }

    /// A connect we started hasn't finished yet. Platforms should pause scanning meanwhile: Linux waits for the 
    /// scan round to end before it even starts connecting, in theory Androids scan starves connectGatt.
    pub fn is_connecting(&self) -> bool {
        !self.connecting.is_empty()
    }

    /// The current state, for logs and debug views. Read only.
    pub fn snapshot(&self) -> Snapshot<P> {
        let links = self
            .links
            .iter()
            .map(|(peer, link)| LinkSnapshot {
                peer: *peer,
                id: self.ids.get(peer).copied(),
                ready: link.is_ready(),
                stats: link.stats(),
            })
            .collect();
        Snapshot { our_id: self.qaul_id, links, connecting: self.connecting.len() }
    }

    /// BLE is stopping: close every link, fail every unfinished message and
    /// report every identified peer as down.
    pub fn shutdown(&mut self) -> Vec<EngineAction<P>>{
        let mut out = Vec::new();
        let peers: Vec<P> = self.links.keys().copied().collect();
        for peer in peers {
            // The connection outlives our service unless we drop it, as with Close.
            out.push(EngineAction::Disconnect { peer });
            out.extend(self.remove_peer(peer));
        }
        out
    }

    /// Rules 0–3: should we connect to this advert's sender now?
    fn should_connect(&mut self, peer: P, id_prefix: [u8; QAUL_ID_ADVERT_BYTES], now: Instant) -> bool {
        // 0. Room for another link.
        if !self.has_room() {
            return false;
        }

        // 1. Check if we are connected already (either role)
        if self.ids.values().any(|id| id.starts_with(&id_prefix)) {
            self.deferred_since.remove(&id_prefix);
            return false;
        }

        // 2. Check theres isnt already a connection in progress
        // One at a time: prallel connect may be possible on linux but buys little and is needed on android
        if !self.connecting.is_empty() || self.links.contains_key(&peer) {
            return false;
        }

        // 3. If we have the lower id_prefix, we can connect as central
        if self.qaul_id[..QAUL_ID_ADVERT_BYTES] < id_prefix[..] {
            return true;
        }
        
        let first_seen = self.deferred_since.entry(id_prefix).or_insert(now);
        now.duration_since(*first_seen) >= Duration::from_millis(WRONG_ROLE_DEFER_MS)
        
    }
}

/// The advertised part of a qaul ID, which is how gossip names nodes.
fn prefix(id: &[u8; QAUL_ID_BYTES]) -> Prefix {
    let mut prefix = [0; QAUL_ID_ADVERT_BYTES];
    prefix.copy_from_slice(&id[..QAUL_ID_ADVERT_BYTES]);
    prefix
}

/// libqaul's message IDs are bytes; the Link keys messages by a hex string.
fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// The inverse of [`to_hex`].
fn from_hex(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(hex.get(i..i + 2)?, 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::LIVENESS_TIMEOUT_MS;
    use crate::flc::FlcMessage;
    use std::time::Duration;

    const ID_A: [u8; QAUL_ID_BYTES] = [1, 1, 1, 1, 1, 1, 1, 1];
    const ID_B: [u8; QAUL_ID_BYTES] = [2, 2, 2, 2, 2, 2, 2, 2];

    /// A peer key for tests: any number plus a role. No bluer needed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct TestPeer(u32, Role);

    impl Peer for TestPeer {
        fn role(&self) -> Role {
            self.1
        }
    }

    /// How engine A addresses B, and B addresses A.
    const PEER_B: TestPeer = TestPeer(7, Role::Peripheral);
    const PEER_A: TestPeer = TestPeer(9, Role::Peripheral);

    type Actions = Vec<EngineAction<TestPeer>>;

    fn now() -> Instant {
        Instant::now()
    }

    /// What a phone with `id` writes first: its SEND_QAUL_ID.
    fn hello(id: &[u8; QAUL_ID_BYTES]) -> EngineEvent<TestPeer> {
        EngineEvent::Frame { peer: PEER_B, bytes: FlcMessage::SendQaulId(id).encode() }
    }

    /// Carry each side's Transmits to the other as Frames until both go quiet.
    /// Returns everything else each side did.
    fn pump(a: &mut Engine<TestPeer>, b: &mut Engine<TestPeer>, mut from_a: Actions, mut from_b: Actions) -> (Actions, Actions) {
        let (mut seen_a, mut seen_b) = (Vec::new(), Vec::new());
        while !from_a.is_empty() || !from_b.is_empty() {
            let mut next_a = Vec::new();
            let mut next_b = Vec::new();
            // Each Transmit is delivered, then reported sent like a free pipe would,
            // which hands the sender its next frame.
            for action in from_a.drain(..) {
                match action {
                    EngineAction::Transmit { peer, bytes } => {
                        next_b.extend(b.handle(EngineEvent::Frame { peer: PEER_A, bytes }, now()));
                        next_a.extend(a.handle(EngineEvent::FrameSent { peer }, now()));
                    }
                    other => seen_a.push(other),
                }
            }
            for action in from_b.drain(..) {
                match action {
                    EngineAction::Transmit { peer, bytes } => {
                        next_a.extend(a.handle(EngineEvent::Frame { peer: PEER_B, bytes }, now()));
                        next_b.extend(b.handle(EngineEvent::FrameSent { peer }, now()));
                    }
                    other => seen_b.push(other),
                }
            }
            from_a = next_a;
            from_b = next_b;
        }
        (seen_a, seen_b)
    }

    /// Two engines whose transports are up and who have exchanged qaul IDs.
    fn connected() -> (Engine<TestPeer>, Engine<TestPeer>) {
        let mut a = Engine::new(ID_A, 0);
        let mut b = Engine::new(ID_B, 0);
        let from_a = a.handle(EngineEvent::PeerReady { peer: PEER_B, mtu: 247 }, now());
        let from_b = b.handle(EngineEvent::PeerReady { peer: PEER_A, mtu: 247 }, now());
        let (seen_a, seen_b) = pump(&mut a, &mut b, from_a, from_b);
        assert_eq!(seen_a, vec![EngineAction::PeerUp { id: ID_B }]);
        assert_eq!(seen_b, vec![EngineAction::PeerUp { id: ID_A }]);
        (a, b)
    }

    #[test]
    fn frame_with_send_id_identifies_peer() {
        let mut a = Engine::new(ID_A, 0);
        // Before PeerReady, so the link exists but holds our own SEND_ID back.
        assert_eq!(a.handle(hello(&ID_B), now()), vec![EngineAction::PeerUp { id: ID_B }]);
    }

    #[test]
    fn peer_ready_after_frame_keeps_the_link() {
        let mut a = Engine::new(ID_A, 0);
        a.handle(hello(&ID_B), now());
        // Identified but not ready yet, so the link holds this message back.
        let send = EngineEvent::Send { to: ID_B, message: vec![5; 10], message_id: vec![0xab] };
        assert_eq!(a.handle(send, now()), vec![]);

        // Same link (not a fresh one): our SEND_ID first, and no second PeerUp.
        let sent = a.handle(EngineEvent::PeerReady { peer: PEER_B, mtu: 247 }, now());
        assert_eq!(
            sent,
            vec![EngineAction::Transmit { peer: PEER_B, bytes: FlcMessage::SendQaulId(&ID_A).encode() }]
        );

        // Once the pipe takes it, our neighbour list (queued on Control when B was identified),
        // then the held message.
        let gossip = a.handle(EngineEvent::FrameSent { peer: PEER_B }, now());
        assert!(matches!(
            gossip.as_slice(),
            [EngineAction::Transmit { peer: PEER_B, bytes }] if matches!(crate::frame::decode(bytes), Ok(crate::Frame::Flc(FlcMessage::SendNeighbours(_))))
        ));
        let next = a.handle(EngineEvent::FrameSent { peer: PEER_B }, now());
        assert!(matches!(
            next.as_slice(),
            [EngineAction::Transmit { peer: PEER_B, bytes }] if matches!(crate::frame::decode(bytes), Ok(crate::Frame::Chunk { .. }))
        ));
    }

    #[test]
    fn late_frame_sent_for_a_removed_link_is_ignored() {
        let mut a = Engine::new(ID_A, 0);
        a.handle(hello(&ID_B), now());
        a.handle(EngineEvent::PeerGone { peer: PEER_B }, now());
        // send_frames can finish its last write after the link is gone.
        assert_eq!(a.handle(EngineEvent::FrameSent { peer: PEER_B }, now()), vec![]);
        assert!(a.links.is_empty(), "no new link from a late report");
    }

    #[test]
    fn send_to_unknown_peer_fails() {
        let mut a: Engine<TestPeer> = Engine::new(ID_A, 0);
        let send = EngineEvent::Send { to: ID_B, message: vec![1, 2, 3], message_id: vec![0xab] };
        assert_eq!(
            a.handle(send, now()),
            vec![EngineAction::SendResult { message_id: vec![0xab], result: Err(SendFailure::NotIdentified) }]
        );
    }

    #[test]
    fn message_is_delivered_and_acked() {
        let (mut a, mut b) = connected();
        let send = EngineEvent::Send { to: ID_B, message: vec![5; 600], message_id: vec![0x00, 0xab] };
        let from_a = a.handle(send, now());
        let (seen_a, seen_b) = pump(&mut a, &mut b, from_a, Vec::new());
        assert_eq!(seen_b, vec![EngineAction::Deliver { from: ID_A, message: vec![5; 600] }]);
        assert_eq!(seen_a, vec![EngineAction::SendResult { message_id: vec![0x00, 0xab], result: Ok(()) }]);
    }

    #[test]
    fn peer_gone_twice_is_harmless() {
        let mut a = Engine::new(ID_A, 0);
        a.handle(hello(&ID_B), now());
        assert_eq!(
            a.handle(EngineEvent::PeerGone { peer: PEER_B }, now()),
            vec![EngineAction::PeerDown { id: ID_B }]
        );
        // read_frames and send_frames can both report Gone.
        assert_eq!(a.handle(EngineEvent::PeerGone { peer: PEER_B }, now()), vec![]);
    }

    #[test]
    fn silent_peer_is_dropped_after_liveness_timeout() {
        let mut a = Engine::new(ID_A, 0);
        let t0 = now();
        a.handle(hello(&ID_B), t0);
        a.handle(EngineEvent::PeerReady { peer: PEER_B, mtu: 247 }, t0);
        // Time is an input, so 30 s pass instantly.
        let later = t0 + Duration::from_millis(LIVENESS_TIMEOUT_MS);
        assert_eq!(
            a.handle(EngineEvent::Tick, later),
            vec![EngineAction::Disconnect { peer: PEER_B }, EngineAction::PeerDown { id: ID_B }]
        );
        assert!(a.links.is_empty());
    }

    #[test]
    fn stray_frame_link_that_never_gets_ready_is_dropped() {
        // A peer that went Gone keeps writing: its frame makes a new link that never becomes ready.
        let mut a = Engine::new(ID_A, 0);
        let t0 = now();
        a.handle(EngineEvent::Frame { peer: PEER_B, bytes: FlcMessage::LivenessPing.encode() }, t0);
        let later = t0 + Duration::from_millis(crate::constants::SETUP_TIMEOUT_MS);
        // Never identified, so no PeerDown: libqaul was never told it was up.
        assert_eq!(a.handle(EngineEvent::Tick, later), vec![EngineAction::Disconnect { peer: PEER_B }]);
        assert!(a.links.is_empty());
    }

    #[test]
    fn shutdown_closes_every_link_and_keeps_its_promises() {
        let (mut a, _b) = connected();
        let send = EngineEvent::Send { to: ID_B, message: vec![5; 10], message_id: vec![0xab] };
        a.handle(send, now()); // in flight, never acked

        assert_eq!(
            a.shutdown(),
            vec![
                EngineAction::Disconnect { peer: PEER_B },
                EngineAction::SendResult { message_id: vec![0xab], result: Err(SendFailure::LinkGone) },
                EngineAction::PeerDown { id: ID_B },
            ]
        );
        assert!(a.links.is_empty());
        assert_eq!(a.shutdown(), vec![]);
    }

    /// How A would address B's advert as a central: same phone as PEER_B, other role.
    const CENTRAL_B: TestPeer = TestPeer(7, Role::Central);

    /// An advert from the node with `id`, heard under `peer`.
    fn advert(peer: TestPeer, id: &[u8; QAUL_ID_BYTES]) -> EngineEvent<TestPeer> {
        let id_prefix = id[..QAUL_ID_ADVERT_BYTES].try_into().unwrap();
        EngineEvent::Discovered { peer, id_prefix, rssi: -60 }
    }

    #[test]
    fn lower_id_connects() {
        let mut a = Engine::new(ID_A, 0);
        assert_eq!(a.handle(advert(CENTRAL_B, &ID_B), now()), vec![EngineAction::Connect { peer: CENTRAL_B }]);
    }

    #[test]
    fn higher_id_waits_to_be_connected_to() {
        let mut b = Engine::new(ID_B, 0);
        assert_eq!(b.handle(advert(TestPeer(9, Role::Central), &ID_A), now()), vec![]);
    }

    #[test]
    fn node_linked_in_the_other_role_is_ignored() {
        // A knows ID_B through PEER_B, so B's advert under a new key changes nothing.
        let (mut a, _b) = connected();
        assert_eq!(a.handle(advert(CENTRAL_B, &ID_B), now()), vec![]);
    }

    #[test]
    fn linked_but_unidentified_peer_is_ignored() {
        // Connected, but B's SEND_ID hasn't arrived, so only the key can tell.
        let mut a = Engine::new(ID_A, 0);
        a.handle(advert(CENTRAL_B, &ID_B), now());
        a.handle(EngineEvent::PeerReady { peer: CENTRAL_B, mtu: 247 }, now());
        assert_eq!(a.handle(advert(CENTRAL_B, &ID_B), now()), vec![]);
    }

    #[test]
    fn one_connect_at_a_time() {
        const ID_C: [u8; QAUL_ID_BYTES] = [3; QAUL_ID_BYTES];
        let central_c = TestPeer(8, Role::Central);
        let mut a = Engine::new(ID_A, 0);
        a.handle(advert(CENTRAL_B, &ID_B), now());
        assert_eq!(a.handle(advert(central_c, &ID_C), now()), vec![]);

        // Once the first attempt is over, the next advert gets its turn.
        a.handle(EngineEvent::ConnectFailed { peer: CENTRAL_B }, now());
        assert_eq!(a.handle(advert(central_c, &ID_C), now()), vec![EngineAction::Connect { peer: central_c }]);
    }

    #[test]
    fn connecting_from_connect_until_ready_or_failed() {
        let mut a = Engine::new(ID_A, 0);
        assert!(!a.is_connecting());
        a.handle(advert(CENTRAL_B, &ID_B), now());
        assert!(a.is_connecting());
        a.handle(EngineEvent::PeerReady { peer: CENTRAL_B, mtu: 512 }, now());
        assert!(!a.is_connecting());

        let mut a = Engine::new(ID_A, 0);
        a.handle(advert(CENTRAL_B, &ID_B), now());
        a.handle(EngineEvent::ConnectFailed { peer: CENTRAL_B }, now());
        assert!(!a.is_connecting());
    }

    /// The `n`th phone connected to our GATT server.
    fn inbound(n: u32) -> TestPeer {
        TestPeer(100 + n, Role::Peripheral)
    }

    /// An engine for ID_A with `n` phones connected to it.
    fn with_inbound(n: usize) -> Engine<TestPeer> {
        let mut a = Engine::new(ID_A, 0);
        for i in 0..n as u32 {
            a.handle(EngineEvent::PeerReady { peer: inbound(i), mtu: 247 }, now());
        }
        a
    }

    #[test]
    fn no_connect_out_at_the_cap() {
        let mut a = with_inbound(MAX_CONNECTIONS);
        assert!(!a.has_room());
        assert_eq!(a.handle(advert(CENTRAL_B, &ID_B), now()), vec![]);
    }

    #[test]
    fn inbound_past_the_cap_is_disconnected_without_a_link() {
        let mut a = with_inbound(MAX_CONNECTIONS);
        let extra = inbound(MAX_CONNECTIONS as u32);
        assert_eq!(a.handle(EngineEvent::Frame { peer: extra, bytes: vec![0x06] }, now()), vec![EngineAction::Disconnect { peer: extra }]);
        assert_eq!(a.handle(EngineEvent::PeerReady { peer: extra, mtu: 247 }, now()), vec![EngineAction::Disconnect { peer: extra }]);
        assert_eq!(a.snapshot().links.len(), MAX_CONNECTIONS);
    }

    #[test]
    fn a_link_at_the_cap_still_works() {
        // The cap only stops new links: a linked peer's frames are handled as before.
        let mut a = with_inbound(MAX_CONNECTIONS);
        let actions = a.handle(EngineEvent::Frame { peer: inbound(0), bytes: FlcMessage::SendQaulId(&ID_B).encode() }, now());
        assert!(actions.contains(&EngineAction::PeerUp { id: ID_B }));
    }

    #[test]
    fn a_connect_in_progress_holds_a_slot() {
        let mut a = with_inbound(MAX_CONNECTIONS - 1);
        a.handle(advert(CENTRAL_B, &ID_B), now());
        assert!(!a.has_room());
        // Our own connect finishing still becomes a link, but an inbound one is refused meanwhile.
        let extra = inbound(MAX_CONNECTIONS as u32);
        assert_eq!(a.handle(EngineEvent::PeerReady { peer: extra, mtu: 247 }, now()), vec![EngineAction::Disconnect { peer: extra }]);
        a.handle(EngineEvent::PeerReady { peer: CENTRAL_B, mtu: 247 }, now());
        assert_eq!(a.snapshot().links.len(), MAX_CONNECTIONS);
    }

    /// Make `peer` (with `id`) a ready, identified link of `a`, sending everything that causes.
    /// Returns the neighbour lists sent.
    fn link_up(a: &mut Engine<TestPeer>, peer: TestPeer, id: &[u8; QAUL_ID_BYTES], at: Instant) -> Vec<(TestPeer, NeighbourList)> {
        let mut actions = a.handle(EngineEvent::PeerReady { peer, mtu: 247 }, at);
        actions.extend(a.handle(EngineEvent::Frame { peer, bytes: FlcMessage::SendQaulId(id).encode() }, at));
        drain_lists(a, actions, at)
    }

    /// Hand every frame `actions` sent to each peer back as sent, until nothing is left. Returns
    /// the neighbour lists among them, by peer.
    fn drain_lists(a: &mut Engine<TestPeer>, mut actions: Actions, at: Instant) -> Vec<(TestPeer, NeighbourList)> {
        let mut lists = Vec::new();
        while !actions.is_empty() {
            let mut next = Vec::new();
            for action in actions {
                if let EngineAction::Transmit { peer, bytes } = action {
                    if let Ok(crate::Frame::Flc(FlcMessage::SendNeighbours(update))) = crate::frame::decode(&bytes) {
                        lists.push((peer, NeighbourList::from_flc(&update).unwrap()));
                    }
                    next.extend(a.handle(EngineEvent::FrameSent { peer }, at));
                }
            }
            actions = next;
        }
        lists
    }

    const ID_C: [u8; QAUL_ID_BYTES] = [3; QAUL_ID_BYTES];
    const PEER_C: TestPeer = TestPeer(8, Role::Peripheral);

    #[test]
    fn identifying_a_peer_sends_our_list_naming_it() {
        let t = now();
        let mut a = Engine::new(ID_A, 0);
        let lists = link_up(&mut a, PEER_B, &ID_B, t);
        let ours: Vec<_> = lists.iter().filter(|(_, l)| l.origin == prefix(&ID_A)).collect();
        let (to, list) = ours.last().unwrap();
        assert_eq!((*to, &list.neighbours, list.sealed), (PEER_B, &vec![prefix(&ID_B)], false));
    }

    #[test]
    fn peers_list_is_relayed_to_the_other_links_with_one_hop_less() {
        let t = now();
        let mut a = Engine::new(ID_A, 0);
        link_up(&mut a, PEER_B, &ID_B, t);
        link_up(&mut a, PEER_C, &ID_C, t);

        let from_b = NeighbourList { origin: prefix(&ID_B), seq: 7, ttl: 3, sealed: true, neighbours: vec![prefix(&ID_A)] };
        let actions = a.handle(EngineEvent::Frame { peer: PEER_B, bytes: from_b.encode() }, t);
        let relayed = drain_lists(&mut a, actions, t);
        assert_eq!(relayed, vec![(PEER_C, NeighbourList { ttl: 2, ..from_b.clone() })]);

        // The same list again isn't relayed twice.
        let again = a.handle(EngineEvent::Frame { peer: PEER_B, bytes: from_b.encode() }, t);
        assert_eq!(drain_lists(&mut a, again, t), vec![]);
    }

    #[test]
    fn our_list_goes_out_again_on_the_keepalive() {
        let t = now();
        let mut a = Engine::new(ID_A, 0);
        link_up(&mut a, PEER_B, &ID_B, t);
        let early = a.handle(EngineEvent::Tick, t + Duration::from_millis(crate::constants::NEIGHBOUR_KEEPALIVE_MS - 1));
        assert_eq!(drain_lists(&mut a, early, t), vec![]);
        let due = a.handle(EngineEvent::Tick, t + Duration::from_millis(crate::constants::NEIGHBOUR_KEEPALIVE_MS));
        let lists = drain_lists(&mut a, due, t);
        assert!(lists.iter().any(|(to, l)| *to == PEER_B && l.origin == prefix(&ID_A)));
    }

    #[test]
    fn losing_a_peer_tells_the_rest() {
        let t = now();
        let mut a = Engine::new(ID_A, 0);
        link_up(&mut a, PEER_B, &ID_B, t);
        link_up(&mut a, PEER_C, &ID_C, t);
        let actions = a.handle(EngineEvent::PeerGone { peer: PEER_B }, t);
        let lists = drain_lists(&mut a, actions, t);
        assert_eq!(lists.len(), 1);
        assert_eq!((lists[0].0, &lists[0].1.neighbours), (PEER_C, &vec![prefix(&ID_C)]));
    }

    #[test]
    fn room_again_once_a_link_goes() {
        let mut a = with_inbound(MAX_CONNECTIONS);
        a.handle(EngineEvent::PeerGone { peer: inbound(0) }, now());
        assert!(a.has_room());
        assert_eq!(a.handle(advert(CENTRAL_B, &ID_B), now()), vec![EngineAction::Connect { peer: CENTRAL_B }]);
    }

    #[test]
    fn snapshot_shows_each_link_identity_and_readiness() {
        let mut a = Engine::new(ID_A, 0);
        let empty = a.snapshot();
        assert_eq!((empty.our_id, empty.links.len(), empty.connecting), (ID_A, 0, 0));

        // Identified by its SEND_ID, but not ready to receive yet.
        a.handle(hello(&ID_B), now());
        let links = a.snapshot().links;
        assert_eq!(links.len(), 1);
        assert_eq!((links[0].peer, links[0].id, links[0].ready), (PEER_B, Some(ID_B), false));

        a.handle(EngineEvent::PeerReady { peer: PEER_B, mtu: 247 }, now());
        assert!(a.snapshot().links[0].ready);
    }

    #[test]
    fn hex_round_trips() {
        let id = [0x00, 0x0a, 0xab, 0xff];
        assert_eq!(to_hex(&id), "000aabff");
        assert_eq!(from_hex(&to_hex(&id)), id);
    }
}
