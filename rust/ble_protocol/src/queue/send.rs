// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! One message being sent: splitting it into chunks, and rebuilding any chunk
//! on request for a resend. Mirrors `SendQueueMessage` in `SendQueue.kt`.

use crate::{ChunkHeader, QAUL_ID_BYTES, constants::SEND_QUEUE_COUNT, crc::crc32, frame::{FirstChunkHeader, CHUNK_HEADER_SIZE, FIRST_CHUNK_HEADER_SIZE}};


pub const MAX_CHUNKS: usize = 1023;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendError {
    ChunkSizeTooSmall(usize),
    InvalidQueueIndex(u8),
    MessageTooLarge(usize),
    TooManyChunks(usize),
}

/// A message, validated and ready to be cut into chunks.
///

pub struct SendQueueMessage {
    qaul_id: [u8; QAUL_ID_BYTES],
    message_id: String,
    message_index: u8,
    message_size: u16,
    total_chunks: u16,
    large_message_indicator: u8,
    crc: u32,
    message: Vec<u8>,
    chunk_size: usize,
}

impl SendQueueMessage {
    pub fn new(qaul_id: [u8; QAUL_ID_BYTES], message_id: String, message_index: u8, large_message_indicator: u8, message: Vec<u8>, chunk_size: usize) -> Result<Self, SendError> {    
        if chunk_size < FIRST_CHUNK_HEADER_SIZE + 1 {
            return Err(SendError::ChunkSizeTooSmall(chunk_size));
        }
        if message_index == 0 || message_index > SEND_QUEUE_COUNT {
            return Err(SendError::InvalidQueueIndex(message_index));
        }
        let message_size = u16::try_from(message.len())
            .map_err(|_| SendError::MessageTooLarge(message.len()))?;

        let chunks = Self::get_chunk_count(message.len(), chunk_size);
        if chunks > MAX_CHUNKS {
            return Err(SendError::TooManyChunks(chunks));
        }
        
        
        Ok(SendQueueMessage {
            qaul_id,
            message_id,
            message_index,
            message_size,
            total_chunks: chunks as u16,
            large_message_indicator,
            crc: crc32(&message), 
            message,
            chunk_size
        })
    }

    fn get_chunk_count(message_len: usize, chunk_size: usize) -> usize {
        let first_capacity = chunk_size - FIRST_CHUNK_HEADER_SIZE;
        let later_capacity = chunk_size - CHUNK_HEADER_SIZE;
        let rest = message_len.saturating_sub(first_capacity);
        1 + rest.div_ceil(later_capacity)
    }

    /// Number of chunks this message is split into.
    pub fn total_chunks(&self) -> u16 {
        self.total_chunks
    }

    /// The id to report back to libqaul when this message is acknowledged.
    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    /// Every chunk, in order, for the first send.
    pub fn get_all_chunks(&self) -> Vec<Vec<u8>> {
        (0..self.total_chunks)
            .map(|index| {
                self.get_chunk(index, false)
                    .expect("every index below total_chunks exists")
            })
            .collect()
    }

    /// One complete chunk, header included, ready for a GATT write.
    ///
    /// Returns `None` if `chunk_index` is past the end. SO if a
    /// peer asks for a chunk that doesnt exist, it wont just panic.
    pub fn get_chunk(&self, chunk_index: u16, resend: bool) -> Option<Vec<u8>> {
        let payload = self.get_payload(chunk_index)?;

        let mut chunk = Vec::with_capacity(self.chunk_size);
        if chunk_index == 0 {
            chunk.extend_from_slice(&self.get_first_header(resend));
        } else {
            chunk.extend_from_slice(&self.get_header(chunk_index, resend));
        }
        chunk.extend_from_slice(payload);
        Some(chunk)
    }

    /// The part of the message carried by one chunk. Mirrors `getPayload()`.
    ///
    /// Borrows from `self.message` rather than copying.
    fn get_payload(&self, chunk_index: u16) -> Option<&[u8]> {
        if chunk_index >= self.total_chunks {
            return None;
        }

        let first_capacity = self.chunk_size - FIRST_CHUNK_HEADER_SIZE;
        let later_capacity = self.chunk_size - CHUNK_HEADER_SIZE;
        let len = self.message.len();

        let (start, end) = if chunk_index == 0 {
            (0, first_capacity.min(len))
        } else {
            let start = first_capacity + (usize::from(chunk_index) - 1) * later_capacity;
            (start, (start + later_capacity).min(len))
        };

        self.message.get(start..end)
    }

    fn get_header(&self, chunk_index: u16, resend: bool) -> [u8; CHUNK_HEADER_SIZE] {
        ChunkHeader {
            queue_index: self.message_index,
            resend, // requires state once thats implemented?
            chunk_index,
        }
        .encode()
        .expect("queue index and chunk count are validated in new()")
    }

    fn get_first_header(&self, resend: bool) -> [u8; FIRST_CHUNK_HEADER_SIZE] {
        FirstChunkHeader {
            chunk: ChunkHeader {
                queue_index: self.message_index,
                resend,
                chunk_index: 0,
            },
            large_message_indicator: self.large_message_indicator,
            message_size: self.message_size,
            total_chunks: self.total_chunks,
            crc: self.crc,
            qaul_id: self.qaul_id,
        }
        .encode()
        .expect("queue index is validated in new(), and chunk index 0 is always in range")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{decode, Frame};

    const ID: [u8; QAUL_ID_BYTES] = [1, 2, 3, 4, 5, 6, 7, 8];

    /// A message of `len` bytes counting 0, 1, 2… so any misplaced byte shows.
    fn message(len: usize, chunk_size: usize) -> Result<SendQueueMessage, SendError> {
        let body = (0..len).map(|i| i as u8).collect();
        SendQueueMessage::new(ID, "msg".into(), 5, 0, body, chunk_size)
    }

    #[test]
    fn chunk_count_matches_the_hand_worked_examples() {
        // chunk size 20: first chunk carries 1 byte, later ones 18
        assert_eq!(SendQueueMessage::get_chunk_count(10, 20), 2);
        // chunk size 509: first chunk carries 490, later ones 507
        assert_eq!(SendQueueMessage::get_chunk_count(0, 509), 1);
        assert_eq!(SendQueueMessage::get_chunk_count(490, 509), 1);
        assert_eq!(SendQueueMessage::get_chunk_count(491, 509), 2);
        assert_eq!(SendQueueMessage::get_chunk_count(997, 509), 2);
        assert_eq!(SendQueueMessage::get_chunk_count(998, 509), 3);
    }

    #[test]
    fn single_chunk_message_gets_the_first_header() {
        let msg = message(10, 509).unwrap();
        let chunks = msg.get_all_chunks();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), FIRST_CHUNK_HEADER_SIZE + 10);
    }

    #[test]
    fn payloads_reassemble_to_the_original() {
        // chunk size 25: first chunk carries 6 bytes, later ones 23
        let msg = message(100, 25).unwrap();
        let mut rebuilt = Vec::new();
        for (i, chunk) in msg.get_all_chunks().iter().enumerate() {
            let header = if i == 0 { FIRST_CHUNK_HEADER_SIZE } else { CHUNK_HEADER_SIZE };
            rebuilt.extend_from_slice(&chunk[header..]);
        }
        assert_eq!(rebuilt, message(100, 25).unwrap().message);
    }

    #[test]
    fn chunk_indices_count_up_without_gaps() {
        let msg = message(100, 25).unwrap();
        for (expected, chunk) in msg.get_all_chunks().iter().enumerate() {
            match decode(chunk).unwrap() {
                Frame::Chunk { header, .. } => {
                    assert_eq!(usize::from(header.chunk_index), expected);
                    assert_eq!(header.queue_index, 5);
                }
                other => panic!("expected a chunk, got {other:?}"),
            }
        }
    }

    #[test]
    fn no_chunk_is_larger_than_chunk_size() {
        for chunk in message(1000, 25).unwrap().get_all_chunks() {
            assert!(chunk.len() <= 25);
        }
    }

    #[test]
    fn empty_message_is_one_header_only_chunk() {
        let chunks = message(0, 509).unwrap().get_all_chunks();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), FIRST_CHUNK_HEADER_SIZE);
    }

    #[test]
    fn first_header_layout() {
        let msg = message(300, 509).unwrap();
        let header = msg.get_first_header(false);
        assert_eq!(header[2], 0, "large message indicator");
        assert_eq!(&header[3..5], &300u16.to_be_bytes(), "message size");
        assert_eq!(&header[5..7], &1u16.to_be_bytes(), "total chunks");
        assert_eq!(&header[7..11], &crc32(&msg.message).to_be_bytes(), "crc");
        assert_eq!(&header[11..19], &ID, "qaul id");
    }

    #[test]
    fn resend_flag_is_set_when_asked() {
        let chunk = message(100, 25).unwrap().get_chunk(2, true).unwrap();
        match decode(&chunk).unwrap() {
            Frame::Chunk { header, .. } => assert!(header.resend),
            other => panic!("expected a chunk, got {other:?}"),
        }
    }

    #[test]
    fn asking_for_a_chunk_past_the_end_is_none() {
        let msg = message(100, 25).unwrap();
        assert_eq!(msg.get_chunk(msg.total_chunks(), false), None);
    }

    #[test]
    fn rejects_invalid_input() {
        assert_eq!(message(10, 19).err(), Some(SendError::ChunkSizeTooSmall(19)));
        assert_eq!(
            SendQueueMessage::new(ID, "msg".into(), 0, 0, vec![1], 509).err(),
            Some(SendError::InvalidQueueIndex(0))
        );
        assert_eq!(message(70_000, 509).err(), Some(SendError::MessageTooLarge(70_000)));
        // at chunk size 20 a 65,535 byte message needs thousands of chunks
        assert!(matches!(message(65_535, 20), Err(SendError::TooManyChunks(_))));
    }
}