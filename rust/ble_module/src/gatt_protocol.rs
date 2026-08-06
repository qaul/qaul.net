// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! BLE GATT direct-message framing helpers.
//!
//! The Linux BLE module writes direct messages as:
//! - JSON (`qaul_id`, `message`) with signed-byte arrays
//! - wrapped by `$$` delimiters
//! - hex-encoded
//! - chunked into <= 20 byte GATT writes (40 hex chars)
//!
//! This module keeps that wire format in one place so production code and
//! integration tests exercise the same framing/reassembly logic.

use serde::{Deserialize, Serialize};
use std::fmt;

pub const GATT_CHUNK_BYTES: usize = 20;
pub const GATT_CHUNK_HEX_LEN: usize = GATT_CHUNK_BYTES * 2;
const FRAME_DELIMITER_HEX: &str = "2424";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectMessage {
    pub qaul_id: Vec<u8>,
    pub message: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GattProtocolError {
    InvalidHexLength,
    InvalidHex,
    InvalidUtf8,
    InvalidJson,
    MissingField(&'static str),
    InvalidFrame,
    EmbeddedDelimiter,
}

impl fmt::Display for GattProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHexLength => write!(f, "invalid hex length"),
            Self::InvalidHex => write!(f, "invalid hex data"),
            Self::InvalidUtf8 => write!(f, "invalid utf8 payload"),
            Self::InvalidJson => write!(f, "invalid json payload"),
            Self::MissingField(field) => write!(f, "missing field: {}", field),
            Self::InvalidFrame => write!(f, "invalid BLE GATT frame"),
            Self::EmbeddedDelimiter => write!(f, "embedded delimiter in BLE GATT frame"),
        }
    }
}

impl std::error::Error for GattProtocolError {}

#[derive(Debug, Default, Clone)]
pub struct GattMessageReassembler {
    buffered_hex: String,
}

impl GattMessageReassembler {
    pub fn push_chunk(&mut self, chunk: &[u8]) -> Result<Option<DirectMessage>, GattProtocolError> {
        if chunk.is_empty() {
            return Ok(None);
        }

        let hex_chunk = bytes_to_hex(chunk);

        if self.buffered_hex.is_empty() {
            if FRAME_DELIMITER_HEX.starts_with(hex_chunk.as_str()) {
                self.buffered_hex.push_str(&hex_chunk);
                return Ok(None);
            }
            if !hex_chunk.starts_with(FRAME_DELIMITER_HEX) {
                return Ok(None);
            }
            self.buffered_hex.push_str(&hex_chunk);
        } else {
            self.buffered_hex.push_str(&hex_chunk);
        }

        if self.buffered_hex.len() < FRAME_DELIMITER_HEX.len() {
            return Ok(None);
        }
        if !self.buffered_hex.starts_with(FRAME_DELIMITER_HEX) {
            self.buffered_hex.clear();
            return Err(GattProtocolError::InvalidFrame);
        }

        if !self.buffered_hex.ends_with(FRAME_DELIMITER_HEX) {
            return Ok(None);
        }

        let frame_hex = std::mem::take(&mut self.buffered_hex);
        let payload_hex = strip_frame(&frame_hex)?;
        decode_direct_message_hex_payload(payload_hex).map(Some)
    }
}

pub fn encode_direct_message_chunks(
    qaul_id: &[u8],
    message: &[u8],
) -> Result<Vec<Vec<u8>>, GattProtocolError> {
    let wire = WireMessageJson {
        qaul_id: Some(qaul_id.iter().map(|b| *b as i8).collect()),
        message: Some(message.iter().map(|b| *b as i8).collect()),
    };

    let json = serde_json::to_vec(&wire).map_err(|_| GattProtocolError::InvalidJson)?;
    let mut framed = Vec::with_capacity(2 + json.len() + 2);
    framed.extend_from_slice(b"$$");
    framed.extend_from_slice(&json);
    framed.extend_from_slice(b"$$");

    let framed_hex = bytes_to_hex(&framed);
    let mut chunks = Vec::new();
    let mut offset = 0;
    while offset < framed_hex.len() {
        let end = (offset + GATT_CHUNK_HEX_LEN).min(framed_hex.len());
        let chunk = hex_to_bytes(&framed_hex[offset..end])?;
        chunks.push(chunk);
        offset = end;
    }

    Ok(chunks)
}

pub fn decode_direct_message_hex_payload(
    payload_hex: &str,
) -> Result<DirectMessage, GattProtocolError> {
    let json_bytes = hex_to_bytes(payload_hex)?;
    let json = String::from_utf8(json_bytes).map_err(|_| GattProtocolError::InvalidUtf8)?;
    let msg: WireMessageJson =
        serde_json::from_str(&json).map_err(|_| GattProtocolError::InvalidJson)?;

    let qaul_id = msg
        .qaul_id
        .ok_or(GattProtocolError::MissingField("qaul_id"))?
        .into_iter()
        .map(|v| v as u8)
        .collect();
    let message = msg
        .message
        .ok_or(GattProtocolError::MissingField("message"))?
        .into_iter()
        .map(|v| v as u8)
        .collect();

    Ok(DirectMessage { qaul_id, message })
}

fn strip_frame(frame_hex: &str) -> Result<&str, GattProtocolError> {
    if frame_hex.len() < FRAME_DELIMITER_HEX.len() * 2
        || !frame_hex.starts_with(FRAME_DELIMITER_HEX)
        || !frame_hex.ends_with(FRAME_DELIMITER_HEX)
    {
        return Err(GattProtocolError::InvalidFrame);
    }

    let inner = &frame_hex[FRAME_DELIMITER_HEX.len()..frame_hex.len() - FRAME_DELIMITER_HEX.len()];
    if inner.contains(FRAME_DELIMITER_HEX) {
        return Err(GattProtocolError::EmbeddedDelimiter);
    }

    Ok(inner)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let hex_chars: Vec<String> = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
    hex_chars.join("")
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, GattProtocolError> {
    if hex.len() % 2 != 0 {
        return Err(GattProtocolError::InvalidHexLength);
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| GattProtocolError::InvalidHex))
        .collect()
}

#[derive(Serialize, Deserialize)]
struct WireMessageJson {
    #[serde(rename = "qaul_id")]
    qaul_id: Option<Vec<i8>>,
    #[serde(rename = "message")]
    message: Option<Vec<i8>>,
}
