// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Everything received over one link: a message slot per queue index, and the
//! decision of when a chunk starts a new message. Mirrors `ReceiveQueue` in
//! `ReceiveQueue.kt`.

use std::collections::BTreeMap;

use crate::frame::ChunkHeader;
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
    /// The message on `queue_index` failed, send an ACK_ERROR
    Failed { queue_index: u8, error: ReceiveError },
}

pub struct ReceiveQueue {
    /// The message currently being received on each queue index (1..=29).
    /// An index with no entry has not been used on this link yet.
    receive_queues: BTreeMap<u8, ReceiveQueueMessage>,
}

impl ReceiveQueue {
    pub fn new() -> Self {
        Self { receive_queues: BTreeMap::new() }
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
            ChunkOutcome::Complete { message, qaul_id } => {
                ReceiveEvent::Received { queue_index, message, qaul_id }
            }
            ChunkOutcome::Failed (error) => {
                ReceiveEvent::Failed { queue_index, error }
            }
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
}


