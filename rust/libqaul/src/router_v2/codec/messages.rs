use crate::router_v2::codec::{
    CodecError, Result, utils::{
        decode_indexes, encode_idx, fill_hop_bytes, read_array, read_u8, read_u16_be, unpack_hop_byte
    }
};

/// User/Node mappings for the current node
#[derive(Debug)]
pub struct Mapping {
    /// Absolute 16-bit index being introduced; the receiver binds this
    /// to `target_id` in its mirror dictionary for the sender (§3.5,
    /// §3.6). Stored resolved here; transmitted as a delta on the wire
    /// (§8.3).
    pub abs_idx: u16,
    /// the index representation of this entry in this node's routing table
    pub target_id: [u8; 8],
    /// the target's profile_version
    pub version: u32,
}

/// An actual entry in the routing table of a node
#[derive(Debug)]
pub struct Entry {
    /// denotes whether this entry has ever been propagated only over the Local shpere
    pub local_only: bool,
    /// the number of hops it'll take to reach the node
    pub hop_count: u8,
    /// denotes how good the connection to the node is. the lesser the better
    pub metric: u16,
    /// the sequence number accompanying this node for this entry
    pub seq: u16,
    /// Resolved absolute index in the entry's index space.
    pub abs_idx: u16,
}

/// The ROUTING_UPDATE wire message.
#[derive(Debug)]
pub struct RoutingUpdate {
    pub user_mappings: Vec<Mapping>,
    pub node_mappings: Vec<Mapping>,
    pub user_entries: Vec<Entry>,
    pub node_entries: Vec<Entry>,
}

impl RoutingUpdate {
    pub fn encode(&self, res: &mut Vec<u8>) -> Result<()> {
        res.push(self.user_mappings.len() as u8);
        let mut cursor = 0u16;
        for u in &self.user_mappings {
            encode_idx(u.abs_idx, &mut cursor, res);
            res.extend_from_slice(&u.target_id);
            res.extend_from_slice(&u.version.to_be_bytes());
        }

        // node mappings
        res.push(self.node_mappings.len() as u8);
        let mut cursor = 0u16;
        for n in &self.node_mappings {
            encode_idx(n.abs_idx, &mut cursor, res);
            res.extend_from_slice(&n.target_id);
            res.extend_from_slice(&n.version.to_be_bytes());
        }

        res.extend_from_slice(&(self.user_entries.len() as u16).to_be_bytes());
        let mut cursor = 0u16;
        for e in &self.user_entries {
            encode_idx(e.abs_idx, &mut cursor, res);
            res.extend_from_slice(&e.seq.to_be_bytes());
            res.extend_from_slice(&e.metric.to_be_bytes());
            res.push(fill_hop_bytes(e.hop_count, e.local_only));
        }

        res.extend_from_slice(&(self.node_entries.len() as u16).to_be_bytes());
        let mut cursor = 0u16;
        for ne in &self.node_entries {
            encode_idx(ne.abs_idx, &mut cursor, res);
            res.extend_from_slice(&ne.seq.to_be_bytes());
            res.extend_from_slice(&ne.metric.to_be_bytes());
            res.push(fill_hop_bytes(ne.hop_count, ne.local_only));
        }

        Ok(())
    }

    pub fn decode(msg: &[u8]) -> Result<RoutingUpdate> {
        let mut buf = msg;
        //user mappings
        let no_of_user_maps = read_u8(&mut buf)? as usize;
        let mut cursor = 0u16;
        let mut user_mappings = Vec::with_capacity(no_of_user_maps);
        for _ in 0..no_of_user_maps {
            let abs_idx = decode_indexes(&mut buf, &mut cursor)?;
            let target_id = read_array::<8>(&mut buf)?;
            let version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            user_mappings.push(Mapping {
                abs_idx,
                target_id,
                version,
            });
        }

        let no_of_node_maps = read_u8(&mut buf)? as usize;
        let mut cursor = 0u16;
        let mut node_mappings = Vec::with_capacity(no_of_node_maps);
        for _ in 0..no_of_node_maps {
            let abs_idx = decode_indexes(&mut buf, &mut cursor)?;
            let target_id = read_array::<8>(&mut buf)?;
            let version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            node_mappings.push(Mapping {
                abs_idx,
                target_id,
                version,
            });
        }

        //entries
        let n_user_entries = read_u16_be(&mut buf)? as usize;
        let mut cursor = 0u16;
        let mut user_entries = Vec::with_capacity(n_user_entries);
        for _ in 0..n_user_entries {
            let abs_idx = decode_indexes(&mut buf, &mut cursor)?;
            let seq = read_u16_be(&mut buf)?;
            let metric = read_u16_be(&mut buf)?;
            let hop_byte = read_u8(&mut buf)?;
            let (hop_count, local_only) = unpack_hop_byte(hop_byte);
            user_entries.push(Entry {
                abs_idx,
                seq,
                metric,
                hop_count,
                local_only,
            });
        }

        let n_node_entries = read_u16_be(&mut buf)? as usize;
        let mut cursor = 0u16;
        let mut node_entries = Vec::with_capacity(n_node_entries);
        for _ in 0..n_node_entries {
            let abs_idx = decode_indexes(&mut buf, &mut cursor)?;
            let seq = read_u16_be(&mut buf)?;
            let metric = read_u16_be(&mut buf)?;
            let hop_byte = read_u8(&mut buf)?;
            let (hop_count, local_only) = unpack_hop_byte(hop_byte);
            node_entries.push(Entry {
                abs_idx,
                seq,
                metric,
                hop_count,
                local_only,
            });
        }

        // to actually check and confirm that there are no trailing bytes
        if !buf.is_empty() {
            return Err(CodecError::Malformed);
        }

        Ok(RoutingUpdate {
            user_mappings,
            node_mappings,
            user_entries,
            node_entries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_mapping_eq(actual: &Mapping, expected: &Mapping) {
        assert_eq!(actual.abs_idx, expected.abs_idx, "abs_idx");
        assert_eq!(actual.target_id, expected.target_id, "target_id");
        assert_eq!(actual.version, expected.version, "version");
    }

    fn assert_entry_eq(actual: &Entry, expected: &Entry) {
        assert_eq!(actual.abs_idx, expected.abs_idx, "abs_idx");
        assert_eq!(actual.seq, expected.seq, "seq");
        assert_eq!(actual.metric, expected.metric, "metric");
        assert_eq!(actual.hop_count, expected.hop_count, "hop_count");
        assert_eq!(actual.local_only, expected.local_only, "local_only");
    }

    fn empty_routing_update() -> RoutingUpdate {
        RoutingUpdate {
            user_mappings: Vec::new(),
            node_mappings: Vec::new(),
            user_entries: Vec::new(),
            node_entries: Vec::new(),
        }
    }

    fn sample_mapping(idx: u16, id_byte: u8, version: u32) -> Mapping {
        Mapping {
            abs_idx: idx,
            target_id: [id_byte; 8],
            version,
        }
    }

    fn sample_entry(idx: u16, seq: u16, metric: u16, hop: u8, local: bool) -> Entry {
        Entry {
            abs_idx: idx,
            seq,
            metric,
            hop_count: hop,
            local_only: local,
        }
    }

    #[test]
    fn empty_routing_update_encodes_to_six_zero_bytes() {
        let ru = empty_routing_update();
        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        // n_user_mappings(u8) + n_node_mappings(u8)
        // + n_user_entries(u16) + n_node_entries(u16) = 6 zero bytes.
        assert_eq!(buf, vec![0x00; 6]);
    }

    #[test]
    fn empty_routing_update_round_trips() {
        let ru = empty_routing_update();
        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");
        assert!(decoded.user_mappings.is_empty());
        assert!(decoded.node_mappings.is_empty());
        assert!(decoded.user_entries.is_empty());
        assert!(decoded.node_entries.is_empty());
    }


    #[test]
    fn one_user_mapping_byte_layout() {
        let mut ru = empty_routing_update();
        ru.user_mappings.push(sample_mapping(5, 0xAB, 0x12345678));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");

        let expected: Vec<u8> = vec![
            0x01, // n_user_mappings = 1
            0x05, // delta from cursor 0 to 5
            0xAB, 0xAB, 0xAB, 0xAB, 0xAB, 0xAB, 0xAB, 0xAB, // target_id
            0x12, 0x34, 0x56, 0x78, // version (BE)
            0x00, // n_node_mappings = 0
            0x00, 0x00, // n_user_entries = 0
            0x00, 0x00, // n_node_entries = 0
        ];

        assert_eq!(buf, expected);
    }

    #[test]
    fn one_user_mapping_round_trips() {
        let mut ru = empty_routing_update();
        ru.user_mappings.push(sample_mapping(5, 0xAB, 0x12345678));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.user_mappings.len(), 1);
        assert_mapping_eq(&decoded.user_mappings[0], &ru.user_mappings[0]);
        assert!(decoded.node_mappings.is_empty());
        assert!(decoded.user_entries.is_empty());
        assert!(decoded.node_entries.is_empty());
    }

    #[test]
    fn one_node_mapping_round_trips() {
        let mut ru = empty_routing_update();
        ru.node_mappings.push(sample_mapping(7, 0xCD, 42));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.node_mappings.len(), 1);
        assert_mapping_eq(&decoded.node_mappings[0], &ru.node_mappings[0]);
        assert!(decoded.user_mappings.is_empty());
    }

    #[test]
    fn one_user_entry_byte_layout() {
        let mut ru = empty_routing_update();
        ru.user_entries.push(sample_entry(10, 100, 20, 5, true));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");

        // hop_byte: bit 7 (local_only) = 1, bits 0..=5 (hop_count) = 5
        // → 0b1000_0101 = 0x85.
        let expected: Vec<u8> = vec![
            0x00, // n_user_mappings
            0x00, // n_node_mappings
            0x00, 0x01, // n_user_entries = 1
            0x0A, // delta 0 -> 10
            0x00, 0x64, // seq = 100
            0x00, 0x14, // metric = 20
            0x85, // hop_byte
            0x00, 0x00, // n_node_entries = 0
        ];

        assert_eq!(buf, expected);
    }

    #[test]
    fn one_user_entry_round_trips() {
        let mut ru = empty_routing_update();
        ru.user_entries.push(sample_entry(10, 100, 20, 5, true));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.user_entries.len(), 1);
        assert_entry_eq(&decoded.user_entries[0], &ru.user_entries[0]);
    }

    #[test]
    fn one_node_entry_round_trips() {
        let mut ru = empty_routing_update();
        ru.node_entries.push(sample_entry(20, 200, 30, 10, false));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.node_entries.len(), 1);
        assert_entry_eq(&decoded.node_entries[0], &ru.node_entries[0]);
    }


    #[test]
    fn local_only_true_sets_bit_7_of_hop_byte() {
        let mut ru = empty_routing_update();
        ru.user_entries.push(sample_entry(1, 0, 0, 0, true));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");

        // hop_byte sits at index 9 (see one_user_entry_byte_layout for the breakdown).
        let hop_byte = buf[9];
        assert_eq!(
            hop_byte & 0b1000_0000,
            0b1000_0000,
            "bit 7 (local_only) must be set"
        );
        assert_eq!(
            hop_byte & 0b0100_0000,
            0,
            "bit 6 (reserved) must be zero on encode"
        );
        assert_eq!(hop_byte & 0b0011_1111, 0, "hop_count bits must be zero");
    }

    #[test]
    fn local_only_false_clears_bit_7_of_hop_byte() {
        let mut ru = empty_routing_update();
        ru.user_entries.push(sample_entry(1, 0, 0, 0, false));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");

        let hop_byte = buf[9];
        assert_eq!(
            hop_byte & 0b1000_0000,
            0,
            "bit 7 (local_only) must be clear"
        );
    }

    #[test]
    fn mixed_round_trip_with_escapes_in_every_section() {
        let ru = RoutingUpdate {
            user_mappings: vec![
                sample_mapping(5, 0x01, 1),
                sample_mapping(300, 0x02, 2), // gap > 255 → escape
            ],
            node_mappings: vec![
                sample_mapping(0, 0x03, 3), // first-at-zero → escape
                sample_mapping(10, 0x04, 4),
            ],
            user_entries: vec![
                sample_entry(1, 100, 200, 0, false),
                sample_entry(50_000, 300, 400, 63, true), // big gap + max hop count
            ],
            node_entries: vec![sample_entry(2, 500, 600, 31, false)],
        };

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.user_mappings.len(), ru.user_mappings.len());
        for (a, b) in decoded.user_mappings.iter().zip(ru.user_mappings.iter()) {
            assert_mapping_eq(a, b);
        }
        assert_eq!(decoded.node_mappings.len(), ru.node_mappings.len());
        for (a, b) in decoded.node_mappings.iter().zip(ru.node_mappings.iter()) {
            assert_mapping_eq(a, b);
        }
        assert_eq!(decoded.user_entries.len(), ru.user_entries.len());
        for (a, b) in decoded.user_entries.iter().zip(ru.user_entries.iter()) {
            assert_entry_eq(a, b);
        }
        assert_eq!(decoded.node_entries.len(), ru.node_entries.len());
        for (a, b) in decoded.node_entries.iter().zip(ru.node_entries.iter()) {
            assert_entry_eq(a, b);
        }
    }


    /// Every prefix shorter than the full encoded message must surface
    /// as an error without panicking.
    #[test]
    fn decode_truncation_never_panics() {
        let ru = RoutingUpdate {
            user_mappings: vec![sample_mapping(5, 0xAB, 1)],
            node_mappings: vec![sample_mapping(7, 0xCD, 2)],
            user_entries: vec![sample_entry(10, 100, 200, 5, true)],
            node_entries: vec![sample_entry(20, 300, 400, 10, false)],
        };
        let mut full = Vec::new();
        ru.encode(&mut full).expect("encode");

        for n in 0..full.len() {
            let prefix = &full[..n];
            let result = RoutingUpdate::decode(prefix);
            assert!(result.is_err(), "len {n}: expected Err, got {result:?}");
        }
    }

    /// With the strict trailing-bytes policy in decode, a fully-valid
    /// message followed by extra bytes must surface as Malformed.
    #[test]
    fn decode_trailing_bytes_returns_malformed() {
        let ru = empty_routing_update();
        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        buf.extend_from_slice(&[0xDE, 0xAD]); // junk

        match RoutingUpdate::decode(&buf) {
            Err(CodecError::Malformed) => {}
            other => panic!("expected Malformed, got {other:?}"),
        }
    }
}
