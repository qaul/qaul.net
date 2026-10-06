// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Manages everything being sent over one connection: messages waiting for a queue index,
//! messages in flight awaiting their ACK, and resends on request. Mirrors
//! `SendQueue` in `SendQueue.kt`.

use std::collections::{BTreeMap, VecDeque};

use crate::constants::QAUL_ID_BYTES;
use crate::queue::index::QueueIndexAllocator;
use crate::queue::send::SendQueueMessage;

/// What the caller should do after the send queue has acted.
#[derive(Debug, PartialEq, Eq)]
pub enum SendEvent {
    /// Write these chunks to the peer, in order.
    Chunks(Vec<Vec<u8>>),
    /// The peer acknowledged this message. Tell libqaul it was delivered.
    Delivered { message_id: String },
    /// This message will not arrive. Tell libqaul it failed.
    Failed { message_id: String },
}

/// A message handed over by libqaul that has no queue index yet.
struct Waiting {
    message: Vec<u8>,
    message_id: String,
}

pub struct SendQueue {
    /// Our own qaul id, written into every first chunk header.
    qaul_id: [u8; QAUL_ID_BYTES],
    /// Bytes per chunk, header included. Set from the negotiated MTU.
    chunk_size: usize,
    /// Hands out queue indices 1..=29.
    allocator: QueueIndexAllocator,
    /// Messages sent and awaiting their ACK, keyed by queue index. Its keys are
    /// always exactly the indices the allocator has in use.
    send_queues: BTreeMap<u8, SendQueueMessage>,
    /// Messages not started yet, oldest first.
    waiting: VecDeque<Waiting>,
}

impl SendQueue {
    pub fn new(qaul_id: [u8; QAUL_ID_BYTES], chunk_size: usize) -> Self {
        Self {
            qaul_id,
            chunk_size,
            allocator: QueueIndexAllocator::new(),
            send_queues: BTreeMap::new(),
            waiting: VecDeque::new(),
        }
    }

    /// Change the chunk size after an MTU negotiation.
    pub fn set_chunk_size(&mut self, chunk_size: usize) {
        self.chunk_size = chunk_size;
    }

    /// Queue a message from libqaul. Later it will be sent by `start_next_message`.
    pub fn add_message(&mut self, message: Vec<u8>, message_id: String) {
        self.waiting.push_back(Waiting { message, message_id });
        // large message part handling needed here
    }

    /// Start the oldest waiting message: give it a queue index and return its
    /// chunks. Returns nothing if no message is waiting.
    pub fn start_next_message(&mut self) -> Vec<SendEvent> {
        // 1. Take the oldest waiting message, if there is one.
        let Some(waiting) = self.waiting.pop_front() else {
            return Vec::new();
        };

        // 2. Allocate a queue index.
        let allocated = self.allocator.allocate();
        let queue_index = allocated.index;
        let evicted = allocated.evicted;

        // 3. Build the SendQueueMessage.
        let send_message = SendQueueMessage::new(
            self.qaul_id,
            waiting.message_id.clone(),
            queue_index,
            0, // large message indicator
            waiting.message,
            self.chunk_size,
        );

        match send_message {
            Ok(message) => {
                // 4. Put it in flight and return its chunks.
                let mut events = Vec::new();
                let chunks = message.get_all_chunks();
                if let Some(old) = self.send_queues.insert(queue_index, message) {
                    events.push(SendEvent::Failed { message_id: old.message_id().to_string() });
                }
                events.push(SendEvent::Chunks(chunks));
                events
            }
            Err(_) => {
                // the new message has failed. Free the index back, unless it was evicted, then leave it with
                // the old message, which carries on.
                if !evicted {
                    self.allocator.release(queue_index);
                }
                vec![SendEvent::Failed { message_id: waiting.message_id }]
            }
        }
    }

    /// An ACK arrived for `queue_index`. Returns a `SendEvent` to report to libqaul, or `None` if the
    /// index was not in use.
    pub fn on_ack(&mut self, queue_index: u8, success: bool) -> Option<SendEvent> {
        if let Some(message) = self.send_queues.remove(&queue_index){
            let message_id = message.message_id().to_string();
            self.allocator.release(queue_index);
            if success {
                Some(SendEvent::Delivered { message_id})
            } else {
                Some(SendEvent::Failed { message_id})
            }
        }
        else { None }
    }

    /// The peer sent MISSING_CHUNKS. Each entry is `queue_index << 11 | chunk_index`
    /// Returns the requested chunks with the resend bit set.
    pub fn resend_chunks(&self, requested: &[u16]) -> Vec<Vec<u8>> {
        requested
            .iter()
            .filter_map(|&entry| {
                // Same layout as a chunk header with the resend bit clear:
                // queue index in the top 5 bits, chunk index in the low 10.
                let queue_index = (entry >> 11) as u8;
                let chunk_index = entry & 0x3FF;
                let message = self.send_queues.get(&queue_index)?;
                message.get_chunk(chunk_index, true)
            })
            .collect()
    }

    /// The link is gone. Everything in flight or waiting has failed. Return the message ids to report to libqaul.
    pub fn fail_all(self) -> Vec<String> {
        let send_queues = self.send_queues.into_values().map(|m| m.message_id().to_string());
        let waiting = self.waiting.into_iter().map(|w| w.message_id);
        send_queues.chain(waiting).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SEND_QUEUE_COUNT;
    use crate::frame::{decode, ChunkHeader, Frame};
    use crate::queue::receive_queue::{ReceiveEvent, ReceiveQueue};

    const ID: [u8; QAUL_ID_BYTES] = [1, 2, 3, 4, 5, 6, 7, 8];

    /// A message of `len` bytes counting 0, 1, 2… so any misplaced byte shows.
    fn body(len: usize) -> Vec<u8> {
        (0..len).map(|i| i as u8).collect()
    }

    /// At chunk size 25 a 100 byte message is 6 chunks.
    fn queue() -> SendQueue {
        SendQueue::new(ID, 25)
    }

    fn header(chunk: &[u8]) -> ChunkHeader {
        match decode(chunk).unwrap() {
            Frame::Chunk { header, .. } => header,
            other => panic!("expected a chunk, got {other:?}"),
        }
    }

    fn failed(id: &str) -> SendEvent {
        SendEvent::Failed { message_id: id.into() }
    }

    #[test]
    fn ack_reports_the_result_and_frees_the_index() {
        let mut q = queue();
        q.add_message(body(10), "ok".into());
        q.add_message(body(10), "bad".into());
        q.start_next_message(); // index 1
        q.start_next_message(); // index 2
        assert_eq!(q.on_ack(1, true), Some(SendEvent::Delivered { message_id: "ok".into() }));
        assert_eq!(q.on_ack(2, false), Some(failed("bad")));
        assert_eq!(q.allocator.free_count(), usize::from(SEND_QUEUE_COUNT));
    }

    #[test]
    fn stale_or_repeated_ack_is_ignored() {
        let mut q = queue();
        assert_eq!(q.on_ack(5, true), None); // nothing ever sent on 5
        q.add_message(body(10), "m".into());
        q.start_next_message();
        assert!(q.on_ack(1, true).is_some());
        assert_eq!(q.on_ack(1, true), None); // duplicate
    }

    #[test]
    fn thirtieth_message_evicts_the_oldest_in_flight() {
        let mut q = queue();
        for i in 0..SEND_QUEUE_COUNT {
            q.add_message(body(10), format!("m{i}"));
            q.start_next_message();
        }
        q.add_message(body(10), "new".into());
        let events = q.start_next_message();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], failed("m0"));
        let SendEvent::Chunks(chunks) = &events[1] else {
            panic!("expected Chunks, got {:?}", events[1]);
        };
        assert_eq!(header(&chunks[0]).queue_index, 1);
    }

    #[test]
    fn too_large_message_fails_and_gives_its_index_back() {
        let mut q = queue();
        q.add_message(vec![0; 70_000], "big".into());
        assert_eq!(q.start_next_message(), vec![failed("big")]);
        assert_eq!(q.allocator.free_count(), usize::from(SEND_QUEUE_COUNT));
        assert!(q.send_queues.is_empty());
    }

    #[test]
    fn too_large_message_on_an_evicted_index_leaves_the_old_one_in_flight() {
        let mut q = queue();
        for i in 0..SEND_QUEUE_COUNT {
            q.add_message(body(10), format!("m{i}"));
            q.start_next_message();
        }
        // All 29 busy, so index 1 is evicted for this one, which is then rejected.
        q.add_message(vec![0; 70_000], "big".into());
        assert_eq!(q.start_next_message(), vec![failed("big")]);
        // m0 was never replaced: it still owns index 1 and its ACK still counts.
        assert!(q.allocator.is_in_use(1));
        assert_eq!(q.on_ack(1, true), Some(SendEvent::Delivered { message_id: "m0".into() }));
    }

    #[test]
    fn resend_rebuilds_requested_chunks_and_skips_the_rest() {
        let mut q = queue();
        q.add_message(body(100), "m".into());
        let events = q.start_next_message();
        let [SendEvent::Chunks(original)] = events.as_slice() else {
            panic!("expected one Chunks event, got {events:?}");
        };

        let resent = q.resend_chunks(&[
            (1 << 11) | 2, // queue 1, chunk 2: resent
            (1 << 11) | 9, // past the end of the message: skipped
            (7 << 11) | 0, // nothing in flight on queue 7: skipped
        ]);

        assert_eq!(resent.len(), 1);
        let h = header(&resent[0]);
        assert_eq!((h.queue_index, h.chunk_index, h.resend), (1, 2, true));
        assert_eq!(resent[0][2..], original[2][2..], "same payload as the first send");
    }

    /// Both halves talking: a chunk is lost, requested, resent, and the
    /// message is acknowledged.
    #[test]
    fn round_trip_with_a_lost_chunk() {
        let mut sender = queue();
        let mut receiver = ReceiveQueue::new();
        let mut feed = |chunk: &[u8]| match decode(chunk).unwrap() {
            Frame::Chunk { header, payload } => receiver.handle_chunk(header, payload),
            other => panic!("expected a chunk, got {other:?}"),
        };

        sender.add_message(body(100), "m".into());
        let events = sender.start_next_message();
        let [SendEvent::Chunks(chunks)] = events.as_slice() else {
            panic!("expected one Chunks event, got {events:?}");
        };

        let mut requested = Vec::new();
        for (i, chunk) in chunks.iter().enumerate() {
            if i == 2 {
                continue; // lost on the air
            }
            if let ReceiveEvent::NeedChunks { queue_index, chunks } = feed(chunk) {
                // What MISSING_CHUNKS carries: queue index << 11 | chunk index.
                requested.extend(chunks.iter().map(|&c| (u16::from(queue_index) << 11) | c));
            }
        }
        assert_eq!(requested, vec![(1 << 11) | 2]);

        let resent = sender.resend_chunks(&requested);
        assert_eq!(
            feed(&resent[0]),
            ReceiveEvent::Received { queue_index: 1, message: body(100), qaul_id: ID }
        );
        assert_eq!(sender.on_ack(1, true), Some(SendEvent::Delivered { message_id: "m".into() }));
    }
}
