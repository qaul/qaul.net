// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Flow control (FLC) messages.
//!
//! These ride the MSG characteristic alongside data chunks and are distinguished
//! by queue index 0 (see [`crate::frame`]). They carry identity, acknowledgements,
//! retransmission requests, liveness and link state gossip.
//!
//! Mirrors `FlcCreate.kt` (encoding) and `ReceiveQueue.incomingFlowControlMessage()`
//! (decoding) in the Android module.

use crate::frame::FrameError;
use crate::{QAUL_ID_ADVERT_BYTES, QAUL_ID_BYTES};

/// Wire values of the flow control message types.
///
/// Only 0x00..=0x07 are expressible: bit 3 of the header byte is shared with the
/// queue index field, so a type of 0x08 or above would be read as a data chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FlcType {
    RequestQaulId = 0x00,
    SendQaulId = 0x01,
    MissingChunks = 0x02,
    AckSuccess = 0x03,
    AckError = 0x04,
    MissingAckMessages = 0x05,
    LivenessCheckPing = 0x06,
    SendNeighbours = 0x07,
}

impl FlcType {
    /// Map a wire value to a type, or `None` if it is not a known FLC type.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::RequestQaulId),
            0x01 => Some(Self::SendQaulId),
            0x02 => Some(Self::MissingChunks),
            0x03 => Some(Self::AckSuccess),
            0x04 => Some(Self::AckError),
            0x05 => Some(Self::MissingAckMessages),
            0x06 => Some(Self::LivenessCheckPing),
            0x07 => Some(Self::SendNeighbours),
            _ => None,
        }
    }
}

/// A decoded flow control message. Payloads borrow from the received frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlcMessage<'a> {
    /// Asks the peer to send its qaul ID.
    RequestQaulId,
    /// The sender's 8 byte qaul ID.
    SendQaulId(&'a [u8]),
    /// Chunks the receiver did not get, by index.
    MissingChunks(Vec<u16>),
    /// The message in this queue index arrived intact.
    AckSuccess { queue_index: u8 },
    /// The message in this queue index failed, with a reason code.
    AckError { queue_index: u8, error_code: u8 },
    /// Asks the peer to resend the acknowledgement for this queue index.
    MissingAckMessages { queue_index: u8 },
    /// Keepalive. Expects no reply.
    LivenessPing,
    /// Link state gossip.
    SendNeighbours(NeighbourUpdate<'a>),
}

impl<'a> FlcMessage<'a> {
    /// The type byte this message encodes to.
    pub fn flc_type(&self) -> FlcType {
        match self {
            Self::RequestQaulId => FlcType::RequestQaulId,
            Self::SendQaulId(_) => FlcType::SendQaulId,
            Self::MissingChunks(_) => FlcType::MissingChunks,
            Self::AckSuccess { .. } => FlcType::AckSuccess,
            Self::AckError { .. } => FlcType::AckError,
            Self::MissingAckMessages { .. } => FlcType::MissingAckMessages,
            Self::LivenessPing => FlcType::LivenessCheckPing,
            Self::SendNeighbours(_) => FlcType::SendNeighbours,
        }
    }

    /// Decode the body of a flow control frame (everything after the header byte).
    pub fn parse(flc_type: FlcType, payload: &'a [u8]) -> Result<Self, FrameError> {
        let malformed = || FrameError::MalformedFlc {
            flc_type,
            len: payload.len(),
        };

        match flc_type {
            FlcType::RequestQaulId => Ok(Self::RequestQaulId),
            FlcType::LivenessCheckPing => Ok(Self::LivenessPing),

            FlcType::SendQaulId => {
                if payload.len() != QAUL_ID_BYTES {
                    return Err(malformed());
                }
                Ok(Self::SendQaulId(payload))
            }

            FlcType::MissingChunks => {
                if payload.is_empty() || payload.len() % 2 != 0 {
                    return Err(malformed());
                }
                let indices = payload
                    .chunks_exact(2)
                    .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
                    .collect();
                Ok(Self::MissingChunks(indices))
            }

            FlcType::AckSuccess => match payload {
                [queue_index] => Ok(Self::AckSuccess {
                    queue_index: *queue_index,
                }),
                _ => Err(malformed()),
            },

            FlcType::AckError => match payload {
                [queue_index, error_code] => Ok(Self::AckError {
                    queue_index: *queue_index,
                    error_code: *error_code,
                }),
                _ => Err(malformed()),
            },

            FlcType::MissingAckMessages => match payload {
                [queue_index] => Ok(Self::MissingAckMessages {
                    queue_index: *queue_index,
                }),
                _ => Err(malformed()),
            },

            FlcType::SendNeighbours => {
                const P: usize = QAUL_ID_ADVERT_BYTES;
                // origin + seq(2) + ttl(1) + flags(1)
                const HEADER: usize = P + 4;

                if payload.len() < HEADER || (payload.len() - HEADER) % P != 0 {
                    return Err(malformed());
                }

                let neighbours = payload[HEADER..].chunks_exact(P).collect();

                Ok(Self::SendNeighbours(NeighbourUpdate {
                    origin: &payload[..P],
                    seq: u16::from_be_bytes([payload[P], payload[P + 1]]),
                    ttl: payload[P + 2],
                    sealed: payload[P + 3] & 0x01 != 0,
                    neighbours,
                }))
            }
        }
    }

    /// Encode this message as a complete frame, header byte included.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = vec![self.flc_type() as u8];
        match self {
            Self::RequestQaulId | Self::LivenessPing => {}
            Self::SendQaulId(id) => out.extend_from_slice(id),
            Self::MissingChunks(indices) => {
                for index in indices {
                    out.extend_from_slice(&index.to_be_bytes());
                }
            }
            Self::AckSuccess { queue_index } | Self::MissingAckMessages { queue_index } => {
                out.push(*queue_index)
            }
            Self::AckError {
                queue_index,
                error_code,
            } => {
                out.push(*queue_index);
                out.push(*error_code);
            }
            Self::SendNeighbours(update) => {
                out.extend_from_slice(update.origin);
                out.extend_from_slice(&update.seq.to_be_bytes());
                out.push(update.ttl);
                out.push(u8::from(update.sealed));
                for neighbour in &update.neighbours {
                    out.extend_from_slice(neighbour);
                }
            }
        }
        out
    }
}
