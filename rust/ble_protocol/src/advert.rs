// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul BLE advert tools 
//!
//! qaul ID prefix decoder: 
//! Unlike linux/android on ios we cannot put the qaul id in the adverts manufacturing data. Instead it
//! has to go as the adverts name, which takes a string so we encode as base64. 



use crate::QAUL_ID_ADVERT_BYTES;

/// Marks a qaul advert name, not necessary but we have a spare byte
const NAME_MARKER: char = 'q';
/// base64url characters after the marker: 5 bytes = 40 bits, base64 is 6 bits per character -> 7.
const ENCODED_LEN: usize = 7;

/// Takes a qaul advert name and checks if it contians the qaul ID prefix before decoding it into the ID.
pub fn id_prefix_from_name(name: &str) -> Option<[u8; QAUL_ID_ADVERT_BYTES]> {
    let name = name.strip_prefix(NAME_MARKER)?;
    if name.len() != ENCODED_LEN {
        return None;
    }

    let acc = name.chars().try_fold(0u64, |acc, c| Some((acc << 6) | b64url_value(c)?))?;
    let bytes = (acc >> 2).to_be_bytes();
    bytes[8 - QAUL_ID_ADVERT_BYTES..].try_into().ok()
}


/// The 6-bit value (0-63) of one base64url character, or None if it isn't one.
fn b64url_value(c: char) -> Option<u64> {
    match c {
        'A'..='Z' => Some(c as u64 - 'A' as u64),        // 0-25
        'a'..='z' => Some(c as u64 - 'a' as u64 + 26),   // 26-51
        '0'..='9' => Some(c as u64 - '0' as u64 + 52),   // 52-61
        '-' => Some(62),
        '_' => Some(63),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_real_id_prefix() {
        // d9 70 58 3c 69, worked out by hand in the design notes
        assert_eq!(id_prefix_from_name("q2XBYPGk"), Some([0xd9, 0x70, 0x58, 0x3c, 0x69]));
    }

    #[test]
    fn decodes_both_ends_of_the_alphabet() {
        assert_eq!(id_prefix_from_name("qAAAAAAA"), Some([0x00; 5]));
        // '_' is 63: 42 one bits, 40 after dropping the 2 padding bits
        assert_eq!(id_prefix_from_name("q_______"), Some([0xff; 5]));
    }

    #[test]
    fn rejects_names_that_are_not_ours() {
        assert_eq!(id_prefix_from_name("qd970583c69"), None); // old hex format, too long
        assert_eq!(id_prefix_from_name("q2XBYPG"), None); // too short
        assert_eq!(id_prefix_from_name("x2XBYPGk"), None); // no marker
        assert_eq!(id_prefix_from_name("q2XB/PGk"), None); // standard base64, not base64url
        assert_eq!(id_prefix_from_name(""), None);
    }
}