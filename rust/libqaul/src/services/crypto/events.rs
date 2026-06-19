// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Rotation event log
//!
//! A small, capped log of notable Noise session rotation events.
//! Emitted at three sites in `CryptoNoise` / `Crypto`:
//!
//! - `rotate_finalize_initiator` on success → `Rotated`
//! - the decrypt path when a draining session finishes draining
//!   (every in-flight message up to the peer's declared final nonce
//!   arrived) → `DrainCompleted`
//! - the decrypt path when an inbound message's session_id matches
//!   the most recent retirement → `MessageDroppedPostDrain`
//!
//! Queried by `Modules::Crypto` clients via `GetRotationEventsRequest`.
//!
//! Persistence (review, Matthias): this knowledge must survive a reboot
//! — e.g. a `MessageDroppedPostDrain` for a session retired before the
//! restart still needs to be surfaced, and rotation history must not be
//! lost on every daemon restart. The log is therefore stored on disk in
//! the per-account `rotation_events` sled tree (the same per-account
//! database as the other crypto state), as a single capped blob keyed
//! by [`LOG_KEY`]. It is loaded straight from that tree on each query,
//! so it is durable across restarts with no separate load step.
//!
//! The blob is capped at `MAX_EVENTS` (256): when full, the oldest
//! event is dropped, bounding the on-disk size regardless of how often
//! rotations occur.

use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use super::CryptoAccount;
use crate::utilities::timestamp::Timestamp;

/// Soft cap on the event log. Oldest events are evicted when a record
/// would exceed this size, bounding the persisted blob.
pub const MAX_EVENTS: usize = 256;

/// Key of the single blob holding the whole capped log in the
/// per-account `rotation_events` tree.
const LOG_KEY: &[u8] = b"log";

/// Serialises the read-modify-write in [`record`] across the process,
/// so concurrent records on the same account's blob can't lose events.
/// Rotation events are infrequent, so a single global lock is fine.
static RECORD_LOCK: Mutex<()> = Mutex::new(());

/// Class of rotation event — matches the proto enum one-to-one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// On-disk form of a [`RotationEvent`]. `PeerId` is not directly
/// serde-serializable, so the peer is stored as its byte encoding.
#[derive(Serialize, Deserialize)]
struct StoredRotationEvent {
    kind: RotationEventKind,
    remote_id: Vec<u8>,
    primary_session_id: u32,
    draining_session_id: u32,
    timestamp_ms: u64,
}

/// Load the persisted log blob (oldest → newest), or empty.
fn load_log(account: &CryptoAccount) -> Vec<StoredRotationEvent> {
    match account.events.get(LOG_KEY) {
        Ok(Some(bytes)) => bincode::deserialize(&bytes).unwrap_or_else(|e| {
            log::error!("rotation events deserialize: {}", e);
            Vec::new()
        }),
        Ok(None) => Vec::new(),
        Err(e) => {
            log::error!("rotation events read: {}", e);
            Vec::new()
        }
    }
}

/// Persist the log blob, flushing to disk.
fn save_log(account: &CryptoAccount, log: &[StoredRotationEvent]) {
    match bincode::serialize(log) {
        Ok(bytes) => {
            if let Err(e) = account.events.insert(LOG_KEY, bytes) {
                log::error!("rotation events insert: {}", e);
            }
            if let Err(e) = account.events.flush() {
                log::error!("rotation events flush: {}", e);
            }
        }
        Err(e) => log::error!("rotation events serialize: {}", e),
    }
}

/// Append one event to the account's persisted log. If the log is at
/// `MAX_EVENTS`, the oldest events are dropped. Survives reboot.
pub fn record(account: &CryptoAccount, mut event: RotationEvent) {
    // If the caller hasn't filled in a timestamp, stamp one here so
    // every event in the log carries wall-clock time.
    if event.timestamp_ms == 0 {
        event.timestamp_ms = Timestamp::get_timestamp();
    }

    let stored = StoredRotationEvent {
        kind: event.kind,
        remote_id: event.remote_id.to_bytes(),
        primary_session_id: event.primary_session_id,
        draining_session_id: event.draining_session_id,
        timestamp_ms: event.timestamp_ms,
    };

    // Serialise the read-modify-write so concurrent records don't clobber.
    let _guard = RECORD_LOCK.lock().unwrap();
    let mut log = load_log(account);
    log.push(stored);
    if log.len() > MAX_EVENTS {
        let drop = log.len() - MAX_EVENTS;
        log.drain(0..drop);
    }
    save_log(account, &log);
}

/// Return every event with `timestamp_ms >= since_ms`, capped at
/// `limit` (0 = unlimited). Ordered oldest → newest.
pub fn query(account: &CryptoAccount, since_ms: u64, limit: usize) -> Vec<RotationEvent> {
    let mut out: Vec<RotationEvent> = load_log(account)
        .into_iter()
        .filter(|e| e.timestamp_ms >= since_ms)
        .filter_map(|e| {
            let remote_id = PeerId::from_bytes(&e.remote_id).ok()?;
            Some(RotationEvent {
                kind: e.kind,
                remote_id,
                primary_session_id: e.primary_session_id,
                draining_session_id: e.draining_session_id,
                timestamp_ms: e.timestamp_ms,
            })
        })
        .collect();
    if limit > 0 && out.len() > limit {
        // Keep the newest `limit` events (the tail of the filtered
        // slice, since the buffer is oldest-first).
        let drop = out.len() - limit;
        out.drain(0..drop);
    }
    out
}

/// Test-only helper: empty the account's log so earlier tests' events
/// don't leak into later tests sharing the same account.
#[cfg(test)]
pub fn clear_for_tests(account: &CryptoAccount) {
    if let Err(e) = account.events.remove(LOG_KEY) {
        log::error!("rotation events clear: {}", e);
    }
    let _ = account.events.flush();
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    /// Build a CryptoAccount whose trees live in `db` (the events tree
    /// is what these tests exercise).
    fn account_on(db: &sled::Db) -> CryptoAccount {
        CryptoAccount {
            state: db.open_tree("crypto_state").unwrap(),
            cache: db.open_tree("crypto_cache").unwrap(),
            rotation_meta: db.open_tree("rotation_meta").unwrap(),
            events: db.open_tree("rotation_events").unwrap(),
        }
    }

    fn peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    // The core of Matthias's requirement: the event log must survive a
    // reboot. Record into a file-backed db, drop it (close), reopen the
    // same path, and confirm the events are still there.
    #[test]
    fn events_persist_across_reopen() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("evdb");
        let p = peer();

        // first boot: record two events, then close the db
        {
            let db = sled::open(&path).unwrap();
            let acct = account_on(&db);
            record(
                &acct,
                RotationEvent {
                    kind: RotationEventKind::Rotated,
                    remote_id: p,
                    primary_session_id: 1,
                    draining_session_id: 0,
                    timestamp_ms: 100,
                },
            );
            record(
                &acct,
                RotationEvent {
                    kind: RotationEventKind::DrainCompleted,
                    remote_id: p,
                    primary_session_id: 0,
                    draining_session_id: 1,
                    timestamp_ms: 200,
                },
            );
            db.flush().unwrap();
        } // db dropped → closed

        // second boot: reopen the same path; the log must survive
        {
            let db = sled::open(&path).unwrap();
            let acct = account_on(&db);
            let log = query(&acct, 0, 0);
            assert_eq!(log.len(), 2, "events must persist across reopen");
            assert_eq!(log[0].timestamp_ms, 100);
            assert_eq!(log[0].kind, RotationEventKind::Rotated);
            assert_eq!(log[1].kind, RotationEventKind::DrainCompleted);
            assert_eq!(log[1].remote_id, p);
        }
    }

    // Cap is enforced on the persisted blob.
    #[test]
    fn persisted_log_caps_at_max() {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let acct = account_on(&db);
        let p = peer();
        for i in 0..(MAX_EVENTS + 25) {
            record(
                &acct,
                RotationEvent {
                    kind: RotationEventKind::Rotated,
                    remote_id: p,
                    primary_session_id: i as u32,
                    draining_session_id: 0,
                    timestamp_ms: 1 + i as u64,
                },
            );
        }
        let all = query(&acct, 0, 0);
        assert_eq!(all.len(), MAX_EVENTS);
        assert_eq!(all.first().unwrap().primary_session_id, 25);
    }
}
