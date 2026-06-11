use bitvec::prelude::*;
use std::{
    collections::{HashMap, VecDeque},
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

///
#[derive(Debug)]
pub struct IndexDictionary {
    /// forward direction, idx → id. Used by mirror lookups.
    /// and it also answers the question of: "what's the ID at this slot?"
    forward_dir: HashMap<u16, [u8; 8]>,
    /// reverse direction, id → idx.
    /// use this when you have a known id and need its index
    reverse_dir: HashMap<[u8; 8], u16>,
}

impl IndexDictionary {
    /// if an id is passed, bind that id to the RESERVED_INDEX
    pub fn new(self_id: Option<[u8; 8]>) -> Self {
        let mut idx_dict = Self {
            forward_dir: HashMap::new(),
            reverse_dir: HashMap::new(),
        };

        if let Some(id) = self_id {
            idx_dict.bind(RESERVED_INDEX, id);
        }

        idx_dict
    }

    /// Records (idx, id) in both directions. If either side was already bound, the stale entries are
    /// cleaned up before the new binding is inserted.
    pub fn bind(&mut self, idx: u16, id: [u8; 8]) {
        // checck if idx was bound to a different ID, drop that id's reverse entry.
        if let Some(old_id) = self.forward_dir.remove(&idx) {
            self.reverse_dir.remove(&old_id);
        }
        // If id was bound to a different idx, drop that idx's forward entry
        if let Some(old_idx) = self.reverse_dir.remove(&id) {
            self.forward_dir.remove(&old_idx);
        }
        self.forward_dir.insert(idx, id);
        self.reverse_dir.insert(id, idx);
    }

    /// Removes the binding at idx and returns the released ID
    pub fn unbind(&mut self, idx: u16) -> Option<[u8; 8]> {
        let id = self.forward_dir.remove(&idx)?;
        self.reverse_dir.remove(&id);
        Some(id)
        // let id = self.forward_dir.remove(&idx);
        // match id {
        //     Some(i) => {
        //         self.reverse_dir.remove(&i);
        //         id
        //     }
        //     None => None,
        // }
    }

    /// Returns the index bound to id
    pub fn idx_of(&self, id: &[u8; 8]) -> Option<u16> {
        self.reverse_dir.get(id).copied()
    }

    /// Returns the ID bound to idx
    pub fn id_of(&self, idx: u16) -> Option<[u8; 8]> {
        self.forward_dir.get(&idx).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_allocator_starts_empty() {
        let alloc = IndexAllocator::new();
        assert_eq!(alloc.cursor, 0);
        assert!(alloc.cooldown.is_empty());
        assert_eq!(alloc.occupiers.count_ones(), 0);
        assert_eq!(alloc.occupiers.len(), 65_536);
    }

    #[test]
    fn allocate_returns_distinct_indexes() {
        let mut alloc = IndexAllocator::new();
        let mut seen = HashSet::new();
        for _ in 0..1_000 {
            let idx = alloc.allocate().expect("allocate succeeds");
            assert!(seen.insert(idx), "allocator returned duplicate idx: {idx}");
        }
    }

    #[test]
    fn allocate_never_returns_reserved_index() {
        let mut alloc = IndexAllocator::new();
        for _ in 0..1_000 {
            let idx = alloc.allocate().expect("allocate succeeds");
            assert_ne!(idx, RESERVED_INDEX, "reserved index must not be returned");
        }
    }

    #[test]
    fn allocate_skips_occupied_slots() {
        let mut alloc = IndexAllocator::new();
        // Pre-mark slot 5 as occupied.
        alloc.occupiers.set(5, true);

        // Cursor starts at 0; allocate must skip 0 (reserved) and 5
        // (occupied), producing the sequence below.
        let seq: Vec<u16> = (0..6)
            .map(|_| alloc.allocate().expect("allocate succeeds"))
            .collect();
        assert_eq!(seq, vec![1, 2, 3, 4, 6, 7]);
    }

    #[test]
    fn allocate_sets_the_occupancy_bit() {
        let mut alloc = IndexAllocator::new();
        let idx = alloc.allocate().expect("allocate succeeds");
        assert!(
            alloc.occupiers[idx as usize],
            "allocated idx must be marked as occupied"
        );
    }

    #[test]
    fn release_clears_bit_and_adds_to_cooldown() {
        let mut alloc = IndexAllocator::new();
        let idx = alloc.allocate().expect("allocate succeeds");
        assert!(alloc.occupiers[idx as usize]);

        alloc.release(idx, Instant::now());

        assert!(
            !alloc.occupiers[idx as usize],
            "released idx must have its occupancy bit cleared"
        );
        assert!(
            alloc.idx_in_cooldown(idx),
            "released idx must be in cooldown"
        );
    }

    #[test]
    fn cooldown_blocks_reuse_while_active() {
        let mut alloc = IndexAllocator::new();
        let idx = alloc.allocate().expect("allocate succeeds");
        alloc.release(idx, Instant::now());

        // Force every other slot to be occupied so the just-released
        // idx is the only otherwise-eligible candidate. With it in
        // cooldown, the allocator must give up and return None.
        alloc.occupiers.fill(true);
        alloc.occupiers.set(idx as usize, false);

        assert_eq!(
            alloc.allocate(),
            None,
            "idx in cooldown must not be reallocated even if it is the only free slot"
        );
    }

    #[test]
    fn drain_restores_eligibility_after_cooldown_elapses() {
        let mut alloc = IndexAllocator::new();

        // Inject an already-expired entry. checked_sub is used in case
        // the test machine's monotonic clock has been running for less
        // than 120 s (rare in CI; impossible on real dev systems).
        let past = Instant::now()
            .checked_sub(Duration::from_secs(120))
            .expect("test host's monotonic clock must be > 2 min old");
        alloc.cooldown.push_back((42, past));

        // Force-occupy every slot except 42 so the allocator can only
        // pick 42 if drain successfully removed the expired entry.
        alloc.occupiers.fill(true);
        alloc.occupiers.set(42, false);

        assert_eq!(
            alloc.allocate(),
            Some(42),
            "expired cooldown entry must be drained and the slot reused"
        );
    }

    #[test]
    fn drain_keeps_non_expired_entries() {
        let mut alloc = IndexAllocator::new();
        let recent = Instant::now();
        alloc.cooldown.push_back((42, recent));

        alloc.remove_expired_idx();

        assert!(
            alloc.idx_in_cooldown(42),
            "recent cooldown entry must not be drained"
        );
        assert_eq!(alloc.cooldown.len(), 1);
    }

    #[test]
    fn drain_pops_expired_but_stops_at_first_fresh() {
        let mut alloc = IndexAllocator::new();
        let past = Instant::now()
            .checked_sub(Duration::from_secs(120))
            .expect("test host's monotonic clock must be > 2 min old");
        let recent = Instant::now();
        alloc.cooldown.push_back((1, past)); // expired
        alloc.cooldown.push_back((2, past)); // expired
        alloc.cooldown.push_back((3, recent)); // fresh

        alloc.remove_expired_idx();

        assert!(!alloc.idx_in_cooldown(1));
        assert!(!alloc.idx_in_cooldown(2));
        assert!(alloc.idx_in_cooldown(3));
        assert_eq!(alloc.cooldown.len(), 1);
    }

    #[test]
    fn exhaustion_returns_none() {
        let mut alloc = IndexAllocator::new();
        // Every slot occupied; allocate must return None after sweeping
        // the full 16-bit space without finding an eligible candidate.
        alloc.occupiers.fill(true);
        assert_eq!(alloc.allocate(), None);
    }

    #[test]
    fn dict_round_trips_both_directions() {
        let mut d = IndexDictionary::new(None);
        let id: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

        d.bind(42, id);

        assert_eq!(d.id_of(42), Some(id), "forward lookup returns the bound id");
        assert_eq!(d.idx_of(&id), Some(42), "reverse lookup returns the bound idx");
    }

    #[test]
    fn dict_self_id_bound_at_reserved_index() {
        let id: [u8; 8] = [9; 8];
        let d = IndexDictionary::new(Some(id));

        assert_eq!(d.id_of(RESERVED_INDEX), Some(id));
        assert_eq!(d.idx_of(&id), Some(RESERVED_INDEX));
    }

    #[test]
    fn dict_no_self_id_leaves_reserved_unbound() {
        let d = IndexDictionary::new(None);
        assert_eq!(d.id_of(RESERVED_INDEX), None);
    }

    #[test]
    fn dict_unbound_lookups_return_none() {
        let d = IndexDictionary::new(None);
        let unbound_id: [u8; 8] = [7; 8];

        assert_eq!(d.id_of(42), None);
        assert_eq!(d.idx_of(&unbound_id), None);
    }

    #[test]
    fn dict_unbind_clears_both_directions_and_returns_released_id() {
        let mut d = IndexDictionary::new(None);
        let id: [u8; 8] = [4; 8];
        d.bind(42, id);

        let released = d.unbind(42);

        assert_eq!(released, Some(id), "unbind returns the released id");
        assert_eq!(d.id_of(42), None, "forward direction cleared");
        assert_eq!(d.idx_of(&id), None, "reverse direction cleared");
    }

    #[test]
    fn dict_unbind_of_unbound_idx_returns_none() {
        let mut d = IndexDictionary::new(None);
        assert_eq!(d.unbind(42), None);
    }

    /// Regression: rebinding the same idx to a new id must drop the old
    /// id's reverse entry. Without the cleanup, idx_of(&old) returns Some
    /// even though id_of(idx) now reports the new id.
    #[test]
    fn dict_rebind_same_idx_cleans_up_old_reverse_entry() {
        let mut d = IndexDictionary::new(None);
        let id_x: [u8; 8] = [1; 8];
        let id_y: [u8; 8] = [2; 8];

        d.bind(42, id_x);
        d.bind(42, id_y);

        assert_eq!(d.id_of(42), Some(id_y), "forward reflects the new id");
        assert_eq!(d.idx_of(&id_y), Some(42), "reverse reflects the new id");
        assert_eq!(d.idx_of(&id_x), None, "stale reverse entry for old id was cleaned up");
    }

    /// Regression: binding an already-bound id at a new idx must drop
    /// the id's old forward entry. Mirrors the previous test but in the
    /// opposite direction.
    #[test]
    fn dict_rebind_same_id_cleans_up_old_forward_entry() {
        let mut d = IndexDictionary::new(None);
        let id: [u8; 8] = [3; 8];

        d.bind(42, id);
        d.bind(99, id);

        assert_eq!(d.idx_of(&id), Some(99), "reverse reflects the new idx");
        assert_eq!(d.id_of(99), Some(id), "forward reflects the new idx");
        assert_eq!(d.id_of(42), None, "stale forward entry for old idx was cleaned up");
    }
}
