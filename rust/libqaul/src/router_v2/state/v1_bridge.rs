// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Bridges v2 routing state into the v1 `RoutingTable` that the RPC surface
//! reads.
//! When the final migration is done, this and yhe callers will be deleted.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use prost::Message;

use crate::{
    connections::ConnectionModule,
    router::proto,
    router::table::{RoutingConnectionEntry, RoutingTable as V1RoutingTable, RoutingUserEntry},
    router_v2::{
        index::Space,
        table::{RoutingEntry, User},
        RouterV2State,
    },
    rpc::Rpc,
    utilities::qaul_id::QaulId,
};

impl RouterV2State {
    /// v1's `RoutingTable`, rebuilt from v2 state for the RPC surface
    pub fn create_v1_routing_table(&self, now_ms: u64) -> V1RoutingTable {
        let user_arcs: Vec<Arc<RwLock<User>>> = {
            let users = self.users.read().unwrap();
            users.iter().map(|(_, arc)| arc.clone()).collect()
        };

        let mut table: HashMap<Vec<u8>, RoutingUserEntry> = HashMap::new();

        for arc in user_arcs {
            let user = arc.read().unwrap();

            // No key means no PeerId, and therefore no q8id to file it under.
            let Some(multikey) = user.public_key.as_ref() else {
                continue;
            };
            let q8id = QaulId::to_q8id(multikey.to_peer_id());

            let Some(connection) = self.v1_connection_for(&user, now_ms) else {
                continue;
            };

            table.insert(
                q8id.clone(),
                RoutingUserEntry {
                    id: q8id,
                    // §7 has no propagation-id concept; these four fields are
                    // v1 bookkeeping with no v2 meaning.
                    pgid: 0,
                    pgid_update: now_ms,
                    pgid_update_hc: connection.hc,
                    online_time: connection.last_update,
                    connections: vec![connection],
                },
            );
        }

        V1RoutingTable { table }
    }

    /// The single v1 connection entry for a user, following §9.2's order:
    /// local identity, then direct routing entry, then the lowest-metric
    /// delegation gateway.
    fn v1_connection_for(&self, user: &User, now_ms: u64) -> Option<RoutingConnectionEntry> {
        // §4.1: a user we host is reachable over the Local transport.
        if user.is_hosted {
            return Some(RoutingConnectionEntry {
                module: ConnectionModule::Local,
                node: self.host_mk.to_peer_id(),
                rtt: 0,
                hc: 0,
                lq: 0,
                last_update: now_ms,
            });
        }

        if let Some(weak) = &user.routing_entry {
            if let Some(entry) = weak.upgrade() {
                let entry = entry.read().unwrap();
                if let Some(connection) = self.v1_connection_from_entry(&entry) {
                    return Some(connection);
                }
            }
        }

        // §9.2 step 3, mirroring `next_hop_for_user`.
        let mut best: Option<(u16, RoutingConnectionEntry)> = None;
        for gateway in &user.delegation_gateways {
            let Some(node) = gateway.upgrade() else {
                continue;
            };
            let node_id = node.read().unwrap().id;

            let Some(idx) = self.node_dict.read().unwrap().idx_of(&node_id) else {
                continue;
            };
            let Some(entry) = self.routing_table.read().unwrap().get(Space::Node, idx) else {
                continue;
            };
            let entry = entry.read().unwrap();

            let Some(connection) = self.v1_connection_from_entry(&entry) else {
                continue;
            };
            if best
                .as_ref()
                .is_none_or(|(metric, _)| entry.metric < *metric)
            {
                best = Some((entry.metric, connection));
            }
        }

        best.map(|(_, connection)| connection)
    }

    /// Translates one v2 routing entry into v1's shape.
    fn v1_connection_from_entry(&self, entry: &RoutingEntry) -> Option<RoutingConnectionEntry> {
        let next_hop_node = self.next_hop_node_id(entry.next_hop)?;
        let peer = self.peer_of_node(&next_hop_node)?;

        Some(RoutingConnectionEntry {
            module: entry.transport,
            node: peer,
            rtt: 0,
            // §7.4: bit 7 is local_only and bit 6 is reserved.
            hc: entry.hop_count & 0b0011_1111,
            lq: 0,
            last_update: entry.last_update,
        })
    }

    /// Answers a `NeighboursRequest`
    pub fn rpc_send_neighbours_list(&self, state: &crate::QaulState, request_id: String) {
        let (mut lan, mut internet, mut ble) = (Vec::new(), Vec::new(), Vec::new());

        for (peer, info) in self.mirrors.read().unwrap().iter() {
            // §4.2: one neighbour may be reachable over several transports.
            for transport in &info.transports {
                let entry = proto::NeighboursEntry {
                    node_id: peer.to_bytes(),
                    // v2 measures cost as the §5 metric, not round-trip time.
                    rtt: 0,
                };
                match transport {
                    ConnectionModule::Lan => lan.push(entry),
                    ConnectionModule::Internet => internet.push(entry),
                    ConnectionModule::Ble1m | ConnectionModule::BleCoded => ble.push(entry),
                    ConnectionModule::Local | ConnectionModule::None => {}
                }
            }
        }

        self.send_router_rpc(
            state,
            request_id,
            proto::router::Message::NeighboursList(proto::NeighboursList { lan, internet, ble }),
        );
    }

    /// Answers a `ConnectionsRequest`
    pub fn rpc_send_connections_list(
        &self,
        state: &crate::QaulState,
        request_id: String,
        now_ms: u64,
    ) {
        let (mut lan, mut internet, mut ble, mut local) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());

        for (q8id, user) in self.create_v1_routing_table(now_ms).table {
            for connection in &user.connections {
                let entry = proto::ConnectionsUserEntry {
                    user_id: q8id.clone(),
                    connections: vec![proto::ConnectionEntry {
                        rtt: connection.rtt,
                        hop_count: connection.hc as u32,
                        via: connection.node.to_bytes(),
                    }],
                };
                match connection.module {
                    ConnectionModule::Lan => lan.push(entry),
                    ConnectionModule::Internet => internet.push(entry),
                    ConnectionModule::Ble1m | ConnectionModule::BleCoded => ble.push(entry),
                    ConnectionModule::Local => local.push(entry),
                    ConnectionModule::None => {}
                }
            }
        }

        self.send_router_rpc(
            state,
            request_id,
            proto::router::Message::ConnectionsList(proto::ConnectionsList {
                lan,
                internet,
                ble,
                local,
            }),
        );
    }

    fn send_router_rpc(
        &self,
        state: &crate::QaulState,
        request_id: String,
        message: proto::router::Message,
    ) {
        let proto_message = proto::Router {
            message: Some(message),
        };
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        Rpc::send_message(
            state,
            buf,
            crate::rpc::proto::Modules::Router.into(),
            request_id,
            Vec::new(),
        );
    }
}
