// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Everything received over one link: a message slot per queue index, and the
//! decision of when a chunk starts a new message. Mirrors `ReceiveQueue` in
//! `ReceiveQueue.kt`.

use std::collections::BTreeMap;

use crate::frame::{ChunkHeader, LargePart};
use crate::queue::recv::{ChunkOutcome, ReceiveError, ReceiveQueueMessage};

/// What the caller should do after a chunk has been handled.
#[derive(Debug, PartialEq, Eq)]
pub enum ReceiveEvent {
    /// Nothing to do: a duplicate, or a chunk that revealed no gaps
    Nothing,
    /// Ask the peer to resend these chunks of the message on `queue_index`
    NeedChunks { queue_index: u8, chunks: Vec<u16> },
    /// A whole message arrived and checked out. Send an ACK_SUCCESS for the
    /// `queue_index` and hand the message to libqaul
    Received { queue_index: u8, message: Vec<u8>, qaul_id: [u8; 8] },
    /// One part of a large message arrived and checked out, but other parts are
    /// still missing. So send an ACK_SUCCESS for the index, but deliver nothing yet.
    PartReceived { queue_index: u8 },
    /// The message on `queue_index` failed, send an ACK_ERROR
    Failed { queue_index: u8, error: ReceiveError },
}

/// The parts of one large message received so far.
struct ReceiveLargeMessage {
    /// How many parts the message has, 1..=4.
    total: u8,
    /// Each part's bytes, by part number. Only the entry's up to 'total' are used.
    parts: [Option<Vec<u8>>; 4],
}

impl ReceiveLargeMessage {
    fn new(total: u8) -> Self {
        Self { total, parts: Default::default() }
    }

    /// Store one part. Returns the whole message once every part is in.
    fn add(&mut self, part: u8, bytes: Vec<u8>) -> Option<Vec<u8>> {
        self.parts[usize::from(part)] = Some(bytes);
        let parts = &self.parts[..usize::from(self.total)];
        if parts.iter().any(Option::is_none) {
            return None;
        }
        Some(parts.iter().flatten().flatten().copied().collect())
    }
}

pub struct ReceiveQueue {
    /// The message currently being received on each queue index (1..=29).
    /// An index with no entry has not been used on this link yet.
    receive_queues: BTreeMap<u8, ReceiveQueueMessage>,
    /// Large messages with some parts received, keyed by their large index.
    large_message_queue: BTreeMap<u8, ReceiveLargeMessage>,
}

impl ReceiveQueue {
    pub fn new() -> Self {
        Self { receive_queues: BTreeMap::new(), large_message_queue: BTreeMap::new() }
    }

    /// Handle one data chunk (queue index 1..=29) from this link.
    pub fn handle_chunk(&mut self, header: ChunkHeader, payload: &[u8]) -> ReceiveEvent {
        // 1. Does this chunk start a new message on its queue index? It does if
        //    there is no message there yet, or if it is not a resend and any of:
        //      - it is chunk 0
        //      - its index is at or below the highest index already seen
        //      - its index is at or beyond the message's total chunks
        //    If so, put a fresh ReceiveQueueMessage in the slot.
      
        let queue_index = header.queue_index;

        let starts_new = match self.receive_queues.get(&queue_index) {
            None => true,
            Some(existing) => match (header.resend, header.chunk_index) {
                (true, _) => false,  // a resend never starts a new message
                (false, 0) => true,
                (false, chunk_index) => {
                    existing.highest_index().is_some_and(|highest| chunk_index <= highest)
                    || existing.total_chunks().is_some_and(|total| chunk_index >= total)
                }
            },
        };
        if starts_new { 
            self.receive_queues.insert(queue_index, ReceiveQueueMessage::new(queue_index));
        }
        // 2. Add the chunk to the message in the slot.
        let message = self.receive_queues.get_mut(&queue_index).expect("the slot was filled just above");
        let chunk_outcome = message.add_received_chunk(header, payload);

        //3. Turn the ChunkOutcome into a ReceiveEvent.
        match chunk_outcome {
            ChunkOutcome::Duplicate => ReceiveEvent::Nothing,
            ChunkOutcome::Incomplete { newly_missing } if newly_missing.is_empty() => ReceiveEvent::Nothing,
            ChunkOutcome::Incomplete { newly_missing } => ReceiveEvent::NeedChunks { queue_index, chunks: newly_missing },
            ChunkOutcome::Complete { message, qaul_id, part: None } => {
                ReceiveEvent::Received { queue_index, message, qaul_id }
            }
            ChunkOutcome::Complete { message, qaul_id, part: Some(part) } => {
                self.add_large_message_part(queue_index, part, message, qaul_id)
            }
            ChunkOutcome::Failed (error) => {
                ReceiveEvent::Failed { queue_index, error }
            }
        }
    }

    /// Store a part of a large message.
    // TODO: a large message that never gets all its parts stays here until its index is reused.
    fn add_large_message_part(&mut self, queue_index: u8, part: LargePart, bytes: Vec<u8>, qaul_id: [u8; 8]) -> ReceiveEvent {
        // Check part is within range, if not, best to still ACK to avoid resends.
        if part.part >= part.total {
            return ReceiveEvent::PartReceived { queue_index };
        }

        // A different total means the index was reused by a new message, start over.
        // TODO: This is a simplistic check that could still lead to messages getting mixed and corrupted
        let large = self.large_message_queue.entry(part.index).or_insert_with(|| ReceiveLargeMessage::new(part.total));
        if large.total != part.total {
            *large = ReceiveLargeMessage::new(part.total);
        }

        match large.add(part.part, bytes) {
            Some(message) => {
                self.large_message_queue.remove(&part.index);
                ReceiveEvent::Received { queue_index, message, qaul_id }
            }
            None => ReceiveEvent::PartReceived { queue_index },
        }
    }
}

impl Default for ReceiveQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{decode, Frame};
    use crate::queue::send::SendQueueMessage;

    const ID: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    /// A message of `len` bytes counting 0, 1, 2… so any misplaced byte shows.
    fn body(len: usize) -> Vec<u8> {
        (0..len).map(|i| i as u8).collect()
    }

    /// A message to send on `queue_index`. At chunk size 25 the first chunk
    /// carries 6 bytes and later ones 23, so 100 bytes is 6 chunks, 50 is 3.
    fn outgoing(queue_index: u8, len: usize) -> SendQueueMessage {
        SendQueueMessage::new(ID, "msg".into(), queue_index, 0, body(len), 25).unwrap()
    }

    /// Chunk `index` as it would come off the wire, fed to the receiver.
    fn feed(queue: &mut ReceiveQueue, msg: &SendQueueMessage, index: u16, resend: bool) -> ReceiveEvent {
        let chunk = msg.get_chunk(index, resend).unwrap();
        match decode(&chunk).unwrap() {
            Frame::Chunk { header, payload } => queue.handle_chunk(header, payload),
            other => panic!("expected a chunk, got {other:?}"),
        }
    }

    fn received(queue_index: u8, len: usize) -> ReceiveEvent {
        ReceiveEvent::Received { queue_index, message: body(len), qaul_id: ID }
    }

    #[test]
    fn whole_message_is_received() {
        let msg = outgoing(5, 100);
        let mut queue = ReceiveQueue::new();
        for i in 0..5 {
            assert_eq!(feed(&mut queue, &msg, i, false), ReceiveEvent::Nothing);
        }
        assert_eq!(feed(&mut queue, &msg, 5, false), received(5, 100));
    }

    #[test]
    fn a_gap_asks_for_the_missing_chunk() {
        let msg = outgoing(5, 100);
        let mut queue = ReceiveQueue::new();
        feed(&mut queue, &msg, 0, false);
        feed(&mut queue, &msg, 1, false);
        assert_eq!(
            feed(&mut queue, &msg, 3, false),
            ReceiveEvent::NeedChunks { queue_index: 5, chunks: vec![2] }
        );
    }

    #[test]
    fn reused_index_starts_a_new_message_at_chunk_0() {
        let a = outgoing(5, 100);
        let b = outgoing(5, 50);
        let mut queue = ReceiveQueue::new();
        for i in 0..6 {
            feed(&mut queue, &a, i, false);
        }
        // B's chunk 0, not a resend: A only ever sends its chunk 0 once.
        for i in 0..2 {
            assert_eq!(feed(&mut queue, &b, i, false), ReceiveEvent::Nothing);
        }
        assert_eq!(feed(&mut queue, &b, 2, false), received(5, 50));
    }

    #[test]
    fn reused_index_with_chunk_0_lost_starts_a_new_message() {
        let a = outgoing(5, 100);
        let b = outgoing(5, 50);
        let mut queue = ReceiveQueue::new();
        for i in 0..4 {
            feed(&mut queue, &a, i, false); // A abandoned at highest 3
        }
        // B's chunk 1 is not a resend and is below A's highest: new message.
        assert_eq!(
            feed(&mut queue, &b, 1, false),
            ReceiveEvent::NeedChunks { queue_index: 5, chunks: vec![0] }
        );
        assert_eq!(feed(&mut queue, &b, 2, false), ReceiveEvent::Nothing);
        // The requested chunk 0 comes back marked as a resend.
        assert_eq!(feed(&mut queue, &b, 0, true), received(5, 50));
    }

    #[test]
    fn late_duplicate_resend_after_completion_is_ignored() {
        let msg = outgoing(5, 100);
        let mut queue = ReceiveQueue::new();
        for i in [0, 1, 3, 4, 5] {
            feed(&mut queue, &msg, i, false); // 2 lost
        }
        // Chunk 2 was requested twice; both resends arrive.
        assert_eq!(feed(&mut queue, &msg, 2, true), received(5, 100));
        assert_eq!(feed(&mut queue, &msg, 2, true), ReceiveEvent::Nothing);
    }

    #[test]
    fn interleaved_messages_on_two_indices_both_arrive() {
        let a = outgoing(3, 100);
        let b = outgoing(4, 50);
        let mut queue = ReceiveQueue::new();
        for i in 0..2 {
            assert_eq!(feed(&mut queue, &a, i, false), ReceiveEvent::Nothing);
            assert_eq!(feed(&mut queue, &b, i, false), ReceiveEvent::Nothing);
        }
        assert_eq!(feed(&mut queue, &b, 2, false), received(4, 50));
        for i in 2..5 {
            assert_eq!(feed(&mut queue, &a, i, false), ReceiveEvent::Nothing);
        }
        assert_eq!(feed(&mut queue, &a, 5, false), received(3, 100));
    }

    /// One part of a large message, as its own message on `queue_index`.
    fn part(queue_index: u8, indicator: u8, bytes: &[u8]) -> SendQueueMessage {
        SendQueueMessage::new(ID, "msg".into(), queue_index, indicator, bytes.to_vec(), 25).unwrap()
    }

    /// Feed every chunk of `msg` in order, returning what the last one gave.
    fn feed_all(queue: &mut ReceiveQueue, msg: &SendQueueMessage) -> ReceiveEvent {
        let mut last = ReceiveEvent::Nothing;
        for i in 0..msg.total_chunks() {
            last = feed(queue, msg, i, false);
        }
        last
    }

    #[test]
    fn large_message_parts_out_of_order_are_joined() {
        let whole = body(150);
        let parts: Vec<&[u8]> = whole.chunks(50).collect();
        let mut queue = ReceiveQueue::new();
        // Large index 1, 3 parts: (1 << 4) | (3 << 2) | part
        assert_eq!(feed_all(&mut queue, &part(3, 0x1E, parts[2])), ReceiveEvent::PartReceived { queue_index: 3 });
        assert_eq!(feed_all(&mut queue, &part(4, 0x1C, parts[0])), ReceiveEvent::PartReceived { queue_index: 4 });
        assert_eq!(
            feed_all(&mut queue, &part(5, 0x1D, parts[1])),
            ReceiveEvent::Received { queue_index: 5, message: whole, qaul_id: ID }
        );
        assert!(queue.large_message_queue.is_empty());
    }

    #[test]
    fn kotlin_four_part_message_is_joined() {
        let whole = body(120);
        let mut queue = ReceiveQueue::new();
        // Kotlin's large index 2 with 4 parts: (2 shl 4) or (4 shl 2) or part = 0x30 | part
        for (i, bytes) in whole.chunks(30).enumerate().take(3) {
            let event = feed_all(&mut queue, &part(i as u8 + 1, 0x30 | i as u8, bytes));
            assert_eq!(event, ReceiveEvent::PartReceived { queue_index: i as u8 + 1 });
        }
        assert_eq!(
            feed_all(&mut queue, &part(4, 0x33, &whole[90..])),
            ReceiveEvent::Received { queue_index: 4, message: whole, qaul_id: ID }
        );
    }

    #[test]
    fn reused_large_index_with_a_new_total_starts_over() {
        let mut queue = ReceiveQueue::new();
        // An old message on large index 1 with 2 parts, part 0 only.
        feed_all(&mut queue, &part(3, 0x18, &body(40)));
        // A new message on large index 1 with 3 parts.
        let whole = body(90);
        let parts: Vec<&[u8]> = whole.chunks(30).collect();
        feed_all(&mut queue, &part(4, 0x1C, parts[0]));
        feed_all(&mut queue, &part(5, 0x1D, parts[1]));
        assert_eq!(
            feed_all(&mut queue, &part(6, 0x1E, parts[2])),
            ReceiveEvent::Received { queue_index: 6, message: whole, qaul_id: ID }
        );
    }

    #[test]
    fn part_number_past_its_total_is_acked_and_dropped() {
        let mut queue = ReceiveQueue::new();
        // Large index 1, 2 parts, part 3.
        assert_eq!(feed_all(&mut queue, &part(3, 0x1B, &body(40))), ReceiveEvent::PartReceived { queue_index: 3 });
        assert!(queue.large_message_queue.is_empty());
    }
}


