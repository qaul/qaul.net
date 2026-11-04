// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Neighbour gossip (`SEND_NEIGHBOURS`): each node floods its own neighbour list up to
//! `NEIGHBOUR_TTL` hops, and relays everyone else's. 
//!
//! Only the gossip, none of the topology decisions YET: for now we report
//! `sealed = false`, so Androidss Stage 2 election never waits on us to drop a link, and the
//! table here is only kept to drop duplicates, not to decide anything.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::QAUL_ID_ADVERT_BYTES;
use crate::constants::{LINK_STATE_TIMEOUT_MS, NEIGHBOUR_KEEPALIVE_MS, NEIGHBOUR_TTL};
use crate::flc::{FlcMessage, NeighbourUpdate};

/// A qaul ID prefix, as gossip names nodes.
pub type Prefix = [u8; QAUL_ID_ADVERT_BYTES];

/// A neighbour list as received or sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeighbourList {
    pub origin: Prefix,
    pub seq: u16,
    pub ttl: u8,
    pub sealed: bool,
    pub neighbours: Vec<Prefix>,
}

impl NeighbourList {
    /// From a parsed frame. The parser only hands out prefix sized slices.
    pub fn from_flc(update: &NeighbourUpdate) -> Option<Self> {
        Some(Self {
            origin: update.origin.try_into().ok()?,
            seq: update.seq,
            ttl: update.ttl,
            sealed: update.sealed,
            neighbours: update.neighbours.iter().map(|n| (*n).try_into().ok()).collect::<Option<_>>()?,
        })
    }

    /// The complete SEND_NEIGHBOURS frame.
    pub fn encode(&self) -> Vec<u8> {
        FlcMessage::SendNeighbours(NeighbourUpdate {
            origin: &self.origin,
            seq: self.seq,
            ttl: self.ttl,
            sealed: self.sealed,
            neighbours: self.neighbours.iter().map(|n| &n[..]).collect(),
        })
        .encode()
    }
}

/// `new` is newer than `old`, allowing for the 16 bit wrap.
fn seq_fresher(new: u16, old: u16) -> bool {
    (1..0x8000).contains(&new.wrapping_sub(old))
}

pub struct Gossip {
    our_prefix: Prefix,
    /// The seq of our last own list. 
    seq: u16,
    /// The newest seq seen per origin, and when, to drop duplicates and stale lists.
    seen: HashMap<Prefix, (u16, Instant)>,
    /// When we last sent our own list, for the keepalive.
    last_sent: Option<Instant>,
}

impl Gossip {
    /// `seq` should move ahead across restarts (see Engine::new)
    pub fn new(our_prefix: Prefix, seq: u16) -> Self {
        Self { our_prefix, seq, seen: HashMap::new(), last_sent: None }
    }

    /// Our own neighbour list, with the next seq and full TTL.
    pub fn ours(&mut self, neighbours: Vec<Prefix>, now: Instant) -> NeighbourList {
        self.seq = self.seq.wrapping_add(1);
        self.last_sent = Some(now);
        NeighbourList { origin: self.our_prefix, seq: self.seq, ttl: NEIGHBOUR_TTL, sealed: false, neighbours }
    }

    /// Check for the keepalive: our list goes out at least this often even if unchanged.
    pub fn keepalive_due(&self, now: Instant) -> bool {
        self.last_sent.is_none_or(|sent| now.duration_since(sent) >= Duration::from_millis(NEIGHBOUR_KEEPALIVE_MS))
    }

    /// A list arrived. Returns it with one hop less if it should be relayed onward (not back to
    /// its sender, this is the callers job), or `None` for our own, a duplicate, a stale one, or its last hop.
    pub fn receive(&mut self, list: NeighbourList, now: Instant) -> Option<NeighbourList> {
        if list.ttl < 1 || list.ttl > NEIGHBOUR_TTL || list.origin == self.our_prefix {
            return None;
        }
        if self.seen.get(&list.origin).is_some_and(|&(seen, _)| !seq_fresher(list.seq, seen)) {
            return None;
        }
        self.seen.insert(list.origin, (list.seq, now));
        (list.ttl > 1).then(|| NeighbourList { ttl: list.ttl - 1, ..list })
    }

    /// Forget origins not heard from in LINK_STATE_TIMEOUT_MS.
    pub fn expire(&mut self, now: Instant) {
        let timeout = Duration::from_millis(LINK_STATE_TIMEOUT_MS);
        self.seen.retain(|_, &mut (_, seen_at)| now.duration_since(seen_at) < timeout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{decode, Frame};

    const US: Prefix = [1; QAUL_ID_ADVERT_BYTES];
    const B: Prefix = [2; QAUL_ID_ADVERT_BYTES];
    const C: Prefix = [3; QAUL_ID_ADVERT_BYTES];

    fn list(origin: Prefix, seq: u16, ttl: u8) -> NeighbourList {
        NeighbourList { origin, seq, ttl, sealed: false, neighbours: vec![C] }
    }

    #[test]
    fn encode_round_trips_through_the_parser() {
        let sent = NeighbourList { origin: B, seq: 0xfffe, ttl: 2, sealed: true, neighbours: vec![US, C] };
        let bytes = sent.encode();
        let Ok(Frame::Flc(FlcMessage::SendNeighbours(update))) = decode(&bytes) else { panic!() };
        assert_eq!(NeighbourList::from_flc(&update), Some(sent));
    }

    #[test]
    fn our_lists_count_up_with_full_ttl_and_never_sealed() {
        let mut g = Gossip::new(US, 0xffff);
        let first = g.ours(vec![B], Instant::now());
        assert_eq!((first.origin, first.seq, first.ttl, first.sealed), (US, 0, NEIGHBOUR_TTL, false));
        assert_eq!(g.ours(vec![], Instant::now()).seq, 1);
    }

    #[test]
    fn fresh_list_is_relayed_with_one_hop_less() {
        let mut g = Gossip::new(US, 0);
        assert_eq!(g.receive(list(B, 5, 3), Instant::now()), Some(list(B, 5, 2)));
    }

    #[test]
    fn duplicate_stale_own_and_last_hop_are_not_relayed() {
        let now = Instant::now();
        let mut g = Gossip::new(US, 0);
        g.receive(list(B, 5, 3), now);
        assert_eq!(g.receive(list(B, 5, 3), now), None, "duplicate");
        assert_eq!(g.receive(list(B, 4, 3), now), None, "stale");
        assert_eq!(g.receive(list(US, 9, 3), now), None, "our own, echoed back");
        assert_eq!(g.receive(list(C, 1, 1), now), None, "last hop");
        assert_eq!(g.receive(list(C, 2, 0), now), None, "ttl out of range");
    }

    #[test]
    fn seq_wraps_around() {
        let now = Instant::now();
        let mut g = Gossip::new(US, 0);
        g.receive(list(B, 0xffff, 3), now);
        assert!(g.receive(list(B, 0, 3), now).is_some());
    }

    #[test]
    fn expired_origin_is_taken_again_even_with_a_lower_seq() {
        let t0 = Instant::now();
        let mut g = Gossip::new(US, 0);
        g.receive(list(B, 500, 3), t0);
        g.expire(t0 + Duration::from_millis(LINK_STATE_TIMEOUT_MS));
        assert!(g.receive(list(B, 1, 3), t0 + Duration::from_millis(LINK_STATE_TIMEOUT_MS)).is_some());
    }

    #[test]
    fn keepalive_is_due_at_first_and_after_the_interval() {
        let t0 = Instant::now();
        let mut g = Gossip::new(US, 0);
        assert!(g.keepalive_due(t0));
        g.ours(vec![], t0);
        assert!(!g.keepalive_due(t0 + Duration::from_millis(NEIGHBOUR_KEEPALIVE_MS - 1)));
        assert!(g.keepalive_due(t0 + Duration::from_millis(NEIGHBOUR_KEEPALIVE_MS)));
    }
}
