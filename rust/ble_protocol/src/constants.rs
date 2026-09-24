// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Protocol constants
//!
//! Only values that belong to the *wire protocol* live here. Platform tunables
//! (scan windows, connection priorities, radio watchdogs etc)
//! live on the platform side, in `ble_module`.
//!
//! Every value carries one of three markers to indicate how it was derived 
//! in comparison to the Android module:
//!
//! - **[universal]**  forced by the bluetooth spec or by physics, true anywhere.
//! - **[android]**    chosen to work around Android; redo for Linux.
//! - **[unvalidated]** inherited guess nobody has measured yet. 

/// Full qaul node ID length in bytes ("q8id"), as carried by `SEND_QAUL_ID`.
///
/// **[universal]** Cross-platform contract.
pub const QAUL_ID_BYTES: usize = 8;

/// Leading qaul ID bytes used as a peer hint, in advertisements and in
/// `SEND_NEIGHBOURS` entries.
///
/// **[universal]** Cross-platform contract. 5 bytes fits the 31 byte legacy
/// advertisement budget and is ample for disambiguating local peers; the full ID
/// is always verified after connecting.
pub const QAUL_ID_ADVERT_BYTES: usize = 5;

/// Number of concurrent send queues, indexed 1..=29. Index 0 is reserved for
/// flow control frames.
///
/// **[universal]** 
pub const SEND_QUEUE_COUNT: u8 = 29;

/// Hard cap on bytes per chunk, independent of the negotiated MTU.
///
/// **[android]** 509 is what Android uses. Re-derive for Linux once the negotiated MTU is known.**
pub const MAX_CHUNK_SIZE: usize = 509;

/// Chunk size before MTU negotiation: the 23 byte default MTU minus 3 bytes of
/// ATT header.
///
/// **[universal]** The 23 byte default MTU is in the Bluetooth core spec.
pub const DEFAULT_CHUNK_SIZE: usize = 20;

/// ATT MTU requested after connecting.
///
/// **[android]** 
pub const TARGET_MTU: usize = 517;

/// Largest message part
///
/// **[universal]** Due to the wire format
pub const MAX_PART_SIZE: usize = u16::MAX as usize;

/// Messages at or below this size ride the MEDIUM lane (routing updates, chat),
/// keeping them ahead of large transfers on BULK.
///
/// **[unvalidated]** Unvalidated on android as well
pub const MEDIUM_MESSAGE_MAX_BYTES: usize = 16_000;

/// Largest run of consecutive missing chunks tolerated in one gap before the
/// receiver gives up and errors the message.
///
/// **[unvalidated]** From `ReceiveQueue.kt`, no recorded justification.
pub const MAX_MISSING_GAP: usize = 12;

/// How often an unresolved connection re sends its `SEND_QAUL_ID`.
///
/// **[universal]** Handles a SEND_ID simply lost over the air, which is the
/// dominant failure at range. Short enough for several attempts inside
/// [`UNRESOLVED_TIMEOUT_MS`].
pub const IDENTITY_RETRY_MS: u64 = 800;

/// How long a connection may stay unresolved (qaul ID never learned) before it
/// is dropped as a stuck handshake, measured from transport ready.
///
/// **[unvalidated]** Plausible but hasnt been validated.
pub const UNRESOLVED_TIMEOUT_MS: u64 = 3_000;

/// Interval between `LIVENESS_CHECK_PING` frames on an idle link.
///
/// **[unvalidated]**
pub const PING_INTERVAL_MS: u64 = 5_000;

/// No data for this long and the link is considered dead.
///
/// **[unvalidated]** 
pub const LIVENESS_TIMEOUT_MS: u64 = 30_000;

/// As [`LIVENESS_TIMEOUT_MS`], for a Coded PHY link.
///
/// **[unvalidated]** This is an artifact that likely never triggers, needs reviewed
pub const CODED_LIVENESS_TIMEOUT_MS: u64 = 90_000;

/// Time to live for link state gossip: the maximum number of relay hops for a
/// `SEND_NEIGHBOURS` message.
///
/// **[universal]** Simulation result. TTL 3 gave the best diameter, while TTL 2 bisected and
/// TTL 4+ over thinned. Cross platform contract so both ends must agree.
pub const NEIGHBOUR_TTL: u8 = 3;

/// Re-broadcast our own neighbour list this often, even when unchanged.
///
/// **[universal]** May need tested in further topology sims but is a cross platform contract
pub const NEIGHBOUR_KEEPALIVE_MS: u64 = 15_000;

/// Discard a gossiped link state entry if nothing fresher arrives within this
/// window.
///
/// **[universal]** Must stay comfortably above [`NEIGHBOUR_KEEPALIVE_MS`] so a
/// single lost keepalive does not expire a live neighbour.
pub const LINK_STATE_TIMEOUT_MS: u64 = 45_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_sizes_fit_the_negotiated_mtu() {
        // 3 bytes of ATT header sit in front of every chunk.
        assert!(MAX_CHUNK_SIZE + 3 <= TARGET_MTU);
        assert!(DEFAULT_CHUNK_SIZE + 3 <= 23, "default MTU is 23 bytes");
    }

    #[test]
    fn keepalive_leaves_room_for_a_lost_message() {
        // At least two keepalives must fit inside the timeout, or one dropped
        // gossip message could expire a neighbour that is still there.
        assert!(NEIGHBOUR_KEEPALIVE_MS * 2 < LINK_STATE_TIMEOUT_MS);
    }

    #[test]
    fn identity_retries_several_times_before_the_reaper() {
        assert!(IDENTITY_RETRY_MS * 3 <= UNRESOLVED_TIMEOUT_MS);
    }

}
