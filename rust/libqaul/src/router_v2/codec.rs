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

    pub fn decode(buf: &[u8]) -> Result<(Header, &[u8]), CodecError> {
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
}
