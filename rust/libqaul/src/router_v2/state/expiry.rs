// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Route expiry and index release (spec §3.7, §3.8, §7.5).

use std::time::Instant;

use crate::router_v2::{
    index::{Space, RESERVED_INDEX},
    RouterV2State,
};

impl RouterV2State {
    /// per 7.5 and 3.7
    pub fn sweep_expired(&self, now: u64) {
        for space in [Space::User, Space::Node] {
            let expired = self.collect_expired(space, now);
            if !expired.is_empty() {
                self.retire_expired(space, &expired, now);
            }
        }
    }

    /// indices whose entry has not been refreshed inside the
    /// expiry window
    pub(crate) fn collect_expired(&self, space: Space, now: u64) -> Vec<u16> {
        let expiry_ms = self.options.route_expiry_ms;
        let rt = self.routing_table.read().unwrap();
        let entries = match space {
            Space::User => &rt.user_entries,
            Space::Node => &rt.node_entries,
        };

        entries
            .iter()
            .enumerate()
            .filter_map(|(idx, slot)| {
                let entry = slot.as_ref()?;
                let stale = entry.read().unwrap().last_update.saturating_add(expiry_ms) < now;
                stale.then_some(idx as u16)
            })
            .collect()
    }

    /// retire the collected indices in canonical lock order.
    pub(crate) fn retire_expired(&self, space: Space, expired: &[u16], now: u64) {
        let expiry_ms = self.options.route_expiry_ms;
        let (dict_lock, alloc_lock) = match space {
            Space::Node => (&self.node_dict, &self.node_allocator),
            Space::User => (&self.user_dict, &self.users_allocator),
        };

        let mut dict = dict_lock.write().unwrap();
        let mut rt = self.routing_table.write().unwrap();
        let mut allocator = alloc_lock.write().unwrap();
        let mut tracker = self.reintroduction_tracker.write().unwrap();

        for &idx in expired {
            let still_stale = rt
                .get(space, idx)
                .is_some_and(|e| e.read().unwrap().last_update.saturating_add(expiry_ms) < now);
            if !still_stale {
                continue;
            }

            rt.clear(space, idx);
            dict.unbind(idx);

            tracker.clear_mark(space, idx);

            if idx != RESERVED_INDEX {
                allocator.release(idx, Instant::now());
            }
        }
    }

    pub(crate) fn release_index(&self, space: Space, id: &[u8; 8]) -> Option<u16> {
        let (dict_lock, alloc_lock) = match space {
            Space::Node => (&self.node_dict, &self.node_allocator),
            Space::User => (&self.user_dict, &self.users_allocator),
        };

        let mut dict = dict_lock.write().unwrap();
        let idx = dict.idx_of(id)?;

        self.routing_table.write().unwrap().clear(space, idx);
        if idx != RESERVED_INDEX {
            alloc_lock.write().unwrap().release(idx, Instant::now());
        }
        dict.unbind(idx);

        self.reintroduction_tracker
            .write()
            .unwrap()
            .clear_mark(space, idx);

        Some(idx)
    }
}
