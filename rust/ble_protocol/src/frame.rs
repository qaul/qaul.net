// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Framing of everything written to / notified on the MSG characteristic.
//!

//!
//! Mirrors `ReceiveQueue.messageHeader()` in the Android module.

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
}
