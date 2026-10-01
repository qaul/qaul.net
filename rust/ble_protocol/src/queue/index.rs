// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Hands out send queue indices (1..=29) to outgoing messages.
//!
//! Policy matches Android's getNextMessageIndex in SendQueue.kt: a counter
//! that hands out 1, 2 ... 29 and wraps back to 1. Busy indices are skipped, only
//! when all 29 are busy is the next one overwritten, as Android does.

use crate::constants::SEND_QUEUE_COUNT;

/// The index handed out by `allocate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Allocated {
    /// The queue index for the new message, 1..=29.
    pub index: u8,
    /// True if all 29 were busy and this overwrote a message still in flight.
    /// The caller must drop that message and report it to libqaul as failed.
    pub evicted: bool,
}

/// Tracks which queue indices are free, oldest freed first.
pub struct QueueIndexAllocator {
    /// The index to try first on the next `allocate`, 1..=29.
    next: u8,
    /// `in_use[i]` is true while index `i` belongs to a message. Slot 0 is
    /// never used: index 0 is reserved for flow control frames.
    in_use: [bool; SEND_QUEUE_COUNT as usize + 1],
}

impl QueueIndexAllocator {
    pub fn new() -> Self {
        Self {
            next: 1,
            in_use: [false; SEND_QUEUE_COUNT as usize + 1],
        }
    }

    /// Take the next index, counting on from the last one and wrapping from 29
    /// back to 1. if all are busy, the next one is overwritten.
    pub fn allocate(&mut self) -> Allocated {
        for _ in 0..SEND_QUEUE_COUNT {
            let candidate = self.advance();
            if !self.in_use[usize::from(candidate)] {
                self.in_use[usize::from(candidate)] = true;
                return Allocated { index: candidate, evicted: false };
            }
        }
        Allocated { index: self.advance(), evicted: true }
    }

    /// Return the counter's current value and move it onwards
    fn advance(&mut self) -> u8 {
        let current = self.next;
        self.next = if current == SEND_QUEUE_COUNT { 1 } else { current + 1 };
        current
    }

    /// Release an index once its message is finished (acknowledged, failed, or abandoned) 
    pub fn release(&mut self, index: u8) -> bool {
        if !self.is_in_use(index){
            return false;  // out of range, index 0, or already released
        }
        self.in_use[usize::from(index)] = false;
        true
    }

    /// Whether `index` currently belongs to a message. Somehting liek a MISSING_CHUNKS or ACK
    /// for an index that is not in use is stale and should be ignored.
    pub fn is_in_use(&self, index: u8) -> bool {
        self.in_use
            .get(usize::from(index))
            .copied()
            .unwrap_or(false)
    }

    /// Number of indices currently free.
    pub fn free_count(&self) -> usize {
        self.in_use[1..].iter().filter(|busy| !**busy).count()
    }
}

impl Default for QueueIndexAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hands_out_every_index_once_then_overwrites() {
        let mut alloc = QueueIndexAllocator::new();
        let seen: Vec<u8> = (0..SEND_QUEUE_COUNT).map(|_| alloc.allocate().index).collect();
        assert_eq!(seen, (1..=SEND_QUEUE_COUNT).collect::<Vec<_>>());
        assert_eq!(alloc.free_count(), 0);
        // All busy: like Android, overwrite the next one — and say so.
        assert_eq!(alloc.allocate(), Allocated { index: 1, evicted: true });
    }

    #[test]
    fn released_index_can_be_allocated_again() {
        let mut alloc = QueueIndexAllocator::new();
        for _ in 0..SEND_QUEUE_COUNT {
            alloc.allocate();
        }

        assert!(alloc.release(7));
        assert!(!alloc.is_in_use(7));
        assert_eq!(alloc.allocate(), Allocated { index: 7, evicted: false });
        assert!(alloc.is_in_use(7));
    }

    #[test]
    fn index_0_is_never_handed_out_or_accepted() {
        let mut alloc = QueueIndexAllocator::new();
        assert_ne!(alloc.allocate().index, 0);
        assert!(!alloc.release(0));
        assert!(!alloc.is_in_use(0));
    }

    #[test]
    fn out_of_range_release_is_refused() {
        let mut alloc = QueueIndexAllocator::new();
        assert!(!alloc.release(SEND_QUEUE_COUNT + 1));
        assert!(!alloc.release(255));
    }

    #[test]
    fn wraps_from_29_back_to_1_skipping_busy_ones() {
        let mut alloc = QueueIndexAllocator::new();
        let long_running = alloc.allocate().index; // 1, stays in flight
        for expected in 2..=SEND_QUEUE_COUNT {
            let index = alloc.allocate().index;
            assert_eq!(index, expected);
            alloc.release(index); // finished straight away
        }
        // The counter wraps to 1, but 1 is still busy, so 2 comes next.
        assert_eq!(alloc.allocate(), Allocated { index: 2, evicted: false });
        assert!(alloc.is_in_use(long_running));
    }

    #[test]
    fn beep() {
        let mut alloc = QueueIndexAllocator::new();
        let v = alloc.allocate().index;  // take 1

        assert!(alloc.release(v));           // normal release
        assert!(!alloc.release(v));          // a duplicate ACK releases it again , should be refused

        alloc.allocate();   // the one freed longest ago
        alloc.allocate();

        let a = alloc.allocate().index;
        let b = alloc.allocate().index;
        assert_ne!(a, b, "two messages must never share a queue index");
    }
}
