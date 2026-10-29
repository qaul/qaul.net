// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Manages everything being sent over one connection: messages waiting for a queue index,
//! messages in flight awaiting their ACK, and resends on request. Mirrors
//! `SendQueue` in `SendQueue.kt`.

use std::cmp;
use std::collections::{BTreeMap, VecDeque};

use crate::constants::{MAX_LARGE_PARTS, MAX_PART_SIZE, QAUL_ID_BYTES};
use crate::frame::{CHUNK_HEADER_SIZE, FIRST_CHUNK_HEADER_SIZE, LargePart};
use crate::queue::index::QueueIndexAllocator;
use crate::queue::send::{MAX_CHUNKS, SendQueueMessage};

/// What the caller should do after the send queue has acted.
#[derive(Debug, PartialEq, Eq)]
pub enum SendEvent {
    /// Write these chunks to the peer, in order.
    Chunks(Vec<Vec<u8>>),
    /// The peer acknowledged this message. Tell libqaul it was delivered.
    Delivered { message_id: String },
    /// This message will not arrive. Tell libqaul it failed, and why.
    Failed { message_id: String, reason: SendFailure },
}

/// Why a message didn't arrive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendFailure {
    /// No identified peer with that qaul id (engine).
    NotIdentified,
    /// Identified, but its link is gone (engine).
    NoLink,
    /// All 29 queue indices were busy, so its index went to a newer message.
    Evicted,
    /// Too big to send: over 4 parts, or too many chunks at the current chunk size.
    TooLarge,
    /// The peer answered with ACK_ERROR.
    AckError,
    /// The link closed while it was waiting or in flight.
    LinkGone,
}

/// A message handed over by libqaul that has no queue index yet.
struct Waiting {
    message: Vec<u8>,
    message_id: String,
    large_message_indicator: u8, // 0 for a normal message
}

/// A message split into parts that are still waiting or in flight. 
struct LargeSend {
    message_id: String,
    /// Parts not ACKed yet. The message is delivered when this reaches 0.
    parts_left: u8,
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
    /// The large index given to the last split message, 1..=15. 0 before the first.
    large_message_index: u8,
    /// Split messages still in progress, keyed by large index. An index is only
    /// reused while still here if 15 split messages are unfinished at once.
    large_sends: BTreeMap<u8, LargeSend>
}

impl SendQueue {
    pub fn new(qaul_id: [u8; QAUL_ID_BYTES], chunk_size: usize) -> Self {
        Self {
            qaul_id,
            chunk_size,
            allocator: QueueIndexAllocator::new(),
            send_queues: BTreeMap::new(),
            waiting: VecDeque::new(),
            large_message_index: 0,
            large_sends: BTreeMap::new()
        }
    }

    /// Messages holding a queue index: started and not ACKed yet.
    pub fn sending_count(&self) -> usize {
        self.send_queues.len()
    }

    /// Messages (and parts) not started yet.
    pub fn waiting_count(&self) -> usize {
        self.waiting.len()
    }

    /// Change the chunk size after an MTU negotiation.
    pub fn set_chunk_size(&mut self, chunk_size: usize) {
        self.chunk_size = chunk_size;
    }

    /// Queue a message from libqaul. Later it will be sent by `start_next_message`.
    pub fn add_message(&mut self, message: Vec<u8>, message_id: String) {
        let part_size = self.max_part_size();
        if message.len() > part_size { // Large message

            let total = message.len().div_ceil(part_size);
            if total > MAX_LARGE_PARTS {
                // TODO: report this message_id as Failed, needs add_message to return events.
                return;
            }
            // At most 4, so the casts below can't truncate.
            let total = total as u8;
            let index = self.next_large_index();

            for (part, bytes) in message.chunks(part_size).enumerate() {
                let indicator = LargePart { index, total, part: part as u8}.to_indicator();
                self.waiting.push_back(Waiting {message: bytes.to_vec(), message_id: message_id.clone(), large_message_indicator: indicator });
            }
            // Track the large send so we can see which parts are acked/lost
            self.large_sends.insert(index, LargeSend { message_id, parts_left: total });
        }
        else {
            self.waiting.push_back(Waiting { message, message_id, large_message_indicator: 0 });
        }
    }

    /// The large index for the next split message: 1..=15, wrapping back to 1.
    /// Never 0, this would make part 0 of a 4 part message read as an ordinary message as its 0x00.
    fn next_large_index(&mut self) -> u8 {
        self.large_message_index = self.large_message_index % 15 + 1;
        self.large_message_index
    }

    /// Most bytes one message can hold at the current chunk size: what
    /// fits in `MAX_CHUNKS` chunks, capped by the 2 byte size field. Anything larger needs split into large parts.
    fn max_part_size(&self) -> usize {
        cmp::min(
            (self.chunk_size - FIRST_CHUNK_HEADER_SIZE) + (MAX_CHUNKS - 1) * (self.chunk_size - CHUNK_HEADER_SIZE),
            MAX_PART_SIZE,
        )
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
            waiting.large_message_indicator,
            waiting.message,
            self.chunk_size,
        );

        match send_message {
            Ok(message) => {
                // 4. Put it in flight and return its chunks.
                let mut events = Vec::new();
                let chunks = message.get_all_chunks();
                if let Some(old) = self.send_queues.insert(queue_index, message) {
                    let failed = Err(SendFailure::Evicted);
                    events.extend(self.report(old.message_id().to_string(), old.large_message_indicator(), failed));
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
                let failed = Err(SendFailure::TooLarge);
                self.report(waiting.message_id, waiting.large_message_indicator, failed).into_iter().collect()
            }
        }
    }

    /// An ACK arrived for `queue_index`. Returns a `SendEvent` to report to libqaul, or `None` if the
    /// index was not in use.
    pub fn on_ack(&mut self, queue_index: u8, success: bool) -> Option<SendEvent> {
        let message = self.send_queues.remove(&queue_index)?;
        self.allocator.release(queue_index);
        let result = if success { Ok(()) } else { Err(SendFailure::AckError) };
        self.report(message.message_id().to_string(), message.large_message_indicator(), result)
    }

    /// A message is done, by ACK or failure. Returns what to tell libqaul: an ordinary
    /// message reports itself, a part reports for its whole split message.
    fn report(&mut self, message_id: String, indicator: u8, result: Result<(), SendFailure>) -> Option<SendEvent> {
        match LargePart::from_indicator(indicator) {
            None => Some(match result {
                Ok(()) => SendEvent::Delivered { message_id },
                Err(reason) => SendEvent::Failed { message_id, reason },
            }),
            Some(part) => self.on_part_ack(part, result),
        }
    }

    /// One part is done. The split message is delivered once every part is ACKed,
    /// and fails on its first failed part. Parts aren't retried, unlike
    /// Android which requeues them forever. libqaul resends the message end to end? TODO: re-assess whether we should reque, 
    fn on_part_ack(&mut self, part: LargePart, result: Result<(), SendFailure>) -> Option<SendEvent> {
        // Not there: the message already failed through another part.
        let large_send = self.large_sends.get_mut(&part.index)?;
        let Err(reason) = result else {
            large_send.parts_left -= 1;
            if large_send.parts_left > 0 {
                return None;
            }
            return self.large_sends.remove(&part.index).map(|l| SendEvent::Delivered { message_id: l.message_id });
        };
        // remove the rest of this parts other large parts still waiting
        self.waiting.retain(|w| {
            LargePart::from_indicator(w.large_message_indicator).is_none_or(|p| p.index != part.index)
        });
        self.large_sends.remove(&part.index).map(|l| SendEvent::Failed { message_id: l.message_id, reason })
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
    /// A split message is reported once, from `large_sends`, so one that already failed isn't reported again.
    pub fn fail_all(self) -> Vec<String> {
        let send_queues = self
            .send_queues
            .into_values()
            .filter(|m| m.large_message_indicator() == 0)
            .map(|m| m.message_id().to_string());
        let waiting = self.waiting.into_iter().filter(|w| w.large_message_indicator == 0).map(|w| w.message_id);
        let large_sends = self.large_sends.into_values().map(|l| l.message_id);
        send_queues.chain(waiting).chain(large_sends).collect()
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

    fn failed(id: &str, reason: SendFailure) -> SendEvent {
        SendEvent::Failed { message_id: id.into(), reason }
    }

    #[test]
    fn ack_reports_the_result_and_frees_the_index() {
        let mut q = queue();
        q.add_message(body(10), "ok".into());
        q.add_message(body(10), "bad".into());
        q.start_next_message(); // index 1
        q.start_next_message(); // index 2
        assert_eq!(q.on_ack(1, true), Some(SendEvent::Delivered { message_id: "ok".into() }));
        assert_eq!(q.on_ack(2, false), Some(failed("bad", SendFailure::AckError)));
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
        assert_eq!(events[0], failed("m0", SendFailure::Evicted));
        let SendEvent::Chunks(chunks) = &events[1] else {
            panic!("expected Chunks, got {:?}", events[1]);
        };
        assert_eq!(header(&chunks[0]).queue_index, 1);
    }

    /// Queue a 60 KB message at chunk size 509, where it fits in one part, then
    /// shrink the chunks so it needs more than `MAX_CHUNKS` and can't be built.
    fn add_then_shrink(q: &mut SendQueue) {
        q.set_chunk_size(509);
        q.add_message(vec![0; 60_000], "big".into());
        q.set_chunk_size(25);
    }

    #[test]
    fn message_too_big_for_the_new_chunk_size_fails_and_gives_its_index_back() {
        let mut q = queue();
        add_then_shrink(&mut q);
        assert_eq!(q.start_next_message(), vec![failed("big", SendFailure::TooLarge)]);
        assert_eq!(q.allocator.free_count(), usize::from(SEND_QUEUE_COUNT));
        assert!(q.send_queues.is_empty());
    }

    #[test]
    fn message_too_big_on_an_evicted_index_leaves_the_old_one_in_flight() {
        let mut q = queue();
        for i in 0..SEND_QUEUE_COUNT {
            q.add_message(body(10), format!("m{i}"));
            q.start_next_message();
        }
        // All 29 busy, so index 1 is evicted for this one, which is then rejected.
        add_then_shrink(&mut q);
        assert_eq!(q.start_next_message(), vec![failed("big", SendFailure::TooLarge)]);
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

    /// At chunk size 25 a part holds 23 512 bytes, so 50 000 is 3 parts.
    const SPLIT_LEN: usize = 50_000;

    fn delivered(id: &str) -> SendEvent {
        SendEvent::Delivered { message_id: id.into() }
    }

    #[test]
    fn part_size_is_capped_by_the_size_field_or_the_chunk_count() {
        assert_eq!(SendQueue::new(ID, 509).max_part_size(), MAX_PART_SIZE);
        assert_eq!(SendQueue::new(ID, 25).max_part_size(), 6 + 1022 * 23);
    }

    #[test]
    fn large_message_is_split_into_parts_with_their_indicators() {
        let mut q = queue();
        q.add_message(body(SPLIT_LEN), "big".into());
        let parts: Vec<_> = q.waiting.iter().map(|w| (w.message.len(), LargePart::from_indicator(w.large_message_indicator))).collect();
        let part = |part| Some(LargePart { index: 1, total: 3, part });
        assert_eq!(parts, vec![(23_512, part(0)), (23_512, part(1)), (2_976, part(2))]);
    }

    #[test]
    fn large_index_counts_1_to_15_and_skips_0() {
        let mut q = queue();
        let indices: Vec<u8> = (0..16).map(|_| q.next_large_index()).collect();
        assert_eq!(indices[..15], (1..=15).collect::<Vec<u8>>()[..]);
        assert_eq!(indices[15], 1);
    }

    #[test]
    fn message_over_four_parts_is_not_queued() {
        let mut q = queue();
        q.add_message(body(5 * 23_512), "huge".into());
        assert!(q.waiting.is_empty());
        assert!(q.large_sends.is_empty());
    }

    #[test]
    fn split_message_is_delivered_once_after_its_last_part() {
        let mut q = queue();
        q.add_message(body(SPLIT_LEN), "big".into());
        for _ in 0..3 {
            q.start_next_message(); // queue indices 1, 2, 3
        }
        assert_eq!(q.on_ack(2, true), None);
        assert_eq!(q.on_ack(1, true), None);
        assert_eq!(q.on_ack(3, true), Some(delivered("big")));
        assert!(q.large_sends.is_empty());
    }

    #[test]
    fn failed_part_fails_the_message_once_and_drops_its_other_parts() {
        let mut q = queue();
        q.add_message(body(SPLIT_LEN), "big".into());
        q.add_message(body(10), "small".into());
        q.start_next_message(); // part 0 on 1
        q.start_next_message(); // part 1 on 2, part 2 still waiting
        assert_eq!(q.on_ack(1, false), Some(failed("big", SendFailure::AckError)));
        // Part 2 is dropped, the ordinary message behind it is kept.
        assert_eq!(q.waiting.iter().map(|w| w.message_id.as_str()).collect::<Vec<_>>(), vec!["small"]);
        // Part 1 was in flight, its late ACK reports nothing more.
        assert_eq!(q.on_ack(2, true), None);
    }

    #[test]
    fn fail_all_reports_a_split_message_once() {
        let mut q = queue();
        q.add_message(body(SPLIT_LEN), "big".into());
        q.add_message(body(10), "small".into());
        q.start_next_message(); // part 0 in flight, parts 1 and 2 waiting
        assert_eq!(q.fail_all(), vec!["small".to_string(), "big".to_string()]);
    }

    #[test]
    fn fail_all_does_not_report_a_split_message_that_already_failed() {
        let mut q = queue();
        q.add_message(body(SPLIT_LEN), "big".into());
        q.start_next_message();
        q.start_next_message();
        assert_eq!(q.on_ack(1, false), Some(failed("big", SendFailure::AckError)));
        assert!(q.fail_all().is_empty()); // part 1 is still in flight
    }

    /// Both halves talking: every part goes through a ReceiveQueue, which joins
    /// them, and its ACKs deliver the message once.
    #[test]
    fn split_message_round_trip() {
        let mut sender = queue();
        let mut receiver = ReceiveQueue::new();
        sender.add_message(body(SPLIT_LEN), "big".into());

        let mut results = Vec::new();
        let mut received = Vec::new();
        loop {
            let events = sender.start_next_message();
            let chunks = match events.as_slice() {
                [] => break,
                [SendEvent::Chunks(chunks)] => chunks,
                other => panic!("expected one Chunks event, got {other:?}"),
            };
            for chunk in chunks {
                let Frame::Chunk { header, payload } = decode(chunk).unwrap() else { panic!("expected a chunk") };
                match receiver.handle_chunk(header, payload) {
                    ReceiveEvent::Received { queue_index, message, .. } => {
                        received.push(message);
                        results.extend(sender.on_ack(queue_index, true));
                    }
                    ReceiveEvent::PartReceived { queue_index } => results.extend(sender.on_ack(queue_index, true)),
                    _ => {}
                }
            }
        }
        assert_eq!(received, vec![body(SPLIT_LEN)]);
        assert_eq!(results, vec![delivered("big")]);
    }
}
