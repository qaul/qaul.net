// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! #Qaul's Varint Prefixed Codec
//!
//! This coded does the following:
//!
//! - Check if a package fit's into the defined maximal length.
//! - Prefix a package with it's length as and `unsigned_varint`.
//! - Receive and deliver the raw bytes of a package as `Vec<u8>`.
//!
//! The codec is used in qaul's libp2p protocols: `qaul_info`
//! protocol and `qaul_messaging` protocol.
//!
//! This codec is fully compatible with the way qaul was sending
//! the packages before.

use asynchronous_codec::{Decoder, Encoder};
use bytes::{Buf, BytesMut};
use std::io::{Error, ErrorKind};

/// Varint Prefixed Codec Structure
pub struct VarintPrefixedCodec {
    /// The maximal length of the protobuf message.
    /// The bytes needed for the `unsigned_varint` are not included
    /// in this limit.
    max_message_len_bytes: usize,
}

impl VarintPrefixedCodec {
    /// Initialize Codec with maximal message length.
    pub fn new(max_message_len_bytes: usize) -> Self {
        Self {
            max_message_len_bytes,
        }
    }
}

impl Encoder for VarintPrefixedCodec {
    type Item<'a> = Vec<u8>;
    type Error = Error;

    /// Encode the `Vec<u8>` raw bytes message, by adding a message length prefix
    /// which is encoded as an `unsigned_varint`.
    /// The encoder further checks if the raw bytes packages exceeds the configured maximal message length.
    fn encode(&mut self, src: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // check length of package to send and return if package limit is exceeded
        let message_length = src.len();
        if message_length > self.max_message_len_bytes {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!(
                    "message with {message_length} bytes exceeds maximum of {} bytes",
                    self.max_message_len_bytes
                ),
            ));
        }

        // create varint for package length
        let mut uvi_buf = unsigned_varint::encode::usize_buffer();
        let encoded_length = unsigned_varint::encode::usize(message_length, &mut uvi_buf);

        // send varint encoded package length as a prefix
        dst.extend_from_slice(encoded_length);

        // send package bytes
        dst.extend_from_slice(&src);

        Ok(())
    }
}

impl Decoder for VarintPrefixedCodec {
    type Item = Vec<u8>;
    type Error = Error;

    /// Decode the incoming bytes, by checking the prefixed message length which is encoded
    /// as an `unsigned_varint`. Remove the length prefix and pass the raw bytes further as a `Vec<u8>`.
    /// The decoder further checks if the message length exceeds the configured maximal message length.
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // return if package is empty or smaller than the biggest
        // unsigned_varint encoding of an unsigned 64 bit integer.
        if src.len() <= 10 {
            return Ok(None);
        }

        // get `unsigned_varint` encoded message length
        let (message_length, remaining) = match unsigned_varint::decode::usize(src) {
            Ok((len, remaining)) => (len, remaining),
            Err(unsigned_varint::decode::Error::Insufficient) => return Ok(None),
            Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
        };

        // check if package length exceeds limit.
        if message_length > self.max_message_len_bytes {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!(
                    "message with {message_length} bytes exceeds maximum of {} bytes",
                    self.max_message_len_bytes
                ),
            ));
        }

        // how many bytes were used for the encoding?
        let varint_length = src.len() - remaining.len();

        // Ensure we can read an entire message.
        if src.len() < message_length + varint_length {
            return Ok(None);
        }

        // advance buffer for varint encoding size
        src.advance(varint_length);

        // return raw message bytes as Vec<u8>
        Ok(Some(src.split_to(message_length).freeze().to_vec()))
    }
}
