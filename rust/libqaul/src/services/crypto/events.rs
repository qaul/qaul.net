// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Rotation event log
//!
//! A tiny process-global ring buffer of notable Noise session
//! rotation events. Emitted at three sites in `CryptoNoise`:
//!
//! - `rotate_finalize_initiator` on success → `Rotated`
//! - `drain_expired_rotations` when retiring a draining row
//!   → `GraceExpired`
//! - the decrypt path when an inbound message's session_id matches
//!   the most recent retirement → `MessageDroppedPastGrace`
//!
//! Queried by `Modules::Crypto` clients via
//! `GetRotationEventsRequest`. The buffer is capped at
//! `MAX_EVENTS` (256) so a flood of rotations cannot grow
//! memory without bound; when full, the oldest event is dropped.
//!
//! The log is intentionally not persisted to disk — rotation is
//! security-sensitive state whose history is already observable
//! on-wire to an adversary, and a disk log would add a forensic
//! artefact with no clear defender benefit. Clients that want
//! long-term records should persist the events themselves.

use libp2p::PeerId;
use state::InitCell;
use std::collections::VecDeque;
use std::sync::RwLock;

use crate::utilities::timestamp::Timestamp;

/// Soft cap on the in-memory event log. Oldest events are evicted
/// when a record would exceed this size.
pub const MAX_EVENTS: usize = 256;

/// Class of rotation event — matches the proto enum one-to-one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationEventKind {
    /// A rotation successfully finalised on this node.
    Rotated,
    /// A draining session's grace window elapsed; its keys were
    /// zeroised.
    GraceExpired,
    /// An inbound message arrived whose session_id had already been
    /// retired past the grace window. Surface to the UI as "ask the
    /// sender to resend" rather than a generic decrypt failure.
    MessageDroppedPastGrace,
}

/// A single rotation event. Fields unused for a particular kind
/// (e.g. `primary_session_id` on `GraceExpired`) are `0`.
#[derive(Debug, Clone)]
pub struct RotationEvent {
    pub kind: RotationEventKind,
    pub remote_id: PeerId,
    pub primary_session_id: u32,
    pub draining_session_id: u32,
    /// Wall-clock ms since epoch, filled in at emission time.
    pub timestamp_ms: u64,
}

static EVENT_LOG: InitCell<RwLock<VecDeque<RotationEvent>>> = InitCell::new();

/// Append one event to the log. If the log has not been initialised
/// yet (first call in this process) it is created lazily. If the
/// log is at `MAX_EVENTS`, the oldest event is dropped.
pub fn record(mut event: RotationEvent) {
    // If the caller hasn't filled in a timestamp, stamp one here so
    // every event in the log carries wall-clock time.
    if event.timestamp_ms == 0 {
        event.timestamp_ms = Timestamp::get_timestamp();
    }

    // Lazy init — `InitCell::set` returns false if already set,
    // which we silently accept.
    let _ = EVENT_LOG.set(RwLock::new(VecDeque::with_capacity(MAX_EVENTS)));

    let mut log = EVENT_LOG.get().write().unwrap();
    if log.len() >= MAX_EVENTS {
        log.pop_front();
    }
    log.push_back(event);
}

/// Return every event with `timestamp_ms >= since_ms`, capped at
/// `limit` (0 = unlimited). Ordered oldest → newest.
pub fn query(since_ms: u64, limit: usize) -> Vec<RotationEvent> {
    let guard = match EVENT_LOG.try_get() {
        Some(g) => g.read().unwrap(),
        None => return Vec::new(),
    };
    let mut out: Vec<RotationEvent> = guard
        .iter()
        .filter(|e| e.timestamp_ms >= since_ms)
        .cloned()
        .collect();
    if limit > 0 && out.len() > limit {
        // Keep the newest `limit` events (the tail of the filtered
        // slice, since the buffer is oldest-first).
        let drop = out.len() - limit;
        out.drain(0..drop);
    }
    out
}

/// Test-only helper: empty the log so earlier tests' events don't
/// leak into later tests that read the log through the shared
/// `InitCell`.
#[cfg(test)]
pub fn clear_for_tests() {
    if let Some(cell) = EVENT_LOG.try_get() {
        cell.write().unwrap().clear();
    }
}
