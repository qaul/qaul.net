// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! This file handles decoding and encoding the message types in the
//! protocol.

use crate::router_v2::codec::{
    utils::{
        decode_indexes, encode_idx, fill_hop_bytes, read_array, read_u16_be, read_u32_be,
        read_u64_be, read_u8, unpack_hop_byte,
    },
    CodecError, Result,
};

/// User/Node mappings for the current node
#[derive(Debug, Clone)]
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

/// An actual user entry in the routing table of a node
#[derive(Debug)]
pub struct UserEntry {
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

#[derive(Debug)]
pub struct NodeEntry {
    pub local_only: bool,
    pub hop_count: u8,
    pub metric: u16,
    pub seq: u16,
    pub abs_idx: u16,
    pub manifest_version: u32,
}

/// The ROUTING_UPDATE wire message.
#[derive(Debug)]
pub struct RoutingUpdate {
    pub user_mappings: Vec<Mapping>,
    pub node_mappings: Vec<Mapping>,
    pub user_entries: Vec<UserEntry>,
    pub node_entries: Vec<NodeEntry>,
}

/// The INDEX_DUMP wire message.
#[derive(Debug)]
pub struct IndexDump {
    pub user_mappings: Vec<Mapping>,
    pub node_mappings: Vec<Mapping>,
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
            res.extend_from_slice(&ne.manifest_version.to_be_bytes());
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
            user_entries.push(UserEntry {
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
            let manifest_version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            node_entries.push(NodeEntry {
                abs_idx,
                seq,
                metric,
                hop_count,
                local_only,
                manifest_version,
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

impl IndexDump {
    pub fn encode(&self, res: &mut Vec<u8>) -> Result<()> {
        res.extend_from_slice(&(self.user_mappings.len() as u16).to_be_bytes());
        let mut cursor = 0u16;
        for m in &self.user_mappings {
            encode_idx(m.abs_idx, &mut cursor, res);
            res.extend_from_slice(&m.target_id);
            res.extend_from_slice(&m.version.to_be_bytes());
        }

        res.extend_from_slice(&(self.node_mappings.len() as u16).to_be_bytes());
        let mut cursor = 0u16;
        for m in &self.node_mappings {
            encode_idx(m.abs_idx, &mut cursor, res);
            res.extend_from_slice(&m.target_id);
            res.extend_from_slice(&m.version.to_be_bytes());
        }

        Ok(())
    }

    pub fn decode(msg: &[u8]) -> Result<IndexDump> {
        let mut buf = msg;

        let n_user = read_u16_be(&mut buf)? as usize;
        let mut cursor = 0u16;
        let mut user_mappings = Vec::with_capacity(n_user);
        for _ in 0..n_user {
            let abs_idx = decode_indexes(&mut buf, &mut cursor)?;
            let target_id = read_array::<8>(&mut buf)?;
            let version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            user_mappings.push(Mapping {
                abs_idx,
                target_id,
                version,
            });
        }

        let n_node = read_u16_be(&mut buf)? as usize;
        let mut cursor = 0u16;
        let mut node_mappings = Vec::with_capacity(n_node);
        for _ in 0..n_node {
            let abs_idx = decode_indexes(&mut buf, &mut cursor)?;
            let target_id = read_array::<8>(&mut buf)?;
            let version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            node_mappings.push(Mapping {
                abs_idx,
                target_id,
                version,
            });
        }

        if !buf.is_empty() {
            return Err(CodecError::Malformed);
        }

        Ok(IndexDump {
            user_mappings,
            node_mappings,
        })
    }
}

/// an entry in a node's manifest
#[derive(Debug, Clone, Copy)]
pub struct ManifestEntry {
    /// the delegating user's 8-byte ID
    pub user_id: [u8; 8],
    /// milliseconds since Unix epoch
    pub timeout: u64,
    /// the signature signed by this user
    pub entry_signature: [u8; 64],
    pub profile_version: u32,
}

impl ManifestEntry {
    pub fn encode(&self, res: &mut Vec<u8>) {
        res.extend_from_slice(&self.user_id);
        res.extend_from_slice(&self.timeout.to_be_bytes());
        res.extend_from_slice(&self.entry_signature);
        res.extend_from_slice(&self.profile_version.to_be_bytes());
    }
}

/// the manifest that a node holds which keeps track of
/// all users that delegated to it
#[derive(Debug, Clone)]
pub struct NodeManifest {
    /// the index of the originatin node
    pub origin_node_id: [u8; 8],
    /// monotonic u32 counter, incremented on any change to the entries list
    pub manifest_version: u32,
    /// the index of the current chunk
    pub chunk_index: u8,
    pub chunk_count: u8,
    /// certain flags that give more details about this node, like,
    /// bit 0 is `is_gateway` and other bits are reserved
    pub flags: u8,
    /// the signature signed by the originating node
    pub manifest_signature: [u8; 64],
    /// the users delegated to this node
    pub entries: Vec<ManifestEntry>,
}

impl NodeManifest {
    pub fn encode(&self, res: &mut Vec<u8>) -> Result<()> {
        res.extend_from_slice(&self.origin_node_id);
        res.extend_from_slice(&self.manifest_version.to_be_bytes());
        res.push(self.chunk_index);
        res.push(self.chunk_count);
        res.push(self.flags);
        res.extend_from_slice(&self.manifest_signature);
        res.extend_from_slice(&(self.entries.len() as u16).to_be_bytes());
        self.entries.iter().for_each(|e| e.encode(res));
        Ok(())
    }

    pub fn decode(msg: &[u8]) -> Result<NodeManifest> {
        let mut buf = msg;

        let origin_node_id = read_array::<8>(&mut buf)?;
        let manifest_version = read_u32_be(&mut buf)?;
        let chunk_index = read_u8(&mut buf)?;
        let chunk_count = read_u8(&mut buf)?;
        let flags = read_u8(&mut buf)?;
        let manifest_signature = read_array::<64>(&mut buf)?;
        let no_of_entries = read_u16_be(&mut buf)? as usize;

        let mut entries = Vec::with_capacity(no_of_entries);
        for _ in 0..no_of_entries {
            let user_id = read_array::<8>(&mut buf)?;
            let timeout = read_u64_be(&mut buf)?;
            let entry_signature = read_array::<64>(&mut buf)?;
            let profile_version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            entries.push(ManifestEntry {
                user_id,
                timeout,
                entry_signature,
                profile_version,
            });
        }

        if !buf.is_empty() {
            return Err(CodecError::Malformed);
        }

        Ok(NodeManifest {
            origin_node_id,
            manifest_version,
            chunk_index,
            chunk_count,
            flags,
            manifest_signature,
            entries,
        })
    }
}

/// entries recorded that are yet to be announced
#[derive(Debug)]
pub struct DeltaAdd {
    pub record_version: u32,
    pub entry: ManifestEntry,
}

/// entries recorded that are yet to be announced
#[derive(Debug)]
pub struct DeltaRemove {
    pub user_id: [u8; 8],
    pub record_version: u32,
}

/// The `MANIFEST_DELTA` message carries an incremental update to a
/// manifest: a list of entries to add and a list of user IDs to remove.
#[derive(Debug)]
pub struct ManifestDelta {
    /// the index in the node index space of the host the manifest belongs to
    pub origin_node_id: [u8; 8],
    /// the `manifest_version` the delta builds upon.
    pub from_version: u32,
    /// the resulting `manifest_version` after applying the delta.
    pub to_version: u32,
    /// the one-byte manifest-flags field carrying the `is_gateway` bit and reserved bits.
    pub flags: u8,
    /// the host's ed25519 signature over the canonical encoding of the *resulting* full entry set.
    pub manifest_signature: [u8; 64],
    /// full entries to be updated
    pub adds: Vec<DeltaAdd>,
    /// the receiver removes the matching entry from its stored state.
    pub removes: Vec<DeltaRemove>,
}

impl ManifestDelta {
    pub fn encode(&self, data: &mut Vec<u8>) -> Result<()> {
        data.extend_from_slice(&self.origin_node_id);
        data.extend_from_slice(&self.from_version.to_be_bytes());
        data.extend_from_slice(&self.to_version.to_be_bytes());
        data.push(self.flags);
        data.extend_from_slice(&self.manifest_signature);
        data.extend_from_slice(&(self.adds.len() as u16).to_be_bytes());
        for a in &self.adds {
            data.extend_from_slice(&a.record_version.to_be_bytes());
            a.entry.encode(data);
        }
        data.extend_from_slice(&(self.removes.len() as u16).to_be_bytes());
        for r in &self.removes {
            data.extend_from_slice(&r.user_id);
            data.extend_from_slice(&r.record_version.to_be_bytes());
        }
        Ok(())
    }

    pub fn decode(msg: &[u8]) -> Result<ManifestDelta> {
        let mut buf = msg;
        let origin_node_id = read_array::<8>(&mut buf)?;
        let from_version = read_u32_be(&mut buf)?;
        let to_version = read_u32_be(&mut buf)?;
        let flags = read_u8(&mut buf)?;
        let manifest_signature = read_array::<64>(&mut buf)?;
        let no_of_additions = read_u16_be(&mut buf)?;

        let mut adds = Vec::with_capacity(no_of_additions as usize);
        for _ in 0..no_of_additions {
            let record_version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            let user_id = read_array::<8>(&mut buf)?;
            let timeout = read_u64_be(&mut buf)?;
            let entry_signature = read_array::<64>(&mut buf)?;
            let profile_version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            adds.push(DeltaAdd {
                record_version,
                entry: ManifestEntry {
                    user_id,
                    timeout,
                    entry_signature,
                    profile_version,
                },
            });
        }

        let no_of_removes = read_u16_be(&mut buf)?;
        let mut removes = Vec::with_capacity(no_of_removes as usize);
        for _ in 0..no_of_removes {
            let user_id = read_array::<8>(&mut buf)?;
            let record_version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            removes.push(DeltaRemove {
                user_id,
                record_version,
            });
        }

        if !buf.is_empty() {
            return Err(CodecError::Malformed);
        }

        let manifest_delta = ManifestDelta {
            origin_node_id,
            from_version,
            to_version,
            flags,
            manifest_signature,
            adds,
            removes,
        };

        Ok(manifest_delta)
    }
}

/// A single item in a MANIFEST_REQUEST. Identifies one origin
/// whose manifest state the sender wants to synchronise, and carries the
/// sender's committed `manifest_version` for that origin as `have_version`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestRequestItem {
    /// 8-byte ID of the origin whose manifest is being requested.
    pub origin_node_id: [u8; 8],
    /// Requester's committed `manifest_version` for the origin. Ignored
    /// by the responder when `have_none` is set.
    pub have_version: u32,
    /// Bit 0 = `have_none` (requester holds no manifest state — bootstrap).
    /// Bits 1-7 reserved, MUST be zero on send.
    pub item_flags: u8,
}

impl ManifestRequestItem {
    /// True if the requester declares "no state held for this origin"
    /// (§8.7 have_none bit).
    pub fn have_none(&self) -> bool {
        (self.item_flags & 0x01) != 0
    }
}

/// The MANIFEST_REQUEST wire message
#[derive(Debug, Clone)]
pub struct ManifestRequest {
    pub items: Vec<ManifestRequestItem>,
}

impl ManifestRequest {
    pub fn encode(&self, data: &mut Vec<u8>) -> Result<()> {
        // n_items is a u8 (spec caps items at 255).
        if self.items.len() > u8::MAX as usize {
            return Err(CodecError::Malformed);
        }
        data.push(self.items.len() as u8);
        for item in &self.items {
            data.extend_from_slice(&item.origin_node_id);
            data.extend_from_slice(&item.have_version.to_be_bytes());
            data.push(item.item_flags);
        }
        Ok(())
    }

    pub fn decode(msg: &[u8]) -> Result<ManifestRequest> {
        let mut buf = msg;
        let n_items = read_u8(&mut buf)?;

        let mut items = Vec::with_capacity(n_items as usize);
        for _ in 0..n_items {
            let origin_node_id = read_array::<8>(&mut buf)?;
            let have_version = u32::from_be_bytes(read_array::<4>(&mut buf)?);
            let item_flags = read_u8(&mut buf)?;
            // §8.7: reserved bits (1..=7) MUST be zero. Reject on send-side
            if item_flags & 0b1111_1110 != 0 {
                return Err(CodecError::Malformed);
            }
            items.push(ManifestRequestItem {
                origin_node_id,
                have_version,
                item_flags,
            });
        }

        if !buf.is_empty() {
            return Err(CodecError::Malformed);
        }

        Ok(ManifestRequest { items })
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

    fn assert_user_entry_eq(actual: &UserEntry, expected: &UserEntry) {
        assert_eq!(actual.abs_idx, expected.abs_idx, "abs_idx");
        assert_eq!(actual.seq, expected.seq, "seq");
        assert_eq!(actual.metric, expected.metric, "metric");
        assert_eq!(actual.hop_count, expected.hop_count, "hop_count");
        assert_eq!(actual.local_only, expected.local_only, "local_only");
    }

    fn assert_node_entry_eq(actual: &NodeEntry, expected: &NodeEntry) {
        assert_eq!(actual.abs_idx, expected.abs_idx, "abs_idx");
        assert_eq!(actual.seq, expected.seq, "seq");
        assert_eq!(actual.metric, expected.metric, "metric");
        assert_eq!(actual.hop_count, expected.hop_count, "hop_count");
        assert_eq!(actual.local_only, expected.local_only, "local_only");
        assert_eq!(
            actual.manifest_version, expected.manifest_version,
            "manifest_version",
        );
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

    fn sample_user_entry(idx: u16, seq: u16, metric: u16, hop: u8, local: bool) -> UserEntry {
        UserEntry {
            abs_idx: idx,
            seq,
            metric,
            hop_count: hop,
            local_only: local,
        }
    }

    fn sample_node_entry(
        idx: u16,
        seq: u16,
        metric: u16,
        hop: u8,
        local: bool,
        manifest_version: u32,
    ) -> NodeEntry {
        NodeEntry {
            abs_idx: idx,
            seq,
            metric,
            hop_count: hop,
            local_only: local,
            manifest_version,
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
        ru.user_entries
            .push(sample_user_entry(10, 100, 20, 5, true));

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
        ru.user_entries
            .push(sample_user_entry(10, 100, 20, 5, true));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.user_entries.len(), 1);
        assert_user_entry_eq(&decoded.user_entries[0], &ru.user_entries[0]);
    }

    /// Node entries are 4 bytes larger than user entries (trailing
    /// `manifest_version: u32` per §8.3 / §10.8). Byte-layout regression.
    #[test]
    fn one_node_entry_byte_layout() {
        let mut ru = empty_routing_update();
        ru.node_entries
            .push(sample_node_entry(20, 200, 30, 10, false, 0x12345678));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");

        // hop_byte: local_only=false → bit 7 = 0; hop_count=10 = 0x0A.
        let expected: Vec<u8> = vec![
            0x00, // n_user_mappings
            0x00, // n_node_mappings
            0x00, 0x00, // n_user_entries = 0
            0x00, 0x01, // n_node_entries = 1
            0x14, // delta 0 -> 20
            0x00, 0xC8, // seq = 200
            0x00, 0x1E, // metric = 30
            0x0A, // hop_byte
            0x12, 0x34, 0x56, 0x78, // manifest_version (BE)
        ];

        assert_eq!(buf, expected);
    }

    #[test]
    fn one_node_entry_round_trips() {
        let mut ru = empty_routing_update();
        ru.node_entries
            .push(sample_node_entry(20, 200, 30, 10, false, 42));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.node_entries.len(), 1);
        assert_node_entry_eq(&decoded.node_entries[0], &ru.node_entries[0]);
    }

    /// Node entries carry a distinct `manifest_version` — sending 0
    /// (no committed state for the target, per Phase 8 tick_relay)
    /// must round-trip as 0.
    #[test]
    fn node_entry_zero_manifest_version_round_trips() {
        let mut ru = empty_routing_update();
        ru.node_entries
            .push(sample_node_entry(5, 1, 1, 1, false, 0));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.node_entries[0].manifest_version, 0);
    }

    /// Node entries carry the target's committed `manifest_version` per
    /// §10.8. Verify the full u32 range round-trips (max value).
    #[test]
    fn node_entry_max_manifest_version_round_trips() {
        let mut ru = empty_routing_update();
        ru.node_entries
            .push(sample_node_entry(5, 1, 1, 1, false, u32::MAX));

        let mut buf = Vec::new();
        ru.encode(&mut buf).expect("encode");
        let decoded = RoutingUpdate::decode(&buf).expect("decode");

        assert_eq!(decoded.node_entries[0].manifest_version, u32::MAX);
    }

    #[test]
    fn local_only_true_sets_bit_7_of_hop_byte() {
        let mut ru = empty_routing_update();
        ru.user_entries.push(sample_user_entry(1, 0, 0, 0, true));

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
        ru.user_entries.push(sample_user_entry(1, 0, 0, 0, false));

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
                sample_user_entry(1, 100, 200, 0, false),
                sample_user_entry(50_000, 300, 400, 63, true), // big gap + max hop count
            ],
            node_entries: vec![sample_node_entry(2, 500, 600, 31, false, 7)],
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
            assert_user_entry_eq(a, b);
        }
        assert_eq!(decoded.node_entries.len(), ru.node_entries.len());
        for (a, b) in decoded.node_entries.iter().zip(ru.node_entries.iter()) {
            assert_node_entry_eq(a, b);
        }
    }

    /// Every prefix shorter than the full encoded message must surface
    /// as an error without panicking.
    #[test]
    fn decode_truncation_never_panics_routing_update() {
        let ru = RoutingUpdate {
            user_mappings: vec![sample_mapping(5, 0xAB, 1)],
            node_mappings: vec![sample_mapping(7, 0xCD, 2)],
            user_entries: vec![sample_user_entry(10, 100, 200, 5, true)],
            node_entries: vec![sample_node_entry(20, 300, 400, 10, false, 99)],
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

    fn assert_manifest_entry_eq(actual: &ManifestEntry, expected: &ManifestEntry) {
        assert_eq!(actual.user_id, expected.user_id, "user_id");
        assert_eq!(actual.timeout, expected.timeout, "timeout");
        assert_eq!(
            actual.entry_signature, expected.entry_signature,
            "entry_signature"
        );
        assert_eq!(
            actual.profile_version, expected.profile_version,
            "profile_version"
        );
    }

    fn assert_node_manifest_eq(actual: &NodeManifest, expected: &NodeManifest) {
        assert_eq!(
            actual.origin_node_id, expected.origin_node_id,
            "origin_node_id"
        );
        assert_eq!(
            actual.manifest_version, expected.manifest_version,
            "manifest_version"
        );
        assert_eq!(actual.chunk_index, expected.chunk_index, "chunk_index");
        assert_eq!(actual.chunk_count, expected.chunk_count, "chunk_count");
        assert_eq!(actual.flags, expected.flags, "flags");
        assert_eq!(
            actual.manifest_signature, expected.manifest_signature,
            "manifest_signature"
        );
        assert_eq!(
            actual.entries.len(),
            expected.entries.len(),
            "entries.len()"
        );
        for (a, e) in actual.entries.iter().zip(expected.entries.iter()) {
            assert_manifest_entry_eq(a, e);
        }
    }

    fn sample_manifest_entry(
        user_id_byte: u8,
        timeout: u64,
        sig_byte: u8,
        profile_version: u32,
    ) -> ManifestEntry {
        ManifestEntry {
            user_id: [user_id_byte; 8],
            timeout,
            entry_signature: [sig_byte; 64],
            profile_version,
        }
    }

    fn empty_node_manifest() -> NodeManifest {
        NodeManifest {
            origin_node_id: [0; 8],
            manifest_version: 0,
            chunk_index: 0,
            chunk_count: 1,
            flags: 0,
            manifest_signature: [0; 64],
            entries: Vec::new(),
        }
    }

    #[test]
    fn empty_node_manifest_encodes_to_81_byte_body() {
        let m = empty_node_manifest();
        let mut buf = Vec::new();
        m.encode(&mut buf).expect("encode");
        // origin_node_id(8) + manifest_version(4) + chunk_index(1)
        // + chunk_count(1) + flags(1) + manifest_signature(64)
        // + n_entries(2) = 81 bytes.
        // Spec §8.5 says "approximately 85 + 84·N" total body,
        // where the 85 includes the 4-byte common Header. Body alone
        // (no header) is 81 bytes when entries are empty.
        assert_eq!(buf.len(), 81);
    }

    #[test]
    fn empty_node_manifest_round_trips() {
        let m = NodeManifest {
            origin_node_id: [42; 8],
            manifest_version: 7,
            chunk_index: 0,
            chunk_count: 1,
            flags: 0x01,
            manifest_signature: [0xCC; 64],
            entries: Vec::new(),
        };

        let mut buf = Vec::new();
        m.encode(&mut buf).expect("encode");
        let decoded = NodeManifest::decode(&buf).expect("decode");

        assert_node_manifest_eq(&decoded, &m);
    }

    /// Hand-rolled byte fixture for a one-entry manifest. Verifies every
    /// field lands at the right offset on the wire.
    /// Per new spec §8.5: `origin_node_id` is 8 bytes (was 2-byte index);
    /// each entry is 84 bytes with the trailing `profile_version` (§10.1).
    #[test]
    fn one_entry_byte_layout() {
        let m = NodeManifest {
            origin_node_id: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
            manifest_version: 0x0A0B0C0D,
            chunk_index: 0x02,
            chunk_count: 0x03,
            flags: 0x01,
            manifest_signature: [0xAA; 64],
            entries: vec![ManifestEntry {
                user_id: [0x11; 8],
                timeout: 0x00_11_22_33_44_55_66_77,
                entry_signature: [0xBB; 64],
                profile_version: 0xDEAD_BEEF,
            }],
        };

        let mut buf = Vec::new();
        m.encode(&mut buf).expect("encode");

        // Assemble expected: origin_node_id(8) + manifest_version(4) +
        // chunk_index(1) + chunk_count(1) + flags(1) + manifest_signature(64)
        // + n_entries(2) + one 84-byte entry.
        let mut expected = Vec::new();
        expected.extend_from_slice(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]); // origin_node_id
        expected.extend_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D]); // manifest_version
        expected.push(0x02); // chunk_index
        expected.push(0x03); // chunk_count
        expected.push(0x01); // flags
        expected.extend_from_slice(&[0xAA; 64]); // manifest_signature
        expected.extend_from_slice(&[0x00, 0x01]); // n_entries = 1
                                                   // entry:
        expected.extend_from_slice(&[0x11; 8]); // user_id
        expected.extend_from_slice(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]); // timeout
        expected.extend_from_slice(&[0xBB; 64]); // entry_signature
        expected.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]); // profile_version

        assert_eq!(buf, expected);
        // Total: 8+4+1+1+1+64+2+84 = 165 bytes.
        assert_eq!(buf.len(), 165);
    }

    #[test]
    fn multi_entry_round_trip() {
        let m = NodeManifest {
            origin_node_id: [10; 8],
            manifest_version: 100,
            chunk_index: 0,
            chunk_count: 1,
            flags: 0x00,
            manifest_signature: [0xCC; 64],
            entries: vec![
                sample_manifest_entry(0x01, 1_000_000, 0xA1, 11),
                sample_manifest_entry(0x02, 2_000_000, 0xA2, 22),
                sample_manifest_entry(0x03, 3_000_000, 0xA3, 33),
            ],
        };

        let mut buf = Vec::new();
        m.encode(&mut buf).expect("encode");
        let decoded = NodeManifest::decode(&buf).expect("decode");

        assert_node_manifest_eq(&decoded, &m);
        // Belt-and-suspenders: also verify each entry differs from the
        // others, so a "decoded[0] == decoded[1] == decoded[2]" bug
        // (which is what the pre-fix code produced) is impossible to
        // pass.
        assert_ne!(decoded.entries[0].user_id, decoded.entries[1].user_id);
        assert_ne!(decoded.entries[1].user_id, decoded.entries[2].user_id);
        assert_ne!(
            decoded.entries[0].profile_version, decoded.entries[1].profile_version,
            "profile_version must round-trip per-entry"
        );
    }

    #[test]
    fn flags_is_gateway_round_trips() {
        let mut m = empty_node_manifest();
        m.flags = 0x01; // is_gateway bit set

        let mut buf = Vec::new();
        m.encode(&mut buf).expect("encode");
        let decoded = NodeManifest::decode(&buf).expect("decode");

        assert_eq!(decoded.flags, 0x01);
    }

    #[test]
    fn flags_reserved_bits_pass_through_codec_unchanged() {
        let mut m = empty_node_manifest();
        m.flags = 0xFF;

        let mut buf = Vec::new();
        m.encode(&mut buf).expect("encode");
        let decoded = NodeManifest::decode(&buf).expect("decode");

        assert_eq!(
            decoded.flags, 0xFF,
            "codec must preserve all bits; reserved-bit masking is the receiver's job"
        );
    }

    #[test]
    fn decode_truncation_never_panics() {
        let m = NodeManifest {
            origin_node_id: [42; 8],
            manifest_version: 7,
            chunk_index: 0,
            chunk_count: 1,
            flags: 0x01,
            manifest_signature: [0xCC; 64],
            entries: vec![sample_manifest_entry(0xAA, 12345, 0xDD, 5)],
        };
        let mut full = Vec::new();
        m.encode(&mut full).expect("encode");

        for n in 0..full.len() {
            let prefix = &full[..n];
            let result = NodeManifest::decode(prefix);
            assert!(result.is_err(), "len {n}: expected Err, got {result:?}");
        }
    }

    #[test]
    fn decode_trailing_bytes_returns_malformed_node_manifest() {
        let m = empty_node_manifest();
        let mut buf = Vec::new();
        m.encode(&mut buf).expect("encode");
        buf.extend_from_slice(&[0xDE, 0xAD]); // junk

        match NodeManifest::decode(&buf) {
            Err(CodecError::Malformed) => {}
            other => panic!("expected Malformed, got {other:?}"),
        }
    }

    // ---------- ManifestRequest ----------

    fn sample_request_item(id_byte: u8, have_version: u32, have_none: bool) -> ManifestRequestItem {
        ManifestRequestItem {
            origin_node_id: [id_byte; 8],
            have_version,
            item_flags: if have_none { 0x01 } else { 0x00 },
        }
    }

    #[test]
    fn manifest_request_round_trip_empty_items() {
        let msg = ManifestRequest { items: Vec::new() };
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();
        // Just the u8 n_items = 0.
        assert_eq!(buf, vec![0x00]);
        let decoded = ManifestRequest::decode(&buf).unwrap();
        assert!(decoded.items.is_empty());
    }

    #[test]
    fn manifest_request_round_trip_single_item_with_version() {
        let msg = ManifestRequest {
            items: vec![sample_request_item(1, 42, false)],
        };
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();
        // 1 byte n_items + 13 bytes/item = 14
        assert_eq!(buf.len(), 14);
        let decoded = ManifestRequest::decode(&buf).unwrap();
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.items[0].origin_node_id, [1; 8]);
        assert_eq!(decoded.items[0].have_version, 42);
        assert!(!decoded.items[0].have_none());
    }

    #[test]
    fn manifest_request_round_trip_have_none_bit() {
        let msg = ManifestRequest {
            items: vec![sample_request_item(5, 0, true)],
        };
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();
        let decoded = ManifestRequest::decode(&buf).unwrap();
        assert!(decoded.items[0].have_none());
    }

    #[test]
    fn manifest_request_round_trip_many_items() {
        let items: Vec<ManifestRequestItem> = (0..5)
            .map(|i| sample_request_item(i as u8, i * 10, i % 2 == 0))
            .collect();
        let msg = ManifestRequest {
            items: items.clone(),
        };
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();
        // 1 + 5 * 13 = 66
        assert_eq!(buf.len(), 66);
        let decoded = ManifestRequest::decode(&buf).unwrap();
        assert_eq!(decoded.items, items);
    }

    /// Reserved bits (1..=7) MUST be zero per §8.7. Non-zero rejected.
    #[test]
    fn manifest_request_decode_rejects_reserved_bits() {
        let mut buf = Vec::new();
        buf.push(0x01); // n_items = 1
        buf.extend_from_slice(&[7u8; 8]); // origin_node_id
        buf.extend_from_slice(&0u32.to_be_bytes()); // have_version
        buf.push(0b0000_0010); // reserved bit set → malformed
        match ManifestRequest::decode(&buf) {
            Err(CodecError::Malformed) => {}
            other => panic!("expected Malformed, got {other:?}"),
        }
    }

    #[test]
    fn manifest_request_encode_rejects_over_255_items() {
        let msg = ManifestRequest {
            items: vec![sample_request_item(0, 0, false); 256],
        };
        let mut buf = Vec::new();
        match msg.encode(&mut buf) {
            Err(CodecError::Malformed) => {}
            other => panic!("expected Malformed, got {other:?}"),
        }
    }

    #[test]
    fn manifest_request_decode_truncated_returns_short() {
        // 1 byte n_items = 3, followed by only one full item's worth of bytes.
        let mut buf = Vec::new();
        buf.push(3);
        buf.extend_from_slice(&[1u8; 8]);
        buf.extend_from_slice(&42u32.to_be_bytes());
        buf.push(0x00);
        // Missing the other two items entirely.
        match ManifestRequest::decode(&buf) {
            Err(CodecError::Short) => {}
            other => panic!("expected Short, got {other:?}"),
        }
    }

    #[test]
    fn manifest_request_decode_trailing_junk_returns_malformed() {
        let msg = ManifestRequest {
            items: vec![sample_request_item(1, 1, false)],
        };
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();
        buf.push(0xFF); // trailing byte
        match ManifestRequest::decode(&buf) {
            Err(CodecError::Malformed) => {}
            other => panic!("expected Malformed, got {other:?}"),
        }
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::router_v2::codec::utils::{decode_indexes, encode_indexes};
    use proptest::prelude::*;

    fn arb_signature() -> impl Strategy<Value = [u8; 64]> {
        prop::collection::vec(any::<u8>(), 64..=64).prop_map(|v| {
            let mut arr = [0u8; 64];
            arr.copy_from_slice(&v);
            arr
        })
    }

    fn arb_mapping_vec(max: usize) -> impl Strategy<Value = Vec<Mapping>> {
        prop::collection::vec((any::<u16>(), any::<[u8; 8]>(), any::<u32>()), 0..=max).prop_map(
            |mut items| {
                items.sort_by_key(|(idx, _, _)| *idx);
                items.dedup_by_key(|(idx, _, _)| *idx);
                items
                    .into_iter()
                    .map(|(abs_idx, target_id, version)| Mapping {
                        abs_idx,
                        target_id,
                        version,
                    })
                    .collect()
            },
        )
    }

    fn arb_user_entry_vec(max: usize) -> impl Strategy<Value = Vec<UserEntry>> {
        prop::collection::vec(
            (
                any::<u16>(),
                any::<u16>(),
                any::<u16>(),
                any::<u8>(),
                any::<bool>(),
            ),
            0..=max,
        )
        .prop_map(|mut items| {
            items.sort_by_key(|(idx, _, _, _, _)| *idx);
            items.dedup_by_key(|(idx, _, _, _, _)| *idx);
            items
                .into_iter()
                .map(|(abs_idx, seq, metric, hop, local_only)| UserEntry {
                    abs_idx,
                    seq,
                    metric,
                    hop_count: hop & 0b0011_1111,
                    local_only,
                })
                .collect()
        })
    }

    fn arb_node_entry_vec(max: usize) -> impl Strategy<Value = Vec<NodeEntry>> {
        prop::collection::vec(
            (
                any::<u16>(),
                any::<u16>(),
                any::<u16>(),
                any::<u8>(),
                any::<bool>(),
                any::<u32>(),
            ),
            0..=max,
        )
        .prop_map(|mut items| {
            items.sort_by_key(|(idx, _, _, _, _, _)| *idx);
            items.dedup_by_key(|(idx, _, _, _, _, _)| *idx);
            items
                .into_iter()
                .map(
                    |(abs_idx, seq, metric, hop, local_only, manifest_version)| NodeEntry {
                        abs_idx,
                        seq,
                        metric,
                        hop_count: hop & 0b0011_1111,
                        local_only,
                        manifest_version,
                    },
                )
                .collect()
        })
    }

    fn arb_manifest_entry_vec(max: usize) -> impl Strategy<Value = Vec<ManifestEntry>> {
        prop::collection::vec(
            (
                any::<[u8; 8]>(),
                any::<u64>(),
                arb_signature(),
                any::<u32>(),
            )
                .prop_map(|(user_id, timeout, entry_signature, profile_version)| {
                    ManifestEntry {
                        user_id,
                        timeout,
                        entry_signature,
                        profile_version,
                    }
                }),
            0..=max,
        )
    }

    fn arb_delta_add_vec(max: usize) -> impl Strategy<Value = Vec<DeltaAdd>> {
        prop::collection::vec(
            (
                any::<u32>(),
                any::<[u8; 8]>(),
                any::<u64>(),
                arb_signature(),
                any::<u32>(),
            )
                .prop_map(
                    |(record_version, user_id, timeout, entry_signature, profile_version)| {
                        DeltaAdd {
                            record_version,
                            entry: ManifestEntry {
                                user_id,
                                timeout,
                                entry_signature,
                                profile_version,
                            },
                        }
                    },
                ),
            0..=max,
        )
    }

    fn arb_delta_remove_vec(max: usize) -> impl Strategy<Value = Vec<DeltaRemove>> {
        prop::collection::vec(
            (any::<[u8; 8]>(), any::<u32>()).prop_map(|(user_id, record_version)| DeltaRemove {
                user_id,
                record_version,
            }),
            0..=max,
        )
    }

    proptest! {
        #[test]
        fn delta_encode_decode_round_trips(
            input in prop::collection::vec(any::<u16>(), 0..200)
        ) {
            let mut sorted = input;
            sorted.sort();
            sorted.dedup();

            let mut bytes = Vec::new();
            encode_indexes(&sorted, &mut bytes);

            let mut buf: &[u8] = &bytes;
            let mut cursor = 0u16;
            let mut decoded = Vec::with_capacity(sorted.len());
            for _ in 0..sorted.len() {
                decoded.push(decode_indexes(&mut buf, &mut cursor).unwrap());
            }

            prop_assert_eq!(decoded, sorted);
            prop_assert!(buf.is_empty());
        }

        #[test]
        fn routing_update_round_trips(
            user_mappings in arb_mapping_vec(50),
            node_mappings in arb_mapping_vec(50),
            user_entries in arb_user_entry_vec(50),
            node_entries in arb_node_entry_vec(50),
        ) {
            let ru = RoutingUpdate { user_mappings, node_mappings, user_entries, node_entries };

            let mut buf = Vec::new();
            ru.encode(&mut buf).unwrap();
            let decoded = RoutingUpdate::decode(&buf).unwrap();

            prop_assert_eq!(decoded.user_mappings.len(), ru.user_mappings.len());
            prop_assert_eq!(decoded.node_mappings.len(), ru.node_mappings.len());
            prop_assert_eq!(decoded.user_entries.len(), ru.user_entries.len());
            prop_assert_eq!(decoded.node_entries.len(), ru.node_entries.len());

            for (a, b) in decoded.user_mappings.iter().zip(ru.user_mappings.iter()) {
                prop_assert_eq!(a.abs_idx, b.abs_idx);
                prop_assert_eq!(a.target_id, b.target_id);
                prop_assert_eq!(a.version, b.version);
            }
            for (a, b) in decoded.node_mappings.iter().zip(ru.node_mappings.iter()) {
                prop_assert_eq!(a.abs_idx, b.abs_idx);
                prop_assert_eq!(a.target_id, b.target_id);
                prop_assert_eq!(a.version, b.version);
            }
            for (a, b) in decoded.user_entries.iter().zip(ru.user_entries.iter()) {
                prop_assert_eq!(a.abs_idx, b.abs_idx);
                prop_assert_eq!(a.seq, b.seq);
                prop_assert_eq!(a.metric, b.metric);
                prop_assert_eq!(a.hop_count, b.hop_count);
                prop_assert_eq!(a.local_only, b.local_only);
            }
            for (a, b) in decoded.node_entries.iter().zip(ru.node_entries.iter()) {
                prop_assert_eq!(a.abs_idx, b.abs_idx);
                prop_assert_eq!(a.seq, b.seq);
                prop_assert_eq!(a.metric, b.metric);
                prop_assert_eq!(a.hop_count, b.hop_count);
                prop_assert_eq!(a.local_only, b.local_only);
                prop_assert_eq!(a.manifest_version, b.manifest_version);
            }
        }

        #[test]
        fn index_dump_round_trips(
            user_mappings in arb_mapping_vec(200),
            node_mappings in arb_mapping_vec(200),
        ) {
            let dump = IndexDump { user_mappings, node_mappings };
            let mut buf = Vec::new();
            dump.encode(&mut buf).unwrap();
            let decoded = IndexDump::decode(&buf).unwrap();

            prop_assert_eq!(decoded.user_mappings.len(), dump.user_mappings.len());
            prop_assert_eq!(decoded.node_mappings.len(), dump.node_mappings.len());
            for (a, b) in decoded.user_mappings.iter().zip(dump.user_mappings.iter()) {
                prop_assert_eq!(a.abs_idx, b.abs_idx);
                prop_assert_eq!(a.target_id, b.target_id);
                prop_assert_eq!(a.version, b.version);
            }
            for (a, b) in decoded.node_mappings.iter().zip(dump.node_mappings.iter()) {
                prop_assert_eq!(a.abs_idx, b.abs_idx);
                prop_assert_eq!(a.target_id, b.target_id);
                prop_assert_eq!(a.version, b.version);
            }
        }

        #[test]
        fn node_manifest_round_trips(
            origin_node_id in any::<[u8; 8]>(),
            manifest_version in any::<u32>(),
            chunk_index in any::<u8>(),
            chunk_count in 1u8..=255u8,
            flags in any::<u8>(),
            manifest_signature in arb_signature(),
            entries in arb_manifest_entry_vec(30),
        ) {
            let m = NodeManifest {
                origin_node_id, manifest_version, chunk_index, chunk_count,
                flags, manifest_signature, entries,
            };

            let mut buf = Vec::new();
            m.encode(&mut buf).unwrap();
            let decoded = NodeManifest::decode(&buf).unwrap();

            prop_assert_eq!(decoded.origin_node_id, m.origin_node_id);
            prop_assert_eq!(decoded.manifest_version, m.manifest_version);
            prop_assert_eq!(decoded.chunk_index, m.chunk_index);
            prop_assert_eq!(decoded.chunk_count, m.chunk_count);
            prop_assert_eq!(decoded.flags, m.flags);
            prop_assert_eq!(decoded.manifest_signature, m.manifest_signature);
            prop_assert_eq!(decoded.entries.len(), m.entries.len());
            for (a, b) in decoded.entries.iter().zip(m.entries.iter()) {
                prop_assert_eq!(a.user_id, b.user_id);
                prop_assert_eq!(a.timeout, b.timeout);
                prop_assert_eq!(a.entry_signature, b.entry_signature);
                prop_assert_eq!(a.profile_version, b.profile_version);
            }
        }

        #[test]
        fn manifest_delta_round_trips(
            origin_node_id in any::<[u8; 8]>(),
            from_version in any::<u32>(),
            to_version in any::<u32>(),
            flags in any::<u8>(),
            manifest_signature in arb_signature(),
            adds in arb_delta_add_vec(30),
            removes in arb_delta_remove_vec(30),
        ) {
            let d = ManifestDelta {
                origin_node_id, from_version, to_version, flags,
                manifest_signature, adds, removes,
            };

            let mut buf = Vec::new();
            d.encode(&mut buf).unwrap();
            let decoded = ManifestDelta::decode(&buf).unwrap();

            prop_assert_eq!(decoded.origin_node_id, d.origin_node_id);
            prop_assert_eq!(decoded.from_version, d.from_version);
            prop_assert_eq!(decoded.to_version, d.to_version);
            prop_assert_eq!(decoded.flags, d.flags);
            prop_assert_eq!(decoded.manifest_signature, d.manifest_signature);
            prop_assert_eq!(decoded.adds.len(), d.adds.len());
            for (a, b) in decoded.adds.iter().zip(d.adds.iter()) {
                prop_assert_eq!(a.record_version, b.record_version);
                prop_assert_eq!(a.entry.user_id, b.entry.user_id);
                prop_assert_eq!(a.entry.timeout, b.entry.timeout);
                prop_assert_eq!(a.entry.entry_signature, b.entry.entry_signature);
                prop_assert_eq!(a.entry.profile_version, b.entry.profile_version);
            }
            prop_assert_eq!(decoded.removes.len(), d.removes.len());
            for (a, b) in decoded.removes.iter().zip(d.removes.iter()) {
                prop_assert_eq!(a.user_id, b.user_id);
                prop_assert_eq!(a.record_version, b.record_version);
            }
        }
    }

    proptest! {
        /// Arbitrary byte sequences must produce a CodecError or success
        /// — NEVER panic. The load-bearing security property: routing
        /// messages come from untrusted peers, and a panic on bad input
        /// would crash the node.
        #[test]
        fn routing_update_decode_never_panics(
            bytes in prop::collection::vec(any::<u8>(), 0..4096)
        ) {
            let _ = RoutingUpdate::decode(&bytes);
        }

        #[test]
        fn index_dump_decode_never_panics(
            bytes in prop::collection::vec(any::<u8>(), 0..4096)
        ) {
            let _ = IndexDump::decode(&bytes);
        }

        #[test]
        fn node_manifest_decode_never_panics(
            bytes in prop::collection::vec(any::<u8>(), 0..4096)
        ) {
            let _ = NodeManifest::decode(&bytes);
        }

        #[test]
        fn manifest_delta_decode_never_panics(
            bytes in prop::collection::vec(any::<u8>(), 0..4096)
        ) {
            let _ = ManifestDelta::decode(&bytes);
        }

        #[test]
        fn manifest_request_decode_never_panics(
            bytes in prop::collection::vec(any::<u8>(), 0..4096)
        ) {
            let _ = ManifestRequest::decode(&bytes);
        }

        #[test]
        fn delta_decode_indexes_never_panics(
            bytes in prop::collection::vec(any::<u8>(), 0..1024),
            initial_cursor in any::<u16>(),
        ) {
            let mut buf: &[u8] = &bytes;
            let mut cursor = initial_cursor;
            for _ in 0..256 {
                if decode_indexes(&mut buf, &mut cursor).is_err() {
                    break;
                }
                if buf.is_empty() {
                    break;
                }
            }
        }
    }
}
