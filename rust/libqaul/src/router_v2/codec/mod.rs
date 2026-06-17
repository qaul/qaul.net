pub mod messages;
pub mod utils;

pub type Result<T> = std::result::Result<T, CodecError>;

pub const PROTOCOL_VERSION: u8 = 0x01;
/// Routing message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RoutingMessage {
    RoutingUpdate = 0x01,
    IndexDump = 0x02,
    NodeManifest = 0x03,
    ManifestDelta = 0x04,
}

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("input ended before the expected number of bytes")]
    Short,
    #[error("type byte didn't match any variant. got: {0}")]
    UnknownType(u8),
    #[error("an error occured")]
    Malformed,
    #[error("version mismatch. expected {PROTOCOL_VERSION} got {version}")]
    BadVersion { version: u8, payload_len: u16 },
    // check spec section 8.3 for what I mean here
    #[error("a reserved bit was set on input")]
    BadHopByte,
}

#[derive(Debug)]
/// Generic header for all wire messages
pub struct Header {
    pub version: u8,
    pub message_type: RoutingMessage,
    pub payload_len: u16,
}

impl Header {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(self.version);
        buf.push(self.message_type as u8);
        buf.extend_from_slice(&self.payload_len.to_be_bytes());
    }

    pub fn decode(buf: &[u8]) -> Result<(Header, &[u8])> {
        if buf.len() < 4 {
            return Err(CodecError::Short);
        }

        let version = buf[0];
        if version != PROTOCOL_VERSION {
            let payload_len = u16::from_be_bytes([buf[2], buf[3]]);
            return Err(CodecError::BadVersion {
                version,
                payload_len,
            });
        }

        let message_type = match buf[1] {
            1 => RoutingMessage::RoutingUpdate,
            2 => RoutingMessage::IndexDump,
            3 => RoutingMessage::NodeManifest,
            4 => RoutingMessage::ManifestDelta,
            _ => return Err(CodecError::UnknownType(buf[1])),
        };

        let header = Header {
            version,
            message_type,
            payload_len: u16::from_be_bytes([buf[2], buf[3]]),
        };
        Ok((header, &buf[4..]))
    }
}

#[cfg(test)]
mod tests {
    use crate::router_v2::codec::utils::*;
    use super::*;

    #[test]
    fn header_round_trips_for_each_message_type() {
        let variants = [
            RoutingMessage::RoutingUpdate,
            RoutingMessage::IndexDump,
            RoutingMessage::NodeManifest,
            RoutingMessage::ManifestDelta,
        ];

        for ty in variants {
            let h = Header {
                version: PROTOCOL_VERSION,
                message_type: ty,
                payload_len: 1234,
            };

            let mut buf = Vec::new();
            h.encode(&mut buf);
            assert_eq!(buf.len(), 4, "header always serialises to 4 bytes");

            let (decoded, rest) = Header::decode(&buf).expect("decode succeeds");
            assert_eq!(decoded.version, PROTOCOL_VERSION);
            assert_eq!(decoded.message_type, ty);
            assert_eq!(decoded.payload_len, 1234);
            assert!(
                rest.is_empty(),
                "no bytes left after consuming a bare header"
            );
        }
    }

    #[test]
    fn header_decode_returns_remaining_body_slice() {
        let h = Header {
            version: PROTOCOL_VERSION,
            message_type: RoutingMessage::RoutingUpdate,
            payload_len: 3,
        };

        let mut buf = Vec::new();
        h.encode(&mut buf);
        // Append a synthetic body so we can verify the body slice is
        // returned and not the whole input.
        buf.extend_from_slice(&[0xAA, 0xBB, 0xCC]);

        let (_, body) = Header::decode(&buf).expect("decode succeeds");
        assert_eq!(body, &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn header_decode_short_input_returns_short_error() {
        let h = Header {
            version: PROTOCOL_VERSION,
            message_type: RoutingMessage::RoutingUpdate,
            payload_len: 0,
        };
        let mut full = Vec::new();
        h.encode(&mut full);

        for n in 0..4 {
            let truncated = &full[..n];
            match Header::decode(truncated) {
                Err(CodecError::Short) => {}
                other => panic!("len {n}: expected Short, got {other:?}"),
            }
        }
    }

    #[test]
    fn header_decode_unknown_type_byte_returns_unknown_type() {
        // Hand-craft 4 bytes with a known good version but a bogus type.
        let bytes = [PROTOCOL_VERSION, 0x7F, 0x00, 0x00];

        match Header::decode(&bytes) {
            Err(CodecError::UnknownType(v)) => assert_eq!(v, 0x7F),
            other => panic!("expected UnknownType(0x7F), got {other:?}"),
        }
    }

    #[test]
    fn header_decode_unknown_version_returns_bad_version_with_payload_len() {
        // version=0xFE, type=0x01, payload_len=0x12_34 = 4660.
        let bytes = [0xFE, 0x01, 0x12, 0x34];

        match Header::decode(&bytes) {
            Err(CodecError::BadVersion {
                version,
                payload_len,
            }) => {
                assert_eq!(version, 0xFE);
                assert_eq!(
                    payload_len, 4660,
                    "receive loop needs payload_len to skip forward-incompatible messages"
                );
            }
            other => panic!("expected BadVersion {{ .. }}, got {other:?}"),
        }
    }

    /// Version is checked before the type byte; an unknown version with
    /// an otherwise-bogus type still surfaces as BadVersion (not
    /// UnknownType). Pins the decode check order.
    #[test]
    fn header_decode_bad_version_short_circuits_unknown_type_check() {
        let bytes = [0xFE, 0x7F, 0x00, 0x00];

        match Header::decode(&bytes) {
            Err(CodecError::BadVersion { version, .. }) => assert_eq!(version, 0xFE),
            other => panic!("BadVersion must short-circuit; got {other:?}"),
        }
    }

    /// Helper: decode a section of `n` indexes from the front of `bytes`,
    /// returning the resolved absolute indexes and the remaining slice.
    fn decode_n_indexes(bytes: &[u8], n: usize) -> Result<(Vec<u16>, &[u8])> {
        let mut buf = bytes;
        let mut cursor: u16 = 0;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(decode_indexes(&mut buf, &mut cursor)?);
        }
        Ok((out, buf))
    }

    #[test]
    fn delta_encode_empty_produces_no_bytes() {
        let mut out = Vec::new();
        encode_indexes(&[], &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn delta_encode_single_small_idx_is_one_byte_delta() {
        let mut out = Vec::new();
        encode_indexes(&[5], &mut out);
        assert_eq!(out, vec![0x05]);
    }

    #[test]
    fn delta_encode_first_entry_at_zero_uses_escape() {
        let mut out = Vec::new();
        encode_indexes(&[0], &mut out);
        assert_eq!(out, vec![0x00, 0x00, 0x00]);
    }

    #[test]
    fn delta_encode_first_entry_above_255_uses_escape() {
        let mut out = Vec::new();
        encode_indexes(&[300], &mut out);
        // 300 = 0x012C in big-endian.
        assert_eq!(out, vec![0x00, 0x01, 0x2C]);
    }

    #[test]
    fn delta_encode_sequential_small_deltas() {
        let mut out = Vec::new();
        encode_indexes(&[5, 8, 9, 100], &mut out);
        // Cursor starts at 0:
        //   5 -> delta 5
        //   8 -> delta 3
        //   9 -> delta 1
        //   100 -> delta 91 = 0x5B
        assert_eq!(out, vec![0x05, 0x03, 0x01, 0x5B]);
    }

    #[test]
    fn delta_encode_gap_over_255_uses_escape() {
        let mut out = Vec::new();
        encode_indexes(&[5, 300], &mut out);
        assert_eq!(out, vec![0x05, 0x00, 0x01, 0x2C]);
    }

    #[test]
    fn delta_decode_single_small_idx() {
        let bytes = [0x05];
        let (decoded, rest) = decode_n_indexes(&bytes, 1).expect("decode");
        assert_eq!(decoded, vec![5]);
        assert!(rest.is_empty());
    }

    #[test]
    fn delta_decode_escape_form_resolves_absolute() {
        let bytes = [0x00, 0x01, 0x2C];
        let (decoded, rest) = decode_n_indexes(&bytes, 1).expect("decode");
        assert_eq!(decoded, vec![300]);
        assert!(rest.is_empty());
    }

    #[test]
    fn delta_round_trip_mixed_inputs() {
        let input: Vec<u16> = vec![0, 5, 100, 300, 1000, 65000];
        let mut bytes = Vec::new();
        encode_indexes(&input, &mut bytes);

        let (decoded, rest) = decode_n_indexes(&bytes, input.len()).expect("decode");
        assert_eq!(decoded, input);
        assert!(rest.is_empty(), "no bytes left after consuming all indexes");
    }

    #[test]
    fn delta_decode_truncation_never_panics() {
        // Encoded form of [5, 300]: [0x05, 0x00, 0x01, 0x2C].
        // First index reads 1 byte; second index reads 3 bytes (escape).
        let input: Vec<u16> = vec![5, 300];
        let mut bytes = Vec::new();
        encode_indexes(&input, &mut bytes);
        assert_eq!(bytes.len(), 4);

        // Length 0: first read fails immediately.
        let mut buf: &[u8] = &bytes[..0];
        let mut cursor: u16 = 0;
        assert!(matches!(
            decode_indexes(&mut buf, &mut cursor),
            Err(CodecError::Short)
        ));

        // Length 1: first read succeeds (5), second fails (escape needs 3).
        let mut buf: &[u8] = &bytes[..1];
        let mut cursor: u16 = 0;
        assert_eq!(decode_indexes(&mut buf, &mut cursor).unwrap(), 5);
        assert!(matches!(
            decode_indexes(&mut buf, &mut cursor),
            Err(CodecError::Short)
        ));

        // Length 2: first succeeds, second sees only 2 bytes of escape, fails.
        let mut buf: &[u8] = &bytes[..2];
        let mut cursor: u16 = 0;
        assert_eq!(decode_indexes(&mut buf, &mut cursor).unwrap(), 5);
        assert!(matches!(
            decode_indexes(&mut buf, &mut cursor),
            Err(CodecError::Short)
        ));

        // Length 3: same — escape needs the 0x00 plus 2 following bytes.
        let mut buf: &[u8] = &bytes[..3];
        let mut cursor: u16 = 0;
        assert_eq!(decode_indexes(&mut buf, &mut cursor).unwrap(), 5);
        assert!(matches!(
            decode_indexes(&mut buf, &mut cursor),
            Err(CodecError::Short)
        ));

        // Length 4 (full): both reads succeed.
        let mut buf: &[u8] = &bytes[..4];
        let mut cursor: u16 = 0;
        assert_eq!(decode_indexes(&mut buf, &mut cursor).unwrap(), 5);
        assert_eq!(decode_indexes(&mut buf, &mut cursor).unwrap(), 300);
        assert!(buf.is_empty());
    }
}
