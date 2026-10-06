use std::collections::BTreeMap;

use crate::{frame::{ChunkHeader, FirstChunkHeader}};
use crate::crc::crc32; 

pub struct ReceiveQueueMessage {
    /// The queue index this message arrives on (1..=29).
    queue_index: u8,

    /// The 19 byte header from chunk 0. `None` until chunk 0 arrives 
    first: Option<FirstChunkHeader>,

    /// Payloads received so far, keyed by chunk index.
    chunks: BTreeMap<u16, Vec<u8>>,

    /// Highest chunk index seen so far. Any index below it that isn't in
    /// `chunks` is a gap. `None` until the first chunk arrives.
    highest_index: Option<u16>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChunkOutcome {
    Duplicate,
    Incomplete { newly_missing: Vec<u16> }, /// `newly_missing` holds only the gaps this chunk revealed, to be requested once, `missing_after_stall()` gives every gap still open.
    Complete { message: Vec<u8>, qaul_id: [u8; 8] },
    Failed(ReceiveError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiveError {
    BadFirstChunk,
    SizeMismatch,
    CrcMismatch,
}


impl ReceiveQueueMessage {
    pub fn new(queue_index: u8) -> Self {
        Self {
            queue_index,
            first: None,
            chunks: BTreeMap::new(),
            highest_index: None,
        }
    }
    
    /// Add a newly received chunk message
    pub fn add_received_chunk(&mut self, header: ChunkHeader, payload: &[u8]) -> ChunkOutcome {
        let index = header.chunk_index;

        // 1. Already have it? Stop here.
        if self.chunks.contains_key(&index) {
            return ChunkOutcome::Duplicate;
        }

        // 2. Store the chunk. Chunk 0 carries the extra header, the rest are plain payload
        if index == 0 {
            match FirstChunkHeader::parse(header, payload) {
                Ok((first, data)) => {
                    self.first = Some(first);
                    self.chunks.insert(0, data.to_vec());
                }
                Err(_) => return ChunkOutcome::Failed(ReceiveError::BadFirstChunk),
            }
        } else {
            self.chunks.insert(index, payload.to_vec());
        }

        // 3. Which gaps did this chunk reveal? Nothing above the highest index has arrived yet, 
        //    so every index between it and this one is a gap. A resend is will be below the highest, so the range is empty.
        let start = match self.highest_index {
            Some(highest) => highest + 1,
            None => 0, // first chunk seen: everything before it is a gap
        };
        let newly_missing: Vec<u16> = (start..index).collect();

        if self.highest_index.map_or(true, |highest| index > highest) {
            self.highest_index = Some(index);
        }

        // 4. Finished, Build and check the message.
        if self.is_complete() {
            return self.assemble();
        }

        // 5. Not finished yet, report the gaps this chunk revealed.
        ChunkOutcome::Incomplete { newly_missing }
    }

    pub fn highest_index(&self) -> Option<u16> {
        self.highest_index
    }

    /// Total chunks in the message, known once chunk 0 has arrived.
    pub fn total_chunks(&self) -> Option<u16> {
        self.first.as_ref().map(|first| first.total_chunks)
    }

    pub fn is_complete(&self) -> bool {
        match &self.first {
            Some(first) => usize::from(first.total_chunks) == self.chunks.len(),
            None => false,   // no chunk 0 yet, so we can't know
        }
    }

    fn assemble(&self) -> ChunkOutcome {
        let first = match &self.first {
            Some(first) => first,
            None => return ChunkOutcome::Failed(ReceiveError::BadFirstChunk),
        };

        let message: Vec<u8> = self.chunks.values().flatten().copied().collect();

        if message.len() != usize::from(first.message_size) {
            return ChunkOutcome::Failed(ReceiveError::SizeMismatch);
        }
        if crc32(&message) != first.crc {
            return ChunkOutcome::Failed(ReceiveError::CrcMismatch);
        }

        // large message part check will be here

        ChunkOutcome::Complete { message, qaul_id: first.qaul_id }
    }

    /// Every chunk still missing, for the engine to re request on a timer. 
    /// Once chunk 0 is known this runs to the end of the message, so lost
    /// final chunks are caught. before that, only up to the highest seen.
    ///
    /// Only call this once the message has stalled (no chunk for a while)
    /// as otherwise it lists chunks that are still on their way.

    // In theory this will help with lost final chunks
    pub fn missing_after_stall(&self) -> Vec<u16> {
        let end = match (&self.first, self.highest_index) {
            (Some(first), _) => first.total_chunks,
            (None, Some(highest)) => highest + 1,
            (None, None) => 0, 
        };
        (0..end).filter(|i| !self.chunks.contains_key(i)).collect()
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

    /// The message cut into chunks by the real send side.
    /// At chunk size 25 the first chunk carries 6 bytes, later ones 23.
    fn chunks(len: usize, chunk_size: usize) -> Vec<Vec<u8>> {
        SendQueueMessage::new(ID, "msg".into(), 5, 0, body(len), chunk_size)
            .unwrap()
            .get_all_chunks()
    }

    /// Decode one chunk off the wire and hand it to the receiver.
    fn feed(msg: &mut ReceiveQueueMessage, chunk: &[u8]) -> ChunkOutcome {
        match decode(chunk).unwrap() {
            Frame::Chunk { header, payload } => msg.add_received_chunk(header, payload),
            other => panic!("expected a chunk, got {other:?}"),
        }
    }

    fn waiting(newly_missing: &[u16]) -> ChunkOutcome {
        ChunkOutcome::Incomplete { newly_missing: newly_missing.to_vec() }
    }

    fn complete(len: usize) -> ChunkOutcome {
        ChunkOutcome::Complete { message: body(len), qaul_id: ID }
    }

    #[test]
    fn in_order_chunks_complete_the_message() {
        let c = chunks(100, 25); // 6 chunks
        let mut msg = ReceiveQueueMessage::new(5);
        for chunk in &c[..5] {
            assert_eq!(feed(&mut msg, chunk), waiting(&[]));
        }
        assert_eq!(feed(&mut msg, &c[5]), complete(100));
    }

    #[test]
    fn out_of_order_chunks_complete_the_message() {
        let c = chunks(100, 25);
        let mut msg = ReceiveQueueMessage::new(5);
        // The last chunk first: everything before it is a gap.
        assert_eq!(feed(&mut msg, &c[5]), waiting(&[0, 1, 2, 3, 4]));
        // The rest fill holes, so they reveal nothing new.
        for chunk in c[1..5].iter().rev() {
            assert_eq!(feed(&mut msg, chunk), waiting(&[]));
        }
        assert_eq!(feed(&mut msg, &c[0]), complete(100));
    }

    #[test]
    fn duplicate_chunk_is_reported() {
        let c = chunks(100, 25);
        let mut msg = ReceiveQueueMessage::new(5);
        feed(&mut msg, &c[0]);
        feed(&mut msg, &c[1]);
        assert_eq!(feed(&mut msg, &c[1]), ChunkOutcome::Duplicate);
    }

    #[test]
    fn a_gap_is_reported_once_and_a_resend_requests_nothing() {
        let c = chunks(100, 25);
        let mut msg = ReceiveQueueMessage::new(5);
        feed(&mut msg, &c[0]);
        feed(&mut msg, &c[1]);
        assert_eq!(feed(&mut msg, &c[3]), waiting(&[2])); // 2 lost
        assert_eq!(feed(&mut msg, &c[4]), waiting(&[])); // not asked again
        assert_eq!(feed(&mut msg, &c[2]), waiting(&[])); // resend fills it
        // A later chunk still reveals no gaps, whatever order chunks arrive in.
        assert_eq!(feed(&mut msg, &c[5]), complete(100));
    }

    #[test]
    fn stall_without_header_then_with_header() {
        let c = chunks(213, 25); // 6 + 9 × 23 bytes = 10 chunks
        assert_eq!(c.len(), 10);
        let mut msg = ReceiveQueueMessage::new(5);
        assert_eq!(msg.missing_after_stall(), Vec::<u16>::new());

        // Only chunk 5 arrives: without the header we only know up to 5.
        feed(&mut msg, &c[5]);
        assert_eq!(msg.missing_after_stall(), vec![0, 1, 2, 3, 4]);

        // Chunk 0 arrives: now we know there are 10, so the tail shows up.
        feed(&mut msg, &c[0]);
        assert_eq!(msg.missing_after_stall(), vec![1, 2, 3, 4, 6, 7, 8, 9]);
    }

    #[test]
    fn lost_final_chunk_is_found_after_a_stall() {
        let c = chunks(100, 25);
        let mut msg = ReceiveQueueMessage::new(5);
        for chunk in &c[..5] {
            feed(&mut msg, chunk);
        }
        // Nothing after chunk 5 could reveal it as a gap; only the stall can.
        assert_eq!(msg.missing_after_stall(), vec![5]);
    }

    #[test]
    fn corrupted_chunk_fails_the_crc() {
        let mut c = chunks(100, 25);
        let last = c[2].len() - 1;
        c[2][last] ^= 0xFF; // flip one payload byte
        let mut msg = ReceiveQueueMessage::new(5);
        let outcomes: Vec<ChunkOutcome> = c.iter().map(|chunk| feed(&mut msg, chunk)).collect();
        assert_eq!(outcomes.last(), Some(&ChunkOutcome::Failed(ReceiveError::CrcMismatch)));
    }

    #[test]
    fn single_chunk_and_empty_messages_complete_at_once() {
        let mut msg = ReceiveQueueMessage::new(5);
        assert_eq!(feed(&mut msg, &chunks(10, 509)[0]), complete(10));

        let mut msg = ReceiveQueueMessage::new(5);
        assert_eq!(feed(&mut msg, &chunks(0, 509)[0]), complete(0));
    }

    #[test]
    fn truncated_first_chunk_fails() {
        let c = chunks(100, 25);
        let mut msg = ReceiveQueueMessage::new(5);
        assert_eq!(
            feed(&mut msg, &c[0][..5]),
            ChunkOutcome::Failed(ReceiveError::BadFirstChunk)
        );
    }
}