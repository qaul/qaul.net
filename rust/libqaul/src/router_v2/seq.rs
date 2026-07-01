// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Sequence Numbers and relaeted functionality

/// according to the spec in section 6:
/// A node maintains a single sequence number that applies to all routing
/// entries originated by that node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeqNum(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// determines what to do with the sequencce number
pub enum Acceptance {
    /// keep the current seq num
    NoChange,
    /// the received seq num is fresher than the current one.
    Fresher,
    /// if the diff is greater than 100
    Reboot,
}

impl From<u16> for SeqNum {
    fn from(value: u16) -> Self {
        SeqNum(value)
    }
}

impl SeqNum {
    pub fn new() -> Self {
        SeqNum(rand::random::<u16>())
    }

    /// A sequence number `new` is fresher than `old` if and only if:
    /// (new - old) mod 65536 < 32768
    pub fn is_fresher(&self, new: SeqNum) -> bool {
        let res = new.0.wrapping_sub(self.0);
        res >= 1 && res <= 32_767
    }

    pub fn is_reboot(&self, new: SeqNum) -> bool {
        let res = new.0.wrapping_sub(self.0);
        res > 100
    }

    pub fn acceptance(&self, new: SeqNum) -> Acceptance {
        let res = new.0.wrapping_sub(self.0);
        if res == 0 {
            Acceptance::NoChange
        } else if res >= 1 && res <= 100 {
            Acceptance::Fresher
        } else {
            Acceptance::Reboot
        }
    }
}

pub fn is_fresher_u32(new: u32, old: u32) -> bool {
    let res = new.wrapping_sub(old);
    res >= 1 && res <= 2_147_483_647
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads as `is_fresher(new, old)` in plan terms.
    fn is_fresher(new: u16, old: u16) -> bool {
        SeqNum::from(old).is_fresher(SeqNum::from(new))
    }

    fn is_reboot(new: u16, old: u16) -> bool {
        SeqNum::from(old).is_reboot(SeqNum::from(new))
    }

    fn acceptance(new: u16, old: u16) -> Acceptance {
        SeqNum::from(old).acceptance(SeqNum::from(new))
    }

    /// 0 vs 65_535 is the wrap case: 0.wrapping_sub(65_535) == 1, which
    /// sits inside the fresher half-space.
    #[test]
    fn is_fresher_handles_wrap_from_max_to_zero() {
        assert!(is_fresher(0, 65_535));
    }

    #[test]
    fn is_fresher_normal_increment() {
        assert!(is_fresher(1, 0));
        assert!(is_fresher(100, 99));
    }

    /// A gap of exactly 32_767 sits at the inclusive upper bound of the
    /// fresher half-space — the spec's tie-break favouring `new`.
    #[test]
    fn is_fresher_at_half_space_upper_edge() {
        assert!(is_fresher(32_767, 0));
    }

    /// A gap of 32_768 falls outside the half-space — under wrap the
    /// forward distance equals the backward distance and the spec
    /// resolves this as `new` being behind.
    #[test]
    fn is_fresher_past_half_space_is_not_fresher() {
        assert!(!is_fresher(32_768, 0));
    }

    #[test]
    fn is_fresher_equal_is_not_fresher() {
        for x in [0u16, 1, 100, 32_767, 32_768, 65_534, 65_535] {
            assert!(!is_fresher(x, x), "equal seqs must not be fresher (x={x})");
        }
    }

    /// Going backwards within the half-space must not register as fresher.
    #[test]
    fn is_fresher_strictly_backward_is_not_fresher() {
        assert!(!is_fresher(0, 1));
        assert!(!is_fresher(50, 100));
    }

    #[test]
    fn is_reboot_gap_above_threshold() {
        // 150 - 30 = 120 → over the 100 line
        assert!(is_reboot(150, 30));
    }

    #[test]
    fn is_reboot_gap_at_or_below_threshold_is_not_reboot() {
        // 50 - 30 = 20 (normal fresher)
        assert!(!is_reboot(50, 30));
        // Exactly 100 — strict `>` means not a reboot.
        assert!(!is_reboot(130, 30));
    }

    /// A backward jump (e.g. peer restarted with a random low seq) shows
    /// up as an enormous forward distance under wrap, which is exactly
    /// the §6.3 signal for "the origin restarted."
    #[test]
    fn is_reboot_backward_jump_is_huge_forward_distance() {
        // 30.wrapping_sub(200) == 65_366, well above 100
        assert!(is_reboot(30, 200));
    }

    #[test]
    fn is_reboot_equal_seqs_is_not_reboot() {
        assert!(!is_reboot(42, 42));
    }

    #[test]
    fn acceptance_no_change_when_equal() {
        assert_eq!(acceptance(30, 30), Acceptance::NoChange);
        assert_eq!(acceptance(0, 0), Acceptance::NoChange);
        assert_eq!(acceptance(65_535, 65_535), Acceptance::NoChange);
    }

    #[test]
    fn acceptance_fresher_for_small_forward_gap() {
        assert_eq!(acceptance(50, 30), Acceptance::Fresher);
        assert_eq!(acceptance(1, 0), Acceptance::Fresher);
        // Exactly at the threshold
        assert_eq!(acceptance(130, 30), Acceptance::Fresher);
    }

    #[test]
    fn acceptance_reboot_for_gap_above_threshold() {
        // 150 - 30 = 120 → over the line
        assert_eq!(acceptance(150, 30), Acceptance::Reboot);
        // Backward jump under wrap
        assert_eq!(acceptance(30, 200), Acceptance::Reboot);
    }

    /// The bucket boundary: 100 = Fresher, 101 = Reboot. Pins the strict
    /// `>` in the reboot check against accidental `>=` regressions.
    #[test]
    fn acceptance_bucket_boundary_at_100_and_101() {
        assert_eq!(acceptance(100, 0), Acceptance::Fresher);
        assert_eq!(acceptance(101, 0), Acceptance::Reboot);
    }

    /// `new()` returns a uniformly random u16. We can't assert exact
    /// values, but across 100 draws the probability of all-identical is
    /// effectively zero. Catches the failure mode where `new()`
    /// accidentally returns a constant.
    #[test]
    fn seq_num_new_is_not_constant() {
        let first = SeqNum::new();
        let any_different = (0..100).any(|_| SeqNum::new() != first);
        assert!(
            any_different,
            "SeqNum::new() returned the same value 100 times"
        );
    }

    /// Verifies the From impl by exercising every method through it.
    #[test]
    fn from_u16_round_trips_through_methods() {
        let s = SeqNum::from(7);
        assert!(s.is_fresher(SeqNum::from(8)));
        assert!(!s.is_fresher(SeqNum::from(7)));
        assert_eq!(s.acceptance(SeqNum::from(7)), Acceptance::NoChange);
    }
}
