// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The v2-native inspector

use crate::{
    router::proto,
    router_v2::{index::Space, RouterV2State},
};

impl RouterV2State {
    /// §8.2: what this node currently is.
    pub fn rpc_send_v2_status(&self, state: &crate::QaulState, request_id: String) {
        let (manifest_version, manifest_entries, is_gateway) = {
            let manifest = self.manifest.read().unwrap();
            (
                manifest.manifest_version,
                manifest.entries().len() as u32,
                manifest.is_gateway,
            )
        };

        let (user_entries, node_entries) = {
            let rt = self.routing_table.read().unwrap();
            (
                rt.user_entries.iter().filter(|s| s.is_some()).count() as u32,
                rt.node_entries.iter().filter(|s| s.is_some()).count() as u32,
            )
        };

        let (pending_user_intros, pending_node_intros) = {
            let tracker = self.reintroduction_tracker.read().unwrap();
            (
                tracker.pending_len(Space::User) as u32,
                tracker.pending_len(Space::Node) as u32,
            )
        };

        let status = proto::RouterV2Status {
            node_id: self.host_mk.to_id().to_vec(),
            is_node_form: *self.propagation_form.read().unwrap()
                == crate::router_v2::PropagationForm::Node,
            is_gateway,
            seq_num: self.seq_num.read().unwrap().value() as u32,
            manifest_version,
            manifest_entries,
            manifest_log_base: self.own_manifest_log.read().unwrap().log_base,
            neighbours: self.mirrors.read().unwrap().len() as u32,
            user_entries,
            node_entries,
            user_dict_size: self.user_dict.read().unwrap().forward_dir.len() as u32,
            node_dict_size: self.node_dict.read().unwrap().forward_dir.len() as u32,
            pending_user_intros,
            pending_node_intros,
            profile_fetches_in_flight: self.management_in_flight.read().unwrap().len() as u32,
            manifest_requests_outstanding: self.outstanding_manifest_requests.read().unwrap().len()
                as u32,
        };

        self.send_router_rpc(
            state,
            request_id,
            proto::router::Message::RouterV2Status(status),
        );
    }

    /// §8.2: the routing table as v2 actually holds it, both index spaces.
    pub fn rpc_send_v2_table(&self, state: &crate::QaulState, request_id: String, now_ms: u64) {
        // Snapshot both dictionaries before touching the routing table.
        // `retire_expired` takes the dictionary before the table, so reading
        // them in the opposite order would invert the lock ordering.
        let user_dict = self.user_dict.read().unwrap().forward_dir.clone();
        let node_dict = self.node_dict.read().unwrap().forward_dir.clone();

        let mut entries = Vec::new();
        {
            let rt = self.routing_table.read().unwrap();
            for (space, slots, dict) in [
                (Space::User, &rt.user_entries, &user_dict),
                (Space::Node, &rt.node_entries, &node_dict),
            ] {
                for (index, slot) in slots.iter().enumerate() {
                    let Some(entry) = slot else { continue };
                    let entry = entry.read().unwrap();

                    entries.push(proto::RouterV2Entry {
                        space: match space {
                            Space::User => proto::IndexSpace::UserSpace as i32,
                            Space::Node => proto::IndexSpace::NodeSpace as i32,
                        },
                        index: index as u32,
                        target_id: dict
                            .get(&(index as u16))
                            .map(|id| id.to_vec())
                            .unwrap_or_default(),
                        seq_num: entry.seq_num.value() as u32,
                        metric: entry.metric as u32,
                        // §7.4: bit 7 is local_only, bit 6 reserved.
                        hop_count: (entry.hop_count & 0b0011_1111) as u32,
                        local_only: entry.local_only,
                        next_hop_index: entry.next_hop as u32,
                        next_hop_id: node_dict
                            .get(&entry.next_hop)
                            .map(|id| id.to_vec())
                            .unwrap_or_default(),
                        transport: entry.transport.as_int(),
                        age_ms: now_ms.saturating_sub(entry.last_update),
                    });
                }
            }
        }

        entries.sort_by_key(|e| (e.space, e.index));

        self.send_router_rpc(
            state,
            request_id,
            proto::router::Message::RouterV2Table(proto::RouterV2Table { entries }),
        );
    }

    /// §8.2: neighbours with their mirror state.
    pub fn rpc_send_v2_neighbours(&self, state: &crate::QaulState, request_id: String) {
        let neighbours = {
            let mirrors = self.mirrors.read().unwrap();
            let mut out: Vec<proto::RouterV2Neighbour> = mirrors
                .iter()
                .map(|(peer, info)| {
                    let mut transports: Vec<i32> =
                        info.transports.iter().map(|t| t.as_int()).collect();
                    transports.sort_unstable();
                    proto::RouterV2Neighbour {
                        peer_id: peer.to_bytes(),
                        node_id: info.node_id.to_vec(),
                        transports,
                        rtt_micros: info.rtt_micros,
                        user_mirror_size: info.users.len() as u32,
                        node_mirror_size: info.nodes.len() as u32,
                        dump_stale_users: info.dump_stale.users.len() as u32,
                        dump_stale_nodes: info.dump_stale.nodes.len() as u32,
                    }
                })
                .collect();
            out.sort_by(|a, b| a.node_id.cmp(&b.node_id));
            out
        };

        self.send_router_rpc(
            state,
            request_id,
            proto::router::Message::RouterV2Neighbours(proto::RouterV2Neighbours { neighbours }),
        );
    }
}
