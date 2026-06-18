use crate::router_v2::codec::CodecError;

// packs the hop_count and local_only flag into one single byte. bit 6 is reserved for now
pub fn fill_hop_bytes(hop_count: u8, local_only: bool) -> u8 {
    // corece the hop count to fit into six bytes. which means the max is 63, as is described in the spec
    // that os bit 0 -5
    let inital_five_bits = hop_count & 0b0011_1111;
    // that is bit 7. since the spec says the bit 6, should be reserved.
    let final_bit = (local_only as u8) << 7;
    final_bit | inital_five_bits
}

pub fn encode_idx(i: u16, cursor: &mut u16, res: &mut Vec<u8>) {
    let gap = i.wrapping_sub(*cursor);
    if gap >= 1 && gap <= 255 {
        res.push(gap as u8);
    } else {
        res.push(0x00);
        res.extend_from_slice(&i.to_be_bytes());
    }
    *cursor = i;
}

/// Each index is written as either:
/// - a 1-byte delta `0x01..=0xFF` relative to the running cursor, or
/// - a 3-byte escape (`0x00` + 2-byte big-endian absolute) when the gap
///   from the cursor is zero (first entry at index `0`) or larger than 255.
pub fn encode_indexes(sorted_idx: &[u16], res: &mut Vec<u8>) {
    let mut cursor: u16 = 0;
    for i in sorted_idx {
        encode_idx(*i, &mut cursor, res);
    }
}

/// Reads one delta-encoded index from the front of `buf`
/// Advances `*buf` past the consumed bytes (1 byte for a small delta, 3
/// for an escape) and updates `*cursor` to the resolved absolute index,
/// which is also returned.
pub fn decode_indexes(buf: &mut &[u8], cursor: &mut u16) -> Result<u16, CodecError> {
    if buf.is_empty() {
        return Err(CodecError::Short);
    }

    let initial_byte = buf[0];
    if initial_byte == 0x00 {
        if buf.len() < 3 {
            return Err(CodecError::Short);
        }
        *cursor = u16::from_be_bytes([buf[1], buf[2]]);
        *buf = &buf[3..];
    } else {
        *cursor = cursor.wrapping_add(buf[0] as u16);
        *buf = &buf[1..];
    }
    Ok(*cursor)
}

pub fn read_u8(buf: &mut &[u8]) -> Result<u8, CodecError> {
    if buf.is_empty() {
        return Err(CodecError::Short);
    }
    let v = buf[0];
    *buf = &buf[1..];
    Ok(v)
}

pub fn read_u16_be(buf: &mut &[u8]) -> Result<u16, CodecError> {
    if buf.len() < 2 {
        return Err(CodecError::Short);
    }
    let v = u16::from_be_bytes([buf[0], buf[1]]);
    *buf = &buf[2..];
    Ok(v)
}

pub fn read_u32_be(buf: &mut &[u8]) -> Result<u32, CodecError> {
    if buf.len() < 4 {
        return Err(CodecError::Short);
    }
    let v = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
    *buf = &buf[4..];
    Ok(v)
}

pub fn read_u64_be(buf: &mut &[u8]) -> Result<u64, CodecError> {
    if buf.len() < 8 {
        return Err(CodecError::Short);
    }
    let v = u64::from_be_bytes([
        buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
    ]);
    *buf = &buf[8..];
    Ok(v)
}

pub fn read_array<const N: usize>(buf: &mut &[u8]) -> Result<[u8; N], CodecError> {
    if buf.len() < N {
        return Err(CodecError::Short);
    }
    let mut out = [0u8; N];
    out.copy_from_slice(&buf[..N]);
    *buf = &buf[N..];
    Ok(out)
}

pub fn unpack_hop_byte(byte: u8) -> (u8, bool) {
    let hop_count = byte & 0b0011_1111; // mask bits 0..=5
    let local_only = (byte & 0b1000_0000) != 0;
    // bit 6 is masked off intentionally as mentioned in the spec
    (hop_count, local_only)
}
