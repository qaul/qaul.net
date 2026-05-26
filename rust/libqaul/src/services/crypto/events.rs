// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Rotation event log
//!
//! A tiny process-global ring buffer of notable Noise session
//! rotation events. Emitted at three sites in `CryptoNoise`:
//!
//! - `rotate_finalize_initiator` on success → `Rotated`
//! - the decrypt path when a draining session finishes draining
//!   (every in-flight message up to the peer's declared final nonce
//!   arrived) → `DrainCompleted`
//! - the decrypt path when an inbound message's session_id matches
//!   the most recent retirement → `MessageDroppedPostDrain`
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
use std::collections::VecDeque;
use std::sync::{OnceLock, RwLock};

use crate::utilities::timestamp::Timestamp;

/// Soft cap on the in-memory event log. Oldest events are evicted
/// when a record would exceed this size.
pub const MAX_EVENTS: usize = 256;

/// Class of rotation event — matches the proto enum one-to-one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationEventKind {
    /// A rotation successfully finalised on this node.
    Rotated,
    /// A draining session finished draining — every in-flight message
    /// up to the peer's declared final nonce arrived — and its keys
    /// were zeroised.
    DrainCompleted,
    /// An inbound message arrived whose session_id had already
    /// drained and been retired. Surface to the UI as "ask the
    /// sender to resend" rather than a generic decrypt failure.
    MessageDroppedPostDrain,
}

/// A single rotation event. Fields unused for a particular kind
/// (e.g. `primary_session_id` on `DrainCompleted`) are `0`.
#[derive(Debug, Clone)]
pub struct RotationEvent {
    pub kind: RotationEventKind,
    pub remote_id: PeerId,
    pub primary_session_id: u32,
    pub draining_session_id: u32,
    /// Wall-clock ms since epoch, filled in at emission time.
    pub timestamp_ms: u64,
}

static EVENT_LOG: OnceLock<RwLock<VecDeque<RotationEvent>>> = OnceLock::new();

fn event_log() -> &'static RwLock<VecDeque<RotationEvent>> {
    EVENT_LOG.get_or_init(|| RwLock::new(VecDeque::with_capacity(MAX_EVENTS)))
}

/// Append one event to the log. If the log has not been initialised
/// yet (first call in this process) it is created lazily. If the
/// log is at `MAX_EVENTS`, the oldest event is dropped.
pub fn record(mut event: RotationEvent) {
    // If the caller hasn't filled in a timestamp, stamp one here so
    // every event in the log carries wall-clock time.
    if event.timestamp_ms == 0 {
        event.timestamp_ms = Timestamp::get_timestamp();
    }

    let mut log = event_log().write().unwrap();
    if log.len() >= MAX_EVENTS {
        log.pop_front();
    }
    log.push_back(event);
}

/// Record the event in the in-memory log AND push it onto the
/// `crypto.rotation` subscribe topic. Production call sites should
/// use this so push-based clients (qauld-tui's Crypto tab) see
/// rotations within ms instead of waiting for the next 3-second
/// poll tick. `state` is `Option` so unit tests that exercise the
/// rotation primitives without a full `QaulState` can keep calling
/// `events::record` directly via `None` here.
pub fn record_and_emit(state: Option<&crate::QaulState>, event: RotationEvent) {
    // Stamp the timestamp before cloning so both the log and the
    // emitted proto carry the same wall-clock value.
    let stamped = if event.timestamp_ms == 0 {
        RotationEvent {
            timestamp_ms: Timestamp::get_timestamp(),
            ..event
        }
    } else {
        event
    };

    if let Some(s) = state {
        let proto_event = to_proto(&stamped);
        crate::rpc::subscribe::emit_crypto_rotation(s, &proto_event);
    }
    record(stamped);
}

fn to_proto(
    event: &RotationEvent,
) -> qaul_proto::qaul_rpc_crypto::RotationEvent {
    use qaul_proto::qaul_rpc_crypto as proto;
    let kind = match event.kind {
        RotationEventKind::Rotated => proto::RotationEventKind::Rotated,
        RotationEventKind::DrainCompleted => proto::RotationEventKind::DrainCompleted,
        RotationEventKind::MessageDroppedPostDrain => {
            proto::RotationEventKind::MessageDroppedPostDrain
        }
    };
    proto::RotationEvent {
        kind: kind as i32,
        remote_id: event.remote_id.to_bytes(),
        primary_session_id: event.primary_session_id,
        draining_session_id: event.draining_session_id,
        timestamp_ms: event.timestamp_ms,
    }
}

/// Return every event with `timestamp_ms >= since_ms`, capped at
/// `limit` (0 = unlimited). Ordered oldest → newest.
pub fn query(since_ms: u64, limit: usize) -> Vec<RotationEvent> {
    let guard = match EVENT_LOG.get() {
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
/// `OnceLock`.
#[cfg(test)]
pub fn clear_for_tests() {
    if let Some(cell) = EVENT_LOG.get() {
        cell.write().unwrap().clear();
    }
}
