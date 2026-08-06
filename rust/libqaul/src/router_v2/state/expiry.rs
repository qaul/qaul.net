// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Route expiry and index release (spec §3.7, §3.8, §7.5).

use std::time::Instant;

use crate::router_v2::{
    index::{Space, RESERVED_INDEX},
    RouterV2State,
};

impl RouterV2State {
    /// gets expired indexes
    pub fn sweep_expired(&self, now: u64) {
        let expiry_ms = self.options.route_expiry_ms;
        let mut rt = self.routing_table.write().unwrap();

        {
            let mut users_dict = self.user_dict.write().unwrap();
            let mut allocator = self.users_allocator.write().unwrap();
            let user_entries = &mut rt.user_entries;

            for idx in 0..user_entries.len() {
                // skip empty entries
                let Some(e) = &user_entries[idx] else {
                    continue;
                };
                let expired = {
                    let entry = e.read().unwrap();
                    entry.last_update.saturating_add(expiry_ms) < now
                };
                if expired {
                    user_entries[idx] = None;
                    users_dict.unbind(idx as u16);
                    allocator.release(idx as u16, Instant::now());
                }
            }
        }

        {
            let mut nodes_dict = self.node_dict.write().unwrap();
            let mut allocator = self.node_allocator.write().unwrap();
            let node_entries = &mut rt.node_entries;

            for idx in 0..node_entries.len() {
                // skip empty entries
                let Some(e) = &node_entries[idx] else {
                    continue;
                };
                let expired = {
                    let entry = e.read().unwrap();
                    entry.last_update.saturating_add(expiry_ms) < now
                };
                if expired {
                    node_entries[idx] = None;
                    nodes_dict.unbind(idx as u16);
                    allocator.release(idx as u16, Instant::now());
                }
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
