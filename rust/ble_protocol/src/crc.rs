// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! CRC-32 (IEEE 802.3), as computed by Android's `java.util.zip.CRC32`.
//!
//! Written here to keep the crate dependency free. 

/// Reversed form of the IEEE 802.3 polynomial 0x04C11DB7.
const POLYNOMIAL: u32 = 0xEDB8_8320;

/// Compute the CRC-32 of `data`.
pub(crate) fn crc32(data: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            // If the lowest bit is set, shift and XOR in the polynomial; if not,
            // just shift. `wrapping_neg` turns 1 into all ones and 0 into 0, so
            // this is that `if` without a branch.
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (POLYNOMIAL & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_standard_check_value() {
        // The published check value for CRC-32/IEEE. If this fails, every
        // message we send would be rejected by Android as corrupt.
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn empty_input_is_zero() {
        assert_eq!(crc32(&[]), 0);
    }
}
