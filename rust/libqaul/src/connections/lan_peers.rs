// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # LAN Neighbour Memory
//!
//! **Remember where LAN neighbours can be dialled, and redial them.**
//!
//! mDNS forgets a neighbour when the neighbour's record expires, which
//! happens when the neighbour has stopped answering mDNS for the record's
//! TTL. qaul used to redial only the neighbours mDNS still listed, so a
//! neighbour whose mDNS had gone quiet was never redialled once its link
//! broke, even after the link came back.
//!
//! This behaviour keeps every address mDNS has reported for a neighbour,
//! offers those addresses to every LAN dial of that neighbour, and schedules
//! redials of disconnected neighbours: every 5 s for the first 2 minutes
//! after the last contact, then every 30 s. A neighbour is forgotten after
//! 30 minutes without contact.
//!
//! Detecting that a neighbour is lost, and telling the routing layer, is
//! unchanged: the ping and connection-closed paths still do it (routing
//! spec section 4.3).

use libp2p::{
    core::{transport::PortUse, Endpoint},
    swarm::{
        dummy, ConnectionDenied, ConnectionId, FromSwarm, NetworkBehaviour, THandler,
        THandlerInEvent, THandlerOutEvent, ToSwarm,
    },
    Multiaddr, PeerId,
};
use std::collections::HashMap;
use std::convert::Infallible;
use std::task::{Context, Poll};

use crate::utilities::timestamp::Timestamp;

/// Addresses kept per neighbour; the oldest is dropped first.
const MAX_ADDRESSES: usize = 8;
/// Redial gap during the first `FAST_PHASE_MS` after the last contact.
const FAST_REDIAL_MS: u64 = 5_000;
/// How long after the last contact redials stay fast.
const FAST_PHASE_MS: u64 = 120_000;
/// Redial gap after the fast phase.
const SLOW_REDIAL_MS: u64 = 30_000;
/// A disconnected neighbour is forgotten this long after the last contact.
const FORGET_AFTER_MS: u64 = 30 * 60_000;

struct Neighbour {
    /// Dialable addresses, oldest first.
    addresses: Vec<Multiaddr>,
    /// Open connections to the neighbour.
    connections: usize,
    /// Last discovery, connect or disconnect.
    last_contact_ms: u64,
    /// Earliest time of the next redial.
    next_dial_ms: u64,
}

impl Neighbour {
    fn new(now_ms: u64) -> Self {
        Self {
            addresses: Vec::new(),
            connections: 0,
            last_contact_ms: now_ms,
            next_dial_ms: now_ms,
        }
    }
}

/// The outcome of one redial pass.
#[derive(Debug, Default, PartialEq)]
pub struct RedialPass {
    /// Disconnected neighbours to dial now.
    pub dial: Vec<PeerId>,
    /// Neighbours forgotten in this pass.
    pub forgotten: Vec<PeerId>,
}

/// The LAN neighbours this node has discovered or connected to.
#[derive(Default)]
pub struct LanPeers {
    neighbours: HashMap<PeerId, Neighbour>,
}

impl LanPeers {
    pub fn new() -> Self {
        Self::default()
    }

    /// mDNS reported `address` for `peer`.
    pub fn discovered(&mut self, peer: PeerId, address: Multiaddr, now_ms: u64) {
        let neighbour = self
            .neighbours
            .entry(peer)
            .or_insert_with(|| Neighbour::new(now_ms));
        neighbour.last_contact_ms = now_ms;
        neighbour.next_dial_ms = neighbour.next_dial_ms.min(now_ms);
        if !neighbour.addresses.contains(&address) {
            if neighbour.addresses.len() == MAX_ADDRESSES {
                neighbour.addresses.remove(0);
            }
            neighbour.addresses.push(address);
        }
    }

    /// A connection to `peer` opened; `other_established` were already open.
    pub fn connected(&mut self, peer: PeerId, other_established: usize, now_ms: u64) {
        let neighbour = self
            .neighbours
            .entry(peer)
            .or_insert_with(|| Neighbour::new(now_ms));
        neighbour.connections = other_established + 1;
        neighbour.last_contact_ms = now_ms;
    }

    /// A connection to `peer` closed; `remaining` are still open.
    pub fn disconnected(&mut self, peer: PeerId, remaining: usize, now_ms: u64) {
        if let Some(neighbour) = self.neighbours.get_mut(&peer) {
            neighbour.connections = remaining;
            if remaining == 0 {
                neighbour.last_contact_ms = now_ms;
                neighbour.next_dial_ms = now_ms;
            }
        }
    }

    /// The remembered addresses of `peer`.
    pub fn addresses(&self, peer: &PeerId) -> Vec<Multiaddr> {
        self.neighbours
            .get(peer)
            .map(|n| n.addresses.clone())
            .unwrap_or_default()
    }

    /// Pick the disconnected neighbours due for a redial and forget the
    /// ones not heard from for `FORGET_AFTER_MS`.
    pub fn redial_pass(&mut self, now_ms: u64) -> RedialPass {
        let mut pass = RedialPass::default();
        self.neighbours.retain(|peer, n| {
            if n.connections > 0 {
                return true;
            }
            let silent_ms = now_ms.saturating_sub(n.last_contact_ms);
            if silent_ms > FORGET_AFTER_MS {
                pass.forgotten.push(*peer);
                return false;
            }
            if n.addresses.is_empty() || now_ms < n.next_dial_ms {
                return true;
            }
            let gap = if silent_ms < FAST_PHASE_MS {
                FAST_REDIAL_MS
            } else {
                SLOW_REDIAL_MS
            };
            n.next_dial_ms = now_ms.saturating_add(gap);
            pass.dial.push(*peer);
            true
        });
        pass
    }
}

impl NetworkBehaviour for LanPeers {
    type ConnectionHandler = dummy::ConnectionHandler;
    type ToSwarm = Infallible;

    fn handle_established_inbound_connection(
        &mut self,
        _: ConnectionId,
        _: PeerId,
        _: &Multiaddr,
        _: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(dummy::ConnectionHandler)
    }

    fn handle_pending_outbound_connection(
        &mut self,
        _: ConnectionId,
        maybe_peer: Option<PeerId>,
        _: &[Multiaddr],
        _: Endpoint,
    ) -> Result<Vec<Multiaddr>, ConnectionDenied> {
        Ok(maybe_peer
            .map(|peer| self.addresses(&peer))
            .unwrap_or_default())
    }

    fn handle_established_outbound_connection(
        &mut self,
        _: ConnectionId,
        _: PeerId,
        _: &Multiaddr,
        _: Endpoint,
        _: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(dummy::ConnectionHandler)
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        match event {
            FromSwarm::ConnectionEstablished(e) => {
                self.connected(e.peer_id, e.other_established, Timestamp::get_timestamp());
            }
            FromSwarm::ConnectionClosed(e) => {
                self.disconnected(
                    e.peer_id,
                    e.remaining_established,
                    Timestamp::get_timestamp(),
                );
            }
            _ => {}
        }
    }

    fn on_connection_handler_event(
        &mut self,
        _: PeerId,
        _: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        match event {}
    }

    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Infallible, THandlerInEvent<Self>>> {
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(port: u16) -> Multiaddr {
        format!("/ip4/10.0.0.1/tcp/{port}").parse().unwrap()
    }

    #[test]
    fn a_discovered_neighbour_is_dialled_until_it_connects() {
        let mut peers = LanPeers::new();
        let p = PeerId::random();
        peers.discovered(p, addr(1), 0);

        assert_eq!(peers.redial_pass(0).dial, vec![p]);
        // not again within the fast gap
        assert!(peers.redial_pass(FAST_REDIAL_MS - 1).dial.is_empty());
        assert_eq!(peers.redial_pass(FAST_REDIAL_MS).dial, vec![p]);

        peers.connected(p, 0, FAST_REDIAL_MS + 1);
        assert!(peers.redial_pass(10 * FAST_REDIAL_MS).dial.is_empty());
    }

    #[test]
    fn a_lost_neighbour_is_redialled_at_its_remembered_address() {
        let mut peers = LanPeers::new();
        let p = PeerId::random();
        peers.discovered(p, addr(1), 0);
        peers.connected(p, 0, 1_000);
        peers.disconnected(p, 0, 400_000);

        // mDNS forgetting the peer does not touch the memory
        assert_eq!(peers.addresses(&p), vec![addr(1)]);
        assert_eq!(peers.redial_pass(400_000).dial, vec![p]);
    }

    #[test]
    fn redials_slow_down_after_the_fast_phase() {
        let mut peers = LanPeers::new();
        let p = PeerId::random();
        peers.discovered(p, addr(1), 0);
        peers.connected(p, 0, 0);
        peers.disconnected(p, 0, 0);

        let mut dials = Vec::new();
        for t in (0..=FAST_PHASE_MS + 2 * SLOW_REDIAL_MS).step_by(1_000) {
            if !peers.redial_pass(t).dial.is_empty() {
                dials.push(t);
            }
        }
        let fast = dials.iter().filter(|t| **t < FAST_PHASE_MS).count() as u64;
        assert_eq!(fast, FAST_PHASE_MS / FAST_REDIAL_MS);
        let slow: Vec<_> = dials.iter().filter(|t| **t >= FAST_PHASE_MS).collect();
        assert_eq!(
            slow,
            vec![
                &FAST_PHASE_MS,
                &(FAST_PHASE_MS + SLOW_REDIAL_MS),
                &(FAST_PHASE_MS + 2 * SLOW_REDIAL_MS)
            ]
        );
    }

    #[test]
    fn a_connection_keeps_the_neighbour_until_the_last_one_closes() {
        let mut peers = LanPeers::new();
        let p = PeerId::random();
        peers.discovered(p, addr(1), 0);
        peers.connected(p, 0, 0);
        peers.connected(p, 1, 0);
        peers.disconnected(p, 1, 1_000);
        assert!(peers.redial_pass(2_000).dial.is_empty());
        peers.disconnected(p, 0, 3_000);
        assert_eq!(peers.redial_pass(3_000).dial, vec![p]);
    }

    #[test]
    fn a_silent_neighbour_is_forgotten_and_a_connected_one_is_not() {
        let mut peers = LanPeers::new();
        let gone = PeerId::random();
        let here = PeerId::random();
        peers.discovered(gone, addr(1), 0);
        peers.discovered(here, addr(2), 0);
        peers.connected(here, 0, 0);

        let pass = peers.redial_pass(FORGET_AFTER_MS + 1);
        assert_eq!(pass.forgotten, vec![gone]);
        assert!(peers.addresses(&gone).is_empty());
        assert_eq!(peers.addresses(&here), vec![addr(2)]);
    }

    #[test]
    fn a_neighbour_without_addresses_is_not_dialled() {
        let mut peers = LanPeers::new();
        let p = PeerId::random();
        // an inbound connection teaches no dialable address
        peers.connected(p, 0, 0);
        peers.disconnected(p, 0, 1_000);
        assert!(peers.redial_pass(1_000).dial.is_empty());
    }

    #[test]
    fn addresses_are_capped_oldest_first() {
        let mut peers = LanPeers::new();
        let p = PeerId::random();
        for port in 0..(MAX_ADDRESSES as u16 + 2) {
            peers.discovered(p, addr(port), 0);
        }
        peers.discovered(p, addr(5), 0); // already known
        let kept = peers.addresses(&p);
        assert_eq!(kept.len(), MAX_ADDRESSES);
        assert_eq!(kept.first(), Some(&addr(2)));
        assert_eq!(kept.last(), Some(&addr(MAX_ADDRESSES as u16 + 1)));
    }
}
