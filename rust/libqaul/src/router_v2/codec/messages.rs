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
