// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Framing of everything written to / notified on the MSG characteristic.
//!

//!
//! Mirrors `ReceiveQueue.messageHeader()` in the Android module.

use crate::constants::QAUL_ID_BYTES;
use crate::flc::{FlcMessage, FlcType};

/// Size of a data chunk header in bytes.
pub const CHUNK_HEADER_SIZE: usize = 2;

/// Size of the extra header carried by the first chunk of a message.
pub const FIRST_CHUNK_HEADER_SIZE: usize = 19;

/// Highest queue index that can be expressed in 5 bits. Index 0 is reserved for
/// flow control, so real message queues use 1 to 29 (Android allocates 29 of them).
pub const MAX_QUEUE_INDEX: u8 = 31;

/// Highest chunk index that fits the 10 bit field.
pub const MAX_CHUNK_INDEX: u16 = 0x03FF;

/// Header of a data chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkHeader {
    /// Which send queue this chunk belongs to
    pub queue_index: u8,
    /// Set when this chunk is a retransmission requested by the receiver.
    pub resend: bool,
    /// Position of this chunk within its message (0 to 1023).
    pub chunk_index: u16,
}

impl ChunkHeader {
    /// Pack the header into its two bytes.
    ///
    /// Returns `None` if the values do not fit the header's bit fields, or if
    /// `queue_index` is 0, which is reserved for flow control frames.
    pub fn encode(&self) -> Option<[u8; CHUNK_HEADER_SIZE]> {
        if self.queue_index == 0
            || self.queue_index > MAX_QUEUE_INDEX
            || self.chunk_index > MAX_CHUNK_INDEX
        {
            return None;
        }

        let b0 = (self.queue_index << 3)
            | (u8::from(self.resend) << 2)
            | ((self.chunk_index >> 8) as u8 & 0x03);
        let b1 = (self.chunk_index & 0xFF) as u8;
        Some([b0, b1])
    }
}

/// The 19 byte header carried by chunk 0 of every message.
///
/// ```text
/// 0–1   chunk header           (chunk index is always 0)
/// 2     large message indicator
/// 3–4   message size          
/// 5–6   total chunks         
/// 7–10  CRC-32 of the message 
/// 11–18 qaul id
/// ```
///
/// Mirrors `getFirstHeader()` in SendQueue.kt. `frame::decode` does not treat
/// chunk 0 specially: it returns an ordinary `Frame::Chunk`, and the receive
/// side calls [`FirstChunkHeader::parse`] on that chunk to read the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstChunkHeader {
    /// Bytes 0–1. `chunk.chunk_index` must be 0.
    pub chunk: ChunkHeader,
    /// 0 if this message is not part of a larger one.
    pub large_message_indicator: u8,
    pub message_size: u16,
    pub total_chunks: u16,
    pub crc: u32,
    pub qaul_id: [u8; QAUL_ID_BYTES],
}

impl FirstChunkHeader {
    /// Bytes 2–18: everything after the ordinary 2 byte chunk header.
    const EXTRA: usize = FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE;

    /// Pack the header into its 19 bytes.
    ///
    /// Returns `None` if the chunk header is out of range, or is not chunk 0.
    pub fn encode(&self) -> Option<[u8; FIRST_CHUNK_HEADER_SIZE]> {
        if self.chunk.chunk_index != 0 {
            return None;
        }
        let mut out = [0u8; FIRST_CHUNK_HEADER_SIZE];
        out[0..2].copy_from_slice(&self.chunk.encode()?);
        out[2] = self.large_message_indicator;
        out[3..5].copy_from_slice(&self.message_size.to_be_bytes());
        out[5..7].copy_from_slice(&self.total_chunks.to_be_bytes());
        out[7..11].copy_from_slice(&self.crc.to_be_bytes());
        out[11..19].copy_from_slice(&self.qaul_id);
        Some(out)
    }

    /// Read the first chunk header from a chunk that `frame::decode` has
    /// already split into its 2 byte `ChunkHeader` and payload.
    ///
    /// Returns the header, and the part of the payload that is actual message
    /// data (everything after byte 18). That slice borrows from `payload`.
    pub fn parse(chunk: ChunkHeader, payload: &[u8]) -> Result<(Self, &[u8]), FrameError> {
        if chunk.chunk_index != 0 {
            return Err(FrameError::NotFirstChunk(chunk.chunk_index));
        }
        if payload.len() < Self::EXTRA {
            return Err(FrameError::TooShort {
                expected: FIRST_CHUNK_HEADER_SIZE,
                actual: CHUNK_HEADER_SIZE + payload.len(),
            });
        }

        // `payload` starts at byte 2 of the chunk, so every offset here is the
        // wire offset minus 2.
        let (extra, data) = payload.split_at(Self::EXTRA);
        let mut qaul_id = [0u8; QAUL_ID_BYTES];
        qaul_id.copy_from_slice(&extra[9..17]);

        let header = FirstChunkHeader {
            chunk,
            large_message_indicator: extra[0],
            message_size: u16::from_be_bytes([extra[1], extra[2]]),
            total_chunks: u16::from_be_bytes([extra[3], extra[4]]),
            crc: u32::from_be_bytes([extra[5], extra[6], extra[7], extra[8]]),
            qaul_id,
        };
        Ok((header, data))
    }
}

/// A frame received on the MSG characteristic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame<'a> {
    /// A flow control message (queue index 0).
    Flc(FlcMessage<'a>),
    /// A chunk of a queued message, with its payload borrowed from the input.
    Chunk {
        header: ChunkHeader,
        payload: &'a [u8],
    },
}

/// Why a frame could not be decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    /// The frame was shorter than its header requires.
    TooShort {
        expected: usize,
        actual: usize,
    },
    /// Queue index 0 with an FLC type this implementation does not know.
    UnknownFlcType(u8),
    /// The payload length does not match what this FLC type requires.
    MalformedFlc {
        flc_type: FlcType,
        len: usize,
    },
    /// A first chunk header was requested from a chunk that is not chunk 0.
    NotFirstChunk(u16),
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FrameError::TooShort { expected, actual } => {
                write!(f, "frame too short: need {expected} bytes, got {actual}")
            }
            FrameError::UnknownFlcType(t) => write!(f, "unknown flow control type {t:#04x}"),
            FrameError::MalformedFlc { flc_type, len } => {
                write!(f, "malformed {flc_type:?} flow control message ({len} payload bytes)")
            }
            FrameError::NotFirstChunk(index) => {
                write!(f, "chunk {index} has no first chunk header; only chunk 0 does")
            }
        }
    }
}

impl std::error::Error for FrameError {}

/// Decode one frame received on the MSG characteristic.
///
/// The returned value borrows from `bytes`, so no payload is copied.
pub fn decode(bytes: &[u8]) -> Result<Frame<'_>, FrameError> {
    let &b0 = bytes.first().ok_or(FrameError::TooShort {
        expected: 1,
        actual: 0,
    })?;

    let queue_index = b0 >> 3;

    if queue_index == 0 {
        // Flow control. Bit 3 is necessarily 0 here, so `& 0x0F` and `& 0x07`
        // agree; we mask with 0x0F to match Android exactly.
        let raw_type = b0 & 0x0F;
        let flc_type = FlcType::from_u8(raw_type).ok_or(FrameError::UnknownFlcType(raw_type))?;
        return FlcMessage::parse(flc_type, &bytes[1..]).map(Frame::Flc);
    }

    if bytes.len() < CHUNK_HEADER_SIZE {
        return Err(FrameError::TooShort {
            expected: CHUNK_HEADER_SIZE,
            actual: bytes.len(),
        });
    }

    let header = ChunkHeader {
        queue_index,
        resend: (b0 >> 2) & 0x01 == 1,
        chunk_index: (u16::from(b0 & 0x03) << 8) | u16::from(bytes[1]),
    };

    Ok(Frame::Chunk {
        header,
        payload: &bytes[CHUNK_HEADER_SIZE..],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flc::FlcMessage;

    /// Captured from a real Android node with btmon: an ATT Write
    /// command to MSG_CHAR carrying SEND_QAUL_ID and the devices q8id.
    
    const ANDROID_SEND_QAUL_ID: [u8; 9] = [0x01, 0xab, 0x59, 0x7b, 0xa8, 0x80, 0x6c, 0x04, 0x43];

    #[test]
    fn decodes_android_send_qaul_id_capture() {
        match decode(&ANDROID_SEND_QAUL_ID).expect("captured frame should decode") {
            Frame::Flc(FlcMessage::SendQaulId(id)) => {
                assert_eq!(id, &ANDROID_SEND_QAUL_ID[1..]);
            }
            other => panic!("expected SendQaulId, got {other:?}"),
        }
    }

    #[test]
    fn decodes_a_data_chunk() {
        // queue index 3, resend set, chunk index 0x105 = 261
        let b0 = (3 << 3) | (1 << 2) | 0x01;
        let bytes = [b0, 0x05, 0xde, 0xad];
        assert_eq!(
            decode(&bytes).unwrap(),
            Frame::Chunk {
                header: ChunkHeader {
                    queue_index: 3,
                    resend: true,
                    chunk_index: 261,
                },
                payload: &[0xde, 0xad],
            }
        );
    }

    #[test]
    fn chunk_header_round_trips() {
        for queue_index in 1..=29u8 {
            for chunk_index in [0u16, 1, 255, 256, MAX_CHUNK_INDEX] {
                for resend in [false, true] {
                    let header = ChunkHeader {
                        queue_index,
                        resend,
                        chunk_index,
                    };
                    let bytes = header.encode().expect("valid header should encode");
                    match decode(&bytes).unwrap() {
                        Frame::Chunk { header: got, .. } => assert_eq!(got, header),
                        other => panic!("expected chunk, got {other:?}"),
                    }
                }
            }
        }
    }

    #[test]
    fn chunk_header_rejects_out_of_range_values() {
        // 10 bit chunk index
        assert_eq!(
            ChunkHeader {
                queue_index: 1,
                resend: false,
                chunk_index: MAX_CHUNK_INDEX + 1,
            }
            .encode(),
            None
        );
        // queue index 0 is reserved: it would be read back as flow control
        assert_eq!(
            ChunkHeader {
                queue_index: 0,
                resend: false,
                chunk_index: 0,
            }
            .encode(),
            None
        );
    }

    #[test]
    fn empty_input_is_too_short() {
        assert_eq!(
            decode(&[]),
            Err(FrameError::TooShort {
                expected: 1,
                actual: 0
            })
        );
    }

    #[test]
    fn type_byte_above_0x07_is_read_as_a_chunk() {
        // The header has no room for an FLC type of 0x08: that bit pattern is
        // queue index 1, so it decodes as a data chunk. This is why FlcType stops
        // at 0x07, and why a ninth flow control message would need a new framing
        // scheme on both platforms.
        let bytes = [0x08, 0x00, 0x99];
        match decode(&bytes).unwrap() {
            Frame::Chunk { header, payload } => {
                assert_eq!(header.queue_index, 1);
                assert_eq!(header.chunk_index, 0);
                assert!(!header.resend);
                assert_eq!(payload, &[0x99]);
            }
            other => panic!("expected a chunk, got {other:?}"),
        }
    }

    fn sample_first_header() -> FirstChunkHeader {
        FirstChunkHeader {
            chunk: ChunkHeader {
                queue_index: 5,
                resend: false,
                chunk_index: 0,
            },
            large_message_indicator: 0x12,
            message_size: 0x0102,
            total_chunks: 0x0003,
            crc: 0x1234_5678,
            qaul_id: [1, 2, 3, 4, 5, 6, 7, 8],
        }
    }

    #[test]
    fn first_chunk_header_byte_layout() {

        let q5 = 5 << 3; // queue index 5, resend 0, chunk index 0
        assert_eq!(
            sample_first_header().encode().unwrap(),
            [
                q5, 0x00, // chunk header
                0x12, // large message indicator
                0x01, 0x02, // message size
                0x00, 0x03, // total chunks
                0x12, 0x34, 0x56, 0x78, // crc
                1, 2, 3, 4, 5, 6, 7, 8, // qaul id
            ]
        );
    }

    #[test]
    fn first_chunk_header_round_trips_through_decode_and_parse() {
        // Build a whole chunk 0: header plus some message data...
        let mut chunk = sample_first_header().encode().unwrap().to_vec();
        chunk.extend_from_slice(b"hello");

        // ...then read it back the way the receive side will: frame::decode
        // first, FirstChunkHeader::parse second.
        match decode(&chunk).unwrap() {
            Frame::Chunk { header, payload } => {
                let (parsed, data) = FirstChunkHeader::parse(header, payload).unwrap();
                assert_eq!(parsed, sample_first_header());
                assert_eq!(data, b"hello");
            }
            other => panic!("expected a chunk, got {other:?}"),
        }
    }

    #[test]
    fn first_chunk_header_only_exists_on_chunk_0() {
        let later = ChunkHeader {
            queue_index: 5,
            resend: false,
            chunk_index: 1,
        };
        assert_eq!(
            FirstChunkHeader::parse(later, &[0; 20]),
            Err(FrameError::NotFirstChunk(1))
        );

        let mut header = sample_first_header();
        header.chunk.chunk_index = 1;
        assert_eq!(header.encode(), None);
    }

    #[test]
    fn first_chunk_header_rejects_a_truncated_chunk() {
        let chunk_0 = ChunkHeader {
            queue_index: 5,
            resend: false,
            chunk_index: 0,
        };
        // 10 payload bytes: the 17 bytes after the chunk header are incomplete.
        assert_eq!(
            FirstChunkHeader::parse(chunk_0, &[0; 10]),
            Err(FrameError::TooShort {
                expected: FIRST_CHUNK_HEADER_SIZE,
                actual: 12
            })
        );
    }
}
