use bitvec::prelude::*;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

const RESERVED_INDEX: u16 = 0x0000;

/// an independent allocator for each of the two index spaces that a node maintains.
pub struct IndexAllocator {
    /// a monotonic cursor over the 16-bit index range, wrapping from 65,535 back to zero.
    cursor: u16,
    /// a 60s period where the index is not used in allocating
    cooldown: VecDeque<(u16, Instant)>,
    /// the actual indexes used by the node
    occupiers: bitvec::vec::BitVec<u8, Lsb0>,
}

impl IndexAllocator {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            cooldown: VecDeque::new(),
            occupiers: BitVec::repeat(false, 65_536),
        }
    }

    fn idx_in_cooldown(&self, idx: u16) -> bool {
        for &(i, _) in &self.cooldown {
            if i == idx {
                return true;
            }
        }
        false
    }

    /// simply drains the vecedequeue
    fn remove_expired_idx(&mut self) {
        let now = Instant::now();
        while let Some(&(_, released_at)) = self.cooldown.front() {
            if now.duration_since(released_at) >= Duration::from_secs(60) {
                self.cooldown.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn allocate(&mut self) -> Option<u16> {
        self.remove_expired_idx();
        // the entire 16 biy space
        for _ in 0..65_536 {
            let idx = self.cursor;
            // advance the cursor, but it'll wrap
            self.cursor = self.cursor.wrapping_add(1);

            if idx == RESERVED_INDEX {
                continue;
            }
            if self.idx_in_cooldown(idx) {
                continue;
            }
            if self.occupiers[idx as usize] {
                continue;
            }

            self.occupiers.set(idx as usize, true);
            return Some(idx);
        }
        None
    }

    pub fn release(&mut self, idx: u16, now: Instant) {
        self.occupiers.set(idx as usize, false);
        self.cooldown.push_back((idx, now));
    }
}
